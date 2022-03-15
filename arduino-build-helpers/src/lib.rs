use cc::Build;
use std::env;
use std::path::Path;

pub fn usr_share_arduino() -> &'static str {
    "/usr/share/arduino"
}

pub fn arduino_include_root() -> String {
    env::var("ARDUINO_INCLUDE_ROOT").unwrap_or(format!(
        "{}/{}",
        usr_share_arduino(),
        "hardware/arduino/avr"
    ))
}

pub trait ArduinoBuilder {
    /// You would use this like:
    /// ```
    /// let anp_dir = format!("{}/vendor/Adafruit_NeoPixel", env!("HOME"));
    ///
    /// let mut builder = cc::Build::new();
    /// builder
    ///     .include(anp_dir)
    ///     .rig_arduino(true)
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
        self.compiler(if c_plus_plus { "avr-g++" } else { "avr-gcc" })
            .include(format!("{}/{}", arduino_include_root(), "cores/arduino/"))
            .include(format!(
                "{}/{}",
                arduino_include_root(),
                "variants/standard/"
            ))
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
            .cpp(c_plus_plus)
            .cpp_set_stdlib(None)
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
    ///     let wrapper_h = "src-cpp/wrapper.h";
    ///     println!("cargo:rerun-if-changed={}", wrapper_h);
    ///     let bindings = bindgen::Builder::default()
    ///         .header(wrapper_h)
    ///         .rig_arduino_uno()
    ///         .clang_args(&["-x", "c++"])
    ///         .parse_callbacks(Box::new(bindgen::CargoCallbacks))
    ///         .ctypes_prefix("cty")
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
            &format!("-I{}/cores/arduino/", arduino_include_root()),
            &format!("-I{}/variants/standard/", arduino_include_root()),
            &format!("-I{}", avr_include_dir()),
            "-DF_CPU=16000000L",
            "-DARDUINO=10807",
            "-DARDUINO_AVR_UNO",
            "-DARDUINO_ARCH_AVR",
            "-mmcu=atmega328p",
        ])
        .use_core() // because no_std
    }
}

pub fn avr_include_dir() -> String {
    if let Ok(val) = env::var("AVR_INCLUDE_DIRECTORY") {
        return val.into();
    }
    for &path in &[
        "/usr/avr/include",     // gentoo
        "/usr/lib/avr/include", // debian
    ] {
        if Path::new(path).exists() {
            return String::from(path);
        }
    }
    panic!("unable to find AVR include directory (where is <avr/pgmspace.h> ?)")
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
