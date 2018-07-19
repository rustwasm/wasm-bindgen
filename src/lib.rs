//! Runtime support for the `wasm-bindgen` tool
//!
//! This crate contains the runtime support necessary for `wasm-bindgen` the
//! attribute and tool. Crates pull in the `#[wasm_bindgen]` attribute through
//! this crate and this crate also provides JS bindings through the `JsValue`
//! interface.

#![feature(use_extern_macros, wasm_import_module, unsize)]
#![cfg_attr(feature = "js_globals", feature(use_extern_macros))]
#![no_std]

#[cfg(feature = "serde-serialize")]
extern crate serde;
#[cfg(feature = "serde-serialize")]
extern crate serde_json;

extern crate wasm_bindgen_macro;

use core::cell::UnsafeCell;
use core::fmt;
use core::ops::Deref;
use core::ptr;

use convert::FromWasmAbi;

macro_rules! if_std {
    ($($i:item)*) => ($(
        #[cfg(feature = "std")] $i
    )*)
}

/// A module which is typically glob imported from:
///
/// ```
/// use wasm_bindgen::prelude::*;
/// ```
pub mod prelude {
    pub use wasm_bindgen_macro::wasm_bindgen;
    pub use JsValue;

    if_std! {
        pub use closure::Closure;
    }
}

pub mod convert;
pub mod describe;
#[cfg(feature = "js_globals")]
pub mod js;

#[cfg(feature = "js_globals")]
mod wasm_bindgen {
    pub use super::*;
}

if_std! {
    extern crate std;
    use std::prelude::v1::*;
    pub mod closure;
}

/// Representation of an object owned by JS.
///
/// A `JsValue` doesn't actually live in Rust right now but actually in a table
/// owned by the `wasm-bindgen` generated JS glue code. Eventually the ownership
/// will transfer into wasm directly and this will likely become more efficient,
/// but for now it may be slightly slow.
pub struct JsValue {
    idx: u32,
}

const JSIDX_NULL: u32 = 0;
const JSIDX_UNDEFINED: u32 = 2;
const JSIDX_TRUE: u32 = 4;
const JSIDX_FALSE: u32 = 6;
const JSIDX_RESERVED: u32 = 8;

impl JsValue {
    /// The `null` JS value constant.
    pub const NULL: JsValue = JsValue { idx: JSIDX_NULL };

    /// The `undefined` JS value constant.
    pub const UNDEFINED: JsValue = JsValue {
        idx: JSIDX_UNDEFINED,
    };

    /// The `true` JS value constant.
    pub const TRUE: JsValue = JsValue { idx: JSIDX_TRUE };

    /// The `false` JS value constant.
    pub const FALSE: JsValue = JsValue { idx: JSIDX_FALSE };

    /// Creates a new JS value which is a string.
    ///
    /// The utf-8 string provided is copied to the JS heap and the string will
    /// be owned by the JS garbage collector.
    pub fn from_str(s: &str) -> JsValue {
        unsafe {
            JsValue {
                idx: __wbindgen_string_new(s.as_ptr(), s.len()),
            }
        }
    }

    /// Creates a new JS value which is a number.
    ///
    /// This function creates a JS value representing a number (a heap
    /// allocated number) and returns a handle to the JS version of it.
    pub fn from_f64(n: f64) -> JsValue {
        unsafe {
            JsValue {
                idx: __wbindgen_number_new(n),
            }
        }
    }

    /// Creates a new JS value which is a boolean.
    ///
    /// This function creates a JS object representing a boolean (a heap
    /// allocated boolean) and returns a handle to the JS version of it.
    pub fn from_bool(b: bool) -> JsValue {
        JsValue {
            idx: if b { JSIDX_TRUE } else { JSIDX_FALSE },
        }
    }

    /// Creates a new JS value representing `undefined`.
    pub fn undefined() -> JsValue {
        JsValue {
            idx: JSIDX_UNDEFINED,
        }
    }

