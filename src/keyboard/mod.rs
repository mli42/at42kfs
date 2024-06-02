use crate::println;

macro_rules! create_keymap_array {
    // Match the pattern where specific values are provided at certain indices
    ($default:expr; $len:expr; $($index:expr => $value:expr),*) => {{
        let mut arr = [$default; $len];
        $(
            arr[$index] = $value;
        )*
        arr
    }};
}

const KEYMAP_US: [u8; 0x300] = create_keymap_array!(
    0;
    0x300;

    0x1 => 0x1B,
    0x2 => '1' as u8,
    0x3 => '2' as u8,
    0x4 => '3' as u8,
    0x5 => '4' as u8,
    0x6 => '5' as u8,
    0x7 => '6' as u8,
    0x8 => '7' as u8,
    0x9 => '8' as u8,
    0xA => '9' as u8,
    0xB => '0' as u8,
    0xC => '-' as u8,
    0xD => '=' as u8,

    0x10 => 'q' as u8,
    0x11 => 'w' as u8,
    0x12 => 'e' as u8,
    0x13 => 'r' as u8,
    0x14 => 't' as u8,
    0x15 => 'y' as u8,
    0x16 => 'u' as u8,
    0x17 => 'i' as u8,
    0x18 => 'o' as u8,
    0x19 => 'p' as u8,
    0x1A => '[' as u8,
    0x1B => ']' as u8,

    0x1E => 'a' as u8,
    0x1F => 's' as u8,
    0x20 => 'd' as u8,
    0x21 => 'f' as u8,
    0x22 => 'g' as u8,
    0x23 => 'h' as u8,
    0x24 => 'j' as u8,
    0x25 => 'k' as u8,
    0x26 => 'l' as u8,
    0x27 => ';' as u8,
    0x28 => '\'' as u8,
    0x29 => '`' as u8,

    0x2B => '\\' as u8,
    0x2C => 'z' as u8,
    0x2D => 'x' as u8,
    0x2E => 'c' as u8,
    0x2F => 'v' as u8,
    0x30 => 'b' as u8,
    0x31 => 'n' as u8,
    0x32 => 'm' as u8,
    0x33 => ',' as u8,
    0x34 => '.' as u8,
    0x35 => '/' as u8,
    0x39 => ' ' as u8,

    0x110 => 'Q' as u8,
    0x111 => 'W' as u8,
    0x112 => 'E' as u8,
    0x113 => 'R' as u8,
    0x114 => 'T' as u8,
    0x115 => 'Y' as u8,
    0x116 => 'U' as u8,
    0x117 => 'I' as u8,
    0x118 => 'O' as u8,
    0x119 => 'P' as u8,
    0x11A => '[' as u8,
    0x11B => ']' as u8,

    0x11E => 'A' as u8,
    0x11F => 'S' as u8,
    0x120 => 'D' as u8,
    0x121 => 'F' as u8,
    0x122 => 'G' as u8,
    0x123 => 'H' as u8,
    0x124 => 'J' as u8,
    0x125 => 'K' as u8,
    0x126 => 'L' as u8,
    0x127 => ';' as u8,
    0x128 => '\'' as u8,
    0x129 => '`' as u8,

    0x12B => '\\' as u8,
    0x12C => 'Z' as u8,
    0x12D => 'X' as u8,
    0x12E => 'C' as u8,
    0x12F => 'V' as u8,
    0x130 => 'B' as u8,
    0x131 => 'N' as u8,
    0x132 => 'M' as u8,
    0x133 => ',' as u8,
    0x134 => '.' as u8,
    0x135 => '/' as u8,
    0x139 => ' ' as u8

);

