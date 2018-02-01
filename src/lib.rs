//! Runtime support for the `wasm-bindgen` tool
//!
//! This crate contains the runtime support necessary for `wasm-bindgen` the
//! macro and tool. Crates pull in the `wasm_bindgen!` macro through this crate
//! and this crate also provides JS bindings through the `JsObject` interface.

#![feature(use_extern_macros)]

extern crate wasm_bindgen_macro;

use std::mem;
use std::ptr;

/// A module which is typically glob imported from:
///
/// ```
/// use wasm_bindgen::prelude::*;
/// ```
pub mod prelude {
    pub use wasm_bindgen_macro::wasm_bindgen;
    pub use JsObject;
}

/// Representation of an object owned by JS.
///
/// A `JsObject` doesn't actually live in Rust right now but actually in a table
/// owned by the `wasm-bindgen` generated JS glue code. Eventually the ownership
/// will transfer into wasm directly and this will likely become more efficient,
/// but for now it may be slightly slow.
pub struct JsObject {
    idx: u32,
}

impl JsObject {
    /// Creates a new JS value which is a string.
    ///
    /// The utf-8 string provided is copied to the JS heap and the string will
    /// be owned by the JS garbage collector.
    pub fn from_str(s: &str) -> JsObject {
        unsafe {
            JsObject::__from_idx(__wbindgen_string_new(s.as_ptr(), s.len()))
        }
    }

    /// Creates a new JS value which is a number.
    ///
    /// This function creates a JS value representing a number (a heap
    /// allocated number) and returns a handle to the JS version of it.
    pub fn from_f64(n: f64) -> JsObject {
        unsafe {
            JsObject::__from_idx(__wbindgen_number_new(n))
        }
    }

    /// Creates a new JS value which is a boolean.
    ///
    /// This function creates a JS object representing a boolean (a heap
    /// allocated boolean) and returns a handle to the JS version of it.
    pub fn from_bool(b: bool) -> JsObject {
        unsafe {
            JsObject::__from_idx(__wbindgen_boolean_new(b as u32))
        }
    }

    /// Creates a new JS value representing `undefined`.
    pub fn undefined() -> JsObject {
        unsafe {
            JsObject::__from_idx(__wbindgen_undefined_new())
        }
    }

    /// Creates a new JS value representing `null`.
    pub fn null() -> JsObject {
        unsafe {
            JsObject::__from_idx(__wbindgen_null_new())
        }
    }

    /// Creates a new JS symbol with the optional description specified.
    ///
    /// This function will invoke the `Symbol` constructor in JS and return the
    /// JS object corresponding to the symbol created.
    pub fn symbol(description: Option<&str>) -> JsObject {
        unsafe {
            let ptr = description.map(|s| s.as_ptr()).unwrap_or(ptr::null());
            let len = description.map(|s| s.len()).unwrap_or(0);
            JsObject::__from_idx(__wbindgen_symbol_new(ptr, len))
        }
    }

    #[doc(hidden)]
    pub fn __from_idx(idx: u32) -> JsObject {
        JsObject { idx }
    }

    #[doc(hidden)]
    pub fn __get_idx(&self) -> u32 {
        self.idx
    }

    #[doc(hidden)]
    pub fn __into_idx(self) -> u32 {
        let ret = self.idx;
        mem::forget(self);
        return ret
    }

    /// Returns the `f64` value of this JS value if it's an instance of a
    /// number.
    ///
    /// If this JS value is not an instance of a number then this returns
    /// `None`.
    pub fn as_f64(&self) -> Option<f64> {
        let mut invalid = 0;
        unsafe {
            let ret = __wbindgen_number_get(self.idx, &mut invalid);
            if invalid == 1 {
                None
            } else {
                Some(ret)
            }
        }
    }

    /// Returns the `String` of this JS value if it's an instance of a
    /// string and it's valid utf-8.
    ///
    /// If this JS value is not an instance of a string or if it's not valid
    /// utf-8 then this returns `None`.
    pub fn as_string(&self) -> Option<String> {
        unsafe {
            let mut len = 0;
            let ptr = __wbindgen_string_get(self.idx, &mut len);
            if ptr.is_null() {
                None
            } else {
                let data = Vec::from_raw_parts(ptr, len, len);
                Some(String::from_utf8_unchecked(data))
            }
        }
    }

    /// Returns the `bool` value of this JS value if it's an instance of a
    /// boolean.
    ///
    /// If this JS value is not an instance of a boolean then this returns
    /// `None`.
    pub fn as_bool(&self) -> Option<bool> {
        unsafe {
            match __wbindgen_boolean_get(self.idx) {
                0 => Some(false),
                1 => Some(true),
                _ => None,
            }
        }
    }

    /// Tests whether this JS value is `null`
    pub fn is_null(&self) -> bool {
        unsafe {
            __wbindgen_is_null(self.idx) == 1
        }
    }

    /// Tests whether this JS value is `undefined`
    pub fn is_undefined(&self) -> bool {
        unsafe {
            __wbindgen_is_undefined(self.idx) == 1
        }
    }

    /// Tests whether the type of this JS value is `symbol`
    pub fn is_symbol(&self) -> bool {
        unsafe {
            __wbindgen_is_symbol(self.idx) == 1
        }
    }
}

impl<'a> From<&'a str> for JsObject {
    fn from(s: &'a str) -> JsObject {
        JsObject::from_str(s)
    }
}

impl<'a> From<&'a String> for JsObject {
    fn from(s: &'a String) -> JsObject {
        JsObject::from_str(s)
    }
}

impl From<bool> for JsObject {
    fn from(s: bool) -> JsObject {
        JsObject::from_bool(s)
    }
}

