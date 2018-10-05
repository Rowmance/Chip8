/// Represents the CPU
struct Cpu {
    /// Index register
    pub i: u16,

    /// The program counter
    pub cp: u16,

    /// The memory (4KB).
    ///
    /// `0x000` through to `0x200` is reserved. Most programs start at
    /// `0x200` though some start at `0x600`.
    pub memory: [u8; 4096],

    /// Registers
    pub v: [u8; 16],

    /// The stack
    pub stack: [u16; 16],

    /// The stack pointer.
    pub sp: u8,

    /// The delay timer.
    ///
    /// Counts down one on every cycle.
    pub dt: u8,

    /// The sound timer.
    ///
    /// Counts down one on every cycle and plays a sound whilst >0.
    pub st: u8,

    /// The display
    pub display: Display,

    /// The keypad
    pub keypad: Keypad,
}

// TODO pull these out into separate files
pub const HEIGHT: usize = 32;
pub const WIDTH: usize = 64;

struct Display {
    /// The display pixels
    pub pixels: [u8; WIDTH * HEIGHT]
}

struct Keypad {
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
    pub keys: [bool; 16]
}

/// The font set which needs to be initialized in memory (at `0x000` through to `0x1FF`)
/// and can be referenced in ROMs.
///
/// Each letter is represented by 5 bytes (or 8x5 pixels) of pixels
/// set. For example, the representation for '5' is:
/// ```
///   0xF0, 0x80, 0xF0, 0x10, 0xF0
/// ```
/// The binary representation of these values makes up the letter:
/// ```
/// Hex   Bin        Bin 1s
/// 0xF0  1111 0000  ****
/// 0x80  1000 0000  *
/// 0xF0  1111 0000  ****
/// 0x10  0001 0000     *
/// 0xF0  1111 0000  ****
/// ```
/// The 4 least significant bits are ignored.
pub const FONT_SET: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80  // F
];
