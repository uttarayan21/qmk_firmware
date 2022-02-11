use crate::keycodes::*;
// use crate::macros::keymaps;

const MATRIX_ROWS: usize = 5;
const MATRIX_COLS: usize = 16;

keymaps!(
    rows => MATRIX_ROWS,
    cols => MATRIX_COLS,
    layer_count => 4,
    layer!(// layer_0 or _WINDOWS
        r!(     GESC    | 1  | 2  | 3 | 4 | 5 |  6  | 7 | 8 | 9 | 0  |   -   |  =    |BSPC | xxx | DEL  ),
        r!(     TAB     | Q  | W  | E | R | T |  Y  | U | I | O | P  |  '['  | ']'   |BSLS | xxx | HOME ),
        r!(     LCTL    | A  | S  | D | F | G |  H  | J | K | L | ;  | QUOTE | xxxx  | '⏎' | xxx | PGUP ),
        r!(     LSFT    |xxxx| Z  | X | C | V |  B  | N | M | , | .  |   /   | xxxx  | RSFT| '↑' | PGDN ),
        r!(    [MO{2}]  |LGUI|LALT|xxx|xxx|xxx|SPACE|xxx|xxx|xxx|RALT|[MO{1}]|[MO{3}]| '←' | '↓' | '→' ),
    ),
    layer!(// layer_1 or _MAC
        r!(     '`'     | F1 | F2 |F3 |F4 |F5 | F6  |F7 |F8 |F9 |F10 |   F11 |  F12  |BSPC | xxx | DEL  ),
        r!( [TAB &LT{3}]| Q  | W  | E | R | T |  Y  | U | I | O | P  |  '['  | ']'   |BSLS | xxx | HOME ),
        r!(     LCTL    | A  | S  | D | F | G |  H  | J | K | L | ;  | QUOTE | xxxx  | '⏎' | xxx | PGUP ),
        r!(     LSFT    |xxxx| Z  | X | C | V |  B  | N | M | , | .  |   /   | xxxx  | RSFT| '↑' | PGDN ),
        r!(    [MO{2}]  |LGUI|LALT|xxx|xxx|xxx|SPACE|xxx|xxx|xxx|RALT|[MO{1}]|[MO{2}]| '←' | '↓' | '→' ),
    ),
    layer!(// layer_2
        r!(     '`'     | F1 | F2 |F3 |F4 | F5|  F6 | F7 | F8 |  F9 |F10 |   F11 |  F12  |BSPC | xxx | DEL  ),
        r!(     TAB     |xxxx|xxxx|xxx|xxx|xxx|xxxxx|xxxx|xxxx|xxxxx|xxxx|  '['  | ']'   |BSLS | xxx | HOME ),
        r!(     LCTL    |xxxx|xxxx|xxx|xxx|xxx| LEFT|DOWN| UP |RIGHT| ;  | QUOTE | xxxx  | '⏎' | xxx | PGUP ),
        r!(     LSFT    |xxxx|xxxx|xxx|xxx|xxx|xxxxx|xxxx|xxxx|xxxxx|xxxx|   /   | xxxx  | RSFT| '↑' | PGDN ),
        r!(    [MO{2}]  |LGUI|LALT|xxx|xxx|xxx|SPACE|xxxx|xxxx|xxxxx|xxxx|[MO{1}]|[MO{2}]| '←' | '↓' | '→' ),
    ),
    layer!(// layer_3
        r!(     GESC    | 1  | 2  | 3 | 4 | 5 |  6  | 7 | 8 | 9 | 0  |   -   |  =    |BSPC | xxx | DEL  ),
        r!( [TAB &LT{3}]| Q  | W  | E | R | T |  Y  | U | I | O | P  |  '['  | ']'   |BSLS | xxx | HOME ),
        r!(     LCTL    | A  | S  | D | F | G |  H  | J | K | L | ;  | QUOTE | xxxx  | '⏎' | xxx | PGUP ),
        r!(     LSFT    |xxxx| Z  | X | C | V |  B  | N | M | , | .  |   /   | xxxx  | RSFT| '↑' | PGDN ),
        r!(    [MO{2}]  |LGUI|LALT|xxx|xxx|xxx|RESET|xxx|xxx|xxx|RALT|[MO{1}]|[MO{2}]| '←' | '↓' | '→' ),
    ),
);
