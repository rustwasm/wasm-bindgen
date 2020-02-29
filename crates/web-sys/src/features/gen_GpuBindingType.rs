use wasm_bindgen::prelude::*;

#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
///The `GpuBindingType` enum.
///
///*This API requires the following crate features to be activated: `GpuBindingType`*
///
///*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as
///[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*
#[derive(Debug, Clone, Copy, PartialEq, Eq)]

pub enum GpuBindingType {
    UniformBuffer = "uniform-buffer",
    StorageBuffer = "storage-buffer",
    ReadonlyStorageBuffer = "readonly-storage-buffer",
    Sampler = "sampler",
    SampledTexture = "sampled-texture",
    StorageTexture = "storage-texture",
}
