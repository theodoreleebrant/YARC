use CHIP8_WIDTH;
use CHIP8_HEIGHT;
use CHIP8_RAM;
use font::FONT_SET;

use rand;
use rand::Rng;

pub struct CPU {
	opcode: u16,			// opcodes
	v: [u8; 16],			// registers v0, v1, ..., vF; 8-bit
	i: u16,					// register i; 16-bit for memory address
	sound_timer: u8,		// Sound timer; 8-bit
	delay_timer:u8,			// Delay time: 8-bit
	pc: u16,				// Program counter 
	sp: u8,					// Stack pointer
	ram: [u8; CHIP8_RAM],		// RAM, 4kB

	vram: [[u8; CHIP8_WIDTH]; CHIP8_HEIGHT], // Video RAM
	vram_changed: bool,
	stack: [u16; 16],
	keypad: [bool; 16],
	keypad_waiting: bool,
	keypad_register: u16,
}

pub struct OutputState<'a> {
	vram: &'a [[ud; CHIP8_WIDTH]; CHIP8_HEIGHT], // Check lifetimes
	vram_changed: bool,
	beep: bool,
}

enum ProgramCounter {
	// what to do with pointer
    Stay,
	Next,
	Skip,
	Jump(u32),
}

impl CPU {
	fn new() -> Self {
		let mut ram = [0u8; CHIP8_RAM];

		// Load RAM with font_set
		for i in 0..FONT_SET.len() {
			ram[i] = FONT_SET[i];
		}

		CPU {
	    	opcode: 0,
	    	v: [0; 16],
	    	i: 0x200,
	    	sound_timer: 0,
	    	delay_timer: 0,
	    	pc: 0x200,
	    	sp: 0,
	    	memory: [0; CHIP8_RAM],
	    	vram: [[0; CHIP8_WIDTH]; CHIP8_HEIGHT],
			vram_changed: false,
			stack: [0; 16],
			keypad: [false; 16],
			keypad_waiting: false,
			keypad_register: 0,
		}
	}

	fn load_program(&mut self, program: Vec<u8>) {
		let data = vec![0; 0x200];
		for byte in program {
			data.push(byte);
		}
		for (i, &byte) in data.iter().enumerate() {
			// TODO: Implement a check: address should be less than 0x1000
			self.ram[0x200 + i] = byte;  //programs start at 0x200
		}
	}

	

	fn tick(&mut self, keypad: [bool; 16]) -> OutputState {
		// Initialisation
		self.keypad = keypad;
		self.vram_changed = false;

		// Each tick, either (input from keypad) or (decrement timer & do opcode)
		if self.keypad_waiting {
			for i in 0..keypad.len() {
				if keypad[i] {
					self.keypad_waiting = false;				// Stop the keypad_waiting
					self.v[self.keypad_register] = i as u8;		// Put the keypad entry into register vX
					break;
				}
			}
		} else {
			if self.delay_timer > 0 {
				// If delay timer is not zero, decrement (until zero)
				self.delay_timer -= 1
			}
			if self.sound_timer > 0 {
				// If sound timer is not zero, decrement (until zero)
				self.sound_timer -= 1
			}
			let opcode = self.get_opcode();
			self.run_opcode(opcode);
		}

		OutputState {
			vram: &self.vram,
			vram_changed: self.vram_changed,
			beep: self.sound_timer > 0
		}
	}

	// Gets opcode from RAM; pc points to the opcode
	// Function to merge 2 bytes into u16
	fn get_opcode(&self) -> u16 {
		// cast to u16 as ram[i] is u8
		(self.ram[self.pc] as u16) << 8 | (self.ram[self.pc + 1] as u16)
	}


