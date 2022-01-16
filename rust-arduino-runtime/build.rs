use std::io::stderr;
//use arduino_build_helpers::list_core_arduino_source_files;
use arduino_build_helpers::ArduinoBuilder;
use std::fs;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;
use std::env;

fn list_core_arduino_source_files<P: AsRef<Path>>(
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
        .filter(|path| is_c_or_cpp(path)))
}

fn is_c_or_cpp(path: &PathBuf) -> bool {
    path.extension()
        .and_then(|extension| extension.to_str())
        .map(|extension| {
            extension.eq_ignore_ascii_case("c") || extension.eq_ignore_ascii_case("cpp")
        })
        .unwrap_or(false)
}

fn generate_bindings_rs()
{
    let arduino_h = "/usr/share/arduino/hardware/arduino/avr/cores/arduino/Arduino.h";
    println!("cargo:rerun-if-changed={}", arduino_h);
       let bindings = bindgen::Builder::default()
           .header(arduino_h)
           .clang_args(&[
               "-I/usr/share/arduino/hardware/arduino/avr/cores/arduino/",
               "-I/usr/share/arduino/hardware/arduino/avr/variants/standard/",
               "-I/usr/avr/include",
               "-D__COMPILING_AVR_LIBC__",
               "-DF_CPU=16000000L",
               "-x",
               "c++",
               "-mmcu=atmega328p",
           ])
           .parse_callbacks(Box::new(bindgen::CargoCallbacks))
           .use_core() // because no_std
           .ctypes_prefix("crate::workaround_cty") // the cty crate won't compile
           .generate()
           .expect("Unable to generate bindings");

       let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
       let bindings_out_file = out_path.join("bindings.rs");
       bindings
           .write_to_file(bindings_out_file)
           .expect("Couldn't write bindings!");

}

/// let's build core.a from /usr/share/arduino/hardware/arduino/avr/cores/arduino/*.{c,cpp}
fn main() {
    generate_bindings_rs();

    build_core_a();
}

fn build_core_a() {
    let mut builder = cc::Build::new();

    let mut builder_plus_plus = builder.clone();
    builder_plus_plus
        .rig_arduino(true)
        .cpp(true)
        .compiler("avr-g++");

    builder.rig_arduino(false).cpp(false).compiler("avr-gcc");

    //

    for path_buf in
    list_core_arduino_source_files("/usr/share/arduino/hardware/arduino/avr/cores/arduino/")
        .unwrap()
    {
        let is_c = match path_buf.to_str() {
            None => false,
            Some(str) => str.ends_with(".c"),
        };

        if is_c {
            writeln!(stderr(), "using avr-gcc for {:?}", path_buf);
            &mut builder
        } else {
            writeln!(stderr(), "using avr-g++ for {:?}", path_buf);
            &mut builder_plus_plus
        }
            .file(path_buf.to_str().unwrap());
    }
    writeln!(stderr(), "added arduino core files");

    //writeln!(stderr(), "compiler {:?}", compiler.get_compiler());

    println!("cargo:rustc-link-lib=static=arduino-runtime");
    builder.compile("libarduino-runtime.a");

    println!("cargo:rustc-link-lib=static=arduino-runtime++");
    builder_plus_plus.compile("libarduino-runtime++.a");
}
