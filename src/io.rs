use crate::gpu;
use crate::gpu::Gpu;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

/// Represents the drawn display.
pub struct Display {
    /// The canvas.
    canvas: Canvas<Window>,

    /// The scale.
    scale: u32,
}

impl Display {
    /// Creates a new display instance
    pub fn new(sdl_context: &sdl2::Sdl, scale: u32) -> Self {
        let video_subsystem = sdl_context.video().expect("No SDL video context found");

        let window = video_subsystem
            .window("Chip8", gpu::WIDTH * scale, gpu::HEIGHT * scale)
            .position_centered()
            .opengl()
            .build()
            .expect("Failed to build window");

        let mut canvas = window.into_canvas().build().expect("Failed to build canvas");
        canvas.clear();
        canvas.present();

        Display { canvas, scale }
    }

    /// Draws the contents of the VRAM onto the canvas.
    pub fn render(&mut self, graphics: &mut Gpu) {
        for x in 0..gpu::WIDTH {
            for y in 0..gpu::HEIGHT {
                let bit = graphics.memory[y as usize * gpu::WIDTH as usize + x as usize];
                let color = if bit {
                    Color::RGB(0xFF, 0xFF, 0xFF)
                } else {
                    Color::RGB(0x00, 0x00, 0x00)
                };
                self.canvas.set_draw_color(color);
                self.canvas
                    .fill_rect(Rect::new(
                        (x * self.scale) as i32,
                        (y * self.scale) as i32,
                        self.scale,
                        self.scale,
                    ))
                    .expect("Failed to draw to canvas");
            }
        }
        graphics.pending_draw = false;
        self.canvas.present()
    }
}
