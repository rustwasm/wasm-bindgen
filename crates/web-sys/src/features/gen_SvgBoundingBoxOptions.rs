use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = SVGBoundingBoxOptions ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `SvgBoundingBoxOptions` dictionary.
    ///
    ///*This API requires the following crate features to be activated: `SvgBoundingBoxOptions`*
    pub type SvgBoundingBoxOptions;

}

impl SvgBoundingBoxOptions {
    ///Construct a new `SvgBoundingBoxOptions`.
    ///
    ///*This API requires the following crate features to be activated: `SvgBoundingBoxOptions`*

    pub fn new() -> Self {

        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());

        ret
    }

    ///Change the `clipped` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `SvgBoundingBoxOptions`*

    pub fn clipped(&mut self, val: bool) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("clipped"),
            &JsValue::from(val),
        );

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }

    ///Change the `fill` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `SvgBoundingBoxOptions`*

    pub fn fill(&mut self, val: bool) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("fill"), &JsValue::from(val));

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }

    ///Change the `markers` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `SvgBoundingBoxOptions`*

    pub fn markers(&mut self, val: bool) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("markers"),
            &JsValue::from(val),
        );

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }

    ///Change the `stroke` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `SvgBoundingBoxOptions`*

    pub fn stroke(&mut self, val: bool) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r =
            ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("stroke"), &JsValue::from(val));

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }
}
