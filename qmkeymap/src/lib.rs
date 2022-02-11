#![no_std]
#![no_builtins]
#![crate_type = "staticlib"]

pub mod keycodes;
#[macro_use]
extern crate paste;

#[panic_handler]
fn qmk_panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
