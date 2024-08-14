use core::borrow::Borrow;
use core::ops::{Deref, DerefMut};

use crate::describe::*;
use crate::JsValue;

/// A trait for anything that can be converted into a type that can cross the
/// wasm ABI directly, eg `u32` or `f64`.
///
/// This is the opposite operation as `FromWasmAbi` and `Ref[Mut]FromWasmAbi`.
///
/// # ⚠️ Unstable
///
/// This is part of the internal [`convert`](crate::convert) module, **no
/// stability guarantees** are provided. Use at your own risk. See its
/// documentation for more details.
pub trait IntoWasmAbi: WasmDescribe {
    /// The type which wasm ABI is provided when crossing the ABI
    /// boundary.
    type Abi: WasmAbi;

    /// Convert `self` into `Self::Abi` so that it can be sent across the wasm
    /// ABI boundary.
    fn into_abi(self) -> Self::Abi;
}

/// A trait for anything that can be recovered by-value from the wasm ABI
/// boundary, eg a Rust `u8` can be recovered from the wasm ABI `u32` type.
///
/// This is the by-value variant of the opposite operation as `IntoWasmAbi`.
///
/// # ⚠️ Unstable
///
/// This is part of the internal [`convert`](crate::convert) module, **no
/// stability guarantees** are provided. Use at your own risk. See its
/// documentation for more details.
pub trait FromWasmAbi: WasmDescribe {
    /// The type which wasm ABI provides when coming back from ABI boundary,
    /// to be converted to a rust type.
    type Abi: WasmAbi;

    /// Recover a `Self` from `Self::Abi`.
    ///
    /// # Safety
    ///
    /// This is only safe to call when -- and implementations may assume that --
    /// the supplied `Self::Abi` was previously generated by a call to `<Self as
    /// IntoWasmAbi>::into_abi()` or the moral equivalent in JS.
    unsafe fn from_abi(js: Self::Abi) -> Self;
}

/// A trait for anything that can be recovered as some sort of shared reference
/// from the wasm ABI boundary.
///
/// This is the shared reference variant of the opposite operation as
/// `IntoWasmAbi`.
///
/// # ⚠️ Unstable
///
/// This is part of the internal [`convert`](crate::convert) module, **no
/// stability guarantees** are provided. Use at your own risk. See its
/// documentation for more details.
pub trait RefFromWasmAbi: WasmDescribe {
    /// The wasm ABI type references to `Self` are recovered from.
    type Abi: WasmAbi;

    /// The type that holds the reference to `Self` for the duration of the
    /// invocation of the function that has an `&Self` parameter. This is
    /// required to ensure that the lifetimes don't persist beyond one function
    /// call, and so that they remain anonymous.
    type Anchor: Deref<Target = Self>;

    /// Recover a `Self::Anchor` from `Self::Abi`.
    ///
    /// # Safety
    ///
    /// Same as `FromWasmAbi::from_abi`.
    unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor;
}

/// A version of the `RefFromWasmAbi` trait with the additional requirement
/// that the reference must remain valid as long as the anchor isn't dropped.
///
/// This isn't the case for `JsValue`'s `RefFromWasmAbi` implementation. To
/// avoid having to allocate a spot for the `JsValue` on the `JsValue` heap,
/// the `JsValue` is instead pushed onto the `JsValue` stack, and popped off
/// again after the function that the reference was passed to returns. So,
/// `JsValue` has a different `LongRefFromWasmAbi` implementation that behaves
/// the same as `FromWasmAbi`, putting the value on the heap.
///
/// This is needed for async functions, where the reference needs to be valid
/// for the whole length of the `Future`, rather than the initial synchronous
/// call.
///
/// 'long ref' is short for 'long-lived reference'.
///
/// # ⚠️ Unstable
///
/// This is part of the internal [`convert`](crate::convert) module, **no
/// stability guarantees** are provided. Use at your own risk. See its
/// documentation for more details.
pub trait LongRefFromWasmAbi: WasmDescribe {
    /// Same as `RefFromWasmAbi::Abi`
    type Abi: WasmAbi;

    /// Same as `RefFromWasmAbi::Anchor`
    type Anchor: Borrow<Self>;

    /// Same as `RefFromWasmAbi::ref_from_abi`
    unsafe fn long_ref_from_abi(js: Self::Abi) -> Self::Anchor;
}

