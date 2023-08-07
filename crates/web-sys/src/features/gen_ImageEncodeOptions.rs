#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = ImageEncodeOptions)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ImageEncodeOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageEncodeOptions`*"]
    pub type ImageEncodeOptions;
}
impl ImageEncodeOptions {
    #[doc = "Construct a new `ImageEncodeOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageEncodeOptions`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `quality` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageEncodeOptions`*"]
    pub fn quality(&mut self, val: f64) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("quality"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageEncodeOptions`*"]
    pub fn type_(&mut self, val: &str) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("type"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
}
impl Default for ImageEncodeOptions {
    fn default() -> Self {
        Self::new()
    }
}
