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
    #[wasm_bindgen(method, getter = "lastModified")]
    fn last_modified_shim(this: &ChromeFilePropertyBag) -> f64;
    #[wasm_bindgen(method, setter = "lastModified")]
    fn set_last_modified_shim(this: &ChromeFilePropertyBag, val: f64);
    #[wasm_bindgen(method, getter = "type")]
    fn type__shim(this: &ChromeFilePropertyBag) -> &str;
    #[wasm_bindgen(method, setter = "type")]
    fn set_type__shim(this: &ChromeFilePropertyBag, val: &str);
    #[wasm_bindgen(method, getter = "existenceCheck")]
    fn existence_check_shim(this: &ChromeFilePropertyBag) -> bool;
    #[wasm_bindgen(method, setter = "existenceCheck")]
    fn set_existence_check_shim(this: &ChromeFilePropertyBag, val: bool);
    #[wasm_bindgen(method, getter = "name")]
    fn name_shim(this: &ChromeFilePropertyBag) -> &str;
    #[wasm_bindgen(method, setter = "name")]
    fn set_name_shim(this: &ChromeFilePropertyBag, val: &str);
}
#[doc = "The trait to access properties on the `ChromeFilePropertyBag` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `ChromeFilePropertyBag`*"]
pub trait ChromeFilePropertyBagGetters {
    #[doc = "Get the `lastModified` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ChromeFilePropertyBag`*"]
    fn last_modified(&self) -> f64;
    #[doc = "Get the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ChromeFilePropertyBag`*"]
    fn type_(&self) -> &str;
    #[doc = "Get the `existenceCheck` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ChromeFilePropertyBag`*"]
    fn existence_check(&self) -> bool;
    #[doc = "Get the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ChromeFilePropertyBag`*"]
    fn name(&self) -> &str;
}
impl ChromeFilePropertyBagGetters for ChromeFilePropertyBag {
    fn last_modified(&self) -> f64 {
        self.last_modified_shim()
    }
    fn type_(&self) -> &str {
        self.type__shim()
    }
    fn existence_check(&self) -> bool {
        self.existence_check_shim()
    }
    fn name(&self) -> &str {
        self.name_shim()
    }
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
        self.set_last_modified_shim(val);
        self
    }
    #[doc = "Change the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ChromeFilePropertyBag`*"]
    pub fn type_(&mut self, val: &str) -> &mut Self {
        self.set_type__shim(val);
        self
    }
    #[doc = "Change the `existenceCheck` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ChromeFilePropertyBag`*"]
    pub fn existence_check(&mut self, val: bool) -> &mut Self {
        self.set_existence_check_shim(val);
        self
    }
    #[doc = "Change the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ChromeFilePropertyBag`*"]
    pub fn name(&mut self, val: &str) -> &mut Self {
        self.set_name_shim(val);
        self
    }
}
impl Default for ChromeFilePropertyBag {
    fn default() -> Self {
        Self::new()
    }
}
