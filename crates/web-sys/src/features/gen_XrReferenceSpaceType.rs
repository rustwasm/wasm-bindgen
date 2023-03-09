#![allow(unused_imports)]
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
#[doc = "The `XrReferenceSpaceType` enum."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `XrReferenceSpaceType`*"]
#[doc = ""]
#[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
#[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum XrReferenceSpaceType {
    Viewer = "viewer",
    Local = "local",
    LocalFloor = "local-floor",
    BoundedFloor = "bounded-floor",
    Unbounded = "unbounded",
}
