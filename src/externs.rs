#![allow(unused_variables)]

use crate::convert::{WasmRet, WasmSlice};

pub unsafe fn __wbindgen_object_clone_ref(idx: u32) -> u32 {
    unimplemented!("__wbindgen_object_clone_ref")
}
pub unsafe fn __wbindgen_object_drop_ref(idx: u32) -> () {
    unimplemented!("__wbindgen_object_drop_ref")
}

pub unsafe fn __wbindgen_string_new(ptr: *const u8, len: usize) -> u32 {
    unimplemented!("__wbindgen_string_new")
}
pub unsafe fn __wbindgen_number_new(f: f64) -> u32 {
    unimplemented!("__wbindgen_number_new")
}
pub unsafe fn __wbindgen_bigint_from_str(ptr: *const u8, len: usize) -> u32 {
    unimplemented!("__wbindgen_bigint_from_str")
}
pub unsafe fn __wbindgen_bigint_from_i64(n: i64) -> u32 {
    unimplemented!("__wbindgen_bigint_from_i64")
}
pub unsafe fn __wbindgen_bigint_from_u64(n: u64) -> u32 {
    unimplemented!("__wbindgen_bigint_from_u64")
}
pub unsafe fn __wbindgen_bigint_from_i128(hi: i64, lo: u64) -> u32 {
    unimplemented!("__wbindgen_bigint_from_i128")
}
pub unsafe fn __wbindgen_bigint_from_u128(hi: u64, lo: u64) -> u32 {
    unimplemented!("__wbindgen_bigint_from_u128")
}
pub unsafe fn __wbindgen_symbol_named_new(ptr: *const u8, len: usize) -> u32 {
    unimplemented!("__wbindgen_symbol_named_new")
}
pub unsafe fn __wbindgen_symbol_anonymous_new() -> u32 {
    unimplemented!("__wbindgen_symbol_anonymous_new")
}

pub unsafe fn __wbindgen_externref_heap_live_count() -> u32 {
    unimplemented!("__wbindgen_externref_heap_live_count")
}

pub unsafe fn __wbindgen_is_null(idx: u32) -> u32 {
    unimplemented!("__wbindgen_is_null")
}
pub unsafe fn __wbindgen_is_undefined(idx: u32) -> u32 {
    unimplemented!("__wbindgen_is_undefined")
}
pub unsafe fn __wbindgen_is_symbol(idx: u32) -> u32 {
    unimplemented!("__wbindgen_is_symbol")
}
pub unsafe fn __wbindgen_is_object(idx: u32) -> u32 {
    unimplemented!("__wbindgen_is_object")
}
pub unsafe fn __wbindgen_is_array(idx: u32) -> u32 {
    unimplemented!("__wbindgen_is_array")
}
pub unsafe fn __wbindgen_is_function(idx: u32) -> u32 {
    unimplemented!("__wbindgen_is_function")
}
pub unsafe fn __wbindgen_is_string(idx: u32) -> u32 {
    unimplemented!("__wbindgen_is_string")
}
pub unsafe fn __wbindgen_is_bigint(idx: u32) -> u32 {
    unimplemented!("__wbindgen_is_bigint")
}
pub unsafe fn __wbindgen_typeof(idx: u32) -> u32 {
    unimplemented!("__wbindgen_typeof")
}

pub unsafe fn __wbindgen_in(prop: u32, obj: u32) -> u32 {
    unimplemented!("__wbindgen_in")
}

