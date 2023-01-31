extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    let b = bindgen::builder()
        .ctypes_prefix("cty")
        .use_core()
        .header("../quantum/quantum_keycodes.h")
        .header("../quantum/keycode.h")
        // .header("../quantum/action.h") // TODO
        .clang_arg("-I../quantum")
        .clang_arg("-I../quantum/sequencer")
        .clang_arg("-I../quantum/logging")
        .clang_arg("-I../platforms");
    // .rustified_enum("hid_keyboard_keypad_usage")
    // .rustified_enum("internal_special_keycodes")
    // .rustified_enum("mouse_keys")
    // .rustified_enum("quantum_keycodes")
    #[cfg(target_os = "macos")]
    let bindings = b
        .clang_arg("-I/opt/homebrew/Cellar/avr-gcc@8/8.5.0/avr/include")
        .clang_arg("-DNO_PRINT")
        .rustfmt_bindings(true)
        .generate()
        .expect("Unable to generate bindings");
    #[cfg(target_os = "linux")]
    let bindings = b
        .clang_arg("-I/usr/include")
        .rustfmt_bindings(true)
        .generate()
        .expect("Unable to generate bindings");
    // panic!("{:#?}",bindings);
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
