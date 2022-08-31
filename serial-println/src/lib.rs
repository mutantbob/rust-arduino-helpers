#![no_std]

///
///```
///let dp = arduino_hal::Peripherals::take().unwrap();
///let pins = arduino_hal::pins!(dp);
///let mut serial = arduino_hal::default_serial!(dp, pins, 115200);
///serial_println::initialize_serial_static(serial);
///
/// println!("Hello from {}", "Arduino");
///```
///
use avr_device::interrupt::Mutex;
pub use board::DefaultSerial;
use core::cell::RefCell;
pub use ufmt;

static mut SERIAL_STATIC: Mutex<RefCell<Option<DefaultSerial>>> = Mutex::new(RefCell::new(None));

/// prints to the serial object that was configured using [initialize_serial_static]
#[macro_export]
macro_rules! println {
    ( $($stuff: expr),+) => { {
        $crate::with_serial(|serial| {let _ = $crate::ufmt::uwriteln!(serial,$($stuff),+);})
    }
    }
}

/// prints to the serial object that was configured using [initialize_serial_static]
#[macro_export]
macro_rules! print {
    ( $($stuff: expr),+) => { {
        use $crate::ufmt::UnstableDoAsFormatter;
        $crate::with_serial(|serial| {let _ = $crate::ufmt::uwrite!(serial,$($stuff),+);})
    }
    }
}

/// load the static global with a serial port object
///
///```
///let dp = arduino_hal::Peripherals::take().unwrap();
///let pins = arduino_hal::pins!(dp);
///let mut serial = arduino_hal::default_serial!(dp, pins, 115200);
///serial_println::initialize_serial_static(serial);
///```
pub fn initialize_serial_static(serial: DefaultSerial) {
    avr_device::interrupt::free(|cs| {
        unsafe { &SERIAL_STATIC }.borrow(&cs).replace(Some(serial));
    });
}

/// Execute the closure, having locked the serial object for its use. (see [initialize_serial_static] for how to provision the serial object)
pub fn with_serial<T, F>(core: F) -> T
where
    F: FnOnce(&mut DefaultSerial) -> T,
    T: Default,
{
    let mut serial =
        avr_device::interrupt::free(|cs| unsafe { &SERIAL_STATIC }.borrow(cs).borrow_mut().take());
    let rval = if let Some(serial) = serial.as_mut() {
        core(serial)
    } else {
        T::default()
    };
    avr_device::interrupt::free(|cs| unsafe { &SERIAL_STATIC }.borrow(cs).replace(serial));
    rval
}

#[cfg(feature = "atmega328p")]
mod board {
    use arduino_hal::hal::port::*;
    use arduino_hal::port::Pin;
    use arduino_hal::usart::Usart;
    use avr_device::atmega328p::USART0;
    use avr_hal_generic::port::mode::{Input, Output};

    pub type DefaultSerial = Usart<USART0, Pin<Input, PD0>, Pin<Output, PD1>>;
}

#[cfg(feature = "atmega2560")]
mod board {
    use arduino_hal::hal::port::*;
    use arduino_hal::port::Pin;
    use arduino_hal::usart::Usart;
    use avr_device::atmega2560::USART0;
    use avr_hal_generic::port::mode::{Input, Output};

    pub type DefaultSerial = Usart<USART0, Pin<Input, PE0>, Pin<Output, PE1>>;
}
