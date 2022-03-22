#![no_std]

///
///```
///let dp = arduino_hal::Peripherals::take().unwrap();
///let pins = arduino_hal::pins!(dp);
///let mut serial = default_serial!(dp, pins, 115200);
///initialize_serial_static(serial);
///
/// println!("Hello from {}", "Arduino");
///```
///
use arduino_hal::hal::port::{PD0, PD1};
use arduino_hal::port::mode::{Input, Output};
use arduino_hal::port::Pin;
use arduino_hal::Usart;
use avr_device::atmega328p::USART0;
use avr_device::interrupt::Mutex;
use core::cell::RefCell;
pub use ufmt;

pub type DefaultSerial = Usart<USART0, Pin<Input, PD0>, Pin<Output, PD1>>;
static mut SERIAL_STATIC: Mutex<RefCell<Option<DefaultSerial>>> = Mutex::new(RefCell::new(None));

/// prints to the serial object that was configured using [initialize_serial_static]
#[macro_export]
macro_rules! println {
    ( $($stuff: expr),+) => {
        $crate::with_serial(|serial| $crate::ufmt::uwriteln!(serial,$($stuff),+))
    }
}

/// load the static global with a serial port object
///
///```
///let dp = arduino_hal::Peripherals::take().unwrap();
///let pins = arduino_hal::pins!(dp);
///let mut serial = default_serial!(dp, pins, 115200);
///initialize_serial_static(serial);
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
{
    let mut serial =
        avr_device::interrupt::free(|cs| unsafe { &SERIAL_STATIC }.borrow(cs).borrow_mut().take());
    let rval = core(serial.as_mut().unwrap());
    avr_device::interrupt::free(|cs| unsafe { &SERIAL_STATIC }.borrow(cs).replace(serial));
    rval
}
