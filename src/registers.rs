pub const NUM_REGISTERS: usize = 16;

pub struct Registers {
    values: [u8; NUM_REGISTERS],
}

#[derive(Debug)]
pub enum RegisterError {
    IndexOutOfBounds(u8),
}

impl Registers {
    pub fn new() -> Registers {
        Registers {
            values: [0; NUM_REGISTERS],
        }
    }

    fn get_value(&self, index: u8) -> Result<u8, RegisterError> {
        self.values
            .get(index as usize)
            .cloned()
            .ok_or(RegisterError::IndexOutOfBounds(index))
    }

    fn get_value_mut(&mut self, index: u8) -> Result<&mut u8, RegisterError> {
        self.values
            .get_mut(index as usize)
            .ok_or(RegisterError::IndexOutOfBounds(index))
    }

    pub fn read(&self, index: u8) -> Result<u8, RegisterError> {
        self.get_value(index)
    }

    pub fn read_multiple(&self, start_index: u8, end_index: u8) -> Result<&[u8], RegisterError> {
        if start_index > end_index {
            return Err(RegisterError::IndexOutOfBounds(start_index));
        }

        self.values
            .get((start_index as usize)..(end_index as usize))
            .ok_or_else(|| RegisterError::IndexOutOfBounds(std::cmp::max(start_index, end_index)))
    }

    pub fn write(&mut self, index: u8, value: u8) -> Result<(), RegisterError> {
        *self.get_value_mut(index)? = value;

        Ok(())
    }

    pub fn write_multiple(&mut self, start_index: u8, values: &[u8]) -> Result<(), RegisterError> {
        let end_index = start_index as usize + values.len();

        let target_slice = self
            .values
            .get_mut((start_index as usize)..(end_index as usize))
            .ok_or(RegisterError::IndexOutOfBounds(start_index))?;

        target_slice.copy_from_slice(values);

        Ok(())
    }

    pub fn copy(&mut self, index1: u8, index2: u8) -> Result<(), RegisterError> {
        let value = self.get_value(index2)?;

        *self.get_value_mut(index1)? = value;

        Ok(())
    }

    fn handle_overflow_operation<F>(
        &mut self,
        index1: u8,
        index2: u8,
        op: F,
    ) -> Result<(), RegisterError>
    where
        F: Fn(u8, u8) -> (u8, bool),
    {
        let val1 = self.get_value(index1)?;
        let val2 = self.get_value(index2)?;

        let (result, overflow) = op(val1, val2);

        *self.get_value_mut(index1)? = result;
        *self.get_value_mut(0xF)? = overflow as u8;

        Ok(())
    }

    pub fn add_with_overflow(&mut self, index1: u8, index2: u8) -> Result<(), RegisterError> {
        self.handle_overflow_operation(index1, index2, u8::overflowing_add)
    }

    pub fn subtract_with_overflow(&mut self, index1: u8, index2: u8) -> Result<(), RegisterError> {
        self.handle_overflow_operation(index1, index2, |v1, v2| {
            let (result, overflow) = v1.overflowing_sub(v2);
            (result, !overflow)
        })
    }

    pub fn subtract_with_overflow_reversed(
        &mut self,
        index1: u8,
        index2: u8,
    ) -> Result<(), RegisterError> {
        self.handle_overflow_operation(index1, index2, |v1, v2| {
            let (result, overflow) = v2.overflowing_sub(v1);
            (result, !overflow)
        })
    }

    pub fn add_byte(&mut self, index: u8, value: u8) -> Result<(), RegisterError> {
        let target = self.get_value_mut(index)?;

        *target = target.wrapping_add(value);

        Ok(())
    }

    fn handle_bitwise_operation<F>(
        &mut self,
        index1: u8,
        index2: u8,
        op: F,
    ) -> Result<(), RegisterError>
    where
        F: Fn(u8, u8) -> u8,
    {
        let val1 = self.get_value(index1)?;
        let val2 = self.get_value(index2)?;

        *self.get_value_mut(index1)? = op(val1, val2);

        Ok(())
    }

    pub fn or(&mut self, index1: u8, index2: u8) -> Result<(), RegisterError> {
        self.handle_bitwise_operation(index1, index2, |a, b| a | b)
    }

    pub fn and(&mut self, index1: u8, index2: u8) -> Result<(), RegisterError> {
        self.handle_bitwise_operation(index1, index2, |a, b| a & b)
    }

    pub fn xor(&mut self, index1: u8, index2: u8) -> Result<(), RegisterError> {
        self.handle_bitwise_operation(index1, index2, |a, b| a ^ b)
    }

    fn handle_shift_operation<F>(&mut self, index: u8, op: F) -> Result<(), RegisterError>
    where
        F: Fn(u8) -> (u8, u8),
    {
        let value = self.get_value(index)?.clone();
        let (result, vf_value) = op(value);

        *self.get_value_mut(0xF)? = vf_value as u8;

        *self.get_value_mut(index)? = result;

        Ok(())
    }

    pub fn shift_right(&mut self, index: u8) -> Result<(), RegisterError> {
        self.handle_shift_operation(index, |value| {
            let last_bit = value & 0x1;
            let shifted = value >> 1;
            (shifted, last_bit)
        })
    }

    pub fn shift_left(&mut self, index: u8) -> Result<(), RegisterError> {
        self.handle_shift_operation(index, |value| {
            let leading_bit = value >> 7;
            let shifted = value << 1;
            (shifted, leading_bit)
        })
    }
}
