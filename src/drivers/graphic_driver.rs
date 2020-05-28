use sdl2;
use sdl2::pixels;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

//use CHIP8_WIDTH; later after CPU been implemented
//use CHIP8_HEIGHT;

const CHIP8_WIDTH: u8 = 64;
const CHIP8_HEIGHT: u8 = 32;

const SCALE_FACTOR: u8 = 20;
const SCREEN_WIDTH: u8 = (CHIP8_WIDTH as u32) * SCALE_FACTOR;
const SCREEN_HEIGHT: u8 = (CHIP8_HEIGHT as u32) * SCALE_FACTOR;
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
                
        
        
        
        







