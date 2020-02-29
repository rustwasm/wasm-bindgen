use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = ClientRectsAndTexts ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `ClientRectsAndTexts` dictionary.
    ///
    ///*This API requires the following crate features to be activated: `ClientRectsAndTexts`*
    pub type ClientRectsAndTexts;

}

impl ClientRectsAndTexts {
    #[cfg(feature = "DomRectList")]
    ///Construct a new `ClientRectsAndTexts`.
    ///
    ///*This API requires the following crate features to be activated: `ClientRectsAndTexts`, `DomRectList`*

    pub fn new(rect_list: &DomRectList, text_list: &::wasm_bindgen::JsValue) -> Self {

        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());

        ret.rect_list(rect_list);

        ret.text_list(text_list);

        ret
    }

    #[cfg(feature = "DomRectList")]
    ///Change the `rectList` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `ClientRectsAndTexts`, `DomRectList`*

    pub fn rect_list(&mut self, val: &DomRectList) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("rectList"),
            &JsValue::from(val),
        );

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }

    ///Change the `textList` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `ClientRectsAndTexts`*

    pub fn text_list(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("textList"),
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
