# Yet another (Rust) CHIP-8 emulator

This is a CHIP-8 emulator written in Rust. Done for emulation proof of concept in Orbital 2020!

Tests with games show that it works as expected.

<a href="https://i.imgur.com/4l3gxNh.png"><img src="https://i.imgur.com/4l3gxNh.png" title="PONG Chip-8 Game" alt="PONG game test"></a>


## Requirements
You will need to install Rust, as well as sdl2 with headers. 
Instruction to install Rust can be seen at the [Installation guide](https://www.rust-lang.org/tools/install)

## Run the program

You can run any game included by running the following in terminal:
`````
cargo run filename
`````

For example:
`````
cargo run pong.c8
`````
Games are included in the 'games' folder.

### Game instructions:
> Pong 

Left bar up: 1

Left bar down: q  

Right bar up: 4

Right bar down: r


> Space invader

Move left: q

Shoot: w

Move right: e



