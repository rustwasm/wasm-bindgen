use core::char;
use core::mem::{self, ManuallyDrop};

use crate::convert::traits::{WasmAbi, WasmPrimitive};
use crate::convert::{FromWasmAbi, IntoWasmAbi, LongRefFromWasmAbi, RefFromWasmAbi};
use crate::convert::{OptionFromWasmAbi, OptionIntoWasmAbi, ReturnWasmAbi};
use crate::{Clamped, JsError, JsValue, UnwrapThrowExt};

if_std! {
    use std::boxed::Box;
    use std::convert::{TryFrom, TryInto};
    use std::fmt::Debug;
    use std::vec::Vec;
}

// Primitive types can always be passed over the ABI.
impl<T: WasmPrimitive> WasmAbi for T {
    type Prim1 = Self;
    type Prim2 = ();
    type Prim3 = ();
    type Prim4 = ();

    #[inline]
    fn split(self) -> (Self, (), (), ()) {
        (self, (), (), ())
    }

    #[inline]
    fn join(prim: Self, _: (), _: (), _: ()) -> Self {
        prim
    }
}

impl<T: WasmAbi<Prim4 = ()>> WasmAbi for Option<T> {
    /// Whether this `Option` is a `Some` value.
    type Prim1 = u32;
    type Prim2 = T::Prim1;
    type Prim3 = T::Prim2;
    type Prim4 = T::Prim3;

    #[inline]
    fn split(self) -> (u32, T::Prim1, T::Prim2, T::Prim3) {
        match self {
            None => (
                0,
                Default::default(),
                Default::default(),
                Default::default(),
            ),
            Some(value) => {
                let (prim1, prim2, prim3, ()) = value.split();
                (1, prim1, prim2, prim3)
            }
        }
    }

    #[inline]
    fn join(is_some: u32, prim1: T::Prim1, prim2: T::Prim2, prim3: T::Prim3) -> Self {
        if is_some == 0 {
            None
        } else {
            Some(T::join(prim1, prim2, prim3, ()))
        }
    }
}

macro_rules! type_wasm_native {
    ($($t:tt as $c:tt)*) => ($(
        impl IntoWasmAbi for $t {
            type Abi = $c;

            #[inline]
            fn into_abi(self) -> $c { self as $c }
        }

        impl FromWasmAbi for $t {
            type Abi = $c;

            #[inline]
            unsafe fn from_abi(js: $c) -> Self { js as $t }
        }

        impl IntoWasmAbi for Option<$t> {
            type Abi = Option<$c>;

            #[inline]
            fn into_abi(self) -> Self::Abi {
                self.map(|v| v as $c)
            }
        }

        impl FromWasmAbi for Option<$t> {
            type Abi = Option<$c>;

            #[inline]
            unsafe fn from_abi(js: Self::Abi) -> Self {
                js.map(|v: $c| v as $t)
            }
        }
    )*)
}

type_wasm_native!(
    i32 as i32
    isize as i32
    u32 as u32
    usize as u32
    i64 as i64
    u64 as u64
    f32 as f32
    f64 as f64
);

macro_rules! type_abi_as_u32 {
    ($($t:tt)*) => ($(
        impl IntoWasmAbi for $t {
            type Abi = u32;

            #[inline]
            fn into_abi(self) -> u32 { self as u32 }
        }

        impl FromWasmAbi for $t {
            type Abi = u32;

            #[inline]
            unsafe fn from_abi(js: u32) -> Self { js as $t }
        }

        impl OptionIntoWasmAbi for $t {
            #[inline]
            fn none() -> u32 { 0x00FF_FFFFu32 }
        }

        impl OptionFromWasmAbi for $t {
            #[inline]
            fn is_none(js: &u32) -> bool { *js == 0x00FF_FFFFu32 }
        }
    )*)
}

type_abi_as_u32!(i8 u8 i16 u16);

impl IntoWasmAbi for bool {
    type Abi = u32;

    #[inline]
    fn into_abi(self) -> u32 {
        self as u32
    }
}

impl FromWasmAbi for bool {
    type Abi = u32;

    #[inline]
    unsafe fn from_abi(js: u32) -> bool {
        js != 0
    }
}

impl OptionIntoWasmAbi for bool {
    #[inline]
    fn none() -> u32 {
        0x00FF_FFFFu32
    }
}

impl OptionFromWasmAbi for bool {
    #[inline]
    fn is_none(js: &u32) -> bool {
        *js == 0x00FF_FFFFu32
    }
}

impl IntoWasmAbi for char {
    type Abi = u32;

    #[inline]
    fn into_abi(self) -> u32 {
        self as u32
    }
}

impl FromWasmAbi for char {
    type Abi = u32;

    #[inline]
    unsafe fn from_abi(js: u32) -> char {
        char::from_u32_unchecked(js)
    }
}

impl OptionIntoWasmAbi for char {
    #[inline]
    fn none() -> u32 {
        0x00FF_FFFFu32
    }
}

impl OptionFromWasmAbi for char {
    #[inline]
    fn is_none(js: &u32) -> bool {
        *js == 0x00FF_FFFFu32
    }
}

impl<T> IntoWasmAbi for *const T {
    type Abi = u32;

    #[inline]
    fn into_abi(self) -> u32 {
        self as u32
    }
}

impl<T> FromWasmAbi for *const T {
    type Abi = u32;

    #[inline]
    unsafe fn from_abi(js: u32) -> *const T {
        js as *const T
    }
}

impl<T> IntoWasmAbi for *mut T {
    type Abi = u32;

    #[inline]
    fn into_abi(self) -> u32 {
        self as u32
    }
}

impl<T> FromWasmAbi for *mut T {
    type Abi = u32;

