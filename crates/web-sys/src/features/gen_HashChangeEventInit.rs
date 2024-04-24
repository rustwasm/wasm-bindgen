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
    #[wasm_bindgen(method, getter = "bubbles")]
    fn bubbles_shim(this: &HashChangeEventInit) -> bool;
    #[wasm_bindgen(method, setter = "bubbles")]
    fn set_bubbles_shim(this: &HashChangeEventInit, val: bool);
    #[wasm_bindgen(method, getter = "cancelable")]
    fn cancelable_shim(this: &HashChangeEventInit) -> bool;
    #[wasm_bindgen(method, setter = "cancelable")]
    fn set_cancelable_shim(this: &HashChangeEventInit, val: bool);
    #[wasm_bindgen(method, getter = "composed")]
    fn composed_shim(this: &HashChangeEventInit) -> bool;
    #[wasm_bindgen(method, setter = "composed")]
    fn set_composed_shim(this: &HashChangeEventInit, val: bool);
    #[wasm_bindgen(method, getter = "newURL")]
    fn new_url_shim(this: &HashChangeEventInit) -> String;
    #[wasm_bindgen(method, setter = "newURL")]
    fn set_new_url_shim(this: &HashChangeEventInit, val: &str);
    #[wasm_bindgen(method, getter = "oldURL")]
    fn old_url_shim(this: &HashChangeEventInit) -> String;
    #[wasm_bindgen(method, setter = "oldURL")]
    fn set_old_url_shim(this: &HashChangeEventInit, val: &str);
}
#[doc = "The trait to access properties on the `HashChangeEventInit` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `HashChangeEventInit`*"]
pub trait HashChangeEventInitGetters {
    #[doc = "Get the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HashChangeEventInit`*"]
    fn bubbles(&self) -> bool;
    #[doc = "Get the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HashChangeEventInit`*"]
    fn cancelable(&self) -> bool;
    #[doc = "Get the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HashChangeEventInit`*"]
    fn composed(&self) -> bool;
    #[doc = "Get the `newURL` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HashChangeEventInit`*"]
    fn new_url(&self) -> String;
    #[doc = "Get the `oldURL` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HashChangeEventInit`*"]
    fn old_url(&self) -> String;
}
impl HashChangeEventInitGetters for HashChangeEventInit {
    fn bubbles(&self) -> bool {
        self.bubbles_shim()
    }
    fn cancelable(&self) -> bool {
        self.cancelable_shim()
    }
    fn composed(&self) -> bool {
        self.composed_shim()
    }
    fn new_url(&self) -> String {
        self.new_url_shim()
    }
    fn old_url(&self) -> String {
        self.old_url_shim()
    }
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
        self.set_bubbles_shim(val);
        self
    }
    #[doc = "Change the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HashChangeEventInit`*"]
    pub fn cancelable(&mut self, val: bool) -> &mut Self {
        self.set_cancelable_shim(val);
        self
    }
    #[doc = "Change the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HashChangeEventInit`*"]
    pub fn composed(&mut self, val: bool) -> &mut Self {
        self.set_composed_shim(val);
        self
    }
    #[doc = "Change the `newURL` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HashChangeEventInit`*"]
    pub fn new_url(&mut self, val: &str) -> &mut Self {
        self.set_new_url_shim(val);
        self
    }
    #[doc = "Change the `oldURL` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HashChangeEventInit`*"]
    pub fn old_url(&mut self, val: &str) -> &mut Self {
        self.set_old_url_shim(val);
        self
    }
}
impl Default for HashChangeEventInit {
    fn default() -> Self {
        Self::new()
    }
}
