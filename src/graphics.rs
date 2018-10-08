pub const HEIGHT: u32 = 32;
pub const WIDTH: u32 = 64;

pub struct Graphics {
    /// Represents the VRAM.
    ///
    /// This represents a monochrome display of pixels whereby the
    /// top-left corner is position (0, 0), the top-left is (63, 0)
    /// and the bottom right is (63, 31).
    pub memory: [u8; (WIDTH * HEIGHT) as usize],
}

impl Graphics {
    /// Creates a new graphics instance
    pub fn new() -> Self {
        Graphics {
            memory: [0; (WIDTH * HEIGHT) as usize]
        }
    }

    /// Clears the screen and VRAM
    pub fn clear(&mut self) {
        self.memory = [0; (WIDTH * HEIGHT) as usize];
    }

    /// Stores the given bytes into the given location in memory.
    /// Returns true if there's a collision (i.e. if an already set
    /// pixel was unset by a new value).
    pub fn draw(&mut self, x: u8, y: u8, bytes: Vec<u8>) -> bool {
        let mut collision = false;
        for xx in 0..8 {
            for yy in 0..bytes.len() {
                let bit = bytes[yy] << xx as u8;
                let index: usize = WIDTH as usize * (y as usize + yy) + (x as usize + xx as usize);
                let curr = self.memory[index];
                self.memory[index] = bit ^ curr;
                if bit == 1 && curr == 1 {
                    collision = true;
                }
            }
        }
        collision
    }
}
