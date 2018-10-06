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
    pub fn is_key_pressed(&self, key: u8) -> bool {
        self.keys[key as usize]
    }
}
