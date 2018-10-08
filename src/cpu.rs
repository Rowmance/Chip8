use graphics::Graphics;
use keypad::Keypad;
use rand::prelude::random;

/// Represents the CPU
struct Cpu {
    /// Index register
    pub i: u16,

    /// The program counter
    pub pc: u16,

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

    /// The graphics/video
    pub graphics: Graphics,

    /// The keypad
    pub keypad: Keypad,
}

impl Cpu {
    /// Executes the given op code.
    ///
    /// All instructions are 2 bytes long
    fn execute_opcode(&mut self, opcode: u16) {
        // split the op-code up to make the matching logic saner
        let nibbles = (
            (opcode & 0xF000) >> 12 as u8,
            (opcode & 0x0F00) >> 8 as u8,
            (opcode & 0x00F0) >> 4 as u8,
            (opcode & 0x000F) as u8
        );

        let x = nibbles.1 as u8;
        let y = nibbles.2 as u8;
        let n = nibbles.3 as u8;
        let kk = (opcode & 0x00FF) as u8;
        let addr = (opcode & 0x0FFF) as u16;

        match nibbles {
            (0x00, 0x00, 0x0E, 0x00) => self.cls(),
            (0x00, 0x00, 0x0E, 0x0E) => self.ret(),
            (0x01, _, _, _) => self.jp_addr(addr),
            (0x02, _, _, _) => self.call(addr),
            (0x03, _, _, _) => self.se_x_kk(x, kk),
            (0x04, _, _, _) => self.sne_x_kk(x, kk),
            (0x05, _, _, 0x00) => self.se_x_y(x, y),
            (0x06, _, _, _) => self.ld_x_kk(x, kk),
            (0x07, _, _, _) => self.add_x_kk(x, kk),
            (0x08, _, _, 0x00) => self.ld_x_y(x, y),
            (0x08, _, _, 0x01) => self.or(x, y),
            (0x08, _, _, 0x02) => self.and(x, y),
            (0x08, _, _, 0x03) => self.xor(x, y),
            (0x08, _, _, 0x04) => self.add_x_y(x, y),
            (0x08, _, _, 0x05) => self.sub_x_y(x, y),
            (0x08, _, _, 0x06) => self.shr(x),
            (0x08, _, _, 0x07) => self.subn(x, y),
            (0x08, _, _, 0x0E) => self.shl(x),
            (0x09, _, _, 0x00) => self.sne_x_y(x, y),
            (0x0A, _, _, _) => self.ld_i_addr(addr),
            (0x0B, _, _, _) => self.jp_v0_addr(addr),
            (0x0C, _, _, _) => self.rnd(x, kk),
            (0x0D, _, _, _) => self.drw(x, y, n),
            (0x0E, _, 0x09, 0x0E) => self.skp(x),
            (0x0E, _, 0x0A, 0x01) => self.sknp(x),
            (0x0F, _, 0x00, 0x07) => self.ld_get_dt(x),
            (0x0F, _, 0x00, 0x0A) => self.ld_x_kk(x, kk),
            (0x0F, _, 0x01, 0x05) => self.ld_set_dt(x),
            (0x0F, _, 0x01, 0x08) => self.ld_set_st(x),
            (0x0F, _, 0x01, 0x0E) => self.add_i_vx(x),
            (0x0F, _, 0x02, 0x09) => self.ld_sprite(x),
            (0x0F, _, 0x03, 0x03) => self.ld_bcd(x),
            (0x0F, _, 0x05, 0x05) => self.ld_set_memory(x),
            (0x0F, _, 0x06, 0x05) => self.ld_get_memory(x),
            (_, _, _, _) => () // noop
        }
    }

    /// Clears the display
    fn cls(&mut self) {
        self.graphics.clear();
        self.pc += 1;
    }

    /// Return from a subroutine
    fn ret(&mut self) {
        self.sp -= 1;
        self.pc = self.stack[self.sp as usize];
    }

    /// Jump to the given address
    fn jp_addr(&mut self, addr: u16) {
        self.pc = addr;
    }

    /// Call subroutine at the given address
    fn call(&mut self, addr: u16) {
        self.stack[self.sp as usize] = self.pc;
        self.sp += 1;
        self.pc = addr;
    }

    /// Skip the next instruction if Vx == kk
    fn se_x_kk(&mut self, x: u8, kk: u8) {
        self.pc += if self.v[x as usize] == kk { 2 } else { 1 }
    }

    /// Skip the next instruction if Vx != kk
    fn sne_x_kk(&mut self, x: u8, kk: u8) {
        self.pc += if self.v[x as usize] != kk { 2 } else { 1 }
    }

    /// Skip the next instruction if Vx == Vy
    fn se_x_y(&mut self, x: u8, y: u8) {
        self.pc += if self.v[x as usize] == self.v[y as usize] { 2 } else { 1 }
    }

    /// Set Vx = kk
    fn ld_x_kk(&mut self, x: u8, kk: u8) {
        self.v[x as usize] = kk;
        self.pc += 1;
    }

    /// Set Vx = Vx + kk
    fn add_x_kk(&mut self, x: u8, kk: u8) {
        self.v[x as usize] += kk;
        self.pc += 1;
    }

    /// Set Vx = Vy
    fn ld_x_y(&mut self, x: u8, y: u8) {
        self.v[x as usize] = self.v[y as usize];
        self.pc += 1;
    }

