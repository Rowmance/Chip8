# Chip8 Emulator

Educational project for learning about emulation and practicing Rust.

## Prerequisites

- Rust toolkit, including cargo
- SDL2 installation. On Mac OS, this is as simple as running `brew install sdl2`.

## Building and Running
The project can be built with the following command for debug mode.
```
cargo build
```
Or the following for release mode with optimisations.
```
cargo build --release
```
These commands will put the compiled binary into the respective folder in the `target` directory. Once the project has been built, it can be run with the following command.

```
chip8 [OPTIONS] <ROM>
```

The emulator can also be run without explicitly building with the following.
```
cargo run -- [OPTIONS] <ROM>
```

The help text can be displayed with
```
chip8 --help
```

### Available ROMs

The controls listed here correspond to the Chip8 keypad. See the Keypad section for details on how it maps to your keyboard.

####`pong`
Player vs CPU game of classic Pong. Move up with `1` and down with `4`.

####`bon`
Test ROM which tests the conditional jump, mathematical and logical operations ofd the CPU. If all tests are successful, 'BON' is displayed.

####`walk`
Demo ROM which randomly walks along the display and inverts pixel values.

####`particle`
Demo ROM which shoots random particles from the bottom centre of the display.

####`puzzle`
Traditional puzzle game where the player aims to reverse the keypad by sliding values into the empty space. To move a tile into the empty space, press the keypad button which corresponds to its current location.

####`space-invaders`
Well known arcade game where the aim is to destroy all the aliens which move towards the bottom of the display. Shoot with `5`, move with `4` and `6`. Press `5` to begin a new game.

####`keypad-test`
Test ROM which highlights the button which has been pressed on the display.

### Options
The following options are available. All of them are optional and have sensible default values which mimic the original Chip8 implementations.

####`KEYMAP`
Specifies the keypad mapping to use. See the Keypad section below for details on what each option corresponds to.

##### Usage
- `-k <KEYMAP>`
- `--keymap <KEYMAP>`
##### Possible Values
- `qwerty` (default)
- `dvorak`

####`MULTIPLIER`
The CPU clock multiplier to use. By default, the CPU runs at 500Hz. A value of `1` will set this. A value of `2` will set it to 1000Hz, and a value of `0.1` will set this to 50Hz.

##### Usage
- `-m <MULTIPLIER>`
- `--multiplier <MULTIPLIER>`
##### Possible Values
- Any positive float (default `1.0`)

####`SCALE`
The scale of the display. Each pixel on the Chip8 display will be represented by a square this many pixels in height/width on your display. The Chip8 display is 64x32 pixels. A value of `10` will mean a 10x10 pixel square represents each pixel on the Chip8 display.

##### Usage
- `-s <SCALE>`
- `--scale <SCALE>`
##### Possible Values
- Any positive integer (default `10`)

### Keypad
The Chip8 keypad takes the following form:
```
 1 2 3 C
 4 5 6 D
 7 8 9 E
 A 0 B F
``` 
This corresponds to the following keys on a QWERTY keyboard:
```
1 2 3 4
Q W E R
A S D F
Z X C V
```
And the following on a Dvorak keyboard:
```
1 2 3 4
' , . p
a o e u
; q j k
```