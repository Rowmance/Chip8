use std::fmt::Display;
use std::fmt::Error;
use std::fmt::Formatter;

pub const HEIGHT: u32 = 32;
pub const WIDTH: u32 = 64;

pub struct Gpu {
    /// Represents the VRAM.
    ///
    /// This represents a monochrome display of pixels whereby the
    /// top-left corner is position (0, 0), the top-left is (63, 0)
    /// and the bottom right is (63, 31).
    pub memory: [bool; (WIDTH * HEIGHT) as usize],

    /// True if a draw is pending.
    pub pending_draw: bool,
}

impl Gpu {
    /// Creates a new graphics instance
    pub fn new() -> Self {
        Gpu {
            memory: [false; (WIDTH * HEIGHT) as usize],
            pending_draw: false,
        }
    }

    /// Clears the screen and VRAM
    pub fn clear(&mut self) {
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                self.set_pixel(x as u8, y as u8, false);
            }
        }
        self.pending_draw = true;
    }

    /// Stores the given bytes into the given location in memory.
    /// Returns true if there's a collision (i.e. if an already set
    /// pixel was unset by a new value).
    pub fn draw(&mut self, x: u8, y: u8, bytes: Vec<u8>) -> bool {
        let mut collision = false;
        for yy in 0..bytes.len() {
            for xx in 0..8 {
                let bit = ((bytes[yy] >> xx) & 0b1) != 0;
                collision |= self.toggle_pixel(x + 7 - xx, y + yy as u8, bit);
            }
        }
        collision
    }

    /// Toggles the value of the given pixel, and returns
    /// true if the pixel was already set. Resets the pixel
    /// if it's set twice.
    fn toggle_pixel(&mut self, x: u8, y: u8, state: bool) -> bool {
        let index = (y as usize % HEIGHT as usize) * WIDTH as usize + (x as usize % WIDTH as usize);
        self.pending_draw = self.memory[index] != state;
        self.memory[index] ^= state;
        state && !self.memory[index]
    }

    /// Sets the value of the given pixel, ignoring what's already there.
    fn set_pixel(&mut self, x: u8, y: u8, state: bool) {
        let index = (y as usize % HEIGHT as usize) * WIDTH as usize + (x as usize % WIDTH as usize);
        self.pending_draw = self.memory[index] != state;
        self.memory[index] = state;
    }
}

impl Display for Gpu {
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
