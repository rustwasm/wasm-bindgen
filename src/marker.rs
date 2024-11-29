//! This module contains marker traits used by `wasm-bindgen` to verify its
//! generated code.
//!
//! # ⚠️ Unstable
//!
//! This is an internal module, no stability guarantees are provided. Use at
//! your own risk.

#[cfg(feature = "diagnostic")]
extern crate rustversion;

/// Marker trait for types that support `#[wasm_bindgen(constructor)]`.
///
/// **DO NOT** implement this trait manually. It is implemented automatically
/// for types that support constructors.
///
/// # ⚠️ Unstable
///
/// This is part of the internal [`convert`](crate::marker) module, **no
/// stability guarantees** are provided. Use at your own risk. See its
/// documentation for more details.
#[cfg_attr(
    feature = "diagnostic",
    rustversion::attr(
        since(1.78),
        diagnostic::on_unimplemented(
            message = "JavaScript constructors are not supported for `{Self}`",
            label = "this function cannot be the constructor of `{Self}`",
            note = "`#[wasm_bindgen(constructor)]` is generally only supported for `struct`s with `#[wasm_bindgen]` and cannot be used for `enum`s.",
            note = "Consider removing the `constructor` option and using a regular static method instead."
        )
    )
)]
pub trait SupportsConstructor {}

/// Marker trait for types that support `#[wasm_bindgen(getter)]` or
/// `#[wasm_bindgen(Setter)]` on instance methods.
///
/// **DO NOT** implement this trait manually. It is implemented automatically
/// for types that support instance properties.
///
/// # ⚠️ Unstable
///
/// This is part of the internal [`convert`](crate::marker) module, **no
/// stability guarantees** are provided. Use at your own risk. See its
/// documentation for more details.
#[cfg_attr(
    feature = "diagnostic",
    rustversion::attr(
        since(1.78),
        diagnostic::on_unimplemented(
            message = "JavaScript instance getters and setters are not supported for `{Self}`",
            label = "this method cannot be a getter or setter for `{Self}`",
            note = "`#[wasm_bindgen(getter)]` and `#[wasm_bindgen(setter)]` are generally only supported for `struct`s with `#[wasm_bindgen]`. They cannot be used for `enum`s."
        )
    )
)]
pub trait SupportsInstanceProperty {}

/// Marker trait for types that support `#[wasm_bindgen(getter)]` or
/// `#[wasm_bindgen(Setter)]` on static methods.
///
/// **DO NOT** implement this trait manually. It is implemented automatically
/// for types that support static properties.
///
/// # ⚠️ Unstable
///
/// This is part of the internal [`convert`](crate::marker) module, **no
/// stability guarantees** are provided. Use at your own risk. See its
/// documentation for more details.
#[cfg_attr(
    feature = "diagnostic",
    rustversion::attr(
        since(1.78),
        diagnostic::on_unimplemented(
            message = "JavaScript static getters and setters are not supported for `{Self}`",
            label = "this static function cannot be a static getter or setter on `{Self}`",
            note = "`#[wasm_bindgen(getter)]` and `#[wasm_bindgen(setter)]` are generally only supported for `struct`s with `#[wasm_bindgen]`. They cannot be used for `enum`s."
        )
    )
)]
pub trait SupportsStaticProperty {}
