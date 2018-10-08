extern crate rand;
extern crate sdl2;

pub mod cpu;
pub mod graphics;
pub mod keypad;
pub mod io;

use io::Display;
use graphics::Graphics;

fn main() {
    println!("Hello, world!");
    let graphics = Graphics::new();
    let display = Display::new(graphics);

//    display.render();
}
