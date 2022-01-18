This is a *very* early attempt at creating crates that make it easier 
to mix Rust applications with C and C++ libraries for the Arduino Uno.

`src/lib.rs` is laughably incomplete.  I have not even figured out what
I want to live in there.

The `rust-arduino-runtime/` crate wraps the `core.a` library that the 
Arduino IDE builds using the files in `
/usr/share/arduino/hardware/arduino/avr/cores/arduino/` The most useful 
thing from there is `arduino_main_init()` which wraps the `init()` 
function from `wiring.c`.  The first C++ library I attempted to 
cross-compile failed to work until I called init().

The `arduino-build-helpers/` crate provides a trait `ArduinoBuilder` 
which adds a `.rig_arduino()` method to `cc:Builder` that adds a 
bunch of `-I` and `-D` stuff that is needed to cross-compile Arduino 
libraries.