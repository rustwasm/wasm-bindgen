use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = RegisteredKey ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `RegisteredKey` dictionary.
    ///
    ///*This API requires the following crate features to be activated: `RegisteredKey`*
    pub type RegisteredKey;

}

impl RegisteredKey {
    ///Construct a new `RegisteredKey`.
    ///
    ///*This API requires the following crate features to be activated: `RegisteredKey`*

    pub fn new() -> Self {

        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());

        ret
    }

    ///Change the `appId` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `RegisteredKey`*

    pub fn app_id(&mut self, val: Option<&str>) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("appId"), &JsValue::from(val));

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }

    ///Change the `keyHandle` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `RegisteredKey`*

    pub fn key_handle(&mut self, val: &str) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("keyHandle"),
            &JsValue::from(val),
        );

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }

    ///Change the `transports` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `RegisteredKey`*

    pub fn transports(&mut self, val: Option<&::wasm_bindgen::JsValue>) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("transports"),
            &JsValue::from(val),
        );

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }

    ///Change the `version` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `RegisteredKey`*

    pub fn version(&mut self, val: &str) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("version"),
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
