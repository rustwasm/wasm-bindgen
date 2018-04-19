//! Support for long-lived closures in `wasm-bindgen`
//!
//! This module defines the `Closure` type which is used to pass "owned
//! closures" from Rust to JS. Some more details can be found on the `Closure`
//! type itself.

use std::cell::UnsafeCell;
use std::marker::Unsize;
use std::mem::{self, ManuallyDrop};
use std::prelude::v1::*;

use JsValue;
use convert::*;
use describe::*;

/// A handle to both a closure in Rust as well as JS closure which will invoke
/// the Rust closure.
///
/// A `Closure` is the primary way that a `'static` lifetime closure is
/// transferred from Rust to JS. `Closure` currently requires that the closures
/// it's created with have the `'static` lifetime in Rust for soundness reasons.
///
/// This type is a "handle" in the sense that whenever it is dropped it will
/// invalidate the JS closure that it refers to. Any usage of the closure in JS
/// after the `Closure` has been dropped will raise an exception. It's then up
/// to you to arrange for `Closure` to be properly deallocate at an appropriate
/// location in your program.
///
/// The type parameter on `Closure` is the type of closure that this represents.
/// Currently this can only be the `Fn` and `FnMut` traits with up to 7
/// arguments (and an optional return value). The arguments/return value of the
/// trait must be numbers like `u32` for now, although this restriction may be
/// lifted in the future!
///
/// # Example
///
/// ```rust,no_run
/// #[wasm_bindgen]
/// extern {
///     fn setTimeout(closure: &Closure<FnMut()>, time: u32);
///
///     #[wasm_bindgen(js_namespace = console)]
///     fn log(s: &str);
/// }
///
/// #[wasm_bindgen]
/// pub struct ClosureHandle(Closure<FnMut()>);
///
/// #[wasm_bindgen]
/// pub fn run() -> ClosureHandle {
///     // First up we use `Closure::new` to wrap up a Rust closure and create
///     a JS closure.
///     let cb = Closure::new(|| {
///         log("timeout elapsed!");
///     });
///
///     // Next we pass this via reference to the `setTimeout` function, and
///     // `setTimeout` gets a handle to the corresponding JS closure.
///     setTimeout(&cb, 1_000);
///
///     // If we were to drop `cb` here it would cause an exception to be raised
///     // when the timeout elapses. Instead we *return* our handle back to JS
///     // so JS can tell us later when it would like to deallocate this handle.
///     ClosureHandle(cb)
/// }
/// ```
pub struct Closure<T: ?Sized> {
    inner: UnsafeCell<Box<T>>,
    js: UnsafeCell<ManuallyDrop<JsValue>>,
}

impl<T> Closure<T>
    where T: ?Sized,
{
    /// Creates a new instance of `Closure` from the provided Rust closure.
    ///
    /// Note that the closure provided here, `F`, has a few requirements
    /// associated with it:
    ///
    /// * It must implement `Fn` or `FnMut`
    /// * It must be `'static`, aka no stack references (use the `move` keyword)
    /// * It can have at most 7 arguments
    /// * Its arguments and return values are all wasm types like u32/f64.
    ///
    /// This is unfortunately pretty restrictive for now but hopefully some of
    /// these restrictions can be lifted in the future!
    pub fn new<F>(t: F) -> Closure<T>
        where F: Unsize<T> + 'static
    {
        Closure::wrap(Box::new(t) as Box<T>)
    }

    /// A mostly internal function to wrap a boxed closure inside a `Closure`
    /// type.
    ///
    /// This is the function where the JS closure is manufactured.
    pub fn wrap(t: Box<T>) -> Closure<T> {
        Closure {
            inner: UnsafeCell::new(t),
            js: UnsafeCell::new(ManuallyDrop::new(JsValue { idx: !0 })),
        }
    }

    /// Leaks this `Closure` to ensure it remains valid for the duration of the
    /// entire program.
    ///
    /// > **Note**: this function will leak memory. It should be used sparingly
    /// > to ensure the memory leak doesn't affect the program too much.
    ///
    /// When a `Closure` is dropped it will invalidate the associated JS
    /// closure, but this isn't always desired. Some callbacks are alive for
    /// the entire duration of the program, so this can be used to conveniently
    /// leak this instance of `Closure` while performing as much internal
    /// cleanup as it can.
    pub fn forget(self) {
        unsafe {
            let idx = (*self.js.get()).idx;
            if idx != !0 {
                super::__wbindgen_cb_forget(idx);
            }
            mem::forget(self);
        }
    }
}

