extern crate sdl2;

use graphics::Graphics;
use graphics;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use std::time::Duration;
use sdl2::render::Canvas;
use sdl2::video::Window;

const SCALE: usize = 1;

pub struct Display {
    graphics: Graphics,
    canvas: Canvas<Window>,
}

impl Display {
    /// Creates a new display instance
    pub fn new(graphics: Graphics) -> Self {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem
            .window("Chip8", graphics::WIDTH, graphics::HEIGHT)
            .position_centered()
            .opengl()
            .build()
            .unwrap();

        let mut canvas = window.into_canvas().build().unwrap();
        canvas.present();

        Display {
            graphics,
            canvas,
        }
    }

    /// Draws the contents of the VRAM onto the canvas.
    pub fn render(&mut self) {
        for x in 0..graphics::WIDTH {
            for y in 0..graphics::HEIGHT {
                let bit = self.graphics.memory[y as usize * graphics::WIDTH as usize + x as usize];
                let color = if bit > 0 {
                    Color::RGB(0xFF, 0xFF, 0xFF)
                } else {
                    Color::RGB(0x00, 0x00, 0x00)
                };
                self.canvas.set_draw_color(color);
                self.canvas.draw_point(Point::new(x as i32, y as i32));
            }
        }
        self.canvas.present()
    }
}

