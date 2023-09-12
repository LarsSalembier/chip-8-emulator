use crate::keyboard::Keyboard;
use crate::memory::{Memory, MemoryError};
use crate::opcode::Opcode;
use crate::registers::Registers;
use crate::screen::Screen;

pub const STACK_SIZE: usize = 16;

pub const PROGRAM_START: u16 = 0x200;

pub struct Cpu {
    opcode: u16,
    memory: Memory,
    registers: Registers,
    index: u16,
    program_counter: u16,

    delay_timer: u8,
    sound_timer: u8,

    stack: [u16; STACK_SIZE],
    stack_pointer: u16,

    pub screen: Screen,

    keyboard_state: Keyboard,
}

impl Cpu {
    pub fn new() -> Cpu {
        let chip = Cpu {
            opcode: 0,
            memory: Memory::new(),
            registers: Registers::new(),
            index: 0,
            program_counter: PROGRAM_START,
            delay_timer: 0,
            sound_timer: 0,
            stack: [0; STACK_SIZE],
            stack_pointer: 0,
            screen: Screen::new(),
            keyboard_state: Keyboard::new(),
        };

        chip
    }

    fn increment_program_counter(&mut self, times: u16) {
        self.program_counter += 2 * times;
    }

    fn fetch_opcode(&mut self) -> Result<u16, MemoryError> {
        let byte1 = self.memory.get_byte(self.program_counter)?;
        let byte2 = self.memory.get_byte(self.program_counter + 1)?;

        Ok((byte1 as u16) << 8 | (byte2 as u16))
    }

