//! wasm-bindgen cli

pub mod bin {
    #[cfg(not(feature = "wasm2es6js"))]
    #[path = "wasm-bindgen.rs"]
    pub mod wasm_bindgen;

    #[cfg(feature = "wasm2es6js")]
    pub mod wasm2es6js;
}

#[cfg(not(feature = "wasm2es6js"))]
pub use bin::wasm_bindgen::main;

#[cfg(feature = "wasm2es6js")]
pub use bin::wasm2es6js::main;
