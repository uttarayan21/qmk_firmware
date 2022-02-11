#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
// #![rustfmt::skip]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[macro_export]
macro_rules! qkc {
    ( $x:ident ) => {
        paste! { [<quantum_keycodes_ $x>] } as u16
    };
}

#[macro_export]
macro_rules! kb {
    ( $x:ident ) => {
        paste! { [<hid_keyboard_keypad_usage_KC_ $x>] } as u16
    };
    ( $x:literal ) => {
        paste! { [<hid_keyboard_keypad_usage_KC_ $x>] } as u16
    };
}

#[macro_export]
macro_rules! fk {
    ( $x:ident ) => {
        paste! { [<internal_special_keycodes_KC_ $x>] } as u16
    }
}

#[macro_export]
macro_rules! LSFT {
    ( $x:ident ) => {
        qkc!(QK_LSFT)
    };
}

#[macro_export]
macro_rules! MO {
    ( $layer:literal ) => {
        qkc!(QK_MOMENTARY) | ($layer & 0xFF)
    };
}

#[macro_export]
macro_rules! LT {
    ( $layer:literal, $kc:expr ) => {
        qkc!(QK_LAYER_TAP) | (($layer & 0xF) << 8) | ($kc & 0xFF)
    };
}

macro_rules! KC {
    ( $key:literal ) => {
        paste! { pub const [<KC_ $key>] : u16 = kb!($key); }
    };
}

KC!("A");
KC!("B");
KC!("C");
KC!("D");
KC!("E");
KC!("F");
KC!("J");
KC!("K");
KC!("L");
KC!("M");
KC!("N");
KC!("O");
KC!("P");
KC!("Q");
KC!("S");
KC!("T");
KC!("U");
KC!("V");
KC!("W");
KC!("X");
KC!("Y");
KC!("Z");

KC!("F1");
KC!("F2");
KC!("F3");
KC!("F4");
KC!("F5");
KC!("F6");
KC!("F7");
KC!("F8");
KC!("F9");
KC!("F10");
KC!("F11");
KC!("F12");

KC!("1");
KC!("2");
KC!("3");
KC!("4");
KC!("5");
KC!("6");
KC!("7");
KC!("8");
KC!("9");
KC!("0");
KC!("MINUS");
KC!("EQUAL");

pub const KC_EXCLM: u16 = LSFT!(KC_1);
pub const KC_AT   : u16 = LSFT!(KC_2);
pub const KC_HASH : u16 = LSFT!(KC_33);
