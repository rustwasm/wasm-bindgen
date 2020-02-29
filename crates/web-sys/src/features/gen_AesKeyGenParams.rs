use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = AesKeyGenParams ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `AesKeyGenParams` dictionary.
    ///
    ///*This API requires the following crate features to be activated: `AesKeyGenParams`*
    pub type AesKeyGenParams;

}

impl AesKeyGenParams {
    ///Construct a new `AesKeyGenParams`.
    ///
    ///*This API requires the following crate features to be activated: `AesKeyGenParams`*

    pub fn new(name: &str, length: u16) -> Self {

        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());

        ret.name(name);

        ret.length(length);

        ret
    }

    ///Change the `name` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `AesKeyGenParams`*

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

    ///Change the `length` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `AesKeyGenParams`*

    pub fn length(&mut self, val: u16) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r =
            ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("length"), &JsValue::from(val));

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }
}
