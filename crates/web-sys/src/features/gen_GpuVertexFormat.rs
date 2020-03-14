#![allow(unused_imports)]
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
#[doc = "The `GpuVertexFormat` enum."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `GpuVertexFormat`*"]
#[doc = ""]
#[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
#[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GpuVertexFormat {
    Uchar2 = "uchar2",
    Uchar4 = "uchar4",
    Char2 = "char2",
    Char4 = "char4",
    Uchar2norm = "uchar2norm",
    Uchar4norm = "uchar4norm",
    Char2norm = "char2norm",
    Char4norm = "char4norm",
    Ushort2 = "ushort2",
    Ushort4 = "ushort4",
    Short2 = "short2",
    Short4 = "short4",
    Ushort2norm = "ushort2norm",
    Ushort4norm = "ushort4norm",
    Short2norm = "short2norm",
    Short4norm = "short4norm",
    Half2 = "half2",
    Half4 = "half4",
    Float = "float",
    Float2 = "float2",
    Float3 = "float3",
    Float4 = "float4",
    Uint = "uint",
    Uint2 = "uint2",
    Uint3 = "uint3",
    Uint4 = "uint4",
    Int = "int",
    Int2 = "int2",
    Int3 = "int3",
    Int4 = "int4",
}
