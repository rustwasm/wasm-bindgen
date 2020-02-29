use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = MediaKeySystemMediaCapability ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `MediaKeySystemMediaCapability` dictionary.
    ///
    ///*This API requires the following crate features to be activated: `MediaKeySystemMediaCapability`*
    pub type MediaKeySystemMediaCapability;

}

impl MediaKeySystemMediaCapability {
    ///Construct a new `MediaKeySystemMediaCapability`.
    ///
    ///*This API requires the following crate features to be activated: `MediaKeySystemMediaCapability`*

    pub fn new() -> Self {

        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());

        ret
    }

    ///Change the `contentType` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `MediaKeySystemMediaCapability`*

    pub fn content_type(&mut self, val: &str) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("contentType"),
            &JsValue::from(val),
        );

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }

    ///Change the `robustness` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `MediaKeySystemMediaCapability`*

    pub fn robustness(&mut self, val: &str) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("robustness"),
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
