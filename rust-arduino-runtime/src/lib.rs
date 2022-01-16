#![no_std]

mod raw;
pub mod workaround_cty;

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

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
