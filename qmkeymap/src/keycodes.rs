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
KC!("G");
KC!("H");
KC!("I");
KC!("J");
KC!("K");
KC!("L");
KC!("M");
KC!("N");
KC!("O");
KC!("P");
KC!("Q");
KC!("R");
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

pub const KC_EXCLM: u16 = LSFT!(KC_1); // !
pub const KC_AT: u16 = LSFT!(KC_2); // @
pub const KC_HASH: u16 = LSFT!(KC_3); // #
pub const KC_DLR: u16 = LSFT!(KC_4); // $
pub const KC_PERC: u16 = LSFT!(KC_5); // %
pub const KC_CIRC: u16 = LSFT!(KC_6); // ^
pub const KC_AMPR: u16 = LSFT!(KC_7); // &
pub const KC_ASTR: u16 = LSFT!(KC_8); // *
pub const KC_LPRN: u16 = LSFT!(KC_9); // (
pub const KC_RPRN: u16 = LSFT!(KC_0); // )
pub const KC_UNDS: u16 = LSFT!(KC_MINUS); // _
pub const KC_PLUS: u16 = LSFT!(KC_EQUAL); // +
pub const KC_QUES: u16 = LSFT!(KC_SLSH);

KC!("UP");
KC!("DOWN");
KC!("LEFT");
KC!("RIGHT");
KC!("SPACE");

pub const KC_TG_NKRO: u16 = qkc!(MAGIC_TOGGLE_NKRO); //Toggle 6KRO / NKRO mode
pub const KC__TODO_: u16 = KC_TRNS as u16;
pub const KC________: u16 = KC__TODO_;
pub const KC_PGDN: u16 = kb!(PAGE_DOWN);
pub const KC_VOLU: u16 = fk!(AUDIO_VOL_UP);
pub const KC_VOLD: u16 = fk!(AUDIO_VOL_DOWN);
pub const KC_PAUS: u16 = kb!(PAUSE);
pub const KC_SLCK: u16 = kb!(SCROLL_LOCK);
pub const KC_PSCR: u16 = kb!(PRINT_SCREEN);
pub const KC_MUTE: u16 = fk!(AUDIO_MUTE);
pub const KC_ESC: u16 = kb!(ESCAPE);
pub const KC_LCTL: u16 = kb!(LEFT_CTRL);
pub const KC_RSFT: u16 = kb!(RIGHT_SHIFT);
pub const KC_SLSH: u16 = kb!(SLASH);
pub const KC_LSFT: u16 = kb!(LEFT_SHIFT);
pub const KC_SCLN: u16 = kb!(SEMICOLON);
pub const KC_CAPS: u16 = kb!(CAPS_LOCK);
pub const KC_BSLS: u16 = kb!(BACKSLASH);
pub const KC_RBRC: u16 = kb!(RIGHT_BRACKET);
pub const KC_LBRC: u16 = kb!(LEFT_BRACKET);
pub const KC_DEL: u16 = kb!(DELETE);
pub const KC_BSPC: u16 = kb!(BACKSPACE);
pub const KC_GRV: u16 = kb!(GRAVE);
pub const KC_TAB: u16 = kb!(TAB);
pub const KC_HOME: u16 = kb!(HOME);
pub const KC_DOT: u16 = kb!(DOT);
pub const KC_COMMA: u16 = kb!(COMMA);
pub const KC_ENTER: u16 = kb!(ENTER);
pub const KC_PGUP: u16 = kb!(PAGE_UP);
pub const KC_QUOTE: u16 = kb!(QUOTE);
pub const KC_LALT: u16 = kb!(LEFT_ALT);
pub const KC_RALT: u16 = kb!(RIGHT_ALT);
pub const KC_LGUI: u16 = kb!(LEFT_GUI);
pub const KC_RGUI: u16 = kb!(RIGHT_GUI);
pub const KC_END: u16 = kb!(END);
pub const KC_GESC: u16 = qkc!(GRAVE_ESC);

pub const RESET: u16 = qkc!(RESET);
pub const DEBUG: u16 = qkc!(DEBUG);
pub const EEPROM_RESET: u16 = qkc!(EEPROM_RESET);
pub const EEP_RST: u16 = qkc!(EEPROM_RESET);
