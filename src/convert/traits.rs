use core::ops::{Deref, DerefMut};

use convert::abi::WasmAbi;
use describe::WasmDescribe;

/// A trait for anything that can be converted into a type that can cross the
/// wasm ABI directly, eg `u32` or `f64`.
///
/// This is the opposite operation as `FromWasmAbi` and `Ref[Mut]FromWasmAbi`.
pub trait IntoWasmAbi: WasmDescribe {
    /// The wasm ABI type that this converts into when crossing the ABI
    /// boundary.
    type Abi: WasmAbi;

    /// Convert `self` into `Self::Abi` so that it can be sent across the wasm
    /// ABI boundary.
    fn into_abi(self, extra: &mut Stack) -> Self::Abi;
}

/// A trait for anything that can be recovered by-value from the wasm ABI
/// boundary, eg a Rust `u8` can be recovered from the wasm ABI `u32` type.
///
/// This is the by-value variant of the opposite operation as `IntoWasmAbi`.
pub trait FromWasmAbi: WasmDescribe {
    /// The wasm ABI type that this converts from when coming back out from the
    /// ABI boundary.
    type Abi: WasmAbi;

    /// Recover a `Self` from `Self::Abi`.
    ///
    /// # Safety
    ///
    /// This is only safe to call when -- and implementations may assume that --
    /// the supplied `Self::Abi` was previously generated by a call to `<Self as
    /// IntoWasmAbi>::into_abi()` or the moral equivalent in JS.
    unsafe fn from_abi(js: Self::Abi, extra: &mut Stack) -> Self;
}

/// A trait for anything that can be recovered as some sort of shared reference
/// from the wasm ABI boundary.
///
/// This is the shared reference variant of the opposite operation as
/// `IntoWasmAbi`.
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
    unsafe fn ref_from_abi(js: Self::Abi, extra: &mut Stack) -> Self::Anchor;
}

/// Dual of the `RefFromWasmAbi` trait, except for mutable references.
pub trait RefMutFromWasmAbi: WasmDescribe {
    /// Same as `RefFromWasmAbi::Abi`
    type Abi: WasmAbi;
    /// Same as `RefFromWasmAbi::Anchor`
    type Anchor: DerefMut<Target = Self>;
    /// Same as `RefFromWasmAbi::ref_from_abi`
    unsafe fn ref_mut_from_abi(js: Self::Abi, extra: &mut Stack) -> Self::Anchor;
}

/// Indicates that this type can be passed to JS as `Option<Self>`.
///
/// This trait is used when implementing `IntoWasmAbi for Option<T>`.
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
pub trait OptionFromWasmAbi: FromWasmAbi {
    /// Tests whether the argument is a "none" instance. If so it will be
    /// deserialized as `None`, and otherwise it will be passed to
    /// `FromWasmAbi`.
    fn is_none(abi: &Self::Abi) -> bool;
}

pub trait Stack {
    fn push(&mut self, bits: u32);
}

/// A trait representing how to interepret the return value of a function for
/// the wasm ABI.
///
/// This is very similar to the `IntoWasmAbi` trait and in fact has a blanket
/// implementation for all implementors of the `IntoWasmAbi`. The primary use
/// case of this trait is to enable functions to return `Result`, interpreting
/// an error as "rethrow this to JS"
pub trait ReturnWasmAbi: WasmDescribe {
    /// Same as `IntoWasmAbi::Abi`
    type Abi: WasmAbi;

    /// Same as `IntoWasmAbi::into_abi`, except that it may throw and never
    /// return in the case of `Err`.
    fn return_abi(self, extra: &mut Stack) -> Self::Abi;
}

impl<T: IntoWasmAbi> ReturnWasmAbi for T {
    type Abi = T::Abi;
    fn return_abi(self, extra: &mut Stack) -> Self::Abi {
        self.into_abi(extra)
    }
}
