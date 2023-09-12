const NUM_KEYS: usize = 16;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Key {
    Pressed,
    Released,
}

pub struct Keyboard {
    keys: [Key; NUM_KEYS],
}

impl Keyboard {
    pub fn new() -> Keyboard {
        Keyboard {
            keys: [Key::Released; 16],
        }
    }

    pub fn _press_key(&mut self, key: u8) {
        self.keys[key as usize] = Key::Pressed;
    }

    pub fn _release_key(&mut self, key: u8) {
        self.keys[key as usize] = Key::Released;
    }

    pub fn is_key_pressed(&self, key: u8) -> bool {
        self.keys[key as usize] == Key::Pressed
    }

    pub fn get_pressed_key(&self) -> Option<u8> {
        for (i, key) in self.keys.iter().enumerate() {
            if *key == Key::Pressed {
                return Some(i as u8);
            }
        }

        None
    }
}
