use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[doc = "The `RequestRedirect` enum."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `RequestRedirect`*"]
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum RequestRedirect {
    Follow = "follow",
    Error = "error",
    Manual = "manual",
}
