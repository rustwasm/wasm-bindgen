use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Foo {
    x: u32,
}

macro_rules! echo {
    ($(($n:ident, $t:ty)),*) => {
        $(
            #[wasm_bindgen]
            pub fn $n(a: $t) -> $t {
                a
            }
        )*
    }
}

echo!(
    (echo_u8, u8),
    (echo_i8, i8),
    (echo_u16, u16),
    (echo_i16, i16),
    (echo_u32, u32),
    (echo_i32, i32),
    (echo_u64, u64),
    (echo_i64, i64),
    (echo_usize, usize),
    (echo_isize, isize),
    (echo_f32, f32),
    (echo_f64, f64),
    (echo_bool, bool),
    (echo_char, char),
    (echo_string, String),
    (echo_vec_u8, Vec<u8>),
    (echo_struct, Foo),
    (echo_vec_struct, Vec<Foo>),
    (echo_option_u8, Option<u8>),
    (echo_option_i8, Option<i8>),
    (echo_option_u16, Option<u16>),
    (echo_option_i16, Option<i16>),
    (echo_option_u32, Option<u32>),
    (echo_option_i32, Option<i32>),
    (echo_option_u64, Option<u64>),
    (echo_option_i64, Option<i64>),
    (echo_option_usize, Option<usize>),
    (echo_option_isize, Option<isize>),
    (echo_option_f32, Option<f32>),
    (echo_option_f64, Option<f64>),
    (echo_option_bool, Option<bool>),
    (echo_option_char, Option<char>),
    (echo_option_string, Option<String>),
    (echo_option_vec_u8, Option<Vec<u8>>),
    (echo_option_struct, Option<Foo>),
    (echo_option_vec_struct, Option<Vec<Foo>>)
);