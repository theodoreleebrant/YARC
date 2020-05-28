use std::fs::File;
use std::io::prelude::*;

pub struct CartridgeDriver {
	pub rom: [u8; 3584], // Memory from 0x200 to 0xFFF
	pub size: usize,
}

impl CartridgeDriver {
	pub fn new(filename: &str) -> Self {
		let mut file = File::open(filename).expect("File not found!");
		let mut buffer = [0u8; 3584];

		let bytes_read = if let Ok(bytes_read) = file.read(&mut buffer) {
			bytes_read
		} else {
			0
		};

		CartridgeDriver {
			rom: buffer,
			size: bytes_read,
		}
	}
}
