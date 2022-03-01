#![no_std]
#![feature(alloc_error_handler)] // at the top of the file

use core::alloc::{GlobalAlloc, Layout};
use core::convert::TryInto;
use cty::c_void;

mod raw;
pub use raw::client;
pub use raw::stream;
/*
 for some insane reason, downstream crates get link failures if they use bindgen.ctypes_prefix("cty") instead of
 bindgen.ctypes_prefix("rust_arduino_runtime::workaround_cty") .
*/
pub use cty as workaround_cty;

/// This is the magical init() function called by the stock Arduino main() function.
/// I have not yet figured out what exactly it does,
/// but I have found that at least one library (Adafruit_NeoPixel) will not work
/// without some subset of the initializations it performs.
pub fn arduino_main_init() {
    unsafe {
        raw::init(); // implemented in /usr/share/arduino/hardware/arduino/avr/cores/arduino/wiring.c
    }
}

pub fn arduino_serial_event_run() {
    unsafe {
        raw::serialEventRun();
    }
}

pub fn micros() -> cty::c_ulong {
    unsafe { raw::micros() }
}

/// The array is defined in /usr/share/arduino/hardware/arduino/avr/variants/standard/pins_arduino.h to have 20 elements .
/// This is not true for the variants/mega/pins_arduino.h which has more than I can be bothered to count.
#[allow(non_snake_case)]
pub fn digital_pin_to_bit_mask_PGM(idx: usize) -> u8 {
    let wrapper = unsafe {
        let shenanigans = raw::digital_pin_to_bit_mask_PGM.as_ptr() as *const [u8; 20];
        avr_progmem::ProgMem::<[u8; 20]>::new(*shenanigans)
    };

    wrapper.load_at(idx)
}

/// maybe this works?
struct ArduinoAlloc;

unsafe impl GlobalAlloc for ArduinoAlloc {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        raw::malloc(layout.size().try_into().unwrap()) as *mut u8
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        raw::free(ptr as *mut c_void)
    }

    unsafe fn realloc(&self, ptr: *mut u8, _layout: Layout, new_size: usize) -> *mut u8 {
        raw::realloc(ptr as *mut c_void, new_size.try_into().unwrap()) as *mut u8
    }
}

#[global_allocator]
static ALLOCATOR: ArduinoAlloc = ArduinoAlloc;

pub mod ip_address {
    pub use crate::raw::ip_address::*;
    use ufmt::{uDisplay, uWrite, Formatter};

    pub fn ip_address_4(a: u8, b: u8, c: u8, d: u8) -> IPAddress {
        unsafe { IPAddress::new1(a, b, c, d) }
    }

    impl uDisplay for IPAddress {
        fn fmt<W>(&self, formatter: &mut Formatter<W>) -> Result<(), W::Error>
        where
            W: uWrite + ?Sized,
        {
            let x = unsafe { &self._address.bytes };
            x[0].fmt(formatter)?;
            '.'.fmt(formatter)?;
            x[1].fmt(formatter)?;
            '.'.fmt(formatter)?;
            x[2].fmt(formatter)?;
            '.'.fmt(formatter)?;
            x[3].fmt(formatter)?;
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