	fn run_opcode(&mut self, opcode: u16) {
		// Taken from CHIP-8 Documentation:
		// nnn or addr - A 12-bit value, the lowest 12 bits of the instruction -> *nnn
		// n or nibble - A 4-bit value, the lowest 4 bits of the instruction
		// x - A 4-bit value, the lower 4 bits of the high byte of the instruction
		// y - A 4-bit value, the upper 4 bits of the low byte of the instruction
		// -> *xyn
		// kk or byte - An 8-bit value, the lowest 8 bits of the instruction -> **kk

		// Split the opcode into 4 parts of 4 bits
		// u8 is the lowest in Rust
		let parts = (
			(opcode & 0xF000) >> 12 as u8,
			(opcode & 0x0F00) >> 8 as u8,
			(opcode & 0x00F0) >> 4 as u8,
			(opcode & 0x000F) as u8,
		);

		let x = parts.1;
		let y = parts.2;
		let n = parts.3;
		let kk = (parts.2 << 4) | parts.3;
		let nnn = ((parts.1 as u16) << 8) | ((parts.2 as u16) << 4) | (parts.1 as u16);

		let pc_change = match parts {
			(0x00, 0x00, 0x0e, 0x00) => self.op_00e0(),
            (0x00, 0x00, 0x0e, 0x0e) => self.op_00ee(),
            (0x01, _, _, _) => self.op_1nnn(nnn),
            (0x02, _, _, _) => self.op_2nnn(nnn),
            (0x03, _, _, _) => self.op_3xkk(x, kk),
            (0x04, _, _, _) => self.op_4xkk(x, kk),
            (0x05, _, _, 0x00) => self.op_5xy0(x, y),
            (0x06, _, _, _) => self.op_6xkk(x, kk),
            (0x07, _, _, _) => self.op_7xkk(x, kk),
            (0x08, _, _, 0x00) => self.op_8xy0(x, y),
            (0x08, _, _, 0x01) => self.op_8xy1(x, y),
            (0x08, _, _, 0x02) => self.op_8xy2(x, y),
            (0x08, _, _, 0x03) => self.op_8xy3(x, y),
            (0x08, _, _, 0x04) => self.op_8xy4(x, y),
            (0x08, _, _, 0x05) => self.op_8xy5(x, y),
            (0x08, _, _, 0x06) => self.op_8x06(x),
            (0x08, _, _, 0x07) => self.op_8xy7(x, y),
            (0x08, _, _, 0x0e) => self.op_8x0e(x),
            (0x09, _, _, 0x00) => self.op_9xy0(x, y),
            (0x0a, _, _, _) => self.op_annn(nnn),
            (0x0b, _, _, _) => self.op_bnnn(nnn),
            (0x0c, _, _, _) => self.op_cxkk(x, kk),
            (0x0d, _, _, _) => self.op_dxyn(x, y, n),
            (0x0e, _, 0x09, 0x0e) => self.op_ex9e(x),
            (0x0e, _, 0x0a, 0x01) => self.op_exa1(x),
            (0x0f, _, 0x00, 0x07) => self.op_fx07(x),
            (0x0f, _, 0x00, 0x0a) => self.op_fx0a(x),
            (0x0f, _, 0x01, 0x05) => self.op_fx15(x),
            (0x0f, _, 0x01, 0x08) => self.op_fx18(x),
            (0x0f, _, 0x01, 0x0e) => self.op_fx1e(x),
            (0x0f, _, 0x02, 0x09) => self.op_fx29(x),
            (0x0f, _, 0x03, 0x03) => self.op_fx33(x),
            (0x0f, _, 0x05, 0x05) => self.op_fx55(x),
            (0x0f, _, 0x06, 0x05) => self.op_fx65(x),
            _ => ProgramCounter::Next,
		};

		match pc_change {
			// Opcode size: 2. Might want to change to 
			ProgramCounter::Next => self.pc += 2,
			ProgramCounter::Skip => self.pc += 4,
			ProgramCounter::Jump(addr) => self.pc = addr,
            ProgramCounter::Stay => self.pc += 0,
		}
	}

	// OPCODES HERE
	// OOEO: CLS -> Clear display
	fn op_00e0(&mut self) -> ProgramCounter {
		for a in 0..CHIP8_HEIGHT {
			for b in 0..CHIP8_WIDTH {
				self.vram[a][b] = 0;
			}
		}
		self.vram_changed = true;
		ProgramCounter::Next
	}

	// 00EE: RET -> Return from subroutine
	// The interpreter sets the program counter to the address at the top of the stack, then subtracts 1 from the stack pointer.
	fn op_00ee(&mut self) -> ProgramCounter {
		self.sp -= 1;
		ProgramCounter::Jump(self.stack[self.sp])
	}

	// 1nnn; JP addr -> Jump to location nnn
	// The interpreter sets the program counter to nnn.
	fn op_1nnn(&mut self, nnn: u16) -> ProgramCounter {
		ProgramCounter::Jump(nnn)
	}

	// 2nnn: CALL addr -> Call subroutine at nnn.
	// The interpreter increments the stack pointer, then puts the current PC on the top of the stack. The PC is then set to nnn.
	fn op_2nnn(&mut self, nnn: u16) -> ProgramCounter {
		self.stack[self.sp] = self.pc + 2; //OPCODE_SIZE
		self.sp += 1;
		ProgramCounter::Jump(nnn)
	}

