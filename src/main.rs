mod cpu;
mod gpu;
mod io;
mod keypad;
mod roms;

use crate::cpu::Cpu;
use crate::gpu::Gpu;
use crate::io::Display;
use crate::keypad::{Keypad, KeypadSetting};
use clap::{App, Arg};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::error::Error;
use std::time::{Duration, SystemTime};

fn main() {
    env_logger::init();

    const VERSION: &str = env!("CARGO_PKG_VERSION");
    let rom_names = roms::ROMS.iter().map(|rom| rom.name).collect::<Vec<&str>>();

    let matches = App::new("Chip 8 Emulator")
        .version(VERSION)
        .author("Roman Kolacz <roman@kolacz.io>")
        .about("Simple Chip8 Emulator")
        .arg(
            Arg::with_name("rom")
                .display_order(1)
                .value_name("ROM")
                .help("The rom to use")
                .required(true)
                .possible_values(&rom_names),
        )
        .arg(
            Arg::with_name("keymap")
                .help("The keymap to use")
                .default_value("qwerty")
                .value_name("KEYMAP")
                .hidden_short_help(true)
                .takes_value(true)
                .possible_values(&["qwerty", "dvorak"])
                .short("k")
                .long("keymap"),
        )
        .arg(
            Arg::with_name("scale")
                .help("The scale of the display")
                .long("scale")
                .short("s")
                .default_value("10")
                .takes_value(true)
                .validator(|val| validate_int(&val))
                .value_name("SCALE"),
        )
        .arg(
            Arg::with_name("multiplier")
                .help("The clock speed multiplier")
                .short("m")
                .long("multiplier")
                .takes_value(true)
                .validator(|val| validate_float(&val))
                .default_value("1.0")
                .value_name("MULTIPLIER"),
        )
        .get_matches();

    // this verification should be handled by Clap, but just in case...
    let rom_name = matches.value_of("rom").expect("Missing ROM");
    let scale = matches
        .value_of("scale")
        .expect("Missing scale")
        .parse()
        .expect("Invalid scale");
    let speed_multiplier: f32 = matches
        .value_of("multiplier")
        .expect("Missing multiplier")
        .parse()
        .expect("Invalid multiplier");
    let keymap = matches.value_of("keymap").expect("Missing keypad");

    let rom = roms::ROMS
        .iter()
        .find(|x| x.name == rom_name)
        .map(|x| x.data)
        .unwrap(); // impossible to happen

    // initialise dependencies
    let gpu = Gpu::new();
    let keypad = Keypad::new(if keymap == "dvorak" {
        KeypadSetting::DVORAK
    } else {
        KeypadSetting::QWERTY
    });

    // initialise CPU
    let mut cpu = Cpu::new(gpu, keypad);
    cpu.load_rom(rom);

    // Initialise display/SDL2
    let sdl_context = sdl2::init().expect("SDL2 is not installed");
    let mut display = Display::new(&sdl_context, scale);
    let mut events = sdl_context.event_pump().unwrap();

    // CPU should run at 500Hz, so one iteration every 2,000,000ns
    let tick_duration_ns = (2_000_000.0 / speed_multiplier) as u64;

    loop {
        let start_time = SystemTime::now();

        // quit gracefully if quit event is pushed
        for event in events.poll_iter() {
            if let Event::Quit { .. } = event {
                return;
            };
        }

        let keys: Vec<Keycode> = events
            .keyboard_state()
            .pressed_scancodes()
            .filter_map(Keycode::from_scancode)
            .collect();

        // clear keycodes and then set the pressed keys
        cpu.keypad.clear();
        for key in keys {
            cpu.keypad.set_from_keycode(key, true);
        }

        cpu.execute_cycle();

        // draw only if necessary - otherwise framerate suffers
        if cpu.gpu.pending_draw {
            display.render(&mut cpu.gpu);
        }

        // sleep to maintain expected clock frequency
        let duration = start_time.elapsed().unwrap();
        let remaining_ns: i64 = (tick_duration_ns as i64) - (duration.as_nanos() as i64);
        if remaining_ns > 0 {
            std::thread::sleep(Duration::from_nanos(remaining_ns as u64));
        }
    }
}

/// Returns true if the given string is a positive integer.
fn validate_int(value: &str) -> Result<(), String> {
    value
        .parse::<u32>()
        .map_err(|err| err.description().to_owned())
        .and_then(|val| {
            if val > 0 {
                Ok(())
            } else {
                Err(String::from("Value must be > 0"))
            }
        })
}

/// Returns true if the given string is a positive float.
fn validate_float(value: &str) -> Result<(), String> {
    value
        .parse::<f32>()
        .map_err(|err| err.description().to_owned())
        .and_then(|val| {
            if val > 0.0 {
                Ok(())
            } else {
                Err(String::from("Value must be > 0"))
            }
        })
}
