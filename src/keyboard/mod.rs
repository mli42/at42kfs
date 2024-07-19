mod keymap_fr;
mod keymap_us;

#[macro_export]
macro_rules! create_keymap_array {
    // value [0] is normal key, [1] is shifted key
    ($default:expr; $len:expr; $($index:expr => $value:expr),*) => {{
        let mut arr = [$default; $len];
        $(
            let as_bytes: [u8; 2] = [$value[0] as u8, $value[1] as u8];
            arr[$index] = as_bytes;
        )*
        arr
    }};
}

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
        _ => {
            if is_pressed {
                if state.ctrl && state.alt && keycode == 0x10 {
                    state.lang = match state.lang {
                        KeymapLanguage::US => KeymapLanguage::FR,
                        KeymapLanguage::FR => KeymapLanguage::US,
                    };
                }

                let keymap_index_mode = match (state.shift | state.capslock, state.alt) {
                    (true, _) => 1,
                    // (_, true) => 2,
                    _ => 0,
                };

                let keymap = match state.lang {
                    KeymapLanguage::US => keymap_us::KEYMAP_US,
                    KeymapLanguage::FR => keymap_fr::KEYMAP_FR,
                };

                let mut key = keymap[keycode as usize][keymap_index_mode];

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
