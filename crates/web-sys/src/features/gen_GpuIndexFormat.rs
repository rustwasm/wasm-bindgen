use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[doc = "The `GpuIndexFormat` enum.\n\n*This API requires the following crate features to be activated: `GpuIndexFormat`*"]
#[cfg(web_sys_unstable_apis)]
#[doc = "\n\n*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as [described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum GpuIndexFormat {
    Uint16 = "uint16",
    Uint32 = "uint32",
}
