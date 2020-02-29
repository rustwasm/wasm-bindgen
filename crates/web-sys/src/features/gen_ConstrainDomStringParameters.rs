use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = ConstrainDOMStringParameters ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `ConstrainDomStringParameters` dictionary.
    ///
    ///*This API requires the following crate features to be activated: `ConstrainDomStringParameters`*
    pub type ConstrainDomStringParameters;

}

impl ConstrainDomStringParameters {
    ///Construct a new `ConstrainDomStringParameters`.
    ///
    ///*This API requires the following crate features to be activated: `ConstrainDomStringParameters`*

    pub fn new() -> Self {

        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());

        ret
    }

    ///Change the `exact` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `ConstrainDomStringParameters`*

    pub fn exact(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("exact"), &JsValue::from(val));

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }

    ///Change the `ideal` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `ConstrainDomStringParameters`*

    pub fn ideal(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("ideal"), &JsValue::from(val));

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }
}