    pub fn emulate_cycle(&mut self) -> Result<(), MemoryError> {
        self.opcode = self.fetch_opcode()?;

        let decoded_opcode = Opcode::decode(self.opcode).unwrap_or_else(|| {
            panic!("Unknown opcode: 0x{:X}", self.opcode);
        });

        match decoded_opcode {
            Opcode::ClearScreen => {
                self.screen.clear();
                self.increment_program_counter(1);
            }
            Opcode::ReturnFromSubroutine => {
                self.stack_pointer -= 1;
                self.program_counter = self.stack[self.stack_pointer as usize];
                self.increment_program_counter(1);
            }
            Opcode::JumpToAddress { address } => {
                self.program_counter = address;
            }
            Opcode::CallAddress { address } => {
                self.stack[self.stack_pointer as usize] = self.program_counter;
                self.stack_pointer += 1;
                self.program_counter = address;
            }
            Opcode::SkipIfEqual { register, byte } => {
                let x = self.registers.read(register as usize);

                self.increment_program_counter(1 + (x == byte) as u16);
            }
            Opcode::SkipIfNotEqual { register, byte } => {
                let x = self.registers.read(register as usize);

                self.increment_program_counter(1 + (x != byte) as u16);
            }
            Opcode::SkipIfRegistersEqual {
                register1,
                register2,
            } => {
                let x = self.registers.read(register1 as usize);
                let y = self.registers.read(register2 as usize);

                self.increment_program_counter(1 + (x == y) as u16);
            }
            Opcode::SetRegisterToByte { register, byte } => {
                self.registers.write(register as usize, byte);
                self.increment_program_counter(1);
            }
            Opcode::AddByteToRegister { register, byte } => {
                self.registers.add_byte(register as usize, byte);
                self.increment_program_counter(1);
            }
            Opcode::SetRegisterToRegister {
                register1,
                register2,
            } => {
                self.registers.copy(register1 as usize, register2 as usize);
                self.increment_program_counter(1);
            }
            Opcode::SetRegisterToRegisterOrRegister {
                register1,
                register2,
            } => {
                self.registers.or(register1 as usize, register2 as usize);
                self.increment_program_counter(1);
            }
            Opcode::SetRegisterToRegisterAndRegister {
                register1,
                register2,
            } => {
                self.registers.and(register1 as usize, register2 as usize);
                self.increment_program_counter(1);
            }
            Opcode::SetRegisterToRegisterXorRegister {
                register1,
                register2,
            } => {
                self.registers.xor(register1 as usize, register2 as usize);
                self.increment_program_counter(1);
            }
            Opcode::AddRegisterToRegister {
                register1,
                register2,
            } => {
                self.registers
                    .add_with_overflow(register1 as usize, register2 as usize);
                self.increment_program_counter(1);
            }
            Opcode::SubtractRegisterFromRegister {
                register1,
                register2,
            } => {
                self.registers
                    .subtract_with_overflow(register1 as usize, register2 as usize);
                self.increment_program_counter(1);
            }
            Opcode::ShiftRegisterRight { register } => {
                self.registers.shift_right(register as usize);
                self.increment_program_counter(1);
            }
            Opcode::SetRegisterToRegisterMinusRegister {
                register1,
                register2,
            } => {
                self.registers
                    .subtract_with_overflow_reversed(register1 as usize, register2 as usize);
                self.increment_program_counter(1);
            }
            Opcode::ShiftRegisterLeft { register } => {
                self.registers.shift_left(register as usize);
                self.increment_program_counter(1);
            }
            Opcode::SkipIfRegisterNotEqualRegister {
                register1,
                register2,
            } => {
                let x = self.registers.read(register1 as usize);
                let y = self.registers.read(register2 as usize);

                self.increment_program_counter(1 + (x != y) as u16);
            }
            Opcode::SetIndexToAddress { address } => {
                self.index = address;
                self.increment_program_counter(1);
            }
            Opcode::JumpToAddressPlusRegister0 { address } => {
                self.program_counter = address + self.registers.read(0) as u16;
            }
            Opcode::SetRegisterToRandAndByte { register, byte } => {
                self.registers
                    .write(register as usize, rand::random::<u8>() & byte);
                self.increment_program_counter(1);
            }
            Opcode::DrawSprite {
                register1,
                register2,
                size,
            } => {
                let x = self.registers.read(register1 as usize) as usize;
                let y = self.registers.read(register2 as usize) as usize;

                let sprite = self.memory.get_bytes(self.index, size as u16)?;

                self.registers
                    .write(0xF, self.screen.draw(x, y, &sprite) as u8);

                self.increment_program_counter(1);
            }
            Opcode::SkipIfKeyPressed { key } => {
                self.increment_program_counter(
                    1 + (self.keyboard_state.is_key_pressed(key)) as u16,
                );
            }
            Opcode::SkipIfKeyNotPressed { key } => {
                self.increment_program_counter(
                    1 + (!self.keyboard_state.is_key_pressed(key)) as u16,
                );
            }
            Opcode::SetRegisterToDelayTimer { register } => {
                self.registers.write(register as usize, self.delay_timer);
                self.increment_program_counter(1);
            }
            Opcode::WaitForKeyPress { key } => {
                let pressed_key = self.keyboard_state.get_pressed_key();

                if let Some(key_) = pressed_key {
                    self.registers.write(key as usize, key_);
                    self.increment_program_counter(1);
                }
            }
            Opcode::SetDelayTimerToRegister { register } => {
                self.delay_timer = self.registers.read(register as usize);
                self.increment_program_counter(1);
            }
            Opcode::SetSoundTimerToRegister { register } => {
                self.sound_timer = self.registers.read(register as usize);
                self.increment_program_counter(1);
            }
            Opcode::AddRegisterToIndex { register } => {
                self.index += self.registers.read(register as usize) as u16;
                self.increment_program_counter(1);
            }
            Opcode::SetIndexToSpriteLocation { register } => {
                self.index = self.registers.read(register as usize) as u16 * 5;
                self.increment_program_counter(1);
            }
            Opcode::StoreBCD { register } => {
                let decimal = self.registers.read(register as usize);

                self.memory
                    .store_binary_coded_decimal(self.index, decimal)?;
                self.increment_program_counter(1);
            }
            Opcode::StoreRegisters { last_index } => {
                let bytes = self.registers.read_multiple(0, last_index as usize + 1);

                self.memory.set_bytes(self.index, bytes)?;
                self.increment_program_counter(1);
            }
            Opcode::LoadRegisters {
                last_memory_address,
            } => {
                let bytes = self
                    .memory
                    .get_bytes(self.index, last_memory_address as u16 + 1)?;

                self.registers.write_multiple(0, &bytes);
                self.increment_program_counter(1);
            }
        }

        self.update_timers();

        Ok(())
    }

    fn update_timers(&mut self) {
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }

        if self.sound_timer > 0 {
            if self.sound_timer == 1 {
                println!("BEEP!");
            }
            self.sound_timer -= 1;
        }
    }

    pub fn load_rom(&mut self, filename: &str) -> Result<(), MemoryError> {
        self.memory.load_rom_from_file(filename)
    }
}
