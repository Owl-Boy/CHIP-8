pub const WIDTH: usize = 64;
pub const HEIGHT:usize = 32;
pub const ON: bool = true;
pub const OFF: bool = false;

pub struct Display {
    pub pixels: [bool; WIDTH * HEIGHT]
}

impl Display {
    pub fn new() -> Display {
        Display { pixels: [OFF; WIDTH * HEIGHT]}
    }

    pub fn cls(&mut self) {
        for mut pixel in self::pixels {
            pixel = OFF;
        }
    }

    pub fn swap_pixel(&mut self, x:usize, y: usize) {
        let mut pixel = self.pixels[x + WIDTH * y];
        pixel = !pixel;
    }

    pub fn draw(&mut self, x: usize, y: usize, sprite: [u8]) {
        for (pos, row) in sprite.iter().enumerate() {
            for i in 0..8 {
                let pixel_state = row >> (7 - i) & 0x01;
                if pixel_state == 1 {
                    let xi = x % WIDTH;
                    let yi = y % HEIGHT;
                    self.swap_pixel(xi, yi);
                }
            }
        }
    }
}

    pub static FONT_SET: [[u8; 5]; 16] = [
        [0xF0, 0x90, 0x90, 0x90, 0xF0], // 0
        [0x20, 0x60, 0x20, 0x20, 0x70], // 1
        [0xF0, 0x10, 0xF0, 0x80, 0xF0], // 2
        [0xF0, 0x10, 0xF0, 0x10, 0xF0], // 3
        [0x90, 0x90, 0xF0, 0x10, 0x10], // 4
        [0xF0, 0x80, 0xF0, 0x10, 0xF0], // 5
        [0xF0, 0x80, 0xF0, 0x90, 0xF0], // 6
        [0xF0, 0x10, 0x20, 0x40, 0x40], // 7
        [0xF0, 0x90, 0xF0, 0x90, 0xF0], // 8
        [0xF0, 0x90, 0xF0, 0x10, 0xF0], // 9
        [0xF0, 0x90, 0xF0, 0x90, 0x90], // A
        [0xE0, 0x90, 0xE0, 0x90, 0xE0], // B
        [0xF0, 0x80, 0x80, 0x80, 0xF0], // C
        [0xE0, 0x90, 0x90, 0x90, 0xE0], // D
        [0xF0, 0x80, 0xF0, 0x80, 0xF0], // E
        [0xF0, 0x80, 0xF0, 0x80, 0x80], // F
    ];