/// Dual of the `RefFromWasmAbi` trait, except for mutable references.
///
/// # ⚠️ Unstable
///
/// This is part of the internal [`convert`](crate::convert) module, **no
/// stability guarantees** are provided. Use at your own risk. See its
/// documentation for more details.
pub trait RefMutFromWasmAbi: WasmDescribe {
    /// Same as `RefFromWasmAbi::Abi`
    type Abi: WasmAbi;
    /// Same as `RefFromWasmAbi::Anchor`
    type Anchor: DerefMut<Target = Self>;
    /// Same as `RefFromWasmAbi::ref_from_abi`
    unsafe fn ref_mut_from_abi(js: Self::Abi) -> Self::Anchor;
}

/// Indicates that this type can be passed to JS as `Option<Self>`.
///
/// This trait is used when implementing `IntoWasmAbi for Option<T>`.
///
/// # ⚠️ Unstable
///
/// This is part of the internal [`convert`](crate::convert) module, **no
/// stability guarantees** are provided. Use at your own risk. See its
/// documentation for more details.
pub trait OptionIntoWasmAbi: IntoWasmAbi {
    /// Returns an ABI instance indicating "none", which JS will interpret as
    /// the `None` branch of this option.
    ///
    /// It should be guaranteed that the `IntoWasmAbi` can never produce the ABI
    /// value returned here.
    fn none() -> Self::Abi;
}

/// Indicates that this type can be received from JS as `Option<Self>`.
///
/// This trait is used when implementing `FromWasmAbi for Option<T>`.
///
/// # ⚠️ Unstable
///
/// This is part of the internal [`convert`](crate::convert) module, **no
/// stability guarantees** are provided. Use at your own risk. See its
/// documentation for more details.
pub trait OptionFromWasmAbi: FromWasmAbi {
    /// Tests whether the argument is a "none" instance. If so it will be
    /// deserialized as `None`, and otherwise it will be passed to
    /// `FromWasmAbi`.
    fn is_none(abi: &Self::Abi) -> bool;
}

/// A trait for any type which maps to a Wasm primitive type when used in FFI
/// (`i32`, `i64`, `f32`, or `f64`).
///
/// This is with the exception of `()` (and other zero-sized types), which are
/// also allowed because they're ignored: no arguments actually get added.
///
/// # Safety
///
/// This is an unsafe trait to implement as there's no guarantee the type
/// actually maps to a primitive type.
///
/// # ⚠️ Unstable
///
/// This is part of the internal [`convert`](crate::convert) module, **no
/// stability guarantees** are provided. Use at your own risk. See its
/// documentation for more details.
pub unsafe trait WasmPrimitive: Default {}

unsafe impl WasmPrimitive for u32 {}
unsafe impl WasmPrimitive for i32 {}
unsafe impl WasmPrimitive for u64 {}
unsafe impl WasmPrimitive for i64 {}
unsafe impl WasmPrimitive for f32 {}
unsafe impl WasmPrimitive for f64 {}
unsafe impl WasmPrimitive for () {}

/// A trait which represents types that can be passed across the Wasm ABI
/// boundary, by being split into multiple Wasm primitive types.
///
/// Up to 4 primitives are supported; if you don't want to use all of them, you
/// can set the rest to `()`, which will cause them to be ignored.
///
/// You need to be careful how many primitives you use, however:
/// `Result<T, JsValue>` uses up 2 primitives to store the error, and so it
/// doesn't work if `T` uses more than 2 primitives.
///
/// So, if you're adding support for a type that needs 3 or more primitives and
/// is able to be returned, you have to add another primitive here.
///
/// There's already one type that uses 3 primitives: `&mut [T]`. However, it
/// can't be returned anyway, so it doesn't matter that
/// `Result<&mut [T], JsValue>` wouldn't work.
///
/// # ⚠️ Unstable
///
/// This is part of the internal [`convert`](crate::convert) module, **no
/// stability guarantees** are provided. Use at your own risk. See its
/// documentation for more details.
pub trait WasmAbi {
    type Prim1: WasmPrimitive;
    type Prim2: WasmPrimitive;
    type Prim3: WasmPrimitive;
    type Prim4: WasmPrimitive;

    /// Splits this type up into primitives to be sent over the ABI.
    fn split(self) -> (Self::Prim1, Self::Prim2, Self::Prim3, Self::Prim4);
    /// Reconstructs this type from primitives received over the ABI.
    fn join(prim1: Self::Prim1, prim2: Self::Prim2, prim3: Self::Prim3, prim4: Self::Prim4)
        -> Self;
}

