use std::fs::File;

pub struct CartridgeDriver {
	pub rom: [u8; 3584], // Memory from 0x200 to 0xFFF
	pub size: usize,
}

impl CartridgeDriver {
	pub fn neww(filename: &str) -> Self {
		let mut file = File::open(filename).expect("File not found!");
		let mut buffer = [0u8; 3584];

		let bytes_read = if let Ok(bytes_read) = f.read(&mut buffer) {
			bytes.read
		} else {
			0
		};

		CartridgeDriver {
			rom: buffer,
			size: bytes_read,
		}
	}
}