//extern crate sdl2;

//use sdl2::event::Event;
//use sdl2::keyboard::Keycode;

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

    pub fn clear(&mut self) {
        for i in 0..0xF {
            self.keys[i] = false;
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
    pub fn set_from_keycode(&mut self,/* key: Keycode*/ state: bool) {
//        match key {
//            Keycode::Num1 => self.keys[0x1 as usize] = state,
//            Keycode::Num2 => self.keys[0x2 as usize] = state,
//            Keycode::Num3 => self.keys[0x3 as usize] = state,
//            Keycode::Num4 => self.keys[0xC as usize] = state,
//
//            Keycode::Quote => self.keys[0x4 as usize] = state,
//            Keycode::Comma => self.keys[0x5 as usize] = state,
//            Keycode::Period => self.keys[0x6 as usize] = state,
//            Keycode::P => self.keys[0xD as usize] = state,
//
//            Keycode::A => self.keys[0x7 as usize] = state,
//            Keycode::O => self.keys[0x8 as usize] = state,
//            Keycode::E => self.keys[0x9 as usize] = state,
//            Keycode::U => self.keys[0xE as usize] = state,
//
//            Keycode::Semicolon => self.keys[0xA as usize] = state,
//            Keycode::Q => self.keys[0x0 as usize] = state,
//            Keycode::J => self.keys[0xB as usize] = state,
//            Keycode::K => self.keys[0xF as usize] = state,
//            _ => ()
//        }
    }
}


// TODO test these
