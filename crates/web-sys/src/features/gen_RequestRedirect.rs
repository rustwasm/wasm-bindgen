use wasm_bindgen::prelude::*;

#[wasm_bindgen]
///The `RequestRedirect` enum.
///
///*This API requires the following crate features to be activated: `RequestRedirect`*
#[derive(Debug, Clone, Copy, PartialEq, Eq)]

pub enum RequestRedirect {
    Follow = "follow",
    Error = "error",
    Manual = "manual",
}
