use super::*;
use wasm_bindgen::prelude::*;

#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = GPUShaderStage , typescript_name = GPUShaderStage ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `GpuShaderStage` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUShaderStage)
    ///
    ///*This API requires the following crate features to be activated: `GpuShaderStage`*
    ///
    ///*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as
    ///[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*
    pub type GpuShaderStage;

}

#[cfg(web_sys_unstable_apis)]

impl GpuShaderStage {
    #[cfg(web_sys_unstable_apis)]
    ///The `GPUShaderStage.VERTEX` const.
    ///
    ///*This API requires the following crate features to be activated: `GpuShaderStage`*
    ///
    ///*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as
    ///[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*

    pub const VERTEX: u32 = 1u64 as u32;

    #[cfg(web_sys_unstable_apis)]
    ///The `GPUShaderStage.FRAGMENT` const.
    ///
    ///*This API requires the following crate features to be activated: `GpuShaderStage`*
    ///
    ///*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as
    ///[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*

    pub const FRAGMENT: u32 = 2u64 as u32;

    #[cfg(web_sys_unstable_apis)]
    ///The `GPUShaderStage.COMPUTE` const.
    ///
    ///*This API requires the following crate features to be activated: `GpuShaderStage`*
    ///
    ///*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as
    ///[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*

    pub const COMPUTE: u32 = 4u64 as u32;
}
