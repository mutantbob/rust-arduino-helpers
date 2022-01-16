use cc::Build;

pub trait ArduinoBuilder {
    fn rig_arduino(&mut self, c_plus_plus: bool) -> &mut Self;
}

impl ArduinoBuilder for Build {
    fn rig_arduino(&mut self, c_plus_plus: bool) -> &mut Self {
        self.compiler("avr-g++")
            .include("/usr/share/arduino/hardware/arduino/avr/cores/arduino/")
            .include("/usr/share/arduino/hardware/arduino/avr/variants/standard/")
            .define("F_CPU", "16000000L")
            .define("ARDUINO", "10807")
            .define("ARDUINO_AVR_UNO", None)
            .define("ARDUINO_ARCH_AVR", None)
            .flag("-mmcu=atmega328p")
            .flag("-Os")
            .flag(if c_plus_plus {
                "-std=gnu++11"
            } else {
                "-std=gnu11"
            })
            .flag("-fpermissive")
            .flag("-fno-exceptions")
            .flag("-ffunction-sections")
            .flag("-fdata-sections")
            .flag("-fno-threadsafe-statics")
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
