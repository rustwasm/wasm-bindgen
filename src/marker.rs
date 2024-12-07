#[cfg(feature = "msrv")]
extern crate rustversion;

/// Marker trait for types that support `#[wasm_bindgen(constructor)]`.
#[cfg_attr(
    feature = "msrv",
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
pub struct CheckSupportsConstructor<T: SupportsConstructor>(T);

/// Marker trait for types that support `#[wasm_bindgen(getter)]` or
/// `#[wasm_bindgen(Setter)]` on instance methods.
#[cfg_attr(
    feature = "msrv",
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
pub struct CheckSupportsInstanceProperty<T: SupportsInstanceProperty>(T);

/// Marker trait for types that support `#[wasm_bindgen(getter)]` or
/// `#[wasm_bindgen(Setter)]` on static methods.
#[cfg_attr(
    feature = "msrv",
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
pub struct CheckSupportsStaticProperty<T: SupportsStaticProperty>(T);
