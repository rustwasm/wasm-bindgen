#![allow(unused_imports)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = GetAnimationsOptions)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `GetAnimationsOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GetAnimationsOptions`*"]
    pub type GetAnimationsOptions;
}
impl GetAnimationsOptions {
    #[doc = "Construct a new `GetAnimationsOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GetAnimationsOptions`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `subtree` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GetAnimationsOptions`*"]
    pub fn subtree(&mut self, val: bool) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("subtree"),
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
impl Default for GetAnimationsOptions {
    fn default() -> Self {
        Self::new()
    }
}