const KEYMAP_FR: [u8; 0x300] = create_keymap_array!(
    0;
    0x300;

    0x1 => 0x1B,
    0x2 => '&' as u8,
    0x3 => 'é' as u8,
    0x4 => '"' as u8,
    0x5 => '\'' as u8,
    0x6 => '(' as u8,
    0x7 => '-' as u8,
    0x8 => 'è' as u8,
    0x9 => '_' as u8,
    0xA => 'ç' as u8,
    0xB => 'à' as u8,
    0xC => ')' as u8,
    0xD => '=' as u8,

    0x101 => 0x1B,
    0x102 => '1' as u8,
    0x103 => '2' as u8,
    0x104 => '3' as u8,
    0x105 => '4' as u8,
    0x106 => '5' as u8,
    0x107 => '6' as u8,
    0x108 => '7' as u8,
    0x109 => '8' as u8,
    0x10A => '9' as u8,
    0x10B => '0' as u8,
    0x10C => '°' as u8,
    0x10D => '+' as u8,

    0x203 => '~' as u8,
    0x204 => '#' as u8,
    0x205 => '{' as u8,
    0x206 => '[' as u8,
    0x207 => '|' as u8,
    0x208 => '`' as u8,
    0x209 => '\\' as u8,
    0x20A => '^' as u8,
    0x20B => '@' as u8,
    0x20C => ']' as u8,
    0x20D => '}' as u8,


    0x10 => 'a' as u8,
    0x11 => 'z' as u8,
    0x12 => 'e' as u8,
    0x13 => 'r' as u8,
    0x14 => 't' as u8,
    0x15 => 'y' as u8,
    0x16 => 'u' as u8,
    0x17 => 'i' as u8,
    0x18 => 'o' as u8,
    0x19 => 'p' as u8,
    0x1A => '^' as u8,
    0x1B => '$' as u8,

    0x1E => 'q' as u8,
    0x1F => 's' as u8,
    0x20 => 'd' as u8,
    0x21 => 'f' as u8,
    0x22 => 'g' as u8,
    0x23 => 'h' as u8,
    0x24 => 'j' as u8,
    0x25 => 'k' as u8,
    0x26 => 'l' as u8,
    0x27 => 'm' as u8,
    0x28 => 'ù' as u8,
    0x29 => '*' as u8,

    0x2B => '<' as u8,
    0x2C => 'w' as u8,
    0x2D => 'x' as u8,
    0x2E => 'c' as u8,
    0x2F => 'v' as u8,
    0x30 => 'b' as u8,
    0x31 => 'n' as u8,
    0x32 => ',' as u8,
    0x33 => ';' as u8,
    0x34 => ':' as u8,
    0x35 => '!' as u8,
    0x39 => ' ' as u8,

    // numpad
    0x52 => '0' as u8,
    0x4F => '1' as u8,
    0x50 => '2' as u8,
    0x51 => '3' as u8,
    0x4B => '4' as u8,
    0x4C => '5' as u8,
    0x4D => '6' as u8,
    0x47 => '7' as u8,
    0x48 => '8' as u8,
    0x49 => '9' as u8,


    0x110 => 'A' as u8,
    0x111 => 'Z' as u8,
    0x112 => 'E' as u8,
    0x113 => 'R' as u8,
    0x114 => 'T' as u8,
    0x115 => 'Y' as u8,
    0x116 => 'U' as u8,
    0x117 => 'I' as u8,
    0x118 => 'O' as u8,
    0x119 => 'P' as u8,
    0x11A => '^' as u8,
    0x11B => '$' as u8,

    0x11E => 'Q' as u8,
    0x11F => 'S' as u8,
    0x120 => 'D' as u8,
    0x121 => 'F' as u8,
    0x122 => 'G' as u8,
    0x123 => 'H' as u8,
    0x124 => 'J' as u8,
    0x125 => 'K' as u8,
    0x126 => 'L' as u8,
    0x127 => 'M' as u8,
    0x128 => '%' as u8,
    0x129 => 'µ' as u8,

    0x12B => '>' as u8,
    0x12C => 'W' as u8,
    0x12D => 'X' as u8,
    0x12E => 'C' as u8,
    0x12F => 'V' as u8,
    0x130 => 'B' as u8,
    0x131 => 'N' as u8,
    0x132 => '?' as u8,
    0x133 => '.' as u8,
    0x134 => '/' as u8,
    0x135 => '§' as u8,
    0x139 => ' ' as u8
);

#[allow(dead_code)]
pub enum KeymapLanguage {
    US,
    FR,
}

pub struct KeyboardState {
    pub lang: KeymapLanguage,
    pub shift: bool,
    pub ctrl: bool,
    pub alt: bool,
    pub capslock: bool,
    pub verrnum: bool,
}

pub fn handle_scancode(scancode: u8, state: &mut KeyboardState, output: &mut [u8]) {
    let keycode = scancode & 0b01111111;
    let is_release = scancode & 0x80 != 0;
    let is_pressed = !is_release;
    let mut i = 0;

    let mut write_change = |c: char| {
        output[i] = c as u8;
        i += 1;
    };

    match keycode {
        0x1d => state.ctrl = is_pressed,
        0x38 => state.alt = is_pressed,
        0x2a | 0x36 => state.shift = is_pressed,
        0x3a => {
            if is_release {
                state.capslock = !state.capslock
            }
        }
        0x1C => {
            if is_pressed {
                write_change('\n');
            }
        }
        0x0E => {
            if is_pressed {
                write_change(0x08 as char);
            }
        }
        0x45 => {
            if is_pressed {
                state.verrnum = !state.verrnum;
            }
        }
        _ => {
            if is_pressed {
                if state.ctrl && state.alt && keycode == 0x10 {
                    state.lang = match state.lang {
                        KeymapLanguage::US => KeymapLanguage::FR,
                        KeymapLanguage::FR => KeymapLanguage::US,
                    };
                }

                let caps_flag = state.capslock || state.shift;
                let altgr_flag = state.alt;
                let verrnum_flag = state.verrnum;
                let keymap_index = ((verrnum_flag as usize) << 10)
                    | ((altgr_flag as usize) << 9)
                    | (caps_flag as usize) << 8
                    | keycode as usize;

                let keymap = match state.lang {
                    KeymapLanguage::US => KEYMAP_US,
                    KeymapLanguage::FR => KEYMAP_FR,
                };

                let mut key = keymap[keymap_index as usize];
                // println!("Key: {}", keycode as usize);
                if key != 0 {
                    if state.ctrl {
                        if !key.is_ascii_alphabetic() {
                            return;
                        }
                        write_change('^');
                        key.make_ascii_uppercase();
                    }
                    write_change(key as char);
                }
            }
        }
    }
}
