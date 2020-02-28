use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[doc = "The `GpuBlendFactor` enum.\n\n*This API requires the following crate features to be activated: `GpuBlendFactor`*"]
#[cfg(web_sys_unstable_apis)]
#[doc = "\n\n*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as [described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum GpuBlendFactor {
    Zero = "zero",
    One = "one",
    SrcColor = "src-color",
    OneMinusSrcColor = "one-minus-src-color",
    SrcAlpha = "src-alpha",
    OneMinusSrcAlpha = "one-minus-src-alpha",
    DstColor = "dst-color",
    OneMinusDstColor = "one-minus-dst-color",
    DstAlpha = "dst-alpha",
    OneMinusDstAlpha = "one-minus-dst-alpha",
    SrcAlphaSaturated = "src-alpha-saturated",
    BlendColor = "blend-color",
    OneMinusBlendColor = "one-minus-blend-color",
}
