extern crate sdl2;

use graphics::Graphics;
use graphics;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use std::time::Duration;

const SCALE: usize = 1;

pub struct Display {
    graphics: Graphics,

}

impl Display {
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
//        canvas.set_draw_color(Color::RGB(255, 0, 0));
//        canvas.draw_point(Point::new(100, 100));
//        canvas.draw_point(Point::new(101, 100));
//        canvas.draw_point(Point::new(102, 100));
//        canvas.draw_point(Point::new(103, 100));

        for x in 0..graphics::WIDTH {
            for y in 0..graphics::HEIGHT {
                canvas.set_draw_color(Color::RGB(0xFF, 0xFF, 0xFF));
                canvas.draw_point(Point::new(x as i32, y as i32));
            }
        }

        canvas.present();

        ::std::thread::sleep(Duration::new(5, 0));

        Display {
            graphics
        }
    }
}

