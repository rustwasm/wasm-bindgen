use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = SpeechRecognitionErrorInit ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `SpeechRecognitionErrorInit` dictionary.
    ///
    ///*This API requires the following crate features to be activated: `SpeechRecognitionErrorInit`*
    pub type SpeechRecognitionErrorInit;

}

impl SpeechRecognitionErrorInit {
    ///Construct a new `SpeechRecognitionErrorInit`.
    ///
    ///*This API requires the following crate features to be activated: `SpeechRecognitionErrorInit`*

    pub fn new() -> Self {

        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());

        ret
    }

    ///Change the `bubbles` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `SpeechRecognitionErrorInit`*

    pub fn bubbles(&mut self, val: bool) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("bubbles"),
            &JsValue::from(val),
        );

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }

    ///Change the `cancelable` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `SpeechRecognitionErrorInit`*

    pub fn cancelable(&mut self, val: bool) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("cancelable"),
            &JsValue::from(val),
        );

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }

    ///Change the `composed` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `SpeechRecognitionErrorInit`*

    pub fn composed(&mut self, val: bool) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("composed"),
            &JsValue::from(val),
        );

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }

    #[cfg(feature = "SpeechRecognitionErrorCode")]
    ///Change the `error` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `SpeechRecognitionErrorCode`, `SpeechRecognitionErrorInit`*

    pub fn error(&mut self, val: SpeechRecognitionErrorCode) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("error"), &JsValue::from(val));

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }

    ///Change the `message` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `SpeechRecognitionErrorInit`*

    pub fn message(&mut self, val: &str) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("message"),
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
