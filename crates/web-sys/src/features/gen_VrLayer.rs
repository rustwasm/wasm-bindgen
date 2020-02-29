use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = VRLayer ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `VrLayer` dictionary.
    ///
    ///*This API requires the following crate features to be activated: `VrLayer`*
    pub type VrLayer;

}

impl VrLayer {
    ///Construct a new `VrLayer`.
    ///
    ///*This API requires the following crate features to be activated: `VrLayer`*

    pub fn new() -> Self {

        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());

        ret
    }

    ///Change the `leftBounds` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `VrLayer`*

    pub fn left_bounds(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("leftBounds"),
            &JsValue::from(val),
        );

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }

    ///Change the `rightBounds` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `VrLayer`*

    pub fn right_bounds(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("rightBounds"),
            &JsValue::from(val),
        );

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }

    #[cfg(feature = "HtmlCanvasElement")]
    ///Change the `source` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `HtmlCanvasElement`, `VrLayer`*

    pub fn source(&mut self, val: Option<&HtmlCanvasElement>) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r =
            ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("source"), &JsValue::from(val));

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }
}
