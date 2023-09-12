pub const STACK_SIZE: usize = 16;

pub struct Stack {
    values: [u16; STACK_SIZE],
    pointer: u16,
}

impl Stack {
    pub fn new() -> Stack {
        Stack {
            values: [0; STACK_SIZE],
            pointer: 0,
        }
    }

    pub fn push(&mut self, value: u16) {
        self.values[self.pointer as usize] = value;
        self.pointer += 1;
    }

    pub fn pop(&mut self) -> u16 {
        self.pointer -= 1;
        self.values[self.pointer as usize]
    }
}
