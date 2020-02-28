use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[doc = "The `GpuInputStepMode` enum."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `GpuInputStepMode`*"]
#[cfg(web_sys_unstable_apis)]
#[doc = ""]
#[doc = ""]
#[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
#[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum GpuInputStepMode {
    Vertex = "vertex",
    Instance = "instance",
}