    #[inline]
    unsafe fn from_abi(js: u32) -> *mut T {
        js as *mut T
    }
}

impl IntoWasmAbi for JsValue {
    type Abi = u32;

    #[inline]
    fn into_abi(self) -> u32 {
        let ret = self.idx;
        mem::forget(self);
        ret
    }
}

impl FromWasmAbi for JsValue {
    type Abi = u32;

    #[inline]
    unsafe fn from_abi(js: u32) -> JsValue {
        JsValue::_new(js)
    }
}

impl<'a> IntoWasmAbi for &'a JsValue {
    type Abi = u32;

    #[inline]
    fn into_abi(self) -> u32 {
        self.idx
    }
}

impl RefFromWasmAbi for JsValue {
    type Abi = u32;
    type Anchor = ManuallyDrop<JsValue>;

    #[inline]
    unsafe fn ref_from_abi(js: u32) -> Self::Anchor {
        ManuallyDrop::new(JsValue::_new(js))
    }
}

impl LongRefFromWasmAbi for JsValue {
    type Abi = u32;
    type Anchor = JsValue;

    #[inline]
    unsafe fn long_ref_from_abi(js: u32) -> Self::Anchor {
        Self::from_abi(js)
    }
}

impl<T: OptionIntoWasmAbi> IntoWasmAbi for Option<T> {
    type Abi = T::Abi;

    #[inline]
    fn into_abi(self) -> T::Abi {
        match self {
            None => T::none(),
            Some(me) => me.into_abi(),
        }
    }
}

impl<T: OptionFromWasmAbi> FromWasmAbi for Option<T> {
    type Abi = T::Abi;

    #[inline]
    unsafe fn from_abi(js: T::Abi) -> Self {
        if T::is_none(&js) {
            None
        } else {
            Some(T::from_abi(js))
        }
    }
}

impl<T: IntoWasmAbi> IntoWasmAbi for Clamped<T> {
    type Abi = T::Abi;

    #[inline]
    fn into_abi(self) -> Self::Abi {
        self.0.into_abi()
    }
}

impl<T: FromWasmAbi> FromWasmAbi for Clamped<T> {
    type Abi = T::Abi;

    #[inline]
    unsafe fn from_abi(js: T::Abi) -> Self {
        Clamped(T::from_abi(js))
    }
}

impl IntoWasmAbi for () {
    type Abi = ();

    #[inline]
    fn into_abi(self) {
        self
    }
}

impl<T: WasmAbi<Prim3 = (), Prim4 = ()>> WasmAbi for Result<T, u32> {
    type Prim1 = T::Prim1;
    type Prim2 = T::Prim2;
    // The order of primitives here is such that we can pop() the possible error
    // first, deal with it and move on. Later primitives are popped off the
    // stack first.
    /// If this `Result` is an `Err`, the error value.
    type Prim3 = u32;
    /// Whether this `Result` is an `Err`.
    type Prim4 = u32;

    #[inline]
    fn split(self) -> (T::Prim1, T::Prim2, u32, u32) {
        match self {
            Ok(value) => {
                let (prim1, prim2, (), ()) = value.split();
                (prim1, prim2, 0, 0)
            }
            Err(err) => (Default::default(), Default::default(), err, 1),
        }
    }

    #[inline]
    fn join(prim1: T::Prim1, prim2: T::Prim2, err: u32, is_err: u32) -> Self {
        if is_err == 0 {
            Ok(T::join(prim1, prim2, (), ()))
        } else {
            Err(err)
        }
    }
}

impl<T, E> ReturnWasmAbi for Result<T, E>
where
    T: IntoWasmAbi,
    E: Into<JsValue>,
    T::Abi: WasmAbi<Prim3 = (), Prim4 = ()>,
{
    type Abi = Result<T::Abi, u32>;

    #[inline]
    fn return_abi(self) -> Self::Abi {
        match self {
            Ok(v) => Ok(v.into_abi()),
            Err(e) => {
                let jsval = e.into();
                Err(jsval.into_abi())
            }
        }
    }
}

impl IntoWasmAbi for JsError {
    type Abi = <JsValue as IntoWasmAbi>::Abi;

    fn into_abi(self) -> Self::Abi {
        self.value.into_abi()
    }
}

if_std! {
    // Note: this can't take `&[T]` because the `Into<JsValue>` impl needs
    // ownership of `T`.
    pub fn js_value_vector_into_abi<T: Into<JsValue>>(vector: Box<[T]>) -> <Box<[JsValue]> as IntoWasmAbi>::Abi {
        let js_vals: Box<[JsValue]> = vector
            .into_vec()
            .into_iter()
            .map(|x| x.into())
            .collect();

        js_vals.into_abi()
    }

    pub unsafe fn js_value_vector_from_abi<T: TryFrom<JsValue>>(js: <Box<[JsValue]> as FromWasmAbi>::Abi) -> Box<[T]> where T::Error: Debug {
        let js_vals = <Vec<JsValue> as FromWasmAbi>::from_abi(js);

        let mut result = Vec::with_capacity(js_vals.len());
        for value in js_vals {
            // We push elements one-by-one instead of using `collect` in order to improve
            // error messages. When using `collect`, this `expect_throw` is buried in a
            // giant chain of internal iterator functions, which results in the actual
            // function that takes this `Vec` falling off the end of the call stack.
            // So instead, make sure to call it directly within this function.
            //
            // This is only a problem in debug mode. Since this is the browser's error stack
            // we're talking about, it can only see functions that actually make it to the
            // final wasm binary (i.e., not inlined functions). All of those internal
            // iterator functions get inlined in release mode, and so they don't show up.
            result.push(value.try_into().expect_throw("array contains a value of the wrong type"));
        }
        result.into_boxed_slice()
    }
}