    /// Creates a new JS value representing `null`.
    pub fn null() -> JsValue {
        JsValue { idx: JSIDX_NULL }
    }

    /// Creates a new JS symbol with the optional description specified.
    ///
    /// This function will invoke the `Symbol` constructor in JS and return the
    /// JS object corresponding to the symbol created.
    pub fn symbol(description: Option<&str>) -> JsValue {
        unsafe {
            let ptr = description.map(|s| s.as_ptr()).unwrap_or(ptr::null());
            let len = description.map(|s| s.len()).unwrap_or(0);
            JsValue {
                idx: __wbindgen_symbol_new(ptr, len),
            }
        }
    }

    /// Creates a new `JsValue` from the JSON serialization of the object `t`
    /// provided.
    ///
    /// This function will serialize the provided value `t` to a JSON string,
    /// send the JSON string to JS, parse it into a JS object, and then return
    /// a handle to the JS object. This is unlikely to be super speedy so it's
    /// not recommended for large payloads, but it's a nice to have in some
    /// situations!
    ///
    /// Usage of this API requires activating the `serde-serialize` feature of
    /// the `wasm-bindgen` crate.
    ///
    /// # Errors
    ///
    /// Returns any error encountered when serializing `T` into JSON.
    #[cfg(feature = "serde-serialize")]
    pub fn from_serde<T>(t: &T) -> serde_json::Result<JsValue>
    where
        T: serde::ser::Serialize + ?Sized,
    {
        let s = serde_json::to_string(t)?;
        unsafe {
            Ok(JsValue {
                idx: __wbindgen_json_parse(s.as_ptr(), s.len()),
            })
        }
    }

    /// Invokes `JSON.stringify` on this value and then parses the resulting
    /// JSON into an arbitrary Rust value.
    ///
    /// This function will first call `JSON.stringify` on the `JsValue` itself.
    /// The resulting string is then passed into Rust which then parses it as
    /// JSON into the resulting value.
    ///
    /// Usage of this API requires activating the `serde-serialize` feature of
    /// the `wasm-bindgen` crate.
    ///
    /// # Errors
    ///
    /// Returns any error encountered when parsing the JSON into a `T`.
    #[cfg(feature = "serde-serialize")]
    pub fn into_serde<T>(&self) -> serde_json::Result<T>
    where
        T: for<'a> serde::de::Deserialize<'a>,
    {
        unsafe {
            let mut ptr = ptr::null_mut();
            let len = __wbindgen_json_serialize(self.idx, &mut ptr);
            let s = Vec::from_raw_parts(ptr, len, len);
            let s = String::from_utf8_unchecked(s);
            serde_json::from_str(&s)
        }
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

    /// Tests whether this JS value is a JS string.
    pub fn is_string(&self) -> bool {
        unsafe { __wbindgen_is_string(self.idx) == 1 }
    }

    /// If this JS value is a string value, this function copies the JS string
    /// value into wasm linear memory, encoded as UTF-8, and returns it as a
    /// Rust `String`.
    ///
    /// To avoid the copying and re-encoding, consider the `as_js_string()`
    /// method instead.
    ///
    /// If this JS value is not an instance of a string or if it's not valid
    /// utf-8 then this returns `None`.
    #[cfg(feature = "std")]
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
        unsafe { __wbindgen_is_null(self.idx) == 1 }
    }

    /// Tests whether this JS value is `undefined`
    pub fn is_undefined(&self) -> bool {
        unsafe { __wbindgen_is_undefined(self.idx) == 1 }
    }

    /// Tests whether the type of this JS value is `symbol`
    pub fn is_symbol(&self) -> bool {
        unsafe { __wbindgen_is_symbol(self.idx) == 1 }
    }

    /// Tests whether `typeof self == "object" && self !== null`.
    pub fn is_object(&self) -> bool {
        unsafe { __wbindgen_is_object(self.idx) == 1 }
    }

