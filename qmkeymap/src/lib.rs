#![no_std]
#![no_builtins]
#![crate_type = "staticlib"]

mod bindings;

#[macro_use]
extern crate paste;
#[macro_use]
pub mod macros;

#[macro_use]
pub mod keycodes;

pub mod keymap;

#[panic_handler]
fn qmk_panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
