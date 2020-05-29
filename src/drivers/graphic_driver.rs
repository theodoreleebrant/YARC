use sdl2;
use sdl2::pixels;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

use crate::CHIP8_WIDTH;
use crate::CHIP8_HEIGHT;


const SCALE_FACTOR: u32 = 20;
const SCREEN_WIDTH: u32 = (CHIP8_WIDTH as u32) * SCALE_FACTOR;
const SCREEN_HEIGHT: u32 = (CHIP8_HEIGHT as u32) * SCALE_FACTOR;
// Screen is computer screen. This is larger than a chip8 screen

pub struct GraphicDriver { //  graphic driver object with a window in it
    canvas: Canvas<Window>,
}

impl GraphicDriver {
    pub fn new(sdl_context: &sdl2::Sdl) -> Self {
        // We first want to open a window as specified in sdl_context
        // sdl_context is just a sdl that has been initialized
        let video_subsystem = sdl_context.video().unwrap(); // .video() returns a VideoSubsystem, which can initialize a WindowBuilder.
        let window = video_subsystem.window(
                "Rust-chip8-window",
                SCREEN_WIDTH,
                SCREEN_HEIGHT,)
                .position_centered() // returns a &mut WindowBuilder, with centered coordinates
                .opengl() // add opengl flag so that rust can use window
                .build() // returns a Result<Window, WindowBuildError>
                .unwrap(); // Should return 

        let mut canvas = window.into_canvas().build().unwrap();
        
        // Configure canvas
        canvas.set_draw_color(pixels::Color::RGB(0,0,0));
        canvas.clear(); // clear canvas
        canvas.present(); // Bring canvas to front buffer

        GraphicDriver{ canvas: canvas }
    }

    pub fn draw(&mut self, pixels: &[[u8; CHIP8_WIDTH]; CHIP8_HEIGHT]) {
        for (y,row) in pixels.iter().enumerate() {
            for (x,&col) in row.iter().enumerate() {

                let x = (x as u32) * SCALE_FACTOR;
                let y = (y as u32) * SCALE_FACTOR;

                self.canvas.set_draw_color(color(col));
                let _ = self.canvas
                        .fill_rect(Rect::new(x as i32, y as i32, SCALE_FACTOR, SCALE_FACTOR)); // want to fill rectangle (pixel) at that particular address
            }
        }

        self.canvas.present(); // Bring canvas to front 
    }
}
        
fn color(p: u8) -> pixels::Color { // There are only 2 colors: blank or non-blank
    if p == 0 {
        pixels::Color::RGB(0,0,0)
    } else {
        pixels::Color::RGB(0,250,0)
    }
    // Can update this function if we want more color
}
        
        







