use sdl2;
use sdl2::audio::{AudioDevice, AudioCallback, AudioSpecDesired};

pub struct AudioDriver {
	// CHIP-8 has one audio channel for square waves
	device: AudioDevice<SquareWave>

	// Explanation:
	// AudioDriver has one (channel) of AudioDevice, which outputs a square wave.
	// SquareWave needs to have an AudioCallback method.
	// Every AudioDevice is an open_playback() of an AudioSubsystem, which is obtained by sdl2::Sdl.audio().unwrap()
	// open_playback<>() takes in &self, device, &AudioSpecDesired, get_callback
		// get_callback is a function FnOnce(AudioSpec) -> AudioCallback
		// AudioSpec: freq: i32, format: AudioFormat, channels: u8, silence: u8, samples: u16, size: u32
}

impl AudioDriver {
	pub fn new(sdl_context: &sdl2::Sdl) -> Self {
		let audio_subsystem = sdl_context.audio().unwrap(); 
		// Might want to check; can cause panic. core:;result::Result
		// Gives an AudioSubsystem

		let desired_spec = AudioSpecDesired {
			freq: Some(42000),
			channels: Some(1), // mono audio
			samples: None, //default sample size
		};

		let device = audio_subsystem
			.open_playback(None, &desired_spec, |spec|{
				// Show obtained AudioSpec
				//println!("{:?}", spec);

				//Initialise audio callback
				SquareWave {
					phase_inc: (240 / spec.freq) as f32,  // ???
					phase: 0.0,
					volume: 0.2,
				}
			})
			.unwrap(); //TODO: can cause panic

		AudioDriver {
			device: device
		}

	}

	pub fn start_beep(&self) {
        self.device.resume();
    }
    pub fn stop_beep(&self) {
        self.device.pause();
    }
}

struct SquareWave {
    phase_inc: f32,
    phase: f32,
    volume: f32
}

impl AudioCallback for SquareWave {
    type Channel = f32;

    fn callback(&mut self, out: &mut [f32]) {
        // Generate a square wave
        for x in out.iter_mut() {
            *x = self.volume * if self.phase < 0.5 { 1.0 } else { -1.0 };
            self.phase = (self.phase + self.phase_inc) % 1.0;
        }
    }
}
