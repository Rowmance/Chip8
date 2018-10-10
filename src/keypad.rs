extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;

#[derive(Default, Debug)]
pub struct Keypad {
    /// The state of the 16 keys.
    ///
    /// These have the following layout:
    /// ```
    ///   1 2 3 C
    ///   4 5 6 D
    ///   7 8 9 E
    ///   A 0 B F
    /// ```
    /// The value is true if the key is pressed.
    pub keys: [bool; 16],
}

impl Keypad {
    pub fn new() -> Self {
        Keypad {
            keys: [false; 16],
        }
    }

    /// Returns true if the given key index is pressed.
    pub fn is_key_pressed(&self, key: u8) -> bool {
        self.keys[key as usize]
    }

    /// Sets the given key index to the given state.
    pub fn set_key(&mut self, key: u8, state: bool) {
        self.keys[key as usize] = state;
    }

    /// Maps the given pressed keyboard-key to an index
    pub fn map_key(&mut self, key: Keycode, state: bool) {
        // TODO
        match key {
            _ => ()
        }
    }
}


// TODO test these
