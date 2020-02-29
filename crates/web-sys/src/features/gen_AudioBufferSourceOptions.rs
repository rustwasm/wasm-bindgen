use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = AudioBufferSourceOptions ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `AudioBufferSourceOptions` dictionary.
    ///
    ///*This API requires the following crate features to be activated: `AudioBufferSourceOptions`*
    pub type AudioBufferSourceOptions;

}

impl AudioBufferSourceOptions {
    ///Construct a new `AudioBufferSourceOptions`.
    ///
    ///*This API requires the following crate features to be activated: `AudioBufferSourceOptions`*

    pub fn new() -> Self {

        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());

        ret
    }

    #[cfg(feature = "AudioBuffer")]
    ///Change the `buffer` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `AudioBuffer`, `AudioBufferSourceOptions`*

    pub fn buffer(&mut self, val: Option<&AudioBuffer>) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r =
            ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("buffer"), &JsValue::from(val));

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }

    ///Change the `detune` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `AudioBufferSourceOptions`*

    pub fn detune(&mut self, val: f32) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r =
            ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("detune"), &JsValue::from(val));

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }

    ///Change the `loop` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `AudioBufferSourceOptions`*

    pub fn loop_(&mut self, val: bool) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("loop"), &JsValue::from(val));

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }

    ///Change the `loopEnd` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `AudioBufferSourceOptions`*

    pub fn loop_end(&mut self, val: f64) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("loopEnd"),
            &JsValue::from(val),
        );

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }

    ///Change the `loopStart` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `AudioBufferSourceOptions`*

    pub fn loop_start(&mut self, val: f64) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("loopStart"),
            &JsValue::from(val),
        );

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }

    ///Change the `playbackRate` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `AudioBufferSourceOptions`*

    pub fn playback_rate(&mut self, val: f32) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("playbackRate"),
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
