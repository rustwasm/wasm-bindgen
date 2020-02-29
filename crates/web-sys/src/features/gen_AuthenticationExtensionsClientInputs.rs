use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = AuthenticationExtensionsClientInputs ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `AuthenticationExtensionsClientInputs` dictionary.
    ///
    ///*This API requires the following crate features to be activated: `AuthenticationExtensionsClientInputs`*
    pub type AuthenticationExtensionsClientInputs;

}

impl AuthenticationExtensionsClientInputs {
    ///Construct a new `AuthenticationExtensionsClientInputs`.
    ///
    ///*This API requires the following crate features to be activated: `AuthenticationExtensionsClientInputs`*

    pub fn new() -> Self {

        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());

        ret
    }

    ///Change the `appid` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `AuthenticationExtensionsClientInputs`*

    pub fn appid(&mut self, val: &str) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("appid"), &JsValue::from(val));

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }
}
