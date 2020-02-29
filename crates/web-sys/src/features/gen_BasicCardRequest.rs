use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = BasicCardRequest ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `BasicCardRequest` dictionary.
    ///
    ///*This API requires the following crate features to be activated: `BasicCardRequest`*
    pub type BasicCardRequest;

}

impl BasicCardRequest {
    ///Construct a new `BasicCardRequest`.
    ///
    ///*This API requires the following crate features to be activated: `BasicCardRequest`*

    pub fn new() -> Self {

        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());

        ret
    }

    ///Change the `supportedNetworks` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `BasicCardRequest`*

    pub fn supported_networks(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("supportedNetworks"),
            &JsValue::from(val),
        );

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }

    ///Change the `supportedTypes` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `BasicCardRequest`*

    pub fn supported_types(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("supportedTypes"),
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
