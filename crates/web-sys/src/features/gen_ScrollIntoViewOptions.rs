use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = ScrollIntoViewOptions ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `ScrollIntoViewOptions` dictionary.
    ///
    ///*This API requires the following crate features to be activated: `ScrollIntoViewOptions`*
    pub type ScrollIntoViewOptions;

}

impl ScrollIntoViewOptions {
    ///Construct a new `ScrollIntoViewOptions`.
    ///
    ///*This API requires the following crate features to be activated: `ScrollIntoViewOptions`*

    pub fn new() -> Self {

        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());

        ret
    }

    #[cfg(feature = "ScrollBehavior")]
    ///Change the `behavior` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `ScrollBehavior`, `ScrollIntoViewOptions`*

    pub fn behavior(&mut self, val: ScrollBehavior) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("behavior"),
            &JsValue::from(val),
        );

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }

    #[cfg(feature = "ScrollLogicalPosition")]
    ///Change the `block` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `ScrollIntoViewOptions`, `ScrollLogicalPosition`*

    pub fn block(&mut self, val: ScrollLogicalPosition) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("block"), &JsValue::from(val));

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }

    #[cfg(feature = "ScrollLogicalPosition")]
    ///Change the `inline` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `ScrollIntoViewOptions`, `ScrollLogicalPosition`*

    pub fn inline(&mut self, val: ScrollLogicalPosition) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r =
            ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("inline"), &JsValue::from(val));

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }
}
