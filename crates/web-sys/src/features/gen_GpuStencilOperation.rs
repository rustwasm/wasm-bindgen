use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[doc = "The `GpuStencilOperation` enum."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `GpuStencilOperation`*"]
#[cfg(web_sys_unstable_apis)]
#[doc = ""]
#[doc = ""]
#[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
#[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum GpuStencilOperation {
    Keep = "keep",
    Zero = "zero",
    Replace = "replace",
    Invert = "invert",
    IncrementClamp = "increment-clamp",
    DecrementClamp = "decrement-clamp",
    IncrementWrap = "increment-wrap",
    DecrementWrap = "decrement-wrap",
}
