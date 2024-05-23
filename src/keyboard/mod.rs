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

const KEYMAP_US: [u8; 128] = create_keymap_array!(
    0;
    128;

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
    0x39 => ' ' as u8
);

const KEYMAP_FR: [u8; 128] = create_keymap_array!(
    0;
    128;

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
    0x39 => ' ' as u8
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
}

pub fn handle_scancode(scancode: u8, state: &mut KeyboardState, output: &mut [u8; 80]) {
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
        _ => {
            if is_pressed {
                if state.ctrl && state.alt && keycode == 0x10 {
                    state.lang = match state.lang {
                        KeymapLanguage::US => KeymapLanguage::FR,
                        KeymapLanguage::FR => KeymapLanguage::US,
                    };
                }

                let mut key = match state.lang {
                    KeymapLanguage::US => KEYMAP_US,
                    KeymapLanguage::FR => KEYMAP_FR,
                }[keycode as usize];

                if state.capslock || state.shift {
                    key = key.to_ascii_uppercase();
                }

                if key != 0 {
                    if state.ctrl {
                        write_change('^');
                    }
                    write_change(key as char);
                }
            }
        }
    }
}
