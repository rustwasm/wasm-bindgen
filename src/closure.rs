//! Support for long-lived closures in `wasm-bindgen`
//!
//! This module defines the `Closure` type which is used to pass "owned
//! closures" from Rust to JS. Some more details can be found on the `Closure`
//! type itself.

use std::mem::{self, ManuallyDrop};
use std::marker::Unsize;

use {throw, JsValue};
use convert::*;
use __rt::WasmRefCell;

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
pub struct Closure<T: WasmShim + ?Sized> {
    _inner: T::Wrapper,
    js: ManuallyDrop<JsValue>,
}

impl<T> Closure<T>
    where T: WasmShim + ?Sized,
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
        Closure::wrap(T::wrap(t))
    }

    /// A mostly internal function to wrap a boxed closure inside a `Closure`
    /// type.
    ///
    /// This is the function where the JS closure is manufactured.
    pub fn wrap(t: T::Wrapper) -> Closure<T> {
        unsafe {
            let data = T::data(&t);
            let js = T::factory()(T::shim(), data[0], data[1]);
            Closure {
                _inner: t,
                js: ManuallyDrop::new(JsValue::from_abi(js, &mut GlobalStack::new())),
            }
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
            super::__wbindgen_cb_forget(self.js.to_abi_ref(&mut GlobalStack::new()));
            mem::forget(self);
        }
    }
}

// `Closure` can only be passed by reference to imports.
impl<T> ToRefWasmBoundary for Closure<T>
    where T: WasmShim + ?Sized,
{
    type Abi = u32;
    const DESCRIPTOR: Descriptor = T::DESCRIPTOR;

    fn to_abi_ref(&self, extra: &mut Stack) -> u32 {
        self.js.to_abi_ref(extra)
    }
}

impl<T> Drop for Closure<T>
    where T: WasmShim + ?Sized,
{
    fn drop(&mut self) {
        unsafe {
            let idx = self.js.to_abi_ref(&mut GlobalStack::new());
            super::__wbindgen_cb_drop(idx);
        }
    }
}

/// An internal trait for the `Closure` type.
///
/// This trait is not stable and it's not recommended to use this in bounds or
/// implement yourself.
pub unsafe trait WasmShim {
    #[doc(hidden)]
    const DESCRIPTOR: Descriptor;
    #[doc(hidden)]
    type Wrapper;
    #[doc(hidden)]
    fn shim() -> u32;
    #[doc(hidden)]
    fn factory() -> unsafe extern fn(u32, u32, u32) -> u32;
    #[doc(hidden)]
    fn wrap<U>(u: U) -> Self::Wrapper where U: Unsize<Self> + 'static;
    #[doc(hidden)]
    fn data(t: &Self::Wrapper) -> [u32; 2];
}

union RawPtr<T: ?Sized> {
    ptr: *const T,
    data: [u32; 2]
}

