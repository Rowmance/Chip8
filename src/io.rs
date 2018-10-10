extern crate sdl2;

use graphics::Graphics;
use graphics;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use std::time::Duration;
use sdl2::render::Canvas;
use sdl2::video::Window;
use cpu::Cpu;

const SCALE: u32 = 2;

pub struct Display {
    canvas: Canvas<Window>,
}

impl Display {
    /// Creates a new display instance
    pub fn new() -> Self {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem
            .window("Chip8", graphics::WIDTH * SCALE, graphics::HEIGHT * SCALE)
            .position_centered()
            .opengl()
            .build()
            .unwrap();

        let mut canvas = window.into_canvas().build().unwrap();

        Display {
            canvas,
        }
    }

    /// Draws the contents of the VRAM onto the canvas.
    pub fn render(&mut self, cpu: &Cpu) {
        info!("{}", cpu.graphics);
        for x in 0..graphics::WIDTH {
            for y in 0..graphics::HEIGHT {
                let bit = cpu.graphics.memory[y as usize * graphics::WIDTH as usize + x as usize];
                let color = if bit {
                    Color::RGB(0xFF, 0xFF, 0xFF)
                } else {
                    Color::RGB(0x00, 0x00, 0x00)
                };
                self.canvas.set_draw_color(color);
                for dx in 0..SCALE {
                    for dy in 0..SCALE {
                        let px = x * SCALE + dx;
                        let py = y * SCALE + dy;
                        self.canvas.draw_point(Point::new(px as i32, py as i32));
                    }
                }
            }
        }
        self.canvas.present()
    }
}

