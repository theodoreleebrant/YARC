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
                .unwrap(); // Should return Window

        let mut canvas = window.into_canvas().build().unwrap(); // initialises a CanvasBuilder->builds a graphic renderer with .build() -> unwrap from Result<>

        // Now we have a WindowCanvas => Where we render everything
        // Config our WindowCanvas
        canvas.set_draw_color(pixels::Color::RBG(0,0,0)); // Set color for drawing operations
        canvas.clear(); // clear the rendering canvas with that draw color
        canvas.present();

        GraphicDriver { canvas: canvas }
    }

    pub fn draw(&mut self, pixels: &[[u8; CHIP8_WIDTH]; CHIP8_HEIGHT]) {
        // Draw on canvas, given 2D pixel array
        for (y_coord, row) in pixels.iter.enumerate() {
            for (x_coord, &col) in row.iter().enumerate() {
                let x_coord = (x_coord as u32) * SCALE_FACTOR;
                let y_coord = (y_coord as u32) * SCALE_FACTOR;

                self.canvas.set_draw_color(color(col)); // set colour based on coordinates. Can do this because each pixel will have a value: 0 if blank, 1 if colored
                let _ = self.canvas
                    .fill_rect(Rect::new(x as i32, y as i32, SCALE_FACTOR, SCALE_FACTOR)); // fill pixel (as a rectangle) at that coordinate with specified draw color
            }
        }
        self.canvas.present(); // After rendering everything at the back buffer, update the screen all together.
    }
}

fn color(col: u8) -> pixels::Color {
    if col == 0 {
        pixels::Color::RBG(0,0,0)
    } else {
        pixels::Color::RBG(0,250,0) // non-blank pixels
    }
    // Update this if we want more colors
}


