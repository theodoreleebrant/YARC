const CHIP8_WIDTH: usize = 64;
const CHIP8_HEIGHT: usize = 32;
const CHIP8_RAM: usize = 4096;

extern crate rand;
extern crate sdl2;

mod drivers; // import all the files we wrote
mod CPU;
mod font;

use std::thread; // for concurrency
use std::time::Duration; // Duration is an enum that supports timing For timing of clock
use std::env; // for input

fn main() {
    let sleep_duration = Duration::from_millis(2);

    let sdl_context = sdl2::init().unwrap();
    // Handle error

    let args: Vec<String> = env::args().collect(); // Command collect all command line arguments into a vector.
    let cartridge_filename = &args[1]; // &args[0] is just program name

    let cartridge_driver = CartridgeDriver::new(cartridge_filename);
    let audio_driver = AudioDriver::new(&sdl_context);
    let mut graphic_driver = GraphicDriver::new(&sdl_context);
    let mut input_driver = InputDriver::new(&sdl_context);
    let mut cpu = CPU::new();

    CPU.load(&cartridge_driver.rom);

    while let Ok(keypad) = input_driver.poll() {

        let output = CPU.tick(keypad);

        if output.vram_changed {
            display_driver.draw(output.vram);
        }

        if output.beep {
            audio_driver.start_beep();
        } else {
            audio_driver.stop_beep();
        }

        thread::sleep(sleep_duration);
    }

}