    /// Tests whether the type of this JS value is `function`.
    pub fn is_function(&self) -> bool {
        unsafe { __wbindgen_is_function(self.idx) == 1 }
    }
}

impl PartialEq for JsValue {
    fn eq(&self, other: &JsValue) -> bool {
        unsafe { __wbindgen_jsval_eq(self.idx, other.idx) != 0 }
    }
}

impl<'a> From<&'a str> for JsValue {
    fn from(s: &'a str) -> JsValue {
        JsValue::from_str(s)
    }
}

if_std! {
    impl<'a> From<&'a String> for JsValue {
        fn from(s: &'a String) -> JsValue {
            JsValue::from_str(s)
        }
    }
}

impl From<bool> for JsValue {
    fn from(s: bool) -> JsValue {
        JsValue::from_bool(s)
    }
}

macro_rules! numbers {
    ($($n:ident)*) => ($(
        impl From<$n> for JsValue {
            fn from(n: $n) -> JsValue {
                JsValue::from_f64(n.into())
            }
        }
    )*)
}

numbers! { i8 u8 i16 u16 i32 u32 f32 f64 }

macro_rules! externs {
    ($(fn $name:ident($($args:tt)*) -> $ret:ty;)*) => (
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        #[wasm_import_module = "__wbindgen_placeholder__"]
        extern {
            $(fn $name($($args)*) -> $ret;)*
        }

        $(
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            #[allow(unused_variables)]
            unsafe extern fn $name($($args)*) -> $ret {
                panic!("function not implemented on non-wasm32 targets")
            }
        )*
    )
}

externs! {
    fn __wbindgen_object_clone_ref(idx: u32) -> u32;
    fn __wbindgen_object_drop_ref(idx: u32) -> ();
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
    fn __wbindgen_is_object(idx: u32) -> u32;
    fn __wbindgen_is_function(idx: u32) -> u32;
    fn __wbindgen_is_string(idx: u32) -> u32;
    fn __wbindgen_string_get(idx: u32, len: *mut usize) -> *mut u8;
    fn __wbindgen_throw(a: *const u8, b: usize) -> !;

    fn __wbindgen_cb_drop(idx: u32) -> ();
    fn __wbindgen_cb_forget(idx: u32) -> ();

    fn __wbindgen_describe(v: u32) -> ();

    fn __wbindgen_json_parse(ptr: *const u8, len: usize) -> u32;
    fn __wbindgen_json_serialize(idx: u32, ptr: *mut *mut u8) -> usize;
    fn __wbindgen_jsval_eq(a: u32, b: u32) -> u32;
}

impl Clone for JsValue {
    fn clone(&self) -> JsValue {
        unsafe {
            let idx = __wbindgen_object_clone_ref(self.idx);
            JsValue { idx }
        }
    }
}

impl fmt::Debug for JsValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(n) = self.as_f64() {
            return n.fmt(f);
        }
        #[cfg(feature = "std")]
        {
            if let Some(n) = self.as_string() {
                return n.fmt(f);
            }
        }
        if let Some(n) = self.as_bool() {
            return n.fmt(f);
        }
        if self.is_null() {
            return fmt::Display::fmt("null", f);
        }
        if self.is_undefined() {
            return fmt::Display::fmt("undefined", f);
        }
        if self.is_symbol() {
            return fmt::Display::fmt("Symbol(..)", f);
        }
        fmt::Display::fmt("[object]", f)
    }
}

impl Drop for JsValue {
    fn drop(&mut self) {
        unsafe {
            // if the first bit is set then this is a stack value, so we for
            // sure need to drop it. Otherwise if this is one of the special
            // reserved values there's no need to drop it.
            if (self.idx & 1) == 1 || self.idx >= JSIDX_RESERVED {
                __wbindgen_object_drop_ref(self.idx);
            }
        }
    }
}

