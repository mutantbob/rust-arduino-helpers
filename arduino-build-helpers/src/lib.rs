use cc::Build;
use std::fs;
use std::io::stderr;
use std::io::Write;
use std::path::{Path, PathBuf};

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

pub fn list_core_arduino_source_files<P: AsRef<Path>>(
    dirname: P,
) -> Result<impl Iterator<Item = PathBuf>, std::io::Error> {
    writeln!(
        stderr(),
        "list_core_arduino_source_files {:?}",
        dirname.as_ref()
    );
    let paths = fs::read_dir(dirname)?;
    Ok(paths
        .flatten()
        .map(|dir_entry| dir_entry.path())
        .filter(|path| {
            path.extension()
                .and_then(|extension| extension.to_str())
                .map(|extension| {
                    extension.eq_ignore_ascii_case("c") || extension.eq_ignore_ascii_case("cpp")
                })
                .unwrap_or(false)
        }))
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
