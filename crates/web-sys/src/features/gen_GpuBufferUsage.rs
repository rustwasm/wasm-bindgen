use super::*;
use wasm_bindgen::prelude::*;

#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = GPUBufferUsage , typescript_name = GPUBufferUsage ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `GpuBufferUsage` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUBufferUsage)
    ///
    ///*This API requires the following crate features to be activated: `GpuBufferUsage`*
    ///
    ///*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as
    ///[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*
    pub type GpuBufferUsage;

}

#[cfg(web_sys_unstable_apis)]

impl GpuBufferUsage {
    #[cfg(web_sys_unstable_apis)]
    ///The `GPUBufferUsage.MAP_READ` const.
    ///
    ///*This API requires the following crate features to be activated: `GpuBufferUsage`*
    ///
    ///*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as
    ///[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*

    pub const MAP_READ: u32 = 1u64 as u32;

    #[cfg(web_sys_unstable_apis)]
    ///The `GPUBufferUsage.MAP_WRITE` const.
    ///
    ///*This API requires the following crate features to be activated: `GpuBufferUsage`*
    ///
    ///*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as
    ///[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*

    pub const MAP_WRITE: u32 = 2u64 as u32;

    #[cfg(web_sys_unstable_apis)]
    ///The `GPUBufferUsage.COPY_SRC` const.
    ///
    ///*This API requires the following crate features to be activated: `GpuBufferUsage`*
    ///
    ///*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as
    ///[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*

    pub const COPY_SRC: u32 = 4u64 as u32;

    #[cfg(web_sys_unstable_apis)]
    ///The `GPUBufferUsage.COPY_DST` const.
    ///
    ///*This API requires the following crate features to be activated: `GpuBufferUsage`*
    ///
    ///*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as
    ///[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*

    pub const COPY_DST: u32 = 8u64 as u32;

    #[cfg(web_sys_unstable_apis)]
    ///The `GPUBufferUsage.INDEX` const.
    ///
    ///*This API requires the following crate features to be activated: `GpuBufferUsage`*
    ///
    ///*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as
    ///[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*

    pub const INDEX: u32 = 16u64 as u32;

    #[cfg(web_sys_unstable_apis)]
    ///The `GPUBufferUsage.VERTEX` const.
    ///
    ///*This API requires the following crate features to be activated: `GpuBufferUsage`*
    ///
    ///*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as
    ///[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*

    pub const VERTEX: u32 = 32u64 as u32;

    #[cfg(web_sys_unstable_apis)]
    ///The `GPUBufferUsage.UNIFORM` const.
    ///
    ///*This API requires the following crate features to be activated: `GpuBufferUsage`*
    ///
    ///*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as
    ///[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*

    pub const UNIFORM: u32 = 64u64 as u32;

    #[cfg(web_sys_unstable_apis)]
    ///The `GPUBufferUsage.STORAGE` const.
    ///
    ///*This API requires the following crate features to be activated: `GpuBufferUsage`*
    ///
    ///*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as
    ///[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*

    pub const STORAGE: u32 = 128u64 as u32;

    #[cfg(web_sys_unstable_apis)]
    ///The `GPUBufferUsage.INDIRECT` const.
    ///
    ///*This API requires the following crate features to be activated: `GpuBufferUsage`*
    ///
    ///*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as
    ///[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*

    pub const INDIRECT: u32 = 256u64 as u32;
}
