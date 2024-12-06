#![allow(unused_imports)]
#![allow(clippy::all)]
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen(no_export)]
#[doc = "The `CompressionFormat` enum."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `CompressionFormat`*"]
#[doc = ""]
#[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
#[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompressionFormat {
    Deflate = "deflate",
    DeflateRaw = "deflate-raw",
    Gzip = "gzip",
}
