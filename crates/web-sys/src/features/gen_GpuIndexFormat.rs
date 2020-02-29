use wasm_bindgen::prelude::*;

#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
///The `GpuIndexFormat` enum.
///
///*This API requires the following crate features to be activated: `GpuIndexFormat`*
///
///*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as
///[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*
#[derive(Debug, Clone, Copy, PartialEq, Eq)]

pub enum GpuIndexFormat {
    Uint16 = "uint16",
    Uint32 = "uint32",
}
