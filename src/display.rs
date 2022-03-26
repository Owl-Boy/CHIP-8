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

