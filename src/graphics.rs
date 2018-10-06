
const HEIGHT: usize = 32;
const WIDTH: usize = 64;

pub struct Graphics {
    pub memory: [u8; WIDTH * HEIGHT],
}

impl Graphics {
    pub fn clear(&mut self) {
        self.memory = [0; WIDTH * HEIGHT];
    }
}