macro_rules! numbers {
    ($($n:ident)*) => ($(
        impl From<$n> for JsObject {
            fn from(n: $n) -> JsObject {
                JsObject::from_f64(n.into())
            }
        }
    )*)
}

numbers! { i8 u8 i16 u16 i32 u32 f32 f64 }

extern {
    fn __wbindgen_object_clone_ref(idx: u32) -> u32;
    fn __wbindgen_object_drop_ref(idx: u32);
    fn __wbindgen_string_new(ptr: *const u8, len: usize) -> u32;
    fn __wbindgen_number_new(f: f64) -> u32;
    fn __wbindgen_number_get(idx: u32, invalid: *mut u8) -> f64;
    fn __wbindgen_null_new() -> u32;
    fn __wbindgen_undefined_new() -> u32;
    fn __wbindgen_is_null(idx: u32) -> u32;
    fn __wbindgen_is_undefined(idx: u32) -> u32;
    fn __wbindgen_boolean_new(val: u32) -> u32;
    fn __wbindgen_boolean_get(idx: u32) -> u32;
    fn __wbindgen_symbol_new(ptr: *const u8, len: usize) -> u32;
    fn __wbindgen_is_symbol(idx: u32) -> u32;
    fn __wbindgen_string_get(idx: u32, len: *mut usize) -> *mut u8;
}

impl Clone for JsObject {
    fn clone(&self) -> JsObject {
        unsafe {
            let idx = __wbindgen_object_clone_ref(self.idx);
            JsObject { idx }
        }
    }
}

impl Drop for JsObject {
    fn drop(&mut self) {
        unsafe {
            __wbindgen_object_drop_ref(self.idx);
        }
    }
}

/// Throws a JS exception.
///
/// This function will throw a JS exception with the message provided. The
/// function will not return as the wasm stack will be popped when the exception
/// is thrown.
#[cold]
#[inline(never)]
pub fn throw(s: &str) -> ! {
    extern {
        fn __wbindgen_throw(a: *const u8, b: usize) -> !;
    }
    unsafe {
        __wbindgen_throw(s.as_ptr(), s.len());
    }
}

#[doc(hidden)]
pub mod __rt {
    use std::cell::{Cell, UnsafeCell};
    use std::ops::{Deref, DerefMut};

    #[inline]
    pub fn assert_not_null<T>(s: *mut T) {
        if s.is_null() {
            throw_null();
        }
    }

    #[cold]
    #[inline(never)]
    fn throw_null() -> ! {
        super::throw("null pointer passed to rust");
    }

    /// A vendored version of `RefCell` from the standard library.
    ///
    /// Now why, you may ask, would we do that? Surely `RefCell` in libstd is
    /// quite good. And you're right, it is indeed quite good! Functionally
    /// nothing more is needed from `RefCell` in the standard library but for
    /// now this crate is also sort of optimizing for compiled code size.
    ///
    /// One major factor to larger binaries in Rust is when a panic happens.
    /// Panicking in the standard library involves a fair bit of machinery
    /// (formatting, panic hooks, synchronization, etc). It's all worthwhile if
    /// you need it but for something like `WasmRefCell` here we don't actually
    /// need all that!
    ///
    /// This is just a wrapper around all Rust objects passed to JS intended to
    /// guard accidental reentrancy, so this vendored version is intended solely
    /// to not panic in libstd. Instead when it "panics" it calls our `throw`
    /// function in this crate which raises an error in JS.
    pub struct WasmRefCell<T> {
        borrow: Cell<usize>,
        value: UnsafeCell<T>,
    }

    impl<T> WasmRefCell<T> {
        pub fn new(value: T) -> WasmRefCell<T> {
            WasmRefCell {
                value: UnsafeCell::new(value),
                borrow: Cell::new(0),
            }
        }

        pub fn get_mut(&mut self) -> &mut T {
            unsafe {
                &mut *self.value.get()
            }
        }

        pub fn borrow(&self) -> Ref<T> {
            unsafe {
                if self.borrow.get() == usize::max_value() {
                    borrow_fail();
                }
                self.borrow.set(self.borrow.get() + 1);
                Ref {
                    value: &*self.value.get(),
                    borrow: &self.borrow,
                }
            }
        }

        pub fn borrow_mut(&self) -> RefMut<T> {
            unsafe {
                if self.borrow.get() != 0 {
                    borrow_fail();
                }
                self.borrow.set(usize::max_value());
                RefMut {
                    value: &mut *self.value.get(),
                    borrow: &self.borrow,
                }
            }
        }

        pub fn into_inner(self) -> T {
            unsafe { self.value.into_inner() }
        }
    }

    pub struct Ref<'b, T: 'b> {
        value: &'b T,
        borrow: &'b Cell<usize>,
    }

    impl<'b, T> Deref for Ref<'b, T> {
        type Target = T;

        #[inline]
        fn deref(&self) -> &T {
            self.value
        }
    }

    impl<'b, T> Drop for Ref<'b, T> {
        fn drop(&mut self) {
            self.borrow.set(self.borrow.get() - 1);
        }
    }

    pub struct RefMut<'b, T: 'b> {
        value: &'b mut T,
        borrow: &'b Cell<usize>,
    }

    impl<'b, T> Deref for RefMut<'b, T> {
        type Target = T;

        #[inline]
        fn deref(&self) -> &T {
            self.value
        }
    }

    impl<'b, T> DerefMut for RefMut<'b, T> {
        #[inline]
        fn deref_mut(&mut self) -> &mut T {
            self.value
        }
    }

    impl<'b, T> Drop for RefMut<'b, T> {
        fn drop(&mut self) {
            self.borrow.set(0);
        }
    }

    fn borrow_fail() -> ! {
        super::throw("recursive use of an object detected which would lead to \
                      unsafe aliasing in rust");
    }
}
