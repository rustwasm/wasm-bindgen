use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = LocaleInfo ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `LocaleInfo` dictionary.
    ///
    ///*This API requires the following crate features to be activated: `LocaleInfo`*
    pub type LocaleInfo;

}

impl LocaleInfo {
    ///Construct a new `LocaleInfo`.
    ///
    ///*This API requires the following crate features to be activated: `LocaleInfo`*

    pub fn new() -> Self {

        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());

        ret
    }

    ///Change the `direction` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `LocaleInfo`*

    pub fn direction(&mut self, val: &str) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("direction"),
            &JsValue::from(val),
        );

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }

    ///Change the `locale` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `LocaleInfo`*

    pub fn locale(&mut self, val: &str) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r =
            ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("locale"), &JsValue::from(val));

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }
}
