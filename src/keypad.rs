use sdl2::keyboard::Keycode;
use std::mem::discriminant;

/// The keymap to use.
pub enum KeypadSetting {
    /// DVORAK bindings.
    DVORAK,

    /// Qwerty Bindings.
    QWERTY,
}

/// Represents a keypad.
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

    /// The keypad setting
    pub setting: KeypadSetting,
}

impl Keypad {
    pub fn new(setting: KeypadSetting) -> Self {
        Keypad {
            keys: [false; 16],
            setting,
        }
    }

    pub fn clear(&mut self) {
        for i in 0..self.keys.len() {
            self.keys[i] = false;
        }
    }

    /// Returns true if the given key index is pressed.
    pub fn is_key_pressed(&self, key: u8) -> bool {
        self.keys[key as usize]
    }

    /// Maps the given pressed keyboard-key to an index
    pub fn set_from_keycode(&mut self, key: Keycode, state: bool) {
        if discriminant(&self.setting) == discriminant(&KeypadSetting::DVORAK) {
            match key {
                Keycode::Num1 => self.keys[0x1 as usize] = state,
                Keycode::Num2 => self.keys[0x2 as usize] = state,
                Keycode::Num3 => self.keys[0x3 as usize] = state,
                Keycode::Num4 => self.keys[0xC as usize] = state,

                Keycode::Quote => self.keys[0x4 as usize] = state,
                Keycode::Comma => self.keys[0x5 as usize] = state,
                Keycode::Period => self.keys[0x6 as usize] = state,
                Keycode::P => self.keys[0xD as usize] = state,

                Keycode::A => self.keys[0x7 as usize] = state,
                Keycode::O => self.keys[0x8 as usize] = state,
                Keycode::E => self.keys[0x9 as usize] = state,
                Keycode::U => self.keys[0xE as usize] = state,

                Keycode::Semicolon => self.keys[0xA as usize] = state,
                Keycode::Q => self.keys[0x0 as usize] = state,
                Keycode::J => self.keys[0xB as usize] = state,
                Keycode::K => self.keys[0xF as usize] = state,
                _ => (),
            }
        } else {
            match key {
                Keycode::Num1 => self.keys[0x1 as usize] = state,
                Keycode::Num2 => self.keys[0x2 as usize] = state,
                Keycode::Num3 => self.keys[0x3 as usize] = state,
                Keycode::Num4 => self.keys[0xC as usize] = state,

                Keycode::Q => self.keys[0x4 as usize] = state,
                Keycode::W => self.keys[0x5 as usize] = state,
                Keycode::E => self.keys[0x6 as usize] = state,
                Keycode::R => self.keys[0xD as usize] = state,

                Keycode::A => self.keys[0x7 as usize] = state,
                Keycode::S => self.keys[0x8 as usize] = state,
                Keycode::D => self.keys[0x9 as usize] = state,
                Keycode::F => self.keys[0xE as usize] = state,

                Keycode::Z => self.keys[0xA as usize] = state,
                Keycode::X => self.keys[0x0 as usize] = state,
                Keycode::C => self.keys[0xB as usize] = state,
                Keycode::V => self.keys[0xF as usize] = state,
                _ => (),
            }
        }
    }
}