    /// Set Vx = Vx | Vy
    fn or(&mut self, x: u8, y: u8) {
        self.v[x as usize] |= self.v[y as usize];
        self.pc += 1;
    }

    /// Set Vx = Vx & Vy
    fn and(&mut self, x: u8, y: u8) {
        self.v[x as usize] &= self.v[y as usize];
        self.pc += 1;
    }

    /// Set Vx = Vx ^ Vy
    fn xor(&mut self, x: u8, y: u8) {
        self.v[x as usize] ^= self.v[y as usize];
        self.pc += 1;
    }

    /// Set Vx = Vx + Vy, and set Vf = carry
    /// Only the lowest 8 bits of the result are set to Vx
    fn add_x_y(&mut self, x: u8, y: u8) {
        let vx = self.v[x as usize] as u16;
        let vy = self.v[y as usize] as u16;
        let result = vx + vy;
        self.v[0xF] = if result > 0xFF { 1 } else { 0 };
        self.v[x as usize] = result as u8;
        self.pc += 1;
    }

    /// Set Vx = Vx - Vy, and set Vy = NOT borrow
    /// If Vx > Vy, Vf is set to 1
    fn sub_x_y(&mut self, x: u8, y: u8) {
        self.v[0xF] = if self.v[x as usize] > self.v[y as usize] { 1 } else { 0 };
        self.v[x as usize] -= self.v[x as usize].wrapping_sub(self.v[y as usize]);
        self.pc += 1;
    }

    /// Set Vx = Vx SHR 1
    /// If the least-significant bit of Vx == 1, Vf is set to 1
    fn shr(&mut self, x: u8) {
        self.v[0xF] = self.v[x as usize] & 1;
        self.v[x as usize] >>= 1;
        self.pc += 1;
    }

    /// Set Vx = Vy - Vx, set Vf = NOT borrow
    fn subn(&mut self, x: u8, y: u8) {
        self.v[0xF] = if self.v[y as usize] > self.v[x as usize] { 1 } else { 0 };
        self.v[x as usize] -= self.v[y as usize].wrapping_sub(self.v[x as usize]);
        self.pc += 1;
    }

    /// Set Vx = Vx SHL 1
    fn shl(&mut self, x: u8) {
        self.v[0xF] = (self.v[x as usize] & 0b10000000) >> 7;
        self.v[x as usize] <<= 1;
        self.pc += 1;
    }

    /// Skip the next instruction if Vx != Vy
    fn sne_x_y(&mut self, x: u8, y: u8) {
        self.pc += if self.v[x as usize] != self.v[y as usize] { 2 } else { 1 };
    }

    /// Set I = addr
    fn ld_i_addr(&mut self, addr: u16) {
        self.i = addr;
        self.pc += 1;
    }

    /// Jump to the location V0 + addr
    fn jp_v0_addr(&mut self, addr: u16) {
        self.pc = addr + self.v[0] as u16;
    }

    /// Set Vx = random byte & kk
    fn rnd(&mut self, x: u8, kk: u8) {
        let rand: u8 = random();
        self.v[x as usize] = rand & kk;
        self.pc += 1;
    }

    /// Display n-byte sprite starting at memory location I at (Vx, Vy).
    /// Set Vf = 1 if any pixels were erased.
    fn drw(&mut self, x: u8, y: u8, n: u8) {
        // TODO
        self.pc += 1;
    }

    /// Skip the next instruction if the key with the value Vx is pressed.
    fn skp(&mut self, x: u8) {
        self.pc += if self.keypad.is_key_pressed(self.v[x as usize]) { 2 } else { 1 };
    }

    /// Skip the next instruction if the key with the value Vx is not pressed.
    fn sknp(&mut self, x: u8) {
        self.pc += if self.keypad.is_key_pressed(self.v[x as usize]) { 1 } else { 2 };
    }

    /// Set Vx = delay timer value
    fn ld_get_dt(&mut self, x: u8) {
        self.v[x as usize] = self.dt;
        self.pc += 1;
    }

    /// Wait for a key press, store the value of the key in Vx
    fn ld_key(&mut self, x: u8) {
        if let Some(key) = self.keypad.keys.iter().position(|key| *key) {
            self.pc += 1;
            self.v[x as usize] = key as u8;
        }
        // Spin otherwise, don't increment pc
    }

    /// Set delay timer = Vx
    fn ld_set_dt(&mut self, x: u8) {
        self.dt = self.v[x as usize];
        self.pc += 1;
    }

    /// Set sound timer = Vx;
    fn ld_set_st(&mut self, x: u8) {
        self.st = self.v[x as usize];
        self.pc += 1;
    }

    /// Set I = I + Vx
    fn add_i_vx(&mut self, x: u8) {
        self.i += self.v[x as usize] as u16;
        self.pc += 1;
    }

    /// Set I = location of sprite for digit Vx
    fn ld_sprite(&mut self, x: u8) {
        // TODO
        self.pc += 1;
    }

    /// Store BCD representation of Vx in memory locations I, I+1 and I+2
    fn ld_bcd(&mut self, x: u8) {
        // TODO
        self.pc += 1;
    }

    /// Store registers V0 through Vx to memory starting at location I
    fn ld_set_memory(&mut self, x: u8) {
        // TODO
        self.pc += 1;
    }

    /// Read registers V0 through to Vx from memory starting at location I
    fn ld_get_memory(&mut self, x: u8) {
        // TODO
        self.pc += 1;
    }
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
