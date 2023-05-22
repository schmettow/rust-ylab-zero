//! # YLabDG UI
//!
//! provides high-level structs to create embedded user interfaces
//!
//! So far implemented:
//! + RGBLed (static output)
//! + Button (dynamic input, event input)
//! 
//! 

// Make an alias for our board support package so copying examples to other boards is easier

use cytron_maker_pi_rp2040 as bsp;

use bsp::hal::{
    clocks::{init_clocks_and_plls, Clock},
    gpio::{FunctionPio0, Pin},
    pac,
    sio::Sio,
    watchdog::Watchdog,
};
use rp2040_hal::{pio::PIOExt};

/* 
trait StaticOutput {
    fn write<T>(&mut self, value: T);
}
 */


use embedded_hal::digital::v2::OutputPin;
use smart_leds::{brightness, SmartLedsWrite, RGB8};
use ws2812_pio::{Ws2812Direct};

mod yui {

type RgbStatusLed = Ws2812Direct<pac::PIO0, 
                    bsp::hal::pio::SM0, 
                    bsp::hal::gpio::pin::bank0::Gpio28>;

struct RgbLed {
    led: RgbStatusLed,
}

impl RgbLed {
    fn new(led: RgbStatusLed) -> Self {
        RgbLed{led: led}
        }
    
    fn write(&mut self, value:RGB8){
        let col = [value,];
        self.led.write(brightness(col.iter().cloned(), 32)).unwrap();
    }

}


use embedded_hal::digital::v2::InputPin;
// Stateful button
struct Button<T: InputPin> {
    pin: T,
    last_state: bool,
    state: bool,
}

impl<T: InputPin> Button<T> {
    fn new(pin: T) -> Self {
        Button { pin: pin, last_state: false, state: false}
    }
}

// Contact sensor trait
trait ContactSensor {
    fn read(&self) -> bool;
}

impl<T: InputPin> ContactSensor for Button<T> {
    fn read(&self) -> bool {
        self.pin.is_low().unwrap_or(true)
    }
}

// Event trait
trait EventButton {
    fn update(&mut self) -> bool;
    fn pressed(&self) -> bool;
}

impl<T: InputPin> EventButton for Button<T> {
    fn update(&mut self) -> bool {
        let this_value = self.read();
        self.last_state = self.state;
        self.state = this_value;
        return self.state
    }

    fn pressed(&self) -> bool {
        self.state
    }
}
}