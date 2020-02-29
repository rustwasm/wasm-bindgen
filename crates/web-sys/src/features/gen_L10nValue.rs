use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = L10nValue ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `L10nValue` dictionary.
    ///
    ///*This API requires the following crate features to be activated: `L10nValue`*
    pub type L10nValue;

}

impl L10nValue {
    ///Construct a new `L10nValue`.
    ///
    ///*This API requires the following crate features to be activated: `L10nValue`*

    pub fn new() -> Self {

        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());

        ret
    }

    ///Change the `attributes` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `L10nValue`*

    pub fn attributes(&mut self, val: Option<&::wasm_bindgen::JsValue>) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("attributes"),
            &JsValue::from(val),
        );

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }

    ///Change the `value` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `L10nValue`*

    pub fn value(&mut self, val: Option<&str>) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("value"), &JsValue::from(val));

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }
}