	// 3xkk: SE Vx, byte -> Skip next instruction if Vx = kk.
	// The interpreter compares register Vx to kk, and if they are equal, increments the program counter by 2.
	fn op_3xkk(&mut self, x: u8, kk: u8) -> ProgramCounter {
		if self.v[x] == kk {
			ProgramCounter::Skip
		} else {
			ProgramCounter::Next
		}
	}

	// 4xkk - SNE Vx, byte -> Skip next instruction if Vx != kk.
	// The interpreter compares register Vx to kk, and if they are not equal, increments the program counter by 2.
	fn op_4xkk(&mut self, x: u8, kk: u8) -> ProgramCounter {
		if self.v[x] != kk {
			ProgramCounter::Skip
		} else {
			ProgramCounter::Next
		}
	}

	// 5xy0 - SE Vx, Vy -> Skip next instruction if Vx = Vy.
	// The interpreter compares register Vx to register Vy, and if they are equal, increments the program counter by 2.
	fn op_5xy0(&mut self, x: u8, y: u8) -> ProgramCounter {
		if self.v[x] == self.v[y] {
			ProgramCounter::Skip
		} else {
			ProgramCounter::Next
		}
	}

	// 6xkk - LD Vx, byte -> Set Vx = kk.
	// The interpreter puts the value kk into register Vx.
	fn op_6xkk(&mut self, x: u8, kk: u8) -> ProgramCounter {
		self.v[x] = kk;
		ProgramCounter::Next
	}

	// 7xkk - ADD Vx, byte -> Set Vx = Vx + kk.
	// Adds the value kk to the value of register Vx, then stores the result in Vx.
	fn op_7xkk(&mut self, x: u8, kk: u8) -> ProgramCounter {
		// TODO: Might have type mismatch
		self.v[x] += kk;
		ProgramCounter::Next
	}

	// 8xy0 - LD Vx, Vy -> Set Vx = Vy.
	// Stores the value of register Vy in register Vx.
	fn op_8xy0(&mut self, x: u8, y: u8) -> ProgramCounter {
		self.v[x] = self.v[y];
		ProgramCounter::Next
	}

	// 8xy1 - OR Vx, Vy -> Set Vx = Vx OR Vy.
	// Performs a bitwise OR on the values of Vx and Vy, then stores the result in Vx. 
	fn op_8xy1(&mut self, x: u8, y: u8) -> ProgramCounter {
		// TODO: Might have error due to borrowing
		self.v[x] = self.v[x] | self.v[y];
		ProgramCounter::Next
	}

	// 8xy2 - AND Vx, Vy -> Set Vx = Vx AND Vy.
	// Performs a bitwise AND on the values of Vx and Vy, then stores the result in Vx. 
	fn op_8xy2(&mut self, x: u8, y: u8) -> ProgramCounter {
		self.v[x] = self.v[x] & self.v[y];
		ProgramCounter::Next
	}

	// 8xy3 - XOR Vx, Vy -> Set Vx = Vx XOR Vy.
	// Performs a bitwise exclusive OR on the values of Vx and Vy, then stores the result in Vx. 
	fn op_8xy3(&mut self, x: u8, y: u8) -> ProgramCounter {
		self.v[x] = self.v[x] ^ self.v[y];
		ProgramCounter::Next
	}

	// 8xy4 - ADD Vx, Vy -> Set Vx = Vx + Vy, set VF = carry.
	// The values of Vx and Vy are added together. If the result is greater than 8 bits (i.e., > 255,) VF is set to 1, otherwise 0. Only the lowest 8 bits of the result are kept, and stored in Vx.
	fn op_8xy4(&mut self, x: u8, y: u8) -> ProgramCounter {
		vx = self.v[x] as u16;
		vy = self.v[y] as u16;
		res = vx + vy;
		carry = res > 255;
		res = res & 0x0011; //keep only last 2 bytes
		self.v[x] = res as u8;
		self.v[0xF] = carry;
	    ProgramCounter::Next
	}

