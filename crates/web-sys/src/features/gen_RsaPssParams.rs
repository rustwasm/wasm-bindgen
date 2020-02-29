use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = RsaPssParams ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `RsaPssParams` dictionary.
    ///
    ///*This API requires the following crate features to be activated: `RsaPssParams`*
    pub type RsaPssParams;

}

impl RsaPssParams {
    ///Construct a new `RsaPssParams`.
    ///
    ///*This API requires the following crate features to be activated: `RsaPssParams`*

    pub fn new(name: &str, salt_length: u32) -> Self {

        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());

        ret.name(name);

        ret.salt_length(salt_length);

        ret
    }

    ///Change the `name` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `RsaPssParams`*

    pub fn name(&mut self, val: &str) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("name"), &JsValue::from(val));

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }

    ///Change the `saltLength` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `RsaPssParams`*

    pub fn salt_length(&mut self, val: u32) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("saltLength"),
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
