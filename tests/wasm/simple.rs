use std::ptr::{self, NonNull};

use wasm_bindgen::prelude::*;
use wasm_bindgen::{intern, unintern};
use wasm_bindgen_test::*;

#[wasm_bindgen(module = "tests/wasm/simple.js")]
extern "C" {
    fn test_add();
    fn test_string_arguments();
    fn test_return_a_string();
    fn test_wrong_types();
    fn test_other_exports_still_available();
    fn test_jsvalue_typeof();

    fn optional_str_none(a: Option<&str>);
    fn optional_str_some(a: Option<&str>);
    fn optional_slice_none(a: Option<&[u8]>);
    fn optional_slice_some(a: Option<&[u8]>);
    fn optional_string_none(a: Option<String>);
    fn optional_string_some(a: Option<String>);
    fn optional_string_some_empty(a: Option<String>);
    fn return_string_none() -> Option<String>;
    fn return_string_some() -> Option<String>;
    fn test_rust_optional();
    #[wasm_bindgen(js_name = import_export_same_name)]
    fn js_import_export_same_name();

    #[wasm_bindgen(js_name = RenamedInRust)]
    type Renamed;
    fn new_renamed() -> Renamed;

    fn test_string_roundtrip();

    fn test_raw_pointers();
    fn test_non_null();
}

#[wasm_bindgen_test]
fn add() {
    test_add();
}

#[wasm_bindgen]
pub fn simple_add(a: u32, b: u32) -> u32 {
    a + b
}

#[wasm_bindgen]
pub fn simple_add3(a: u32) -> u32 {
    a + 3
}

#[wasm_bindgen]
pub fn simple_get2(_b: bool) -> u32 {
    2
}

#[wasm_bindgen]
pub fn simple_return_and_take_bool(a: bool, b: bool) -> bool {
    a && b
}

#[wasm_bindgen]
pub fn simple_return_raw_pointer_u32(value: u32) -> *mut u32 {
    Box::into_raw(Box::new(value))
}

#[wasm_bindgen]
pub fn simple_return_raw_pointer_u8(value: u8) -> *const u8 {
    Box::into_raw(Box::new(value))
}

#[wasm_bindgen]
pub unsafe fn simple_raw_pointers_work(a: *mut u32, b: *const u8) -> *const u32 {
    (*a) = (*b) as u32;
    a
}

#[wasm_bindgen]
pub fn simple_return_option_null_pointer() -> Option<*const u32> {
    Some(ptr::null())
}

#[wasm_bindgen]
pub unsafe fn simple_option_raw_pointers_work(
    a: Option<*mut u32>,
    b: Option<*const u8>,
) -> Option<*const u32> {
    let a = a.and_then(|ptr| ptr.as_mut());
    let b = b.and_then(|ptr| ptr.as_ref());

    if let (Some(a), Some(b)) = (a, b) {
        *a = *b as u32;
        Some(a)
    } else {
        None
    }
}

#[wasm_bindgen_test]
fn raw_pointers() {
    test_raw_pointers();
}

#[wasm_bindgen]
pub fn simple_return_non_null() -> NonNull<u32> {
    NonNull::from(Box::leak(Box::new(42)))
}

#[wasm_bindgen]
pub fn simple_return_option_non_null(value: u32) -> Option<NonNull<u32>> {
    Some(NonNull::from(Box::leak(Box::new(value))))
}

#[wasm_bindgen]
pub unsafe fn simple_option_nonnull_work(a: Option<NonNull<u32>>) -> Option<u32> {
    a.map(|ptr| *Box::from_raw(ptr.as_ptr()))
}

#[wasm_bindgen_test]
fn non_null() {
    test_non_null();
}

#[wasm_bindgen_test]
fn string_arguments() {
    test_string_arguments();
}

#[wasm_bindgen]
pub fn simple_assert_foo_and_bar(a: &str, b: &str) {
    assert_eq!(a, "foo2");
    assert_eq!(b, "bar");
}

#[wasm_bindgen]
pub fn simple_assert_foo(a: &str) {
    assert_eq!(a, "foo");
}

