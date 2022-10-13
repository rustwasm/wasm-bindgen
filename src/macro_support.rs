#![doc(hidden)]

/// Returns the length of the buffer resulting from the concatenation of all the
/// given byte buffers.
pub const fn concat_len(strings: &[&[u8]]) -> usize {
    let mut len = 0;
    let mut i = 0;

    while i < strings.len() {
        len += strings[i].len();
        i += 1;
    }

    len
}

/// Returns a byte buffer containing the concatenation of all the given byte
/// buffers.
pub const fn concat<const LEN: usize>(strings: &[&[u8]]) -> [u8; LEN] {
    let mut buf = [0; LEN];

    let mut buf_i = 0;
    let mut strings_i = 0;

    while strings_i < strings.len() {
        let string = &strings[strings_i];
        let mut string_i = 0;

        while string_i < string.len() {
            buf[buf_i] = string[string_i];
            buf_i += 1;
            string_i += 1;
        }

        strings_i += 1;
    }

    assert!(buf_i == LEN);

    buf
}

/// Returns the length in bytes it takes to encode the given value using VLE.
pub const fn vle_len(mut value: usize) -> usize {
    let mut len = 1;

    while (value >> 7) != 0 {
        len += 1;
        value >>= 7;
    }

    assert!(value >> 7 == 0);

    len
}

/// Returns the VLE of the given value.
pub const fn vle<const LEN: usize>(mut value: usize) -> [u8; LEN] {
    let mut result = [0; LEN];
    let mut result_i = 0;

    while (value >> 7) != 0 {
        result[result_i] = (value as u8) | 0x80;
        result_i += 1;
        value >>= 7;
    }

    assert!(value >> 7 == 0);
    assert!(result_i + 1 == LEN);

    result[result_i] = value as u8;
    result
}
