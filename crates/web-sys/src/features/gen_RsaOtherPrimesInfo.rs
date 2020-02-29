use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = RsaOtherPrimesInfo ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `RsaOtherPrimesInfo` dictionary.
    ///
    ///*This API requires the following crate features to be activated: `RsaOtherPrimesInfo`*
    pub type RsaOtherPrimesInfo;

}

impl RsaOtherPrimesInfo {
    ///Construct a new `RsaOtherPrimesInfo`.
    ///
    ///*This API requires the following crate features to be activated: `RsaOtherPrimesInfo`*

    pub fn new(d: &str, r: &str, t: &str) -> Self {

        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());

        ret.d(d);

        ret.r(r);

        ret.t(t);

        ret
    }

    ///Change the `d` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `RsaOtherPrimesInfo`*

    pub fn d(&mut self, val: &str) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("d"), &JsValue::from(val));

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }

    ///Change the `r` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `RsaOtherPrimesInfo`*

    pub fn r(&mut self, val: &str) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("r"), &JsValue::from(val));

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }

    ///Change the `t` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `RsaOtherPrimesInfo`*

    pub fn t(&mut self, val: &str) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("t"), &JsValue::from(val));

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }
}