#[wasm_bindgen_test]
fn return_a_string() {
    test_return_a_string();
}

#[wasm_bindgen]
pub fn simple_clone(a: &str) -> String {
    a.to_string()
}

#[wasm_bindgen]
pub fn simple_concat(a: &str, b: &str, c: i8) -> String {
    format!("{} {} {}", a, b, c)
}

#[wasm_bindgen_test]
fn wrong_types() {
    test_wrong_types();
}

#[wasm_bindgen]
pub fn simple_int(_a: u32) {}

#[wasm_bindgen]
pub fn simple_bool(_a: bool) {}

#[wasm_bindgen]
pub fn simple_str(_a: &str) {}

#[wasm_bindgen_test]
fn other_exports() {
    test_other_exports_still_available();
}

#[no_mangle]
pub extern "C" fn foo(_a: u32) {}

#[wasm_bindgen_test]
fn jsvalue_typeof() {
    test_jsvalue_typeof();
}

#[wasm_bindgen]
pub fn is_object(val: &JsValue) -> bool {
    val.is_object()
}

#[wasm_bindgen]
pub fn is_function(val: &JsValue) -> bool {
    val.is_function()
}

#[wasm_bindgen]
pub fn is_string(val: &JsValue) -> bool {
    val.is_string()
}

#[wasm_bindgen]
extern "C" {
    #[derive(Clone)]
    type Array;
    #[wasm_bindgen(constructor)]
    fn new() -> Array;
    #[wasm_bindgen(method, catch)]
    fn standardized_method_this_js_runtime_doesnt_implement_yet(
        this: &Array,
    ) -> Result<(), JsValue>;
}

#[wasm_bindgen_test]
fn binding_to_unimplemented_apis_doesnt_break_everything() {
    let array = Array::new();
    let res = array.standardized_method_this_js_runtime_doesnt_implement_yet();
    assert!(res.is_err());
}

#[wasm_bindgen_test]
fn optional_slices() {
    optional_str_none(None);
    optional_str_some(Some("x"));
    optional_str_some(Some(intern("x")));
    unintern("x");
    optional_str_some(Some("x"));
    optional_slice_none(None);
    optional_slice_some(Some(&[1, 2, 3]));
    optional_string_none(None);
    optional_string_some_empty(Some(String::new()));
    optional_string_some(Some("abcd".to_string()));

    assert_eq!(return_string_none(), None);
    assert_eq!(return_string_some(), Some("foo".to_string()));
    test_rust_optional();
}

#[wasm_bindgen]
pub fn take_optional_str_none(x: Option<String>) {
    assert!(x.is_none())
}
#[wasm_bindgen]
pub fn take_optional_str_some(x: Option<String>) {
    assert_eq!(x, Some(String::from("hello")));
}

#[wasm_bindgen]
pub fn return_optional_str_none() -> Option<String> {
    None
}

#[wasm_bindgen]
pub fn return_optional_str_some() -> Option<String> {
    Some("world".to_string())
}

#[wasm_bindgen_test]
fn renaming_imports_and_instanceof() {
    let null = JsValue::NULL;
    assert!(!null.is_instance_of::<Renamed>());

    let arr: JsValue = Array::new().into();
    assert!(!arr.is_instance_of::<Renamed>());

    let renamed: JsValue = new_renamed().into();
    assert!(renamed.is_instance_of::<Renamed>());
}

#[wasm_bindgen]
pub fn import_export_same_name() {
    js_import_export_same_name();
}

#[wasm_bindgen_test]
fn string_roundtrip() {
    test_string_roundtrip();
}

#[wasm_bindgen]
pub fn do_string_roundtrip(s: String) -> String {
    s
}

#[wasm_bindgen_test]
#[allow(clippy::redundant_clone)] // clone to increase heap live count
fn externref_heap_live_count() {
    let x = wasm_bindgen::externref_heap_live_count();
    let y = JsValue::null().clone();
    assert!(wasm_bindgen::externref_heap_live_count() > x);
    drop(y);
    assert_eq!(x, wasm_bindgen::externref_heap_live_count());
}
