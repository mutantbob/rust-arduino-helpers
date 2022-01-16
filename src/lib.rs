#![no_std]

use atmega_hal::port::PD6;

pub trait NumberedPin {
    fn pin_number() -> i16;
}

impl NumberedPin for PD6 {
    fn pin_number() -> i16 {
        6
    }
}
