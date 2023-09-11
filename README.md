# Chip-8 Emulator in Rust

![GitHub](https://img.shields.io/github/license/larssalembier/chip-8-emulator)
![GitHub last commit](https://img.shields.io/github/last-commit/larssalembier/chip-8-emulator)

This is a Chip-8 emulator written in Rust that allows you to play classic Chip-8 games on your computer. Chip-8 is an interpreted programming language designed for early microcomputers.

## Features

- [x] Full Chip-8 instruction set support.
- [x] Emulation of classic Chip-8 hardware.
- [x] Interactive graphical user interface using SDL2.
- [x] Open-source and customizable.

## Usage

1. **Clone the Repository:**

   ```bash
   git clone https://github.com/larssalembier/chip-8-emulator.git
2. **Build the Emulator:**

   Navigate to the project directory and build the emulator using Cargo:

   ```bash
   cd chip-8-emulator
   cargo build --release
   ```

3. **Run a Chip-8 ROM:**

   Run the emulator with a Chip-8 ROM:

   ```bash
   cargo run --release -- roms/your-rom.ch8
   ```

   Replace `roms/your-rom.ch8` with the path to the Chip-8 ROM you want to play.

4. **Controls:**

   The emulator uses the following keyboard mapping for Chip-8 keys:

   ```
   1 2 3 4
   Q W E R
   A S D F
   Z X C V
   ```

5. **Quit the emulator:**

   To exit the emulator, simply close the window or press `Ctrl+C` in the terminal.

## References

- [Chip-8 Technical Reference](http://devernay.free.fr/hacks/chip8/C8TECH10.HTM)

## License

This project is licensed under the [MIT License](/LICENSE). You are free to use, modify, and distribute this code, but you must provide proper attribution to the original author (Lars Salembier) in your derivative works. See the [LICENSE](/LICENSE) file for more details.

## Acknowledgments

Special thanks to the creators of the Chip-8 emulator technical reference for their invaluable documentation, and the Rust and SDL2 communities for their support and resources.

Happy Chip-8 gaming!
