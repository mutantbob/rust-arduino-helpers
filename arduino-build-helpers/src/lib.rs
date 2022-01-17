use cc::Build;

pub trait ArduinoBuilder {
    /// You would use this like:
    /// ```
    /// let anp_dir = format!("{}/vendor/Adafruit_NeoPixel", env!("HOME"));
    ///
    /// let mut builder = cc::Build::new();
    /// builder
    ///     .include(anp_dir)
    ///     .rig_arduino(true)
    ///     .cpp(true)
    ///     .compiler("avr-g++");
    ///
    /// builder.file("src-cpp/neopixel.cpp");
    ///
    /// println!("cargo:rustc-link-lib=static=neopixel");
    /// builder.compile("libneopixel.a");
    ///```
    ///
    /// where `src-cpp/neopixel.cpp`` is
    /// ```c++
    /// #include <Arduino.h>
    /// #include "Adafruit_NeoPixel.cpp"
    /// ```
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

pub trait ArduinoBindgen {
    /// You would use this like
    /// ```
    ///
    /// fn generate_bindings_rs() {
    ///     let arduino_h = "/usr/share/arduino/hardware/arduino/avr/cores/arduino/Arduino.h";
    ///     println!("cargo:rerun-if-changed={}", arduino_h);
    ///     let bindings = bindgen::Builder::default()
    ///         .header(arduino_h)
    ///         .rig_arduino_uno()
    ///         .clang_args(&["-x", "c++"])
    ///         .parse_callbacks(Box::new(bindgen::CargoCallbacks))
    ///         .ctypes_prefix("crate::workaround_cty") // the cty crate won't compile
    ///         .generate()
    ///         .expect("Unable to generate bindings");
    ///
    ///     let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    ///     let bindings_out_file = out_path.join("bindings.rs");
    ///     bindings
    ///         .write_to_file(bindings_out_file)
    ///         .expect("Couldn't write bindings!");
    /// }
    ///
    /// ```
    fn rig_arduino_uno(self) -> Self;
}

impl ArduinoBindgen for bindgen::Builder {
    fn rig_arduino_uno(self) -> Self {
        self.clang_args(&[
            "-I/usr/share/arduino/hardware/arduino/avr/cores/arduino/",
            "-I/usr/share/arduino/hardware/arduino/avr/variants/standard/",
            "-I/usr/avr/include",
            "-DF_CPU=16000000L",
            "-DARDUINO=10807",
            "-DARDUINO_AVR_UNO",
            "-DARDUINO_ARCH_AVR",
            "-mmcu=atmega328p",
        ])
        .use_core() // because no_std
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
