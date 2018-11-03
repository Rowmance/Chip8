use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Error;

pub const HEIGHT: u32 = 32;
pub const WIDTH: u32 = 64;

#[derive(Clone)]
pub struct Graphics {
    /// Represents the VRAM.
    ///
    /// This represents a monochrome display of pixels whereby the
    /// top-left corner is position (0, 0), the top-left is (63, 0)
    /// and the bottom right is (63, 31).
    pub memory: [bool; (WIDTH * HEIGHT) as usize],
}

impl Graphics {
    /// Toggles the value of the given pixel, and returns
    /// true if the pixel was already set. Resets the pixel
    /// if it's set twice.
    pub fn draw_pixel(&mut self, x: u8, y: u8, state: bool) -> bool {
        let index = (y as usize % HEIGHT as usize) * WIDTH as usize + (x as usize % WIDTH as usize);
        self.memory[index] ^= state;
        return state && !self.memory[index];
    }

    /// Sets the value of the given pixel, ignoring what's already there.
    pub fn set_pixel(&mut self, x: u8, y: u8, state: bool) {
        let index = (y as usize % HEIGHT as usize) * WIDTH as usize + (x as usize % WIDTH as usize);
        self.memory[index] = state;
    }
}

impl Display for Graphics {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let mut str = String::new();
        str.push_str("\n");
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let index = y as usize * WIDTH as usize + x as usize;
                let bit = self.memory[index];
                let strbit = if bit { "X" } else { "O" };
                str.push_str(strbit.as_ref());
            }
            str.push_str("\n");
        }
        write!(f, "{}", str)
    }
}

impl Graphics {
    /// Creates a new graphics instance
    pub fn new() -> Self {
        Graphics {
            memory: [false; (WIDTH * HEIGHT) as usize]
        }
    }

    /// Clears the screen and VRAM
    pub fn clear(&mut self) {
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                self.set_pixel(x as u8, y as u8, false);
            }
        }
    }

    /// Stores the given bytes into the given location in memory.
    /// Returns true if there's a collision (i.e. if an already set
    /// pixel was unset by a new value).
    pub fn draw(&mut self, x: u8, y: u8, bytes: Vec<u8>) -> bool {
        let mut collision = false;
        for yy in 0..bytes.len() {
            for xx in 0..8 {
                let bit = ((bytes[yy] >> xx) & 0b1) != 0;
                collision |= self.draw_pixel(x + 7 - xx, y + yy as u8, bit);
            }
        }
        collision
    }
}
