# YLab Edge 

In a sister project we have introduced the [YLab](https://github.com/schmettow/ylab) for building interactive sensor recording devices. 
Using the high-level API in CircuitPython, developing sensors for everyday research never was easier.

The purpose of *YLab eDGe* is to follow YLab in spirit, but improve on what Ylab lacks the most, and that is: speed! 
Current readings are in the range of 250 SPS, which is enough for some applications, but is insufficient for large sensor arrays with high sample rates, e.g. motion capture or EEG.

The solution is to re-implement the YLab API in the systems programming language [Rust](https://www.rust-lang.org/). 
It is expected that this will trade around 20% ease-of-use for a performance improvement anywhere in the range between 2x and 200x.

# Current status

You should include this crate if you are writing code that you want to run on
a `Cytron Maker Pi Pico`. *Note* that this board is similar to the robotics-oriented Maker Pi RP2040, but not the same.

The crate provides a board specification (bsp) based on [rp2040-hal], but also assigns each pin to the following features of the board:

+ LED
+ RGB LED
+ six Grove ports with a and b channels
+ 3 push buttons
+ SD card
+ audio
+ wifi

[rp2040-hal]: https://github.com/rp-rs/rp-hal/tree/main/rp2040-hal


## Usage

To use this crate, your `Cargo.toml` file should contain:

```toml
ylab_edge = "0.1.0"
```

In your program, you will need to call `ylab_edge::Pins::new` to create
a new `Pins` structure. This will set up all the GPIOs for any on-board
devices. See the [examples](./examples) folder for more details.

## Installing examples

To install an example, 

+ clone this repository
+ connect the MP Pico via USB
+ Find the BOOT button (on the green part) and the RUN button (above the three push buttons) 
+ Simultaneously, push both buttons, release RUN, release BOOT. Now the Pico is in flash mode.
+ open a terminal in the ylab-edge folder and type:

```console
$ cargo run --release --example=ylabdg_0
```
If you get an error about not being able to find `elf2uf2-rs`, try:

```console
$ cargo install elf2uf2-rs
```
then try repeating the `cargo run` command above.

## Examples

### [YLab Edge Zero](./examples/ylabdg_0.rs)

A simple recording device for sensor arrays.


<!-- ### [cycle_leds](./examples/cycle_leds.rs)

Flashes a sequence across the Digital IO Status LEDs  
![cycle_leds_short](https://user-images.githubusercontent.com/60134748/147382950-5b604745-e228-4547-98fd-60a724a2722e.gif)

### [stepper_motor](./examples/stepper_motor.rs)

Rotates a stepper motor through 360 degrees clockwise then anticlockwise.  
Note that this requires a specific stepper motor from [Seeedstudio](https://www.seeedstudio.com/Small-Size-and-High-Torque-Stepper-Motor-24BYJ48-p-1922.html)  
![stepper_motor](https://user-images.githubusercontent.com/60134748/147382999-05e75b5f-d0d5-441a-ab51-25bb6e41589e.gif)

### [rgb_leds](./examples/rgb_leds.rs)

Cycle through colors on the pair of onboard RGB LEDs  
![rgb_leds](https://user-images.githubusercontent.com/60134748/147383061-d26a1684-b45e-4e1a-b32d-eb3591e6b085.gif)

### [pwm_buzzer](./examples/pwm_buzzer.rs)

Plays a sweeping frequency pitch through the on-board buzzer.  
Note: the example uses a short pulse length to keep the volume down - see documentation in the code to make it loud!  
 -->
## Links

<!-- - [Maker Pi RP2040 schematic](https://drive.google.com/file/d/1Zp8GYO8x7ThObB1G8RIZx2YdqrXtdUc0/view)
- [Maker Pi RP2040 Datasheet](https://docs.google.com/document/d/1DJASwxgbattM37V4AIlJVR4pxukq0up25LppA8-z_AY/edit?usp=sharing) -->
- [Product page for Raspberry Silicon RP2040](https://www.raspberrypi.org/products/rp2040/)
- [rp2040-hal](https://github.com/rp-rs/rp-hal/tree/main/rp2040-hal)
- [CircuitPython port](https://circuitpython.org/board/cytron_maker_pi_rp2040/)
- [Maker Pi RP2040 micropython port](https://github.com/CytronTechnologies/MAKER-PI-RP2040)

`SPDX-License-Identifier: Apache-2.0 OR MIT`

