pub const SCREEN_WIDTH: usize = 64;
pub const SCREEN_HEIGHT: usize = 32;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum Pixel {
    On,
    Off,
}

pub struct Screen {
    screen: [[Pixel; 64]; 32], // 64x32 pixels
}

impl Screen {
    pub fn new() -> Screen {
        Screen {
            screen: [[Pixel::Off; 64]; 32],
        }
    }

    pub fn clear(&mut self) {
        self.screen = [[Pixel::Off; 64]; 32];
    }

    pub fn draw(&mut self, x: usize, y: usize, sprite: &[u8]) -> bool {
        let mut collision = false;

        for (i, byte) in sprite.iter().enumerate() {
            for j in 0..8 {
                let pixel = (byte >> (7 - j)) & 0x1;
                let screen_x = (x + j) % SCREEN_WIDTH;
                let screen_y = (y + i) % SCREEN_HEIGHT;

                if pixel == 1 {
                    if self.screen[screen_y][screen_x] == Pixel::On {
                        collision = true;
                    }
                    self.screen[screen_y][screen_x] = match self.screen[screen_y][screen_x] {
                        Pixel::On => Pixel::Off,
                        Pixel::Off => Pixel::On,
                    };
                }
            }
        }

        collision
    }

    pub fn get_pixels(&self) -> Vec<u8> {
        self.screen
            .iter()
            .flat_map(|row| {
                row.iter().flat_map(|&pixel| {
                    if pixel == Pixel::On {
                        vec![255, 255, 255, 255]
                    } else {
                        vec![0, 0, 0, 0]
                    }
                })
            })
            .collect()
    }
}
