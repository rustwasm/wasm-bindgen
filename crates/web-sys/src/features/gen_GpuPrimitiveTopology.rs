use wasm_bindgen::prelude::*;

#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
///The `GpuPrimitiveTopology` enum.
///
///*This API requires the following crate features to be activated: `GpuPrimitiveTopology`*
///
///*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as
///[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*
#[derive(Debug, Clone, Copy, PartialEq, Eq)]

pub enum GpuPrimitiveTopology {
    PointList = "point-list",
    LineList = "line-list",
    LineStrip = "line-strip",
    TriangleList = "triangle-list",
    TriangleStrip = "triangle-strip",
}
