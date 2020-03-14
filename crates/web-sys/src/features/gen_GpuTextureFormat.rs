#![allow(unused_imports)]
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
#[doc = "The `GpuTextureFormat` enum."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `GpuTextureFormat`*"]
#[doc = ""]
#[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
#[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GpuTextureFormat {
    R8unorm = "r8unorm",
    R8snorm = "r8snorm",
    R8uint = "r8uint",
    R8sint = "r8sint",
    R16uint = "r16uint",
    R16sint = "r16sint",
    R16float = "r16float",
    Rg8unorm = "rg8unorm",
    Rg8snorm = "rg8snorm",
    Rg8uint = "rg8uint",
    Rg8sint = "rg8sint",
    R32uint = "r32uint",
    R32sint = "r32sint",
    R32float = "r32float",
    Rg16uint = "rg16uint",
    Rg16sint = "rg16sint",
    Rg16float = "rg16float",
    Rgba8unorm = "rgba8unorm",
    Rgba8unormSrgb = "rgba8unorm-srgb",
    Rgba8snorm = "rgba8snorm",
    Rgba8uint = "rgba8uint",
    Rgba8sint = "rgba8sint",
    Bgra8unorm = "bgra8unorm",
    Bgra8unormSrgb = "bgra8unorm-srgb",
    Rgb10a2unorm = "rgb10a2unorm",
    Rg11b10float = "rg11b10float",
    Rg32uint = "rg32uint",
    Rg32sint = "rg32sint",
    Rg32float = "rg32float",
    Rgba16uint = "rgba16uint",
    Rgba16sint = "rgba16sint",
    Rgba16float = "rgba16float",
    Rgba32uint = "rgba32uint",
    Rgba32sint = "rgba32sint",
    Rgba32float = "rgba32float",
    Depth32float = "depth32float",
    Depth24plus = "depth24plus",
    Depth24plusStencil8 = "depth24plus-stencil8",
}