pub unsafe fn __wbindgen_is_falsy(idx: u32) -> u32 {
    unimplemented!("__wbindgen_is_falsy")
}
pub unsafe fn __wbindgen_as_number(idx: u32) -> f64 {
    unimplemented!("__wbindgen_as_number")
}
pub unsafe fn __wbindgen_try_into_number(idx: u32) -> u32 {
    unimplemented!("__wbindgen_try_into_number")
}
pub unsafe fn __wbindgen_neg(idx: u32) -> u32 {
    unimplemented!("__wbindgen_neg")
}
pub unsafe fn __wbindgen_bit_and(a: u32, b: u32) -> u32 {
    unimplemented!("__wbindgen_bit_and")
}
pub unsafe fn __wbindgen_bit_or(a: u32, b: u32) -> u32 {
    unimplemented!("__wbindgen_bit_or")
}
pub unsafe fn __wbindgen_bit_xor(a: u32, b: u32) -> u32 {
    unimplemented!("__wbindgen_bit_xor")
}
pub unsafe fn __wbindgen_bit_not(idx: u32) -> u32 {
    unimplemented!("__wbindgen_bit_not")
}
pub unsafe fn __wbindgen_shl(a: u32, b: u32) -> u32 {
    unimplemented!("__wbindgen_shl")
}
pub unsafe fn __wbindgen_shr(a: u32, b: u32) -> u32 {
    unimplemented!("__wbindgen_shr")
}
pub unsafe fn __wbindgen_unsigned_shr(a: u32, b: u32) -> u32 {
    unimplemented!("__wbindgen_unsigned_shr")
}
pub unsafe fn __wbindgen_add(a: u32, b: u32) -> u32 {
    unimplemented!("__wbindgen_add")
}
pub unsafe fn __wbindgen_sub(a: u32, b: u32) -> u32 {
    unimplemented!("__wbindgen_sub")
}
pub unsafe fn __wbindgen_div(a: u32, b: u32) -> u32 {
    unimplemented!("__wbindgen_div")
}
pub unsafe fn __wbindgen_checked_div(a: u32, b: u32) -> u32 {
    unimplemented!("__wbindgen_checked_div")
}
pub unsafe fn __wbindgen_mul(a: u32, b: u32) -> u32 {
    unimplemented!("__wbindgen_mul")
}
pub unsafe fn __wbindgen_rem(a: u32, b: u32) -> u32 {
    unimplemented!("__wbindgen_rem")
}
pub unsafe fn __wbindgen_pow(a: u32, b: u32) -> u32 {
    unimplemented!("__wbindgen_pow")
}
pub unsafe fn __wbindgen_lt(a: u32, b: u32) -> u32 {
    unimplemented!("__wbindgen_lt")
}
pub unsafe fn __wbindgen_le(a: u32, b: u32) -> u32 {
    unimplemented!("__wbindgen_le")
}
pub unsafe fn __wbindgen_ge(a: u32, b: u32) -> u32 {
    unimplemented!("__wbindgen_ge")
}
pub unsafe fn __wbindgen_gt(a: u32, b: u32) -> u32 {
    unimplemented!("__wbindgen_gt")
}

pub unsafe fn __wbindgen_number_get(idx: u32) -> WasmRet<Option<f64>> {
    unimplemented!("__wbindgen_number_get")
}
pub unsafe fn __wbindgen_boolean_get(idx: u32) -> u32 {
    unimplemented!("__wbindgen_boolean_get")
}
pub unsafe fn __wbindgen_string_get(idx: u32) -> WasmSlice {
    unimplemented!("__wbindgen_string_get")
}
pub unsafe fn __wbindgen_bigint_get_as_i64(idx: u32) -> WasmRet<Option<i64>> {
    unimplemented!("__wbindgen_bigint_get_as_i64")
}

pub unsafe fn __wbindgen_debug_string(ret: *mut [usize; 2], idx: u32) -> () {
    unimplemented!("__wbindgen_debug_string")
}

pub unsafe fn __wbindgen_throw(a: *const u8, b: usize) -> ! {
    unimplemented!("__wbindgen_throw")
}
pub unsafe fn __wbindgen_rethrow(a: u32) -> ! {
    unimplemented!("__wbindgen_rethrow")
}
pub unsafe fn __wbindgen_error_new(a: *const u8, b: usize) -> u32 {
    unimplemented!("__wbindgen_error_new")
}

pub unsafe fn __wbindgen_cb_drop(idx: u32) -> u32 {
    unimplemented!("__wbindgen_cb_drop")
}

pub unsafe fn __wbindgen_describe(v: u32) -> () {
    unimplemented!("__wbindgen_describe")
}
pub unsafe fn __wbindgen_describe_closure(a: u32, b: u32, c: u32) -> u32 {
    unimplemented!("__wbindgen_describe_closure")
}

pub unsafe fn __wbindgen_json_parse(ptr: *const u8, len: usize) -> u32 {
    unimplemented!("__wbindgen_json_parse")
}
pub unsafe fn __wbindgen_json_serialize(idx: u32) -> WasmSlice {
    unimplemented!("__wbindgen_json_serialize")
}
pub unsafe fn __wbindgen_jsval_eq(a: u32, b: u32) -> u32 {
    unimplemented!("__wbindgen_jsval_eq")
}
pub unsafe fn __wbindgen_jsval_loose_eq(a: u32, b: u32) -> u32 {
    unimplemented!("__wbindgen_jsval_loose_eq")
}

pub unsafe fn __wbindgen_copy_to_typed_array(ptr: *const u8, len: usize, idx: u32) -> () {
    unimplemented!("__wbindgen_copy_to_typed_array")
}

pub unsafe fn __wbindgen_not(idx: u32) -> u32 {
    unimplemented!("__wbindgen_not")
}

pub unsafe fn __wbindgen_exports() -> u32 {
    unimplemented!("__wbindgen_exports")
}
pub unsafe fn __wbindgen_memory() -> u32 {
    unimplemented!("__wbindgen_memory")
}
pub unsafe fn __wbindgen_module() -> u32 {
    unimplemented!("__wbindgen_module")
}
pub unsafe fn __wbindgen_function_table() -> u32 {
    unimplemented!("__wbindgen_function_table")
}
