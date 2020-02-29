use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = OpenWindowEventDetail ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `OpenWindowEventDetail` dictionary.
    ///
    ///*This API requires the following crate features to be activated: `OpenWindowEventDetail`*
    pub type OpenWindowEventDetail;

}

impl OpenWindowEventDetail {
    ///Construct a new `OpenWindowEventDetail`.
    ///
    ///*This API requires the following crate features to be activated: `OpenWindowEventDetail`*

    pub fn new() -> Self {

        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());

        ret
    }

    ///Change the `features` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `OpenWindowEventDetail`*

    pub fn features(&mut self, val: &str) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("features"),
            &JsValue::from(val),
        );

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }

    #[cfg(feature = "Node")]
    ///Change the `frameElement` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `Node`, `OpenWindowEventDetail`*

    pub fn frame_element(&mut self, val: Option<&Node>) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("frameElement"),
            &JsValue::from(val),
        );

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }

    ///Change the `name` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `OpenWindowEventDetail`*

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

    ///Change the `url` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `OpenWindowEventDetail`*

    pub fn url(&mut self, val: &str) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("url"), &JsValue::from(val));

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }
}
