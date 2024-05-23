#[macro_export]
macro_rules! u8_to_str {
    ($array:expr) => {{
        let end_of_array = $crate::get_array_end_index!($array);
        let clean_array = core::str::from_utf8(&$array[..end_of_array]).unwrap_or("");
        clean_array
    }};
}

#[macro_export]
macro_rules! get_array_end_index {
    ($array:expr) => {{
        let end_of_array = $array
            .iter()
            .position(|&c| c == b'\0')
            .unwrap_or($array.len());
        end_of_array
    }};
}