	// 8xy5 - SUB Vx, Vy -> Set Vx = Vx - Vy, set VF = NOT borrow.
	// If Vx > Vy, then VF is set to 1, otherwise 0. Then Vy is subtracted from Vx, and the results stored in Vx.
	fn op_8xy5(&mut self, x: u8, y: u8) -> ProgramCounter {
		self.v[0xF] = if self.v[x] > self.v[y] {1} else {0};
		self.v[x] = self.v[x].wrapping_sub(self.v[y]);
		ProgramCounter::Next
	}

	// 8xy6 - SHR Vx {, Vy} -> Set Vx = Vx SHR 1.
	// If the least-significant bit of Vx is 1, then VF is set to 1, otherwise 0. Then Vx is divided by 2.
	fn op_8x06(&mut self, x: u8) -> ProgramCounter {
		self.v[0xF] = v[x] & 1;
		self.v[x] >>= 1;
		ProgramCounter::Next
	}

	// 8xy7 - SUBN Vx, Vy -> Set Vx = Vy - Vx, set VF = NOT borrow.
	// If Vy > Vx, then VF is set to 1, otherwise 0. Then Vx is subtracted from Vy, and the results stored in Vx.
	fn op_8xy7(&mut self, x: u8, y: u8) -> ProgramCounter {
		self.v[0xF] = if self.v[y] > self.v[x] {1} else {0};
		self.v[x] = self.v[y].wrapping_sub(self.v[x]);
		ProgramCounter::Next
	}

	// 8xyE - SHL Vx {, Vy} -> Set Vx = Vx SHL 1.
	// If the most-significant bit of Vx is 1, then VF is set to 1, otherwise to 0. Then Vx is multiplied by 2.
	fn op_8x0E(&mut self, x: u8) -> ProgramCounter {
		self.v[0xF] = v[x] & 0b10000000 >> 7; // TODO: Change binary to Hexadecimal for uniformity
		self.v[x] = self.v[x] << 1;
		ProgramCounter::Next
	}

	// 9xy0 - SNE Vx, Vy -> Skip next instruction if Vx != Vy.
	// The values of Vx and Vy are compared, and if they are not equal, the program counter is increased by 2.
	fn op_9xy0(&mut self, x: u8, y: u8) -> ProgramCounter {
		if self.v[x] != self.v[y] {
			ProgramCounter::Skip
		} else {
			ProgramCounter::Next
		}
	}

	// Annn - LD I, addr -> Set I = nnn.
	// The value of register I is set to nnn.
	fn op_annn(&mut self, nnn: u16) -> ProgramCounter {
		self.i = nnn;
		ProgramCounter::Next
	}


	// Bnnn - JP V0, addr -> Jump to location nnn + V0.
	// The program counter is set to nnn plus the value of V0.
	fn op_bnnn(&mut self, nnn: u16) -> ProgramCounter {
		ProgramCounter::Jump(nnn + self.v[0] as u16)
	}


	// Cxkk - RND Vx, byte
	// Set Vx = random byte AND kk.
	// The interpreter generates a random number from 0 to 255, 
	// which is then ANDed with the value kk. The results are stored in Vx.
	fn op_cxkk(&mut self, x: u8, kk: u8) -> ProgramCounter {
		let mut rng = rand::thread_rng();
		self.v[x] = rng.gen::<u8>() && kk;
		ProgramCounter::Next
	}

	// Dxyn - DRW Vx, Vy, nibble
	// Display n-byte sprite starting at memory location I at (Vx, Vy), set VF = collision.
    fn dxyn(&mut self, x: u8, y: u8, height: u8) -> ProgramCounter {
        let x_coord = self.v[x as usize];
        let y_coord = self.v[y as usize];

        let mut y_offset = 0;

        while y_offset < height {
            let ram_byte = self.ram[(I + y_offset) as usize];
            let mut x_offset = 0;

            while x_offset < 8 {
                // Wrap around the other side
                let pixel_x: usize = x_coord + x_offset % CHIP8_WIDTH;
                let pixel_y: usize = y_coord + y_offset % CHIP8_HEIGHT;

                if ram_byte & (0x80 >> x_offset) != 0 { // Checking every bit in ram_byte
                    if self.vram[pixel_x][pixel_y] == 1 {
                        v[0xF] = 1; // 1 XOR 1 = 0
                    }
                    self.vram[pixel_x][pixel_y] ^= 1;
                }
                x_offset += 1;
            }
            y_offset += 1;
        }

        ProgramCounter::Next
    }
	// The interpreter reads n bytes from memory, starting at the address stored in I. These bytes are then displayed as sprites on screen at coordinates (Vx, Vy). Sprites are XORed onto the existing screen. If this causes any pixels to be erased, VF is set to 1, otherwise it is set to 0. If the sprite is positioned so part of it is outside the coordinates of the display, it wraps around to the opposite side of the screen. See instruction 8xy3 for more information on XOR, and section 2.4, Display, for more information on the Chip-8 screen and sprites.


