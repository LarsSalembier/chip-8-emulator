const NUM_KEYS: usize = 16;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Key {
    Pressed,
    Released,
}

pub struct Keyboard {
    keys: [Key; NUM_KEYS],
}

#[derive(Debug, Clone)]
pub enum KeyboardError {
    KeyOutOfBounds,
}

impl Keyboard {
    pub fn new() -> Keyboard {
        Keyboard {
            keys: [Key::Released; 16],
        }
    }

    pub fn _press_key(&mut self, key: u8) -> Result<(), KeyboardError> {
        match self.keys.get_mut(key as usize) {
            Some(k) => {
                *k = Key::Pressed;
                Ok(())
            }
            None => Err(KeyboardError::KeyOutOfBounds),
        }
    }

    pub fn _release_key(&mut self, key: u8) -> Result<(), KeyboardError> {
        match self.keys.get_mut(key as usize) {
            Some(k) => {
                *k = Key::Released;
                Ok(())
            }
            None => Err(KeyboardError::KeyOutOfBounds),
        }
    }

    pub fn is_key_pressed(&self, key: u8) -> Result<bool, KeyboardError> {
        match self.keys.get(key as usize) {
            Some(&Key::Pressed) => Ok(true),
            Some(_) => Ok(false),
            None => Err(KeyboardError::KeyOutOfBounds),
        }
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
