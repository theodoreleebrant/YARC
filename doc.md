# Yet Another Rust Chip8

Our attempt to emulate the Chip-8 Interpreter through low-level emulation. Tests with games show that it works as expected.

<a href="https://i.imgur.com/4l3gxNh.png"><img src="https://i.imgur.com/4l3gxNh.png" title="PONG Chip-8 Game" alt="PONG game test"></a>


## Test the program yourself

As our program does not inlcude User Interface, it needs to be downloaded and ran with rustc (rust compiler).

### Install Rust on your computer:

Follow the instructions in the ![Installation guide](https://www.rust-lang.org/tools/install)

### Run the program

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
