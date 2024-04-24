#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = ClientRectsAndTexts)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ClientRectsAndTexts` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ClientRectsAndTexts`*"]
    pub type ClientRectsAndTexts;
    #[cfg(feature = "DomRectList")]
    #[wasm_bindgen(method, getter = "rectList")]
    fn rect_list_shim(this: &ClientRectsAndTexts) -> &DomRectList;
    #[cfg(feature = "DomRectList")]
    #[wasm_bindgen(method, setter = "rectList")]
    fn set_rect_list_shim(this: &ClientRectsAndTexts, val: &DomRectList);
    #[wasm_bindgen(method, getter = "textList")]
    fn text_list_shim(this: &ClientRectsAndTexts) -> &::wasm_bindgen::JsValue;
    #[wasm_bindgen(method, setter = "textList")]
    fn set_text_list_shim(this: &ClientRectsAndTexts, val: &::wasm_bindgen::JsValue);
}
#[doc = "The trait to access properties on the `ClientRectsAndTexts` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `ClientRectsAndTexts`*"]
pub trait ClientRectsAndTextsGetters {
    #[cfg(feature = "DomRectList")]
    #[doc = "Get the `rectList` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ClientRectsAndTexts`, `DomRectList`*"]
    fn rect_list(&self) -> &DomRectList;
    #[doc = "Get the `textList` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ClientRectsAndTexts`*"]
    fn text_list(&self) -> &::wasm_bindgen::JsValue;
}
impl ClientRectsAndTextsGetters for ClientRectsAndTexts {
    #[cfg(feature = "DomRectList")]
    fn rect_list(&self) -> &DomRectList {
        self.rect_list_shim()
    }
    fn text_list(&self) -> &::wasm_bindgen::JsValue {
        self.text_list_shim()
    }
}
impl ClientRectsAndTexts {
    #[cfg(feature = "DomRectList")]
    #[doc = "Construct a new `ClientRectsAndTexts`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ClientRectsAndTexts`, `DomRectList`*"]
    pub fn new(rect_list: &DomRectList, text_list: &::wasm_bindgen::JsValue) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.rect_list(rect_list);
        ret.text_list(text_list);
        ret
    }
    #[cfg(feature = "DomRectList")]
    #[doc = "Change the `rectList` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ClientRectsAndTexts`, `DomRectList`*"]
    pub fn rect_list(&mut self, val: &DomRectList) -> &mut Self {
        self.set_rect_list_shim(val);
        self
    }
    #[doc = "Change the `textList` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ClientRectsAndTexts`*"]
    pub fn text_list(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_text_list_shim(val);
        self
    }
}
