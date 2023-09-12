use crate::memory::{Memory, MemoryError};
use crate::opcode::Opcode;

pub const NUM_REGISTERS: usize = 16;
pub const STACK_SIZE: usize = 16;
pub const NUM_KEYS: usize = 16;

pub const SCREEN_WIDTH: usize = 64;
pub const SCREEN_HEIGHT: usize = 32;
pub const SCREEN_SIZE: usize = SCREEN_WIDTH * SCREEN_HEIGHT;

pub const PROGRAM_START: u16 = 0x200;

pub struct Cpu {
    opcode: u16,
    memory: Memory,
    registers: [u8; NUM_REGISTERS],
    index: u16,
    program_counter: u16,

    delay_timer: u8,
    sound_timer: u8,

    stack: [u16; STACK_SIZE],
    stack_pointer: u16,

    pub graphics: [u8; SCREEN_SIZE],

    keys: [u8; NUM_KEYS],
}

impl Cpu {
    pub fn new() -> Cpu {
        let chip = Cpu {
            opcode: 0,
            memory: Memory::new(),
            registers: [0; NUM_REGISTERS],
            index: 0,
            program_counter: PROGRAM_START,
            delay_timer: 0,
            sound_timer: 0,
            stack: [0; STACK_SIZE],
            stack_pointer: 0,
            graphics: [0; SCREEN_SIZE],
            keys: [0; NUM_KEYS],
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
                for i in 0..self.graphics.len() {
                    self.graphics[i] = 0;
                }
                self.increment_program_counter(1);
            }
            Opcode::ReturnFromSubroutine => {
                self.stack_pointer -= 1;
                self.program_counter = self.stack[self.stack_pointer as usize];
                self.increment_program_counter(1);
            }
            Opcode::JumpToAddr { addr } => {
                self.program_counter = addr;
            }
            Opcode::CallAddr { addr } => {
                self.stack[self.stack_pointer as usize] = self.program_counter;
                self.stack_pointer += 1;
                self.program_counter = addr;
            }
            Opcode::SkipIfEqual { vx, byte } => {
                self.increment_program_counter(1 + (self.registers[vx as usize] == byte) as u16);
            }
            Opcode::SkipIfNotEqual { vx, byte } => {
                self.increment_program_counter(1 + (self.registers[vx as usize] != byte) as u16);
            }
            Opcode::SkipIfVxEqualVy { vx, vy } => {
                self.increment_program_counter(
                    1 + (self.registers[vx as usize] == self.registers[vy as usize]) as u16,
                );
            }
            Opcode::SetVxToByte { vx, byte } => {
                self.registers[vx as usize] = byte;
                self.increment_program_counter(1);
            }
            Opcode::AddByteToVx { vx, byte } => {
                self.registers[vx as usize] = self.registers[vx as usize].wrapping_add(byte);
                self.increment_program_counter(1);
            }
            Opcode::SetVxToVy { vx, vy } => {
                self.registers[vx as usize] = self.registers[vy as usize];
                self.increment_program_counter(1);
            }
            Opcode::SetVxToVxOrVy { vx, vy } => {
                self.registers[vx as usize] |= self.registers[vy as usize];
                self.increment_program_counter(1);
            }
            Opcode::SetVxToVxAndVy { vx, vy } => {
                self.registers[vx as usize] &= self.registers[vy as usize];
                self.increment_program_counter(1);
            }
            Opcode::SetVxToVxXorVy { vx, vy } => {
                self.registers[vx as usize] ^= self.registers[vy as usize];
                self.increment_program_counter(1);
            }
            Opcode::AddVyToVx { vx, vy } => {
                let (result, overflowed) =
                    self.registers[vx as usize].overflowing_add(self.registers[vy as usize]);
                self.registers[vx as usize] = result;
                self.registers[0xF] = overflowed as u8;
                self.increment_program_counter(1);
            }
            Opcode::SubtractVyFromVx { vx, vy } => {
                let (result, overflowed) =
                    self.registers[vx as usize].overflowing_sub(self.registers[vy as usize]);
                self.registers[vx as usize] = result;
                self.registers[0xF] = (!overflowed) as u8;
                self.increment_program_counter(1);
            }
            Opcode::ShiftVxRight { vx } => {
                self.registers[0xF] = self.registers[vx as usize] & 0x1;
                self.registers[vx as usize] >>= 1;
                self.increment_program_counter(1);
            }
            Opcode::SetVxToVyMinusVx { vx, vy } => {
                let (result, overflowed) =
                    self.registers[vy as usize].overflowing_sub(self.registers[vx as usize]);
                self.registers[vx as usize] = result;
                self.registers[0xF] = (!overflowed) as u8;
                self.increment_program_counter(1);
            }
            Opcode::ShiftVxLeft { vx } => {
                self.registers[0xF] = (self.registers[vx as usize] & 0x80) >> 7;
                self.registers[vx as usize] <<= 1;
                self.increment_program_counter(1);
            }
            Opcode::SkipIfVxNotEqualVy { vx, vy } => {
                self.increment_program_counter(
                    1 + (self.registers[vx as usize] != self.registers[vy as usize]) as u16,
                );
            }
            Opcode::SetIndexToAddr { addr } => {
                self.index = addr;
                self.increment_program_counter(1);
            }
            Opcode::JumpToAddrPlusV0 { addr } => {
                self.program_counter = addr + self.registers[0] as u16;
            }
            Opcode::SetVxToRandAndByte { vx, byte } => {
                self.registers[vx as usize] = rand::random::<u8>() & byte;
                self.increment_program_counter(1);
            }
            Opcode::DrawSprite { vx, vy, n } => {
                let x = self.registers[vx as usize] as usize;
                let y = self.registers[vy as usize] as usize;

                self.registers[0xF] = 0;

                for row in 0..n {
                    let sprite_byte = self.memory.get_byte(self.index + row as u16)?;

                    for col in 0..8 {
                        let sprite_pixel = sprite_byte & (0x80 >> col) != 0;
                        let screen_pixel =
                            &mut self.graphics[(x + col + ((y + row as usize) * 64)) % 2048];

                        if sprite_pixel {
                            if *screen_pixel == 1 {
                                self.registers[0xF] = 1;
                            }

                            *screen_pixel ^= 1;
                        }
                    }
                }

                self.increment_program_counter(1);
            }
            Opcode::SkipIfKeyPressed { vx } => {
                self.increment_program_counter(
                    1 + (self.keys[self.registers[vx as usize] as usize] == 1) as u16,
                );
            }
            Opcode::SkipIfKeyNotPressed { vx } => {
                self.increment_program_counter(
                    1 + (self.keys[self.registers[vx as usize] as usize] != 1) as u16,
                );
            }
            Opcode::SetVxToDelayTimer { vx } => {
                self.registers[vx as usize] = self.delay_timer;
                self.increment_program_counter(1);
            }
            Opcode::WaitForKeyPress { vx } => {
                let key_pressed = self.keys.iter().position(|&x| x == 1);

                self.registers[vx as usize] = match key_pressed {
                    Some(key) => key as u8,
                    None => return Ok(()),
                };

                self.increment_program_counter(1);
            }
            Opcode::SetDelayTimerToVx { vx } => {
                self.delay_timer = self.registers[vx as usize];
                self.increment_program_counter(1);
            }
            Opcode::SetSoundTimerToVx { vx } => {
                self.sound_timer = self.registers[vx as usize];
                self.increment_program_counter(1);
            }
            Opcode::AddVxToIndex { vx } => {
                self.index += self.registers[vx as usize] as u16;
                self.increment_program_counter(1);
            }
            Opcode::SetIndexToSpriteLocation { vx } => {
                self.index = self.registers[vx as usize] as u16 * 5;
                self.increment_program_counter(1);
            }
            Opcode::StoreBCD { vx } => {
                self.memory
                    .store_binary_coded_decimal(self.index, self.registers[vx as usize])?;
                self.increment_program_counter(1);
            }
            Opcode::StoreRegisters { vx } => {
                self.memory
                    .set_bytes(self.index, &self.registers[0..=vx as usize])?;
                self.increment_program_counter(1);
            }
            Opcode::LoadRegisters { vx } => {
                self.registers[0..=vx as usize]
                    .copy_from_slice(&self.memory.get_bytes(self.index, vx as u16)?);
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
