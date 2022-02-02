#![no_std]
#![feature(alloc_error_handler)] // at the top of the file

use core::alloc::{GlobalAlloc, Layout};
use core::convert::TryInto;
use cty::c_void;

mod raw;
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

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
