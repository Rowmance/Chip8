//extern crate sdl2;

use cpu::Cpu;
use graphics;
use graphics::Graphics;
//use sdl2::pixels::Color;
//use sdl2::rect::Point;
//use sdl2::rect::Rect;
//use sdl2::render::Canvas;
//use sdl2::video::Window;
use std::time::Duration;

const SCALE: u32 = 10;

pub struct Display {
//    canvas: Canvas<Window>,
}

impl Display {
    /// Creates a new display instance
    pub fn new() -> Self {
//        let video_subsystem = sdl_context.video().unwrap();
//
//        let window = video_subsystem
//            .window("Chip8", graphics::WIDTH * SCALE, graphics::HEIGHT * SCALE)
//            .position_centered()
//            .opengl()
//            .build()
//            .unwrap();
//
//        let mut canvas = window.into_canvas().build().unwrap();
//        canvas.clear();
//        canvas.present();

        Display {
//            canvas,
        }
    }

    /// Draws the contents of the VRAM onto the canvas.
    pub fn render(&mut self, cpu: &Cpu) {
//        info!("{}", cpu.graphics);
//        for x in 0..graphics::WIDTH {
//            for y in 0..graphics::HEIGHT {
//                let bit = cpu.graphics.memory[y as usize * graphics::WIDTH as usize + x as usize];
//                let color = if bit {
//
//                    Color::RGB(0xFF, 0xFF, 0xFF)
//                } else {
//                    Color::RGB(0x00, 0x00, 0x00)
//                };
//                self.canvas.set_draw_color(color);
//                self.canvas.fill_rect(Rect::new((x * SCALE) as i32, (y * SCALE) as i32, SCALE, SCALE));
//            }
//        }
//        self.canvas.present()
    }
}
