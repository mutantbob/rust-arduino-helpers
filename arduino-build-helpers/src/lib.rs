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

pub fn spi_include_dir() -> String {
    format!("{}/libraries/SPI/src", arduino_include_root())
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

pub mod exclude_some_headers {
    use crate::{arduino_include_root, avr_include_dir, spi_include_dir};
    use std::fs;
    use std::fs::DirEntry;
    use std::os::unix::ffi::OsStrExt;

    pub trait BlocklistFileMulti {
        fn blocklist_file_multi<T: AsRef<str>, I: IntoIterator<Item = T>>(self, iter: I) -> Self;
    }

    impl BlocklistFileMulti for bindgen::Builder {
        fn blocklist_file_multi<T: AsRef<str>, I: IntoIterator<Item = T>>(self, iter: I) -> Self {
            let mut builder = self;
            for file in iter {
                println!("# blocking file {}", file.as_ref());
                builder = builder.blocklist_file(file)
            }
            builder
        }
    }

    pub fn suppressed_headers() -> impl Iterator<Item = String> {
        let directories = vec![
            format!("{}/cores/arduino", arduino_include_root()),
            spi_include_dir(),
            avr_include_dir(),
        ];

        headers_in_directories(directories)
    }

    pub fn headers_in_directories<'a, T: Into<String>, I: IntoIterator<Item = T>>(
        directories: I,
    ) -> impl Iterator<Item = String> {
        directories
            .into_iter()
            .map(|dir| header_files_in_directory(&dir.into()))
            .flatten()
    }

    pub fn header_files_in_directory(dir: &str) -> impl Iterator<Item = String> {
        let x = fs::read_dir(dir).unwrap();
        x.into_iter()
            .flatten()
            .map(|path| header_file_or_recurse(path))
            .flatten()
        //.map(|path| path.path().to_str().map(|path| path.to_string()))
        //.flatten()
    }

    pub fn header_file_or_recurse(path: DirEntry) -> Vec<String> {
        let path_buf = path.path();
        if path_buf.is_dir() {
            match path_buf.as_os_str().to_str() {
                Some(dirname) => header_files_in_directory(dirname).collect(),
                None => vec![],
            }
        } else if is_header_file(&path) {
            path.path()
                .to_str()
                .map_or(vec![], |path| vec![path.to_string()])
        } else {
            vec![]
        }
    }

    pub fn is_header_file(path: &DirEntry) -> bool {
        let basename = path.file_name();
        let basename = basename.as_os_str().as_bytes();
        let len: usize = basename.len();
        let tmp = &basename[(len - 2)..len];
        tmp == b".h"
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
