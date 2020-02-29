use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = RcwnPerfStats ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `RcwnPerfStats` dictionary.
    ///
    ///*This API requires the following crate features to be activated: `RcwnPerfStats`*
    pub type RcwnPerfStats;

}

impl RcwnPerfStats {
    ///Construct a new `RcwnPerfStats`.
    ///
    ///*This API requires the following crate features to be activated: `RcwnPerfStats`*

    pub fn new() -> Self {

        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());

        ret
    }

    ///Change the `avgLong` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `RcwnPerfStats`*

    pub fn avg_long(&mut self, val: u32) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("avgLong"),
            &JsValue::from(val),
        );

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }

    ///Change the `avgShort` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `RcwnPerfStats`*

    pub fn avg_short(&mut self, val: u32) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("avgShort"),
            &JsValue::from(val),
        );

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }

    ///Change the `stddevLong` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `RcwnPerfStats`*

    pub fn stddev_long(&mut self, val: u32) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("stddevLong"),
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
