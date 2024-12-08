use std::mem::MaybeUninit;
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
    (echo_u128, u128),
    (echo_i128, i128),
    (echo_usize, usize),
    (echo_isize, isize),
    (echo_f32, f32),
    (echo_f64, f64),
    (echo_bool, bool),
    (echo_char, char),
    (echo_string, String),
    (echo_vec_u8, Vec<u8>),
    (echo_vec_i8, Vec<i8>),
    (echo_vec_u16, Vec<u16>),
    (echo_vec_i16, Vec<i16>),
    (echo_vec_u32, Vec<u32>),
    (echo_vec_i32, Vec<i32>),
    (echo_vec_u64, Vec<u64>),
    (echo_vec_i64, Vec<i64>),
    (echo_vec_uninit_u8, Vec<MaybeUninit<u8>>),
    (echo_vec_uninit_i8, Vec<MaybeUninit<i8>>),
    (echo_vec_uninit_u16, Vec<MaybeUninit<u16>>),
    (echo_vec_uninit_i16, Vec<MaybeUninit<i16>>),
    (echo_vec_uninit_u32, Vec<MaybeUninit<u32>>),
    (echo_vec_uninit_i32, Vec<MaybeUninit<i32>>),
    (echo_vec_uninit_u64, Vec<MaybeUninit<u64>>),
    (echo_vec_uninit_i64, Vec<MaybeUninit<i64>>),
    (echo_vec_string, Vec<String>),
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
    (echo_option_u128, Option<u128>),
    (echo_option_i128, Option<i128>),
    (echo_option_usize, Option<usize>),
    (echo_option_isize, Option<isize>),
    (echo_option_f32, Option<f32>),
    (echo_option_f64, Option<f64>),
    (echo_option_bool, Option<bool>),
    (echo_option_char, Option<char>),
    (echo_option_string, Option<String>),
    (echo_option_vec_u8, Option<Vec<u8>>),
    (echo_option_vec_i8, Option<Vec<i8>>),
    (echo_option_vec_u16, Option<Vec<u16>>),
    (echo_option_vec_i16, Option<Vec<i16>>),
    (echo_option_vec_u32, Option<Vec<u32>>),
    (echo_option_vec_i32, Option<Vec<i32>>),
    (echo_option_vec_u64, Option<Vec<u64>>),
    (echo_option_vec_i64, Option<Vec<i64>>),
    (echo_option_vec_uninit_u8, Option<Vec<MaybeUninit<u8>>>),
    (echo_option_vec_uninit_i8, Option<Vec<MaybeUninit<i8>>>),
    (echo_option_vec_uninit_u16, Option<Vec<MaybeUninit<u16>>>),
    (echo_option_vec_uninit_i16, Option<Vec<MaybeUninit<i16>>>),
    (echo_option_vec_uninit_u32, Option<Vec<MaybeUninit<u32>>>),
    (echo_option_vec_uninit_i32, Option<Vec<MaybeUninit<i32>>>),
    (echo_option_vec_uninit_u64, Option<Vec<MaybeUninit<u64>>>),
    (echo_option_vec_uninit_i64, Option<Vec<MaybeUninit<i64>>>),
    (echo_option_vec_string, Option<Vec<String>>),
    (echo_option_struct, Option<Foo>),
    (echo_option_vec_struct, Option<Vec<Foo>>)
);
