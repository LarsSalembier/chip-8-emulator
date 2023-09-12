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
        for i in 0..CHIP8_FONTSET.len() {
            self.data[i] = CHIP8_FONTSET[i];
        }
    }

    pub fn get_byte(&self, addr: u16) -> Result<u8, MemoryError> {
        if addr as usize >= MEMORY_SIZE {
            return Err(MemoryError::OutOfBounds(addr));
        }

        Ok(self.data[addr as usize])
    }

    pub fn get_bytes(&self, addr: u16, n: u16) -> Result<Vec<u8>, MemoryError> {
        if addr as usize + n as usize >= MEMORY_SIZE {
            return Err(MemoryError::OutOfBounds(addr));
        }

        let mut bytes = Vec::new();

        for i in 0..=n {
            bytes.push(self.get_byte(addr + i)?);
        }

        Ok(bytes)
    }

    pub fn set_byte(&mut self, addr: u16, byte: u8) -> Result<(), MemoryError> {
        if addr as usize >= MEMORY_SIZE {
            return Err(MemoryError::OutOfBounds(addr));
        }

        self.data[addr as usize] = byte;

        Ok(())
    }

    pub fn set_bytes(&mut self, addr: u16, bytes: &[u8]) -> Result<(), MemoryError> {
        if addr as usize + bytes.len() >= MEMORY_SIZE {
            return Err(MemoryError::OutOfBounds(addr));
        }

        for (i, byte) in bytes.iter().enumerate() {
            self.set_byte(addr + i as u16, *byte)?;
        }

        Ok(())
    }

    pub fn store_binary_coded_decimal(
        &mut self,
        addr: u16,
        decimal: u8,
    ) -> Result<(), MemoryError> {
        if addr as usize + 2 >= MEMORY_SIZE {
            return Err(MemoryError::OutOfBounds(addr));
        }

        let hundreds = decimal / 100;
        let tens = (decimal % 100) / 10;
        let ones = decimal % 10;

        self.set_byte(addr, hundreds)?;
        self.set_byte(addr + 1, tens)?;
        self.set_byte(addr + 2, ones)?;

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
        let rom = std::fs::read(filename).expect("Failed to read ROM file");

        self.load_rom(&rom)?;

        Ok(())
    }
}
