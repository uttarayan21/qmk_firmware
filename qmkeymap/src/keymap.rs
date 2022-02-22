use crate::bindings::*;
use crate::keycodes::*;

const MATRIX_ROWS: usize = 5;
const MATRIX_COLS: usize = 16;

keymaps!(
    rows => MATRIX_ROWS,
    cols => MATRIX_COLS,
    layer_count => 4,
    layer!(// layer_0 or _WINDOWS
        r!(     GESC    | 1  | 2  | 3 | 4 | 5 |  6  | 7 | 8 | 9 | 0  |   -   |  =    | BSPC | xxxx | DEL  ),
        r!(     TAB     | Q  | W  | E | R | T |  Y  | U | I | O | P  |  '['  | ']'   | BSLS | xxxx | PSCR ),
        r!(     LCTL    | A  | S  | D | F | G |  H  | J | K | L | ;  | QUOTE | xxxx  | '⏎'  | xxxx | PGUP ),
        r!(     LSFT    |xxxx| Z  | X | C | V |  B  | N | M | , | .  |   /   | xxxx  | RSFT |  UP  | PGDN ),
        r!(    [MO{2}]  |LGUI|LALT|xxx|xxx|xxx|SPACE|xxx|xxx|xxx|RALT|[MO{2}]|[MO{3}]| LEFT | DOWN | RIGHT),
    ),
    layer!(// layer_1 or _MAC
        r!(     GESC    | 1  | 2  | 3 | 4 | 5 |  6  | 7 | 8 | 9 | 0  |   -   |  =    |BSPC | xxxx | DEL  ),
        r!(     TAB     | Q  | W  | E | R | T |  Y  | U | I | O | P  |  '['  | ']'   |BSLS | xxxx | PSCR ),
        r!(     LCTL    | A  | S  | D | F | G |  H  | J | K | L | ;  | QUOTE | xxxx  | '⏎' | xxxx | PGUP ),
        r!(     LSFT    |xxxx| Z  | X | C | V |  B  | N | M | , | .  |   /   | xxxx  | RSFT|  UP  | PGDN ),
        r!(    [MO{2}]  |LALT|LGUI|xxx|xxx|xxx|SPACE|xxx|xxx|xxx|RGUI|[MO{2}]|[MO{3}]| LEFT| DOWN | RIGHT ),
    ),
    layer!(// layer_2
        r!(     '`'     | F1 | F2 |F3 |F4 | F5|  F6 | F7 | F8 |  F9 |F10 |   F11 |  F12  |   BSPC   | [RGB M_B] | [RGB TOG] ),
        r!(     TAB     |xxxx|xxxx|xxx|xxx|xxx|xxxxx|xxxx|xxxx|xxxxx|xxxx|  '['  | ']'   |   BSLS   | xxx       | [RGB MOD] ),
        r!(     LCTL    |xxxx|xxxx|xxx|xxx|xxx| LEFT|DOWN| UP |RIGHT| ;  | QUOTE | xxxx  |    '⏎'   | xxx       | [RGB M_R] ),
        r!(     LSFT    |xxxx|xxxx|xxx|xxx|xxx|xxxxx|xxxx|xxxx|xxxxx|xxxx|   /   | xxxx  |   RSFT   | [RGB VAI] | [RGB M_T] ),
        r!(    [MO{2}]  |LGUI|LALT|xxx|xxx|xxx|SPACE|xxxx|xxxx|xxxxx|xxxx|[MO{3}]|[MO{3}]| [RGB HUD]| [RGB VAD] | [RGB HUI] ),
    ),
    layer!(// layer_3
        r!(    OUT_USB  |[I B_D]|[I B_U]|xxxxxxx|xxxxxxx|[RGB HUD]|[RGB HUI]|[I MPT]|[I MPP]|[I MNT]|[I  AM]|[I AVD]|[I AVU]|xxxxx|xxxxx| OUT_BT ),
        r!(     TAB     |[I FN1]|[I FN2]|[I FN3]|[I FN4]| xxxxxxx | xxxxxxx |xxxxxxx|xxxxxxx|xxxxxxx|[I FN5]|xxxxxxx|xxxxxxx|xxxxx|xxxxx|xxxxxxx),
        r!(     LCTL    |xxxxxxx|xxxxxxx|xxxxxxx|xxxxxxx| xxxxxxx | xxxxxxx |xxxxxxx|xxxxxxx|xxxxxxx|xxxxxxx|xxxxxxx|xxxxxxx|xxxxx|xxxxx|xxxxxxx),
        r!(     LSFT    |xxxxxxx|xxxxxxx|xxxxxxx|xxxxxxx| xxxxxxx | xxxxxxx |xxxxxxx|xxxxxxx|xxxxxxx|xxxxxxx|xxxxxxx|xxxxxxx|xxxxx|xxxxx|xxxxxxx),
        r!(    [MO{2}]  |xxxxxxx|xxxxxxx|xxxxxxx|xxxxxxx| xxxxxxx |  RESET  |xxxxxxx|xxxxxxx|xxxxxxx|xxxxxxx|xxxxxxx|xxxxxxx|xxxxx|xxxxx|xxxxxxx),
    ),
);
