#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = WaveShaperOptions)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `WaveShaperOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WaveShaperOptions`*"]
    pub type WaveShaperOptions;
    #[wasm_bindgen(method, setter = "channelCount")]
    fn channel_count_shim(this: &WaveShaperOptions, val: u32);
    #[cfg(feature = "ChannelCountMode")]
    #[wasm_bindgen(method, setter = "channelCountMode")]
    fn channel_count_mode_shim(this: &WaveShaperOptions, val: ChannelCountMode);
    #[cfg(feature = "ChannelInterpretation")]
    #[wasm_bindgen(method, setter = "channelInterpretation")]
    fn channel_interpretation_shim(this: &WaveShaperOptions, val: ChannelInterpretation);
    #[wasm_bindgen(method, setter = "curve")]
    fn curve_shim(this: &WaveShaperOptions, val: &::wasm_bindgen::JsValue);
    #[cfg(feature = "OverSampleType")]
    #[wasm_bindgen(method, setter = "oversample")]
    fn oversample_shim(this: &WaveShaperOptions, val: OverSampleType);
}
impl WaveShaperOptions {
    #[doc = "Construct a new `WaveShaperOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WaveShaperOptions`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `channelCount` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WaveShaperOptions`*"]
    pub fn channel_count(&mut self, val: u32) -> &mut Self {
        self.channel_count_shim(val);
        self
    }
    #[cfg(feature = "ChannelCountMode")]
    #[doc = "Change the `channelCountMode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ChannelCountMode`, `WaveShaperOptions`*"]
    pub fn channel_count_mode(&mut self, val: ChannelCountMode) -> &mut Self {
        self.channel_count_mode_shim(val);
        self
    }
    #[cfg(feature = "ChannelInterpretation")]
    #[doc = "Change the `channelInterpretation` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ChannelInterpretation`, `WaveShaperOptions`*"]
    pub fn channel_interpretation(&mut self, val: ChannelInterpretation) -> &mut Self {
        self.channel_interpretation_shim(val);
        self
    }
    #[doc = "Change the `curve` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WaveShaperOptions`*"]
    pub fn curve(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.curve_shim(val);
        self
    }
    #[cfg(feature = "OverSampleType")]
    #[doc = "Change the `oversample` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OverSampleType`, `WaveShaperOptions`*"]
    pub fn oversample(&mut self, val: OverSampleType) -> &mut Self {
        self.oversample_shim(val);
        self
    }
}
impl Default for WaveShaperOptions {
    fn default() -> Self {
        Self::new()
    }
}
