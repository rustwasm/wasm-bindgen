#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = ChromeFilePropertyBag)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ChromeFilePropertyBag` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ChromeFilePropertyBag`*"]
    pub type ChromeFilePropertyBag;
    #[wasm_bindgen(method, setter = "lastModified")]
    fn last_modified_shim(this: &ChromeFilePropertyBag, val: f64);
    #[wasm_bindgen(method, setter = "type")]
    fn type__shim(this: &ChromeFilePropertyBag, val: &str);
    #[wasm_bindgen(method, setter = "existenceCheck")]
    fn existence_check_shim(this: &ChromeFilePropertyBag, val: bool);
    #[wasm_bindgen(method, setter = "name")]
    fn name_shim(this: &ChromeFilePropertyBag, val: &str);
}
impl ChromeFilePropertyBag {
    #[doc = "Construct a new `ChromeFilePropertyBag`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ChromeFilePropertyBag`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `lastModified` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ChromeFilePropertyBag`*"]
    pub fn last_modified(&mut self, val: f64) -> &mut Self {
        self.last_modified_shim(val);
        self
    }
    #[doc = "Change the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ChromeFilePropertyBag`*"]
    pub fn type_(&mut self, val: &str) -> &mut Self {
        self.type__shim(val);
        self
    }
    #[doc = "Change the `existenceCheck` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ChromeFilePropertyBag`*"]
    pub fn existence_check(&mut self, val: bool) -> &mut Self {
        self.existence_check_shim(val);
        self
    }
    #[doc = "Change the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ChromeFilePropertyBag`*"]
    pub fn name(&mut self, val: &str) -> &mut Self {
        self.name_shim(val);
        self
    }
}
impl Default for ChromeFilePropertyBag {
    fn default() -> Self {
        Self::new()
    }
}
