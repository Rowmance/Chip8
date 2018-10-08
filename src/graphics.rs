pub const HEIGHT: u32 = 32;
pub const WIDTH: u32 = 64;

pub struct Graphics {
    pub memory: [u8; (WIDTH * HEIGHT) as usize],
}

impl Graphics {
    pub fn new() -> Self {
        Graphics {
            memory: [0; (WIDTH * HEIGHT) as usize]
        }
    }

    pub fn clear(&mut self) {
        self.memory = [0; (WIDTH * HEIGHT) as usize];
    }

    pub fn draw() {}
}
