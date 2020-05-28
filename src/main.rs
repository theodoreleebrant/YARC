fn main() {
    let cpu = CPU {
    	opcode: 0,
    	v: [0; 16],
    	i: 0x200,
    	sound_timer: 0,
    	delay_timer: 0,
    	pc: 0x200,
    	sp: 0,
    	memory: [0;4096]
    };
}

struct CPU {
	opcode: u16,			// opcodes
	v: [u8; 16],			// registers v0, v1, ..., vF; 8-bit
	i: u16,					// register i; 16-bit for memory address
	sound_timer: u8,		// Sound timer; 8-bit
	delay_timer:u8,			// Delay time: 8-bit
	pc: u16,				// Program counter 
	sp: u8,					// Stack pointer
	memory: [u8; 4096]		// Memory, 4kB
}