macro_rules! doit {
    ($(
        ($($var:ident)*) => $arity:ident
    )*) => ($(
        // Fn with no return
        unsafe impl<$($var: WasmAbi),*> WasmShim for Fn($($var),*) {
            const DESCRIPTOR: Descriptor = DESCRIPTOR_FUNC;
            type Wrapper = Box<Fn($($var),*)>;

            fn shim() -> u32 {
                #[allow(non_snake_case)]
                unsafe extern fn shim<$($var),*>(
                    a: u32,
                    b: u32,
                    $($var:$var),*
                ) {
                    if a == 0 {
                        throw("closure has been destroyed already");
                    }
                    (*RawPtr::<Fn($($var),*)> { data: [a, b] }.ptr)($($var),*)
                }
                shim::<$($var),*> as u32
            }

            fn factory() -> unsafe extern fn(u32, u32, u32) -> u32 {
                super::$arity
            }

            fn wrap<U>(u: U) -> Self::Wrapper where U: Unsize<Self> + 'static {
                Box::new(u) as Box<Self>
            }

            fn data(t: &Self::Wrapper) -> [u32; 2] {
                unsafe {
                    RawPtr::<Self> { ptr: &**t }.data
                }
            }
        }

        // Fn with a return
        unsafe impl<$($var: WasmAbi,)* R: WasmAbi> WasmShim for Fn($($var),*) -> R {
            const DESCRIPTOR: Descriptor = DESCRIPTOR_FUNC;
            type Wrapper = Box<Fn($($var),*) -> R>;

            fn shim() -> u32 {
                #[allow(non_snake_case)]
                unsafe extern fn shim<$($var,)* R>(
                    a: u32,
                    b: u32,
                    $($var:$var),*
                ) -> R {
                    if a == 0 {
                        throw("closure has been destroyed already");
                    }
                    (*RawPtr::<Fn($($var),*) -> R> { data: [a, b] }.ptr)($($var),*)
                }
                shim::<$($var,)* R> as u32
            }

            fn factory() -> unsafe extern fn(u32, u32, u32) -> u32 {
                super::$arity
            }

            fn wrap<U>(u: U) -> Self::Wrapper where U: Unsize<Self> + 'static {
                Box::new(u) as Box<Self>
            }

            fn data(t: &Self::Wrapper) -> [u32; 2] {
                unsafe {
                    RawPtr::<Self> { ptr: &**t }.data
                }
            }
        }

        // FnMut with no return
        unsafe impl<$($var: WasmAbi),*> WasmShim for FnMut($($var),*) {
            const DESCRIPTOR: Descriptor = DESCRIPTOR_FUNC;
            type Wrapper = Box<WasmRefCell<FnMut($($var),*)>>;

            fn shim() -> u32 {
                #[allow(non_snake_case)]
                unsafe extern fn shim<$($var),*>(
                    a: u32,
                    b: u32,
                    $($var:$var),*
                ) {
                    if a == 0 {
                        throw("closure has been destroyed already");
                    }
                    let ptr: *const WasmRefCell<FnMut($($var),*)> = RawPtr {
                        data: [a, b],
                    }.ptr;
                    let mut ptr = (*ptr).borrow_mut();
                    (&mut *ptr)($($var),*)
                }
                shim::<$($var),*> as u32
            }

            fn factory() -> unsafe extern fn(u32, u32, u32) -> u32 {
                super::$arity
            }

            fn wrap<U>(u: U) -> Self::Wrapper where U: Unsize<Self> + 'static {
                Box::new(WasmRefCell::new(u)) as Box<_>
            }

            fn data(t: &Self::Wrapper) -> [u32; 2] {
                unsafe {
                    RawPtr::<WasmRefCell<Self>> { ptr: &**t }.data
                }
            }
        }

        // FnMut with a return
        unsafe impl<$($var: WasmAbi,)* R: WasmAbi> WasmShim for FnMut($($var),*) -> R {
            const DESCRIPTOR: Descriptor = DESCRIPTOR_FUNC;
            type Wrapper = Box<WasmRefCell<FnMut($($var),*) -> R>>;

            fn shim() -> u32 {
                #[allow(non_snake_case)]
                unsafe extern fn shim<$($var,)* R>(
                    a: u32,
                    b: u32,
                    $($var:$var),*
                ) -> R {
                    if a == 0 {
                        throw("closure has been destroyed already");
                    }
                    let ptr: *const WasmRefCell<FnMut($($var),*) -> R> = RawPtr {
                        data: [a, b],
                    }.ptr;
                    let mut ptr = (*ptr).borrow_mut();
                    (&mut *ptr)($($var),*)
                }
                shim::<$($var,)* R> as u32
            }

            fn factory() -> unsafe extern fn(u32, u32, u32) -> u32 {
                super::$arity
            }

            fn wrap<U>(u: U) -> Self::Wrapper where U: Unsize<Self> + 'static {
                Box::new(WasmRefCell::new(u)) as Box<_>
            }

            fn data(t: &Self::Wrapper) -> [u32; 2] {
                unsafe {
                    RawPtr::<WasmRefCell<Self>> { ptr: &**t }.data
                }
            }
        }
    )*)
}

doit! {
    () => __wbindgen_cb_arity0
    (A) => __wbindgen_cb_arity1
    (A B) => __wbindgen_cb_arity2
    (A B C) => __wbindgen_cb_arity3
    (A B C D) => __wbindgen_cb_arity4
    (A B C D E) => __wbindgen_cb_arity5
    (A B C D E F) => __wbindgen_cb_arity6
    (A B C D E F G) => __wbindgen_cb_arity7
}