	// Ex9E - SKP Vx
	// Skip next instruction if key with the value of Vx is pressed.
    fn op_ex9e(&mut self, x: u8) -> ProgramCounter { 
        if self.keypad[self.v[x as usize]] {
            ProgramCounter::Skip
        } else {
            ProgramCounter::Next
        }
    }

	// ExA1 - SKNP Vx
	// Skip next instruction if key with the value of Vx is not pressed.
    fn op_exa1(&mut self, x: u8) -> ProgramCounter {
        if !self.keypad[self.v[x as usize]] {
            ProgramCounter::Skip
        } else {  
            ProgramCounter::Next
        }
    }

    // Fx07 - LD Vx, DT
    // Set Vx = delay timer value.
    fn op_fx07(&mut self, x: u8) -> ProgramCounter {
        self.v[x as usize] = self.delay_timer;
        ProgramCounter::Next
    }

    // Fx0A - LD Vx, K
    // Wait for a key press, store the value of the key in Vx.
    fn op_fx0a(&mut self, x: u8) -> ProgramCounter {
        let curr_key = 0;
        let arr_len = self.keypad.len();
        let mut pressed = false;

        while curr_key < arr_len {
            if self.keypad[curr_key] {
                self.v[x as usize] = curr_key;
                pressed = true;
                break;
            }
        }

        if pressed {
            ProgramCounter::Next // only increment if a key is pressed.
        } else {
            ProgramCounter::Stay // if not, stay at the same program counter
        }
    }

    // Fx15 - LD DT, Vx
    // Set delay timer = Vx.
    fn op_fx15(&mut self, x: u8) -> ProgramCounter {
        self.delay_timer = self.v[x as usize];
        ProgramCounter::Next
    }

    // Fx18 - LD ST, Vx
    // Set sound timer = Vx.
    fn op_fx18(&mut self, x: u8) -> ProgramCounter {
        self.sound_timer = self.v[x as usize];
        ProgramCounter::Next
    }
    // Fx1E - ADD I, Vx
    // Set I = I + Vx.
    fn op_fx1e(&mut self, x: u8) -> ProgramCounter {
        self.i += self.v[x as usize];
        ProgramCounter::Next
    }

    // Fx29 - LD F, Vx
    // Set I = location of sprite for digit Vx.
    fn op_fx29(&mut self, x: usize) -> ProgramCounter {
        self.i = (self.v[x as usize] * 5) as u16; // position of any digit Vx lies at fontset[Vx * 5]
        ProgramCounter::Next
    }

    // Fx33 - LD B, Vx
    // Store BCD representation of Vx in memory locations I, I+1, and I+2.
    fn op_fx33(&mut self, x) -> ProgramCounter {
        let vx = self.v[x as usize];

        self.ram[i] = vx / (100 as u8); // hundreds digit
        self.ram[i + 1] = (vx / (10 as u8)) % (10 as u8); // tens digit
        self.ram[i + 2] = vx % (10 as u8); // ones digit

        ProgramCounter::Next
    }

    // Fx55 - LD [I], Vx
    // Store registers V0 through Vx in memory starting at location I.
    fn op_fx55(&mut self, x: u8) -> ProgramCounter {
        let reg_index = 0;

        while reg_index <= x {
            self.ram[(I + reg_index) as usize] = self.v[reg_index as usize];
        }

        ProgramCounter::Next
    }
    


    // Fx65 - LD Vx, [I]
    // Read registers V0 through Vx from memory starting at location I.
    fn op_fx65(&mut self, x: u8) -> ProgramCounter {
        let reg_index = 0;

        while reg_index <= x {
            self.v[reg_index as usize] = self.ram[(I + reg_index) as usize];
        }

        ProgramCounter::Next
    }
}

// 3.2 - Super Chip-48 Instructions           [TOC]

// 00Cn - SCD nibble
// 00FB - SCR
// 00FC - SCL
// 00FD - EXIT
// 00FE - LOW
// 00FF - HIGH
// Dxy0 - DRW Vx, Vy, 0
// Fx30 - LD HF, Vx
// Fx75 - LD R, Vx
// Fx85 - LD Vx, R
