use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = DOMPointInit ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `DomPointInit` dictionary.
    ///
    ///*This API requires the following crate features to be activated: `DomPointInit`*
    pub type DomPointInit;

}

impl DomPointInit {
    ///Construct a new `DomPointInit`.
    ///
    ///*This API requires the following crate features to be activated: `DomPointInit`*

    pub fn new() -> Self {

        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());

        ret
    }

    ///Change the `w` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `DomPointInit`*

    pub fn w(&mut self, val: f64) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("w"), &JsValue::from(val));

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }

    ///Change the `x` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `DomPointInit`*

    pub fn x(&mut self, val: f64) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("x"), &JsValue::from(val));

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }

    ///Change the `y` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `DomPointInit`*

    pub fn y(&mut self, val: f64) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("y"), &JsValue::from(val));

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }

    ///Change the `z` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `DomPointInit`*

    pub fn z(&mut self, val: f64) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("z"), &JsValue::from(val));

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }
}
