use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = ContextAttributes2D ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `ContextAttributes2d` dictionary.
    ///
    ///*This API requires the following crate features to be activated: `ContextAttributes2d`*
    pub type ContextAttributes2d;

}

impl ContextAttributes2d {
    ///Construct a new `ContextAttributes2d`.
    ///
    ///*This API requires the following crate features to be activated: `ContextAttributes2d`*

    pub fn new() -> Self {

        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());

        ret
    }

    ///Change the `alpha` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `ContextAttributes2d`*

    pub fn alpha(&mut self, val: bool) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("alpha"), &JsValue::from(val));

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }

    ///Change the `willReadFrequently` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `ContextAttributes2d`*

    pub fn will_read_frequently(&mut self, val: bool) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("willReadFrequently"),
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
