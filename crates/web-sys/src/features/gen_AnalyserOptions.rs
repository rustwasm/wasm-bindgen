use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = AnalyserOptions ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `AnalyserOptions` dictionary.
    ///
    ///*This API requires the following crate features to be activated: `AnalyserOptions`*
    pub type AnalyserOptions;

}

impl AnalyserOptions {
    ///Construct a new `AnalyserOptions`.
    ///
    ///*This API requires the following crate features to be activated: `AnalyserOptions`*

    pub fn new() -> Self {

        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());

        ret
    }

    ///Change the `channelCount` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `AnalyserOptions`*

    pub fn channel_count(&mut self, val: u32) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("channelCount"),
            &JsValue::from(val),
        );

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }

    #[cfg(feature = "ChannelCountMode")]
    ///Change the `channelCountMode` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `AnalyserOptions`, `ChannelCountMode`*

    pub fn channel_count_mode(&mut self, val: ChannelCountMode) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("channelCountMode"),
            &JsValue::from(val),
        );

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }

    #[cfg(feature = "ChannelInterpretation")]
    ///Change the `channelInterpretation` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `AnalyserOptions`, `ChannelInterpretation`*

    pub fn channel_interpretation(&mut self, val: ChannelInterpretation) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("channelInterpretation"),
            &JsValue::from(val),
        );

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }

    ///Change the `fftSize` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `AnalyserOptions`*

    pub fn fft_size(&mut self, val: u32) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("fftSize"),
            &JsValue::from(val),
        );

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }

    ///Change the `maxDecibels` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `AnalyserOptions`*

    pub fn max_decibels(&mut self, val: f64) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("maxDecibels"),
            &JsValue::from(val),
        );

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }

    ///Change the `minDecibels` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `AnalyserOptions`*

    pub fn min_decibels(&mut self, val: f64) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("minDecibels"),
            &JsValue::from(val),
        );

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }

    ///Change the `smoothingTimeConstant` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `AnalyserOptions`*

    pub fn smoothing_time_constant(&mut self, val: f64) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("smoothingTimeConstant"),
            &JsValue::from(val),
        );

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }
}
