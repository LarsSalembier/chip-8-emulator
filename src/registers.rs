pub const NUM_REGISTERS: usize = 16;

pub struct Registers {
    values: [u8; NUM_REGISTERS],
}

impl Registers {
    pub fn new() -> Registers {
        Registers {
            values: [0; NUM_REGISTERS],
        }
    }

    pub fn read(&self, index: usize) -> u8 {
        self.values[index]
    }

    pub fn read_multiple(&self, start_index: usize, end_index: usize) -> &[u8] {
        &self.values[start_index..end_index]
    }

    pub fn write(&mut self, index: usize, value: u8) {
        self.values[index] = value;
    }

    pub fn write_multiple(&mut self, start_index: usize, values: &[u8]) {
        self.values[start_index..start_index + values.len()].copy_from_slice(values);
    }

    pub fn copy(&mut self, index1: usize, index2: usize) {
        self.values[index1] = self.values[index2];
    }

    pub fn add_with_overflow(&mut self, index1: usize, index2: usize) {
        let (result, overflow) = self.values[index1].overflowing_add(self.values[index2]);
        self.values[index1] = result;
        self.values[0xF] = overflow as u8;
    }

    pub fn add_byte(&mut self, index: usize, value: u8) {
        self.values[index] = self.values[index].wrapping_add(value);
    }

    pub fn subtract_with_overflow(&mut self, index1: usize, index2: usize) {
        let (result, overflow) = self.values[index1].overflowing_sub(self.values[index2]);
        self.values[index1] = result;
        self.values[0xF] = (!overflow) as u8;
    }

    pub fn subtract_with_overflow_reversed(&mut self, index1: usize, index2: usize) {
        let (result, overflow) = self.values[index2].overflowing_sub(self.values[index1]);
        self.values[index1] = result;
        self.values[0xF] = (!overflow) as u8;
    }

    pub fn shift_right(&mut self, index: usize) {
        self.values[0xF] = self.values[index] & 0x1;
        self.values[index] >>= 1;
    }

    pub fn shift_left(&mut self, index: usize) {
        self.values[0xF] = self.values[index] >> 7;
        self.values[index] <<= 1;
    }

    pub fn or(&mut self, index1: usize, index2: usize) {
        self.values[index1] |= self.values[index2];
    }

    pub fn and(&mut self, index1: usize, index2: usize) {
        self.values[index1] &= self.values[index2];
    }

    pub fn xor(&mut self, index1: usize, index2: usize) {
        self.values[index1] ^= self.values[index2];
    }
}
