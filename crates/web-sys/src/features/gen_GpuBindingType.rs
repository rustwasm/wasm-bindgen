use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[doc = "The `GpuBindingType` enum.\n\n*This API requires the following crate features to be activated: `GpuBindingType`*"]
#[cfg(web_sys_unstable_apis)]
#[doc = "\n\n*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as [described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum GpuBindingType {
    UniformBuffer = "uniform-buffer",
    StorageBuffer = "storage-buffer",
    ReadonlyStorageBuffer = "readonly-storage-buffer",
    Sampler = "sampler",
    SampledTexture = "sampled-texture",
    StorageTexture = "storage-texture",
}
