use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = MediaRecorderOptions ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `MediaRecorderOptions` dictionary.
    ///
    ///*This API requires the following crate features to be activated: `MediaRecorderOptions`*
    pub type MediaRecorderOptions;

}

impl MediaRecorderOptions {
    ///Construct a new `MediaRecorderOptions`.
    ///
    ///*This API requires the following crate features to be activated: `MediaRecorderOptions`*

    pub fn new() -> Self {

        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());

        ret
    }

    ///Change the `audioBitsPerSecond` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `MediaRecorderOptions`*

    pub fn audio_bits_per_second(&mut self, val: u32) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("audioBitsPerSecond"),
            &JsValue::from(val),
        );

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }

    ///Change the `bitsPerSecond` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `MediaRecorderOptions`*

    pub fn bits_per_second(&mut self, val: u32) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("bitsPerSecond"),
            &JsValue::from(val),
        );

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }

    ///Change the `mimeType` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `MediaRecorderOptions`*

    pub fn mime_type(&mut self, val: &str) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("mimeType"),
            &JsValue::from(val),
        );

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }

    ///Change the `videoBitsPerSecond` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `MediaRecorderOptions`*

    pub fn video_bits_per_second(&mut self, val: u32) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("videoBitsPerSecond"),
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
