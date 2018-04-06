use std::mem::ManuallyDrop;
use std::marker::{self, Unsize};

use {throw, JsValue};
use convert::*;
use __rt::WasmRefCell;

pub unsafe trait WasmShim<'a> {
    #[doc(hidden)]
    const DESCRIPTOR: Descriptor;
    #[doc(hidden)]
    type Wrapper;
    #[doc(hidden)]
    fn shim() -> u32;
    #[doc(hidden)]
    fn factory() -> unsafe extern fn(u32, u32, u32) -> u32;
    #[doc(hidden)]
    fn wrap<U>(u: U) -> Self::Wrapper where U: Unsize<Self> + 'a;
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
        unsafe impl<'a, $($var: WasmAbi),*> WasmShim<'a> for Fn($($var),*) + 'a {
            const DESCRIPTOR: Descriptor = DESCRIPTOR_FUNC;
            type Wrapper = Box<Fn($($var),*) + 'a>;

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

            fn wrap<U>(u: U) -> Self::Wrapper where U: Unsize<Self> + 'a {
                Box::new(u) as Box<Self>
            }

            fn data(t: &Self::Wrapper) -> [u32; 2] {
                unsafe {
                    RawPtr::<Self> { ptr: &**t }.data
                }
            }
        }

        // Fn with a return
        unsafe impl<'a, $($var: WasmAbi,)* R: WasmAbi> WasmShim<'a> for Fn($($var),*) -> R + 'a {
            const DESCRIPTOR: Descriptor = DESCRIPTOR_FUNC;
            type Wrapper = Box<Fn($($var),*) -> R + 'a>;

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

            fn wrap<U>(u: U) -> Self::Wrapper where U: Unsize<Self> + 'a {
                Box::new(u) as Box<Self>
            }

            fn data(t: &Self::Wrapper) -> [u32; 2] {
                unsafe {
                    RawPtr::<Self> { ptr: &**t }.data
                }
            }
        }

        // FnMut with no return
        unsafe impl<'a, $($var: WasmAbi),*> WasmShim<'a> for FnMut($($var),*) + 'a {
            const DESCRIPTOR: Descriptor = DESCRIPTOR_FUNC;
            type Wrapper = Box<WasmRefCell<FnMut($($var),*) + 'a>>;

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

            fn wrap<U>(u: U) -> Self::Wrapper where U: Unsize<Self> + 'a {
                Box::new(WasmRefCell::new(u)) as Box<_>
            }

            fn data(t: &Self::Wrapper) -> [u32; 2] {
                unsafe {
                    RawPtr::<WasmRefCell<Self>> { ptr: &**t }.data
                }
            }
        }

        // FnMut with a return
        unsafe impl<'a, $($var: WasmAbi,)* R: WasmAbi> WasmShim<'a> for FnMut($($var),*) -> R + 'a {
            const DESCRIPTOR: Descriptor = DESCRIPTOR_FUNC;
            type Wrapper = Box<WasmRefCell<FnMut($($var),*) -> R + 'a>>;

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

            fn wrap<U>(u: U) -> Self::Wrapper where U: Unsize<Self> + 'a {
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

pub struct Closure<'a, T: WasmShim<'a> + ?Sized + 'a> {
    _inner: T::Wrapper,
    js: ManuallyDrop<JsValue>,
    _marker: marker::PhantomData<&'a ()>,
}

impl<'a, T> Closure<'a, T>
    where T: WasmShim<'a> + ?Sized,
{
    pub fn new<U>(t: U) -> Closure<'a, T>
        where U: Unsize<T> + 'a
    {
        Closure::wrap(T::wrap(t))
    }

    pub fn wrap(t: T::Wrapper) -> Closure<'a, T> {
        unsafe {
            let data = T::data(&t);
            let js = T::factory()(T::shim(), data[0], data[1]);
            Closure {
                _inner: t,
                js: ManuallyDrop::new(JsValue::from_abi(js, &mut GlobalStack::new())),
                _marker: marker::PhantomData,
            }
        }
    }
}

impl<'a, T> ToRefWasmBoundary for Closure<'a, T>
    where T: WasmShim<'a> + ?Sized,
{
    type Abi = u32;
    const DESCRIPTOR: Descriptor = T::DESCRIPTOR;

    fn to_abi_ref(&self, extra: &mut Stack) -> u32 {
        self.js.to_abi_ref(extra)
    }
}

impl<'a, T> Drop for Closure<'a, T>
    where T: WasmShim<'a> + ?Sized,
{
    fn drop(&mut self) {
        unsafe {
            let idx = self.js.to_abi_ref(&mut GlobalStack::new());
            super::__wbindgen_cb_drop(idx);
        }
    }
}