/// A trait representing how to interpret the return value of a function for
/// the wasm ABI.
///
/// This is very similar to the `IntoWasmAbi` trait and in fact has a blanket
/// implementation for all implementors of the `IntoWasmAbi`. The primary use
/// case of this trait is to enable functions to return `Result`, interpreting
/// an error as "rethrow this to JS"
///
/// # ⚠️ Unstable
///
/// This is part of the internal [`convert`](crate::convert) module, **no
/// stability guarantees** are provided. Use at your own risk. See its
/// documentation for more details.
pub trait ReturnWasmAbi: WasmDescribe {
    /// Same as `IntoWasmAbi::Abi`
    type Abi: WasmAbi;

    /// Same as `IntoWasmAbi::into_abi`, except that it may throw and never
    /// return in the case of `Err`.
    fn return_abi(self) -> Self::Abi;
}

impl<T: IntoWasmAbi> ReturnWasmAbi for T {
    type Abi = T::Abi;

    #[inline]
    fn return_abi(self) -> Self::Abi {
        self.into_abi()
    }
}

use alloc::boxed::Box;
use core::marker::Sized;

/// Trait for element types to implement IntoWasmAbi for vectors of
/// themselves.
///
/// # ⚠️ Unstable
///
/// This is part of the internal [`convert`](crate::convert) module, **no
/// stability guarantees** are provided. Use at your own risk. See its
/// documentation for more details.
pub trait VectorIntoWasmAbi: WasmDescribeVector + Sized {
    type Abi: WasmAbi;

    fn vector_into_abi(vector: Box<[Self]>) -> Self::Abi;
}

/// Trait for element types to implement FromWasmAbi for vectors of
/// themselves.
///
/// # ⚠️ Unstable
///
/// This is part of the internal [`convert`](crate::convert) module, **no
/// stability guarantees** are provided. Use at your own risk. See its
/// documentation for more details.
pub trait VectorFromWasmAbi: WasmDescribeVector + Sized {
    type Abi: WasmAbi;

    unsafe fn vector_from_abi(js: Self::Abi) -> Box<[Self]>;
}

/// A repr(C) struct containing all of the primitives of a `WasmAbi` type, in
/// order.
///
/// This is used as the return type of imported/exported functions. `WasmAbi`
/// types aren't guaranteed to be FFI-safe, so we can't return them directly:
/// instead we return this.
///
/// If all but one of the primitives is `()`, this corresponds to returning the
/// remaining primitive directly, otherwise a return pointer is used.
///
/// # ⚠️ Unstable
///
/// This is part of the internal [`convert`](crate::convert) module, **no
/// stability guarantees** are provided. Use at your own risk. See its
/// documentation for more details.
#[repr(C)]
pub struct WasmRet<T: WasmAbi> {
    prim1: T::Prim1,
    prim2: T::Prim2,
    prim3: T::Prim3,
    prim4: T::Prim4,
}

impl<T: WasmAbi> From<T> for WasmRet<T> {
    fn from(value: T) -> Self {
        let (prim1, prim2, prim3, prim4) = value.split();
        Self {
            prim1,
            prim2,
            prim3,
            prim4,
        }
    }
}

// Ideally this'd just be an `Into<T>` implementation, but unfortunately that
// doesn't work because of the orphan rule.
impl<T: WasmAbi> WasmRet<T> {
    /// Joins the components of this `WasmRet` back into the type they represent.
    pub fn join(self) -> T {
        T::join(self.prim1, self.prim2, self.prim3, self.prim4)
    }
}

/// [`TryFromJsValue`] is a trait for converting a JavaScript value ([`JsValue`])
/// into a Rust type. It is used by the [`wasm_bindgen`](wasm_bindgen_macro::wasm_bindgen)
/// proc-macro to allow conversion to user types.
///
/// Types implementing this trait must specify their conversion logic from
/// [`JsValue`] to the Rust type, handling any potential errors that may occur
/// during the conversion process.
///
/// # ⚠️ Unstable
///
/// This is part of the internal [`convert`](crate::convert) module, **no
/// stability guarantees** are provided. Use at your own risk. See its
/// documentation for more details.
pub trait TryFromJsValue: Sized {
    /// The type returned in the event of a conversion error.
    type Error;

    /// Performs the conversion.
    fn try_from_js_value(value: JsValue) -> Result<Self, Self::Error>;
}
