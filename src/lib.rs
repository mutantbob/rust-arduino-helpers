#![no_std]

use atmega_hal::port::*;

/// Some APIs like Adafruit_NeoPixel and Ethernet use a pin number to select how they are attached.
/// I am not clear if this is separate from the analog pins of an Arduino
/// (which are labelled A0-A5 and seem to have numbers 54-59 inside /usr/share/arduino/hardware/arduino/avr/variants/mega/pins_arduino.h )
pub trait NumberedPin {
    fn pin_number() -> u8;
}

macro_rules! pin_number {
    ($pin_type:ty, $pin_number: expr) => {
        impl NumberedPin for $pin_type {
            fn pin_number() -> u8 {
                $pin_number
            }
        }
    };
}

pin_number! { PD0, 0 }
pin_number! { PD1, 1 }
pin_number! { PD2, 2 }
pin_number! { PD3, 3 }
pin_number! { PD4, 4 }
pin_number! { PD5, 5 }
pin_number! { PD6, 6 }

pin_number! { PB0, 8 }
pin_number! { PB1, 9 }
pin_number! { PB2, 10 }
pin_number! { PB3, 11 }
pin_number! { PB4, 12 }
pin_number! { PB5, 13 }
pin_number! { PB6, 14 }
pin_number! { PB7, 15 }

/// This is an example of how to build an Spi object for an Arduino Uno using the standard SPI pins.
/// ```
/// let dp = arduino_hal::Peripherals::take().unwrap();
/// let pins = arduino_hal::pins!(dp);
/// let (spi, cs) = arduino_uno_spi!(dp, pins, pins.d10);
/// ```
///
/// For `$cs` the most common value will be `pins.d10`,
/// although you may choose to use to wire up a different pin for your SPI device,
/// especially if you have multiple SPI peripherals attached.
///
/// This is a macro because it causes a lot of partial moves (the `Peripherals::SPI` and the `d11-13` from `pins` )
#[macro_export]
macro_rules! arduino_uno_spi {
    ($dp:expr, $pins:expr, $cs:expr) => {{
        let settings: arduino_hal::spi::Settings = arduino_hal::spi::Settings::default();
        Spi::new(
            $dp.SPI,
            $pins.d13.into_output(),
            $pins.d11.into_output(),
            $pins.d12.into_pull_up_input(),
            $cs.into_output(),
            settings,
        )
    }};
}
