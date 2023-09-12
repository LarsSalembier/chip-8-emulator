pub trait Timer {
    fn get_value(&self) -> u8;
    fn set_value(&mut self, value: u8);
    fn update(&mut self);
}

pub struct DelayTimer {
    value: u8,
}

impl DelayTimer {
    pub fn new() -> DelayTimer {
        DelayTimer { value: 0 }
    }
}

impl Timer for DelayTimer {
    fn get_value(&self) -> u8 {
        self.value
    }

    fn set_value(&mut self, value: u8) {
        self.value = value;
    }

    fn update(&mut self) {
        if self.value > 0 {
            self.value -= 1;
        }
    }
}

pub struct SoundTimer {
    value: u8,
}

impl SoundTimer {
    pub fn new() -> SoundTimer {
        SoundTimer { value: 0 }
    }
}

impl Timer for SoundTimer {
    fn get_value(&self) -> u8 {
        self.value
    }

    fn set_value(&mut self, value: u8) {
        self.value = value;
    }

    fn update(&mut self) {
        if self.value > 0 {
            if self.value == 1 {
                println!("BEEP!");
            }
            self.value -= 1;
        }
    }
}
