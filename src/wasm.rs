use wasm_bindgen::prelude::*;
use cpu::Cpu;
use std::sync::{Mutex, MutexGuard};
use graphics::Graphics;

use std::f64;
use wasm_bindgen::JsCast;
use graphics;

use std::panic;

// Need functions for:
// Reset CPU
// Pause, resume, etc
//
const TEST2: [u8; 176] = [
    0x22, 0x80, 0xcc, 0x01, 0x4c, 0x01, 0x12, 0x16, 0xca, 0x3e, 0x6b, 0x02, 0x4a, 0x00, 0x12, 0x02,
    0xa2, 0xa1, 0xda, 0xb1, 0x12, 0x24, 0xcb, 0x1e, 0x6a, 0x02, 0x4b, 0x00, 0x12, 0x02, 0xa2, 0xa1,
    0xda, 0xb1, 0x00, 0x00, 0xcd, 0x03, 0x4d, 0x00, 0x7a, 0xff, 0x4a, 0x01, 0x7a, 0x02, 0x4d, 0x01,
    0x7b, 0xff, 0x4b, 0x01, 0x7b, 0x02, 0x4d, 0x02, 0x7a, 0x01, 0x4a, 0x3e, 0x7a, 0xfe, 0x4d, 0x03,
    0x7b, 0x01, 0x4b, 0x1e, 0x7b, 0xfe, 0xa2, 0xa1, 0xda, 0xb1, 0x3f, 0x01, 0x12, 0x24, 0x60, 0x0f,
    0xe0, 0x9e, 0x12, 0x24, 0x00, 0xe0, 0x12, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x6b, 0x00, 0x22, 0x92, 0x6b, 0x1f, 0x22, 0x92, 0x6a, 0x00, 0x22, 0xa2, 0x6a, 0x3f, 0x22, 0xa2,
    0x00, 0xee, 0x6a, 0x00, 0xa2, 0xa0, 0xda, 0xb1, 0x7a, 0x08, 0x3a, 0x40, 0x12, 0x94, 0x00, 0xee,
    0xff, 0x80, 0x6b, 0x01, 0xa2, 0xa1, 0xda, 0xb1, 0x7b, 0x01, 0x3b, 0x1f, 0x12, 0xa4, 0x00, 0xee
];

const PONG2: [u8; 246] = [
    0x6a, 0x02, 0x6b, 0x0c, 0x6c, 0x3f, 0x6d, 0x0c, 0xa2, 0xea, 0xda, 0xb6, 0xdc, 0xd6, 0x6e, 0x00,
    0x22, 0xd4, 0x66, 0x03, 0x68, 0x02, 0x60, 0x60, 0xf0, 0x15, 0xf0, 0x07, 0x30, 0x00, 0x12, 0x1a,
    0xc7, 0x17, 0x77, 0x08, 0x69, 0xff, 0xa2, 0xf0, 0xd6, 0x71, 0xa2, 0xea, 0xda, 0xb6, 0xdc, 0xd6,
    0x60, 0x01, 0xe0, 0xa1, 0x7b, 0xfe, 0x60, 0x04, 0xe0, 0xa1, 0x7b, 0x02, 0x60, 0x1f, 0x8b, 0x02,
    0xda, 0xb6, 0x8d, 0x70, 0xc0, 0x0a, 0x7d, 0xfe, 0x40, 0x00, 0x7d, 0x02, 0x60, 0x00, 0x60, 0x1f,
    0x8d, 0x02, 0xdc, 0xd6, 0xa2, 0xf0, 0xd6, 0x71, 0x86, 0x84, 0x87, 0x94, 0x60, 0x3f, 0x86, 0x02,
    0x61, 0x1f, 0x87, 0x12, 0x46, 0x02, 0x12, 0x78, 0x46, 0x3f, 0x12, 0x82, 0x47, 0x1f, 0x69, 0xff,
    0x47, 0x00, 0x69, 0x01, 0xd6, 0x71, 0x12, 0x2a, 0x68, 0x02, 0x63, 0x01, 0x80, 0x70, 0x80, 0xb5,
    0x12, 0x8a, 0x68, 0xfe, 0x63, 0x0a, 0x80, 0x70, 0x80, 0xd5, 0x3f, 0x01, 0x12, 0xa2, 0x61, 0x02,
    0x80, 0x15, 0x3f, 0x01, 0x12, 0xba, 0x80, 0x15, 0x3f, 0x01, 0x12, 0xc8, 0x80, 0x15, 0x3f, 0x01,
    0x12, 0xc2, 0x60, 0x20, 0xf0, 0x18, 0x22, 0xd4, 0x8e, 0x34, 0x22, 0xd4, 0x66, 0x3e, 0x33, 0x01,
    0x66, 0x03, 0x68, 0xfe, 0x33, 0x01, 0x68, 0x02, 0x12, 0x16, 0x79, 0xff, 0x49, 0xfe, 0x69, 0xff,
    0x12, 0xc8, 0x79, 0x01, 0x49, 0x02, 0x69, 0x01, 0x60, 0x04, 0xf0, 0x18, 0x76, 0x01, 0x46, 0x40,
    0x76, 0xfe, 0x12, 0x6c, 0xa2, 0xf2, 0xfe, 0x33, 0xf2, 0x65, 0xf1, 0x29, 0x64, 0x14, 0x65, 0x00,
    0xd4, 0x55, 0x74, 0x15, 0xf2, 0x29, 0xd4, 0x55, 0x00, 0xee, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80,
    0x80, 0x00, 0x00, 0x00, 0x00, 0x00
];

