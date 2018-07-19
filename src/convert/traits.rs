//! This is mostly an internal module, no stability guarantees are provided. Use
//! at your own risk.

use core::ops::{Deref, DerefMut};

use describe::*;

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

pub trait RefMutFromWasmAbi: WasmDescribe {
    type Abi: WasmAbi;
    type Anchor: DerefMut<Target = Self>;
    unsafe fn ref_mut_from_abi(js: Self::Abi, extra: &mut Stack) -> Self::Anchor;
}

pub trait Stack {
    fn push(&mut self, bits: u32);
}

/// An unsafe trait which represents types that are ABI-safe to pass via wasm
/// arguments.
///
/// This is an unsafe trait to implement as there's no guarantee the type is
/// actually safe to transfer across the was boundary, it's up to you to
/// guarantee this so codegen works correctly.
pub unsafe trait WasmAbi {}

unsafe impl WasmAbi for u32 {}
unsafe impl WasmAbi for i32 {}
unsafe impl WasmAbi for f32 {}
unsafe impl WasmAbi for f64 {}
