pub const SCREEN_WIDTH: usize = 64;
pub const SCREEN_HEIGHT: usize = 32;
pub const SCREEN_SIZE: usize = SCREEN_WIDTH * SCREEN_HEIGHT;

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
                let x = (x + j) % SCREEN_WIDTH;
                let y = (y + i) % SCREEN_HEIGHT;
                if pixel == 1 {
                    if self.screen[y][x] == Pixel::On {
                        collision = true;
                    }
                    self.screen[y][x] = if (self.screen[y][x] == Pixel::On) ^ true {
                        Pixel::On
                    } else {
                        Pixel::Off
                    };
                }
            }
        }

        collision
    }

    pub fn get_pixels(&self) -> [u8; SCREEN_SIZE] {
        let mut pixels = [0; SCREEN_SIZE];

        for (i, row) in self.screen.iter().enumerate() {
            for (j, pixel) in row.iter().enumerate() {
                pixels[i * SCREEN_WIDTH + j] = match pixel {
                    Pixel::On => 0xFF,
                    Pixel::Off => 0x00,
                };
            }
        }

        pixels
    }
}