const SCALE: u32 = 10;

struct WasmContext {
    pub cpu: Cpu,
    pub previous_graphics: Graphics,
}

impl WasmContext {
    pub fn new() -> Self {
        let cpu = Cpu::new();
        let prev_graphics = cpu.graphics.clone();

        let document = web_sys::window().unwrap().document().unwrap();
        let canvas = document.get_element_by_id("canvas").unwrap();
        let canvas: web_sys::HtmlCanvasElement = canvas
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .map_err(|_| ())
            .unwrap();

        let context = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap();

        context.set_fill_style(&JsValue::from_str("black"));
        context.fill_rect(0.0, 0.0, graphics::WIDTH as f64 * SCALE as f64, graphics::HEIGHT as f64 * SCALE as f64);

        WasmContext {
            cpu,
            previous_graphics: prev_graphics,
        }
    }

    pub fn draw(&mut self) {
        let document = web_sys::window().unwrap().document().unwrap();
        let canvas = document.get_element_by_id("canvas").unwrap();
        let canvas: web_sys::HtmlCanvasElement = canvas
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .map_err(|_| ())
            .unwrap();

        let context = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap();

        for x in 0..graphics::WIDTH {
            for y in 0..graphics::HEIGHT {
                let index = y as usize * graphics::WIDTH as usize + x as usize;
                let bit = self.cpu.graphics.memory[index];
                if bit != self.previous_graphics.memory[index] {
                    context.set_fill_style(&JsValue::from_str(if bit { "white" } else { "black" }));
                    context.fill_rect(x as f64 * SCALE as f64, y as f64 * SCALE as f64, SCALE as f64, SCALE as f64);
                    self.previous_graphics.memory[index] = self.cpu.graphics.memory[index];
                }
            }
        }
    }

    pub fn load_rom(&mut self, rom: &[u8]) {
        self.cpu.load_rom(rom);
    }
}

lazy_static! {
    static ref WASM: Mutex<WasmContext> = {
        Mutex::new(WasmContext::new())
    };
}

pub fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
        console_error_panic_hook::set_once();
}


#[wasm_bindgen]
pub fn main() {
    // Make JS console output Rust panics
    set_panic_hook();
    unsafe {
        let mut wasm = WASM.lock().unwrap();
        wasm.load_rom(&PONG2);
    }
}

#[wasm_bindgen]
pub struct Tick {
    pub op_code: u16,
}

#[wasm_bindgen]
pub fn tick() -> Tick {
    unsafe {
        let mut wasm = WASM.lock().unwrap();
        wasm.cpu.execute_cycle();
//        web_sys::console::log_1(&JsValue::from_f64(wasm.cpu.pc as f64));
        wasm.draw();
        Tick {
            op_code: wasm.cpu.last_opcode
        }
    }
}

#[wasm_bindgen]
pub fn add(a: u32, b: u32) -> u32 {
    a + b
}
