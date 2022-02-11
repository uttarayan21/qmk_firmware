#![no_std]
#![no_builtins]
#![crate_type = "staticlib"]

#[macro_use]
pub mod keycodes;
#[macro_use]
pub mod macros;
pub mod keymap;
#[macro_use]
extern crate paste;

#[panic_handler]
fn qmk_panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
