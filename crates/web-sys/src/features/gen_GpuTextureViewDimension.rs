use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[doc = "The `GpuTextureViewDimension` enum.\n\n*This API requires the following crate features to be activated: `GpuTextureViewDimension`*"]
#[cfg(web_sys_unstable_apis)]
#[doc = "\n\n*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as [described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum GpuTextureViewDimension {
    N1d = "1d",
    N2d = "2d",
    N2dArray = "2d-array",
    Cube = "cube",
    CubeArray = "cube-array",
    N3d = "3d",
}
