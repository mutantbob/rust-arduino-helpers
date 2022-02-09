use std::io::stderr;
//use arduino_build_helpers::list_core_arduino_source_files;
use arduino_build_helpers::ArduinoBuilder;
use arduino_build_helpers::{arduino_include_root, ArduinoBindgen};
use std::env;
use std::ffi::OsStr;
use std::fs;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;

fn list_core_arduino_source_files<P: AsRef<Path>>(
    dirname: P,
) -> Result<impl Iterator<Item = PathBuf>, std::io::Error> {
    writeln!(
        stderr(),
        "list_core_arduino_source_files {:?}",
        dirname.as_ref()
    )?;
    let paths = fs::read_dir(dirname)?;
    Ok(paths
        .flatten()
        .map(|dir_entry| dir_entry.path())
        .filter(|path| is_c_or_cpp(path)))
}

fn is_c_or_cpp(path: &Path) -> bool {
    path.extension()
        .and_then(|extension| extension.to_str())
        .map(|extension| {
            extension.eq_ignore_ascii_case("c") || extension.eq_ignore_ascii_case("cpp")
        })
        .unwrap_or(false)
}

fn arduino_runtime_directory() -> String {
    std::env::var("ARDUINO_RUNTIME_DIRECTORY")
        .unwrap_or(format!("{}/cores/arduino", arduino_include_root()))
}

fn arduino_source_for(path: &str) -> String {
    format!("{}/{}", arduino_runtime_directory(), path)
}

fn generate_bindings_rs() {
    let arduino_h = arduino_source_for("Arduino.h");
    println!("cargo:rerun-if-changed={}", arduino_h);
    let bindings = bindgen::Builder::default()
        .header(arduino_h)
        .rig_arduino_uno()
        .clang_args(&["-x", "c++"])
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .ctypes_prefix("cty")
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

    build_core_a().expect("failed to compile arduino runtime libraries");
}

fn build_core_a() -> Result<(), std::io::Error> {
    let mut builder = cc::Build::new();

    let mut builder_plus_plus = builder.clone();
    builder_plus_plus.rig_arduino(true).compiler("avr-g++");

    builder.rig_arduino(false).compiler("avr-gcc");

    //

    for path_buf in list_core_arduino_source_files(arduino_runtime_directory()).unwrap() {
        if path_buf
            .file_name()
            .map(|osstr| osstr == OsStr::new("main.cpp"))
            .unwrap_or(false)
        {
            continue; // the rust app will provide its own main() function
        }

        let is_c = match path_buf.to_str() {
            None => false,
            Some(str) => str.ends_with(".c"),
        };

        if is_c {
            writeln!(stderr(), "using avr-gcc for {:?}", path_buf)?;
            &mut builder
        } else {
            writeln!(stderr(), "using avr-g++ for {:?}", path_buf)?;
            &mut builder_plus_plus
        }
        .file(path_buf.to_str().unwrap());
    }
    writeln!(stderr(), "added arduino core files")?;

    //writeln!(stderr(), "compiler {:?}", compiler.get_compiler());

    println!("cargo:rustc-link-lib=static=arduino-runtime");
    builder.compile("libarduino-runtime.a");

    println!("cargo:rustc-link-lib=static=arduino-runtime++");
    builder_plus_plus.compile("libarduino-runtime++.a");

    Ok(())
}