impl<T> WasmDescribe for Closure<T>
    where T: WasmClosure + ?Sized,
{
    fn describe() {
        inform(CLOSURE);
        T::describe();
    }
}

// `Closure` can only be passed by reference to imports.
impl<'a, T> IntoWasmAbi for &'a Closure<T>
    where T: WasmClosure + ?Sized,
{
    type Abi = u32;

    fn into_abi(self, extra: &mut Stack) -> u32 {
        unsafe {
            let fnptr = WasmClosure::into_abi(&mut **self.inner.get(), extra);
            extra.push(fnptr);
            &mut (*self.js.get()).idx as *const u32 as u32
        }
    }
}

fn _check() {
    fn _assert<T: IntoWasmAbi>() {}
    _assert::<&Closure<Fn()>>();
    _assert::<&Closure<Fn(String)>>();
    _assert::<&Closure<Fn() -> String>>();
    _assert::<&Closure<FnMut()>>();
    _assert::<&Closure<FnMut(String)>>();
    _assert::<&Closure<FnMut() -> String>>();
}

impl<T> Drop for Closure<T>
    where T: ?Sized,
{
    fn drop(&mut self) {
        unsafe {
            let idx = (*self.js.get()).idx;
            if idx != !0 {
                super::__wbindgen_cb_drop(idx);
            }
        }
    }
}

/// An internal trait for the `Closure` type.
///
/// This trait is not stable and it's not recommended to use this in bounds or
/// implement yourself.
pub unsafe trait WasmClosure: 'static {
    fn describe();

    unsafe fn into_abi(me: *mut Self, extra: &mut Stack) -> u32;
}

macro_rules! doit {
    ($(
        ($($var:ident)*)
    )*) => ($(
        // Fn with no return
        unsafe impl<$($var),*> WasmClosure for Fn($($var),*)
            where $($var: FromWasmAbi + 'static,)*
        {
            fn describe() {
                <&Self>::describe();
            }

            unsafe fn into_abi(me: *mut Self, extra: &mut Stack) -> u32 {
                IntoWasmAbi::into_abi(&*me, extra)
            }
        }
        // Fn with return
        unsafe impl<$($var,)* R> WasmClosure for Fn($($var),*) -> R
            where $($var: FromWasmAbi + 'static,)*
                  R: IntoWasmAbi + 'static,
        {
            fn describe() {
                <&Self>::describe();
            }

            unsafe fn into_abi(me: *mut Self, extra: &mut Stack) -> u32 {
                IntoWasmAbi::into_abi(&*me, extra)
            }
        }
        // FnMut with no return
        unsafe impl<$($var),*> WasmClosure for FnMut($($var),*)
            where $($var: FromWasmAbi + 'static,)*
        {
            fn describe() {
                <&mut Self>::describe();
            }

            unsafe fn into_abi(me: *mut Self, extra: &mut Stack) -> u32 {
                IntoWasmAbi::into_abi(&mut *me, extra)
            }
        }
        // FnMut with return
        unsafe impl<$($var,)* R> WasmClosure for FnMut($($var),*) -> R
            where $($var: FromWasmAbi + 'static,)*
                  R: IntoWasmAbi + 'static,
        {
            fn describe() {
                <&Self>::describe();
            }

            unsafe fn into_abi(me: *mut Self, extra: &mut Stack) -> u32 {
                IntoWasmAbi::into_abi(&mut *me, extra)
            }
        }
    )*)
}

doit! {
    ()
    (A)
    (A B)
    (A B C)
    (A B C D)
    (A B C D E)
    (A B C D E F)
    (A B C D E F G)
}
