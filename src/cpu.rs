use crate::keyboard::{Keyboard, KeyboardError};
use crate::memory::{Memory, MemoryError};
use crate::opcode::Opcode;
use crate::registers::Registers;
use crate::screen::Screen;
use crate::stack::Stack;
use crate::timers::{DelayTimer, SoundTimer, Timer};

pub const PROGRAM_START: u16 = 0x200;

pub struct Cpu {
    opcode: u16,
    memory: Memory,
    registers: Registers,
    index: u16,
    program_counter: u16,

    delay_timer: DelayTimer,
    sound_timer: SoundTimer,

    stack: Stack,

    pub screen: Screen,

    keyboard_state: Keyboard,
}

#[derive(Debug)]
pub enum CpuError {
    MemoryError(MemoryError),
    KeyboardError(KeyboardError),
}

impl From<MemoryError> for CpuError {
    fn from(error: MemoryError) -> Self {
        CpuError::MemoryError(error)
    }
}

impl From<KeyboardError> for CpuError {
    fn from(error: KeyboardError) -> Self {
        CpuError::KeyboardError(error)
    }
}

impl Cpu {
    pub fn new() -> Cpu {
        let chip = Cpu {
            opcode: 0,
            memory: Memory::new(),
            registers: Registers::new(),
            index: 0,
            program_counter: PROGRAM_START,
            delay_timer: DelayTimer::new(),
            sound_timer: SoundTimer::new(),
            stack: Stack::new(),
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

    pub fn emulate_cycle(&mut self) -> Result<(), CpuError> {
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
                self.program_counter = self.stack.pop();

                self.increment_program_counter(1);
            }
            Opcode::JumpToAddress { address } => {
                self.program_counter = address;
            }
            Opcode::CallAddress { address } => {
                self.stack.push(self.program_counter);

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
                let is_pressed = self.keyboard_state.is_key_pressed(key)?;

                self.increment_program_counter(1 + is_pressed as u16);
            }
            Opcode::SkipIfKeyNotPressed { key } => {
                let is_pressed = self.keyboard_state.is_key_pressed(key)?;

                self.increment_program_counter(1 + (!is_pressed) as u16);
            }
            Opcode::SetRegisterToDelayTimer { register } => {
                self.registers
                    .write(register as usize, self.delay_timer.get_value());

                self.increment_program_counter(1);
            }
            Opcode::WaitForKeyPress { register } => {
                let pressed_key = self.keyboard_state.get_pressed_key();

                if let Some(key) = pressed_key {
                    self.registers.write(register as usize, key);
                    self.increment_program_counter(1);
                }
            }
            Opcode::SetDelayTimerToRegister { register } => {
                let x = self.registers.read(register as usize);

                self.delay_timer.set_value(x);

                self.increment_program_counter(1);
            }
            Opcode::SetSoundTimerToRegister { register } => {
                let x = self.registers.read(register as usize);

                self.sound_timer.set_value(x);

                self.increment_program_counter(1);
            }
            Opcode::AddRegisterToIndex { register } => {
                let x = self.registers.read(register as usize);

                self.index += x as u16;

                self.increment_program_counter(1);
            }
            Opcode::SetIndexToSpriteLocation { register } => {
                let x = self.registers.read(register as usize);

                self.index = x as u16 * 5;

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

        self.delay_timer.update();
        self.sound_timer.update();

        Ok(())
    }

    pub fn load_rom(&mut self, filename: &str) -> Result<(), MemoryError> {
        self.memory.load_rom_from_file(filename)
    }
}
