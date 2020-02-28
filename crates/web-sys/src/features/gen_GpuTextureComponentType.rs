use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[doc = "The `GpuTextureComponentType` enum."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `GpuTextureComponentType`*"]
#[cfg(web_sys_unstable_apis)]
#[doc = ""]
#[doc = ""]
#[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
#[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum GpuTextureComponentType {
    Float = "float",
    Sint = "sint",
    Uint = "uint",
}
