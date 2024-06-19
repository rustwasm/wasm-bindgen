#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = HashChangeEventInit)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `HashChangeEventInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HashChangeEventInit`*"]
    pub type HashChangeEventInit;
    #[doc = "Get the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HashChangeEventInit`*"]
    #[wasm_bindgen(method, getter = "bubbles")]
    pub fn get_bubbles(this: &HashChangeEventInit) -> Option<bool>;
    #[wasm_bindgen(method, setter = "bubbles")]
    fn set_bubbles(this: &HashChangeEventInit, val: bool);
    #[doc = "Get the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HashChangeEventInit`*"]
    #[wasm_bindgen(method, getter = "cancelable")]
    pub fn get_cancelable(this: &HashChangeEventInit) -> Option<bool>;
    #[wasm_bindgen(method, setter = "cancelable")]
    fn set_cancelable(this: &HashChangeEventInit, val: bool);
    #[doc = "Get the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HashChangeEventInit`*"]
    #[wasm_bindgen(method, getter = "composed")]
    pub fn get_composed(this: &HashChangeEventInit) -> Option<bool>;
    #[wasm_bindgen(method, setter = "composed")]
    fn set_composed(this: &HashChangeEventInit, val: bool);
    #[doc = "Get the `newURL` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HashChangeEventInit`*"]
    #[wasm_bindgen(method, getter = "newURL")]
    pub fn get_new_url(this: &HashChangeEventInit) -> Option<String>;
    #[wasm_bindgen(method, setter = "newURL")]
    fn set_new_url(this: &HashChangeEventInit, val: &str);
    #[doc = "Get the `oldURL` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HashChangeEventInit`*"]
    #[wasm_bindgen(method, getter = "oldURL")]
    pub fn get_old_url(this: &HashChangeEventInit) -> Option<String>;
    #[wasm_bindgen(method, setter = "oldURL")]
    fn set_old_url(this: &HashChangeEventInit, val: &str);
}
impl HashChangeEventInit {
    #[doc = "Construct a new `HashChangeEventInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HashChangeEventInit`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HashChangeEventInit`*"]
    pub fn bubbles(&mut self, val: bool) -> &mut Self {
        self.set_bubbles(val);
        self
    }
    #[doc = "Change the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HashChangeEventInit`*"]
    pub fn cancelable(&mut self, val: bool) -> &mut Self {
        self.set_cancelable(val);
        self
    }
    #[doc = "Change the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HashChangeEventInit`*"]
    pub fn composed(&mut self, val: bool) -> &mut Self {
        self.set_composed(val);
        self
    }
    #[doc = "Change the `newURL` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HashChangeEventInit`*"]
    pub fn new_url(&mut self, val: &str) -> &mut Self {
        self.set_new_url(val);
        self
    }
    #[doc = "Change the `oldURL` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HashChangeEventInit`*"]
    pub fn old_url(&mut self, val: &str) -> &mut Self {
        self.set_old_url(val);
        self
    }
}
impl Default for HashChangeEventInit {
    fn default() -> Self {
        Self::new()
    }
}
