#![allow(unused_imports)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = GPUTextureUsage , typescript_type = "GPUTextureUsage" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `GpuTextureUsage` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUTextureUsage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuTextureUsage`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type GpuTextureUsage;
}
#[cfg(web_sys_unstable_apis)]
impl GpuTextureUsage {
    #[cfg(web_sys_unstable_apis)]
    #[doc = "The `GPUTextureUsage.COPY_SRC` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuTextureUsage`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub const COPY_SRC: u32 = 1u64 as u32;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "The `GPUTextureUsage.COPY_DST` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuTextureUsage`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub const COPY_DST: u32 = 2u64 as u32;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "The `GPUTextureUsage.SAMPLED` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuTextureUsage`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub const SAMPLED: u32 = 4u64 as u32;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "The `GPUTextureUsage.STORAGE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuTextureUsage`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub const STORAGE: u32 = 8u64 as u32;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "The `GPUTextureUsage.OUTPUT_ATTACHMENT` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuTextureUsage`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub const OUTPUT_ATTACHMENT: u32 = 16u64 as u32;
}
