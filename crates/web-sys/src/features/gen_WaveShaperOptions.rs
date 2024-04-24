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
    #[wasm_bindgen(method, getter = "channelCount")]
    fn channel_count_shim(this: &WaveShaperOptions) -> u32;
    #[wasm_bindgen(method, setter = "channelCount")]
    fn set_channel_count_shim(this: &WaveShaperOptions, val: u32);
    #[cfg(feature = "ChannelCountMode")]
    #[wasm_bindgen(method, getter = "channelCountMode")]
    fn channel_count_mode_shim(this: &WaveShaperOptions) -> ChannelCountMode;
    #[cfg(feature = "ChannelCountMode")]
    #[wasm_bindgen(method, setter = "channelCountMode")]
    fn set_channel_count_mode_shim(this: &WaveShaperOptions, val: ChannelCountMode);
    #[cfg(feature = "ChannelInterpretation")]
    #[wasm_bindgen(method, getter = "channelInterpretation")]
    fn channel_interpretation_shim(this: &WaveShaperOptions) -> ChannelInterpretation;
    #[cfg(feature = "ChannelInterpretation")]
    #[wasm_bindgen(method, setter = "channelInterpretation")]
    fn set_channel_interpretation_shim(this: &WaveShaperOptions, val: ChannelInterpretation);
    #[wasm_bindgen(method, getter = "curve")]
    fn curve_shim(this: &WaveShaperOptions) -> &::wasm_bindgen::JsValue;
    #[wasm_bindgen(method, setter = "curve")]
    fn set_curve_shim(this: &WaveShaperOptions, val: &::wasm_bindgen::JsValue);
    #[cfg(feature = "OverSampleType")]
    #[wasm_bindgen(method, getter = "oversample")]
    fn oversample_shim(this: &WaveShaperOptions) -> OverSampleType;
    #[cfg(feature = "OverSampleType")]
    #[wasm_bindgen(method, setter = "oversample")]
    fn set_oversample_shim(this: &WaveShaperOptions, val: OverSampleType);
}
#[doc = "The trait to access properties on the `WaveShaperOptions` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `WaveShaperOptions`*"]
pub trait WaveShaperOptionsGetters {
    #[doc = "Get the `channelCount` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WaveShaperOptions`*"]
    fn channel_count(&self) -> u32;
    #[cfg(feature = "ChannelCountMode")]
    #[doc = "Get the `channelCountMode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ChannelCountMode`, `WaveShaperOptions`*"]
    fn channel_count_mode(&self) -> ChannelCountMode;
    #[cfg(feature = "ChannelInterpretation")]
    #[doc = "Get the `channelInterpretation` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ChannelInterpretation`, `WaveShaperOptions`*"]
    fn channel_interpretation(&self) -> ChannelInterpretation;
    #[doc = "Get the `curve` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WaveShaperOptions`*"]
    fn curve(&self) -> &::wasm_bindgen::JsValue;
    #[cfg(feature = "OverSampleType")]
    #[doc = "Get the `oversample` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OverSampleType`, `WaveShaperOptions`*"]
    fn oversample(&self) -> OverSampleType;
}
impl WaveShaperOptionsGetters for WaveShaperOptions {
    fn channel_count(&self) -> u32 {
        self.channel_count_shim()
    }
    #[cfg(feature = "ChannelCountMode")]
    fn channel_count_mode(&self) -> ChannelCountMode {
        self.channel_count_mode_shim()
    }
    #[cfg(feature = "ChannelInterpretation")]
    fn channel_interpretation(&self) -> ChannelInterpretation {
        self.channel_interpretation_shim()
    }
    fn curve(&self) -> &::wasm_bindgen::JsValue {
        self.curve_shim()
    }
    #[cfg(feature = "OverSampleType")]
    fn oversample(&self) -> OverSampleType {
        self.oversample_shim()
    }
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
        self.set_channel_count_shim(val);
        self
    }
    #[cfg(feature = "ChannelCountMode")]
    #[doc = "Change the `channelCountMode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ChannelCountMode`, `WaveShaperOptions`*"]
    pub fn channel_count_mode(&mut self, val: ChannelCountMode) -> &mut Self {
        self.set_channel_count_mode_shim(val);
        self
    }
    #[cfg(feature = "ChannelInterpretation")]
    #[doc = "Change the `channelInterpretation` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ChannelInterpretation`, `WaveShaperOptions`*"]
    pub fn channel_interpretation(&mut self, val: ChannelInterpretation) -> &mut Self {
        self.set_channel_interpretation_shim(val);
        self
    }
    #[doc = "Change the `curve` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WaveShaperOptions`*"]
    pub fn curve(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_curve_shim(val);
        self
    }
    #[cfg(feature = "OverSampleType")]
    #[doc = "Change the `oversample` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OverSampleType`, `WaveShaperOptions`*"]
    pub fn oversample(&mut self, val: OverSampleType) -> &mut Self {
        self.set_oversample_shim(val);
        self
    }
}
impl Default for WaveShaperOptions {
    fn default() -> Self {
        Self::new()
    }
}
