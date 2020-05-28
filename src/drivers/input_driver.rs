use sdl2;
use sdl2::event::Event; // event: event handling library, Event is an enum of different event types
use sdl2::keyboard::Keycode; // keyboard: input handling lib. Keycode is an enum for different keys

pub struct InputDriver {
    events: sdl2::EventPump,
}

impl InputDriver {
    pub fn new(sdl_context: &sdl2::Sdl) -> Self {
        InputDriver { 
            events: sdl_context.event_pump().unwrap() // event pump: return an Event pump (stream) 
        }
    }

    pub fn poll(&mut self) -> Result<[bool; 16], ()> {
        // Will return a boolean representing the keypad for chip8
        for event in self.events.poll_iter() {
            // handle if there is a quit event
            if let Event::Quit{..} = event {
                return Err(());
            };
        }

        let keys: Vec<Keycode> = self.events
                        .keyboard_state() // returns KeyboardState instance
                        .pressed_scancodes() // returns a PressedScancodeIterator: Iterates through all scancode
                        .filter_map(Keycode::from_scancode)
                        .collect();
        let mut keypad = [false; 16];

        for key in keys {
            let index = match key {
                Keycode::Num1 => Some(0x1),
                Keycode::Num2 => Some(0x2),
                Keycode::Num3 => Some(0x3),
                Keycode::Num4 => Some(0xc),
                Keycode::Q => Some(0x4),
                Keycode::W => Some(0x5),
                Keycode::E => Some(0x6),
                Keycode::R => Some(0xd),
                Keycode::A => Some(0x7),
                Keycode::S => Some(0x8),
                Keycode::D => Some(0x9),
                Keycode::F => Some(0xe),
                Keycode::Z => Some(0xa),
                Keycode::X => Some(0x0),
                Keycode::C => Some(0xb),
                Keycode::V => Some(0xf),
                _ => None,
            };

            if let Some(i) = index {
                keypad[i] = true;
            }
        }
        
        Ok(keypad)
    }
}
