const MEMORY_SIZE: usize = 4096;
const CHIP8_FONTSET: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];

#[derive(Debug)]
pub enum MemoryError {
    OutOfBounds(u16),
    IoError(std::io::Error),
}

impl From<std::io::Error> for MemoryError {
    fn from(error: std::io::Error) -> Self {
        MemoryError::IoError(error)
    }
}

pub struct Memory {
    data: [u8; MEMORY_SIZE],
}

impl Memory {
    pub fn new() -> Memory {
        let mut memory = Memory {
            data: [0; MEMORY_SIZE],
        };

        memory.load_fontset();

        memory
    }

    fn load_fontset(&mut self) {
        self.data[..CHIP8_FONTSET.len()].copy_from_slice(&CHIP8_FONTSET);
    }

    pub fn get_byte(&self, address: u16) -> Result<u8, MemoryError> {
        self.data
            .get(address as usize)
            .cloned()
            .ok_or(MemoryError::OutOfBounds(address))
    }

    pub fn get_bytes(&self, address: u16, amount: u16) -> Result<Vec<u8>, MemoryError> {
        if address as usize + amount as usize > MEMORY_SIZE {
            return Err(MemoryError::OutOfBounds(address));
        }

        Ok(self.data[address as usize..address as usize + amount as usize].to_vec())
    }

    pub fn set_byte(&mut self, address: u16, byte: u8) -> Result<(), MemoryError> {
        match self.data.get_mut(address as usize) {
            Some(data_byte) => {
                *data_byte = byte;
                Ok(())
            }
            None => Err(MemoryError::OutOfBounds(address)),
        }
    }

    pub fn set_bytes(&mut self, address: u16, bytes: &[u8]) -> Result<(), MemoryError> {
        if address as usize + bytes.len() > MEMORY_SIZE {
            return Err(MemoryError::OutOfBounds(address));
        }

        for (i, &byte) in bytes.iter().enumerate() {
            self.set_byte(address + i as u16, byte)?;
        }

        Ok(())
    }

    pub fn store_binary_coded_decimal(
        &mut self,
        address: u16,
        decimal: u8,
    ) -> Result<(), MemoryError> {
        let hundreds = decimal / 100;
        let tens = (decimal % 100) / 10;
        let ones = decimal % 10;

        self.set_byte(address, hundreds)?;
        self.set_byte(address + 1, tens)?;
        self.set_byte(address + 2, ones)?;

        Ok(())
    }

    fn load_rom(&mut self, rom: &[u8]) -> Result<(), MemoryError> {
        if rom.len() > MEMORY_SIZE - 0x200 {
            return Err(MemoryError::OutOfBounds(0x200));
        }

        self.set_bytes(0x200, rom)?;

        Ok(())
    }

    pub fn load_rom_from_file(&mut self, filename: &str) -> Result<(), MemoryError> {
        let rom = std::fs::read(filename)?;

        self.load_rom(&rom)?;

        Ok(())
    }
}