/// Wrapper type for imported statics.
///
/// This type is used whenever a `static` is imported from a JS module, for
/// example this import:
///
/// ```ignore
/// #[wasm_bindgen]
/// extern {
///     static console: JsValue;
/// }
/// ```
///
/// will generate in Rust a value that looks like:
///
/// ```ignore
/// static console: JsStatic<JsValue> = ...;
/// ```
///
/// This type implements `Deref` to the inner type so it's typically used as if
/// it were `&T`.
pub struct JsStatic<T: 'static> {
    #[doc(hidden)]
    pub __inner: &'static UnsafeCell<Option<T>>,
    #[doc(hidden)]
    pub __init: fn() -> T,
}

unsafe impl<T: Sync> Sync for JsStatic<T> {}
unsafe impl<T: Send> Send for JsStatic<T> {}

impl<T: FromWasmAbi + 'static> Deref for JsStatic<T> {
    type Target = T;
    fn deref(&self) -> &T {
        unsafe {
            // Ideally we want to use `get_or_insert_with` here but
            // unfortunately that has subpar codegen for now.
            //
            // If we get past the `Some` branch here LLVM statically
            // knows that we're `None`, but the after the call to the `__init`
            // function LLVM can no longer know this because `__init` could
            // recursively call this function again (aka if JS came back to Rust
            // and Rust referenced this static).
            //
            // We know, however, that cannot happen. As a result we can
            // conclude that even after the call to `__init` our `ptr` still
            // points to `None` (and a debug assertion to this effect). Then
            // using `ptr::write` should tell rustc to not run destuctors
            // (as one isn't there) and this should tighten up codegen for
            // `JsStatic` a bit as well.
            let ptr = self.__inner.get();
            if let Some(ref t) = *ptr {
                return t;
            }
            let init = Some((self.__init)());
            debug_assert!((*ptr).is_none());
            ptr::write(ptr, init);
            (*ptr).as_ref().unwrap()
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
    unsafe {
        __wbindgen_throw(s.as_ptr(), s.len());
    }
}

#[doc(hidden)]
pub mod __rt {
    use core::cell::{Cell, UnsafeCell};
    use core::ops::{Deref, DerefMut};
    pub extern crate core;
    #[cfg(feature = "std")]
    pub extern crate std;

    #[macro_export]
    #[doc(hidden)]
    #[cfg(feature = "std")]
    macro_rules! __wbindgen_if_not_std {
        ($($i:item)*) => {};
    }

    #[macro_export]
    #[doc(hidden)]
    #[cfg(not(feature = "std"))]
    macro_rules! __wbindgen_if_not_std {
        ($($i:item)*) => ($($i)*)
    }

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
    pub struct WasmRefCell<T: ?Sized> {
        borrow: Cell<usize>,
        value: UnsafeCell<T>,
    }

    impl<T: ?Sized> WasmRefCell<T> {
        pub fn new(value: T) -> WasmRefCell<T>
        where
            T: Sized,
        {
            WasmRefCell {
                value: UnsafeCell::new(value),
                borrow: Cell::new(0),
            }
        }

        pub fn get_mut(&mut self) -> &mut T {
            unsafe { &mut *self.value.get() }
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

        pub fn into_inner(self) -> T
        where
            T: Sized,
        {
            self.value.into_inner()
        }
    }

    pub struct Ref<'b, T: ?Sized + 'b> {
        value: &'b T,
        borrow: &'b Cell<usize>,
    }

    impl<'b, T: ?Sized> Deref for Ref<'b, T> {
        type Target = T;

        #[inline]
        fn deref(&self) -> &T {
            self.value
        }
    }

    impl<'b, T: ?Sized> Drop for Ref<'b, T> {
        fn drop(&mut self) {
            self.borrow.set(self.borrow.get() - 1);
        }
    }

    pub struct RefMut<'b, T: ?Sized + 'b> {
        value: &'b mut T,
        borrow: &'b Cell<usize>,
    }

    impl<'b, T: ?Sized> Deref for RefMut<'b, T> {
        type Target = T;

        #[inline]
        fn deref(&self) -> &T {
            self.value
        }
    }

    impl<'b, T: ?Sized> DerefMut for RefMut<'b, T> {
        #[inline]
        fn deref_mut(&mut self) -> &mut T {
            self.value
        }
    }

    impl<'b, T: ?Sized> Drop for RefMut<'b, T> {
        fn drop(&mut self) {
            self.borrow.set(0);
        }
    }

    fn borrow_fail() -> ! {
        super::throw(
            "recursive use of an object detected which would lead to \
             unsafe aliasing in rust",
        );
    }

    if_std! {
        use std::alloc::{System, GlobalAlloc, Layout};
        use std::mem;

        #[no_mangle]
        pub extern fn __wbindgen_malloc(size: usize) -> *mut u8 {
            let align = mem::align_of::<usize>();
            if let Ok(layout) = Layout::from_size_align(size, align) {
                unsafe {
                    let ptr = System.alloc(layout);
                    if !ptr.is_null() {
                        return ptr
                    }
                }
            }

            super::throw("invalid malloc request");
        }

        #[no_mangle]
        pub unsafe extern fn __wbindgen_free(ptr: *mut u8, size: usize) {
            let align = mem::align_of::<usize>();
            let layout = Layout::from_size_align_unchecked(size, align);
            System.dealloc(ptr, layout);
        }
    }

    pub const GLOBAL_STACK_CAP: usize = 16;

    // Increase the alignment to 8 here because this can be used as a
    // BigUint64Array pointer base which requires alignment 8
    #[repr(align(8))]
    struct GlobalData([u32; GLOBAL_STACK_CAP]);

    static mut GLOBAL_STACK: GlobalData = GlobalData([0; GLOBAL_STACK_CAP]);

    #[no_mangle]
    pub unsafe extern "C" fn __wbindgen_global_argument_ptr() -> *mut u32 {
        GLOBAL_STACK.0.as_mut_ptr()
    }

    /// This is a curious function necessary to get wasm-bindgen working today,
    /// and it's a bit of an unfortunate hack.
    ///
    /// The general problem is that somehow we need the above two symbols to
    /// exist in the final output binary (__wbindgen_malloc and
    /// __wbindgen_free). These symbols may be called by JS for various
    /// bindings, so we for sure need to make sure they're exported.
    ///
    /// The problem arises, though, when what if no Rust code uses the symbols?
    /// For all intents and purposes it looks to LLVM and the linker like the
    /// above two symbols are dead code, so they're completely discarded!
    ///
    /// Specifically what happens is this:
    ///
    /// * The above two symbols are generated into some object file inside of
    ///   libwasm_bindgen.rlib
    /// * The linker, LLD, will not load this object file unless *some* symbol
    ///   is loaded from the object. In this case, if the Rust code never calls
    ///   __wbindgen_malloc or __wbindgen_free then the symbols never get linked
    ///   in.
    /// * Later when `wasm-bindgen` attempts to use the symbols they don't
    ///   exist, causing an error.
    ///
    /// This function is a weird hack for this problem. We inject a call to this
    /// function in all generated code. Usage of this function should then
    /// ensure that the above two intrinsics are translated.
    ///
    /// Due to how rustc creates object files this function (and anything inside
    /// it) will be placed into the same object file as the two intrinsics
    /// above. That means if this function is called and referenced we'll pull
    /// in the object file and link the intrinsics.
    ///
    /// Note that this is an #[inline] function to remove the function call
    /// overhead we inject in functions, but right now it's unclear how to do
    /// this in a zero-cost fashion. The lowest cost seems to be generating a
    /// store that can't be optimized away (to a global), which is listed below.
    ///
    /// Ideas for how to improve this are most welcome!
    #[inline]
    pub fn link_mem_intrinsics() {
        // the above symbols only exist with the `std` feature enabled.
        if !cfg!(feature = "std") {
            return
        }

        use core::sync::atomic::*;
        static FOO: AtomicUsize = ATOMIC_USIZE_INIT;
        FOO.store(0, Ordering::SeqCst);
    }
}
