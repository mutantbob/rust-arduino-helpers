#![no_std]

#[cfg(not(feature = "board-selected"))]
compile_error!(
    "This crate requires you to specify your target Arduino board as a feature.

    Please select one of the following

    * atmega2560
    * atmega328p
    "
);

/// Some APIs like Adafruit_NeoPixel and Ethernet use a pin number to select how they are attached.
/// I am not clear if this is separate from the analog pins of an Arduino
/// (which are labelled A0-A5 and seem to have numbers 54-59 inside /usr/share/arduino/hardware/arduino/avr/variants/mega/pins_arduino.h )
pub trait NumberedPin {
    fn pin_number() -> u8;
}

macro_rules! pin_number {
    ($pin_type:ty, $pin_number: expr) => {
        impl $crate::NumberedPin for $pin_type {
            fn pin_number() -> u8 {
                $pin_number
            }
        }
    };
}

#[cfg(feature = "atmega328p")]
pub mod atmega328p {
    use crate::pin_number;
    use atmega_hal::port::*;
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
}

#[cfg(feature = "atmega2560")]
pub mod atmega2560 {
    use atmega_hal::port::*;
    // perl -ne 'print "pin_number! { $2, $1 }\n" if /pub d(\d+): atmega_hal::port::(\w+) = /' avr-hal/arduino-hal/src/port/mega2560.rs
    pin_number! { PE0, 0 }
    pin_number! { PE1, 1 }
    pin_number! { PE4, 2 }
    pin_number! { PE5, 3 }
    pin_number! { PG5, 4 }
    pin_number! { PE3, 5 }
    pin_number! { PH3, 6 }
    pin_number! { PH4, 7 }
    pin_number! { PH5, 8 }
    pin_number! { PH6, 9 }
    pin_number! { PB4, 10 }
    pin_number! { PB5, 11 }
    pin_number! { PB6, 12 }
    pin_number! { PB7, 13 }
    pin_number! { PJ1, 14 }
    pin_number! { PJ0, 15 }
    pin_number! { PH1, 16 }
    pin_number! { PH0, 17 }
    pin_number! { PD3, 18 }
    pin_number! { PD2, 19 }
    pin_number! { PD1, 20 }
    pin_number! { PA0, 22 }
    pin_number! { PA1, 23 }
    pin_number! { PA2, 24 }
    pin_number! { PA3, 25 }
    pin_number! { PA4, 26 }
    pin_number! { PA5, 27 }
    pin_number! { PA6, 28 }
    pin_number! { PA7, 29 }
    pin_number! { PC7, 30 }
    pin_number! { PC6, 31 }
    pin_number! { PC5, 32 }
    pin_number! { PC4, 33 }
    pin_number! { PC3, 34 }
    pin_number! { PC2, 35 }
    pin_number! { PC1, 36 }
    pin_number! { PC0, 37 }
    pin_number! { PD7, 38 }
    pin_number! { PG2, 39 }
    pin_number! { PG1, 40 }
    pin_number! { PG0, 41 }
    pin_number! { PL7, 42 }
    pin_number! { PL6, 43 }
    pin_number! { PL5, 44 }
    pin_number! { PL4, 45 }
    pin_number! { PL3, 46 }
    pin_number! { PL2, 47 }
    pin_number! { PL1, 48 }
    pin_number! { PL0, 49 }
    pin_number! { PB3, 50 }
    pin_number! { PB2, 51 }
    pin_number! { PB1, 52 }
    pin_number! { PB0, 53 }
}

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
