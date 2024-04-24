#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = BlobPropertyBag)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `BlobPropertyBag` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BlobPropertyBag`*"]
    pub type BlobPropertyBag;
    #[cfg(feature = "EndingTypes")]
    #[wasm_bindgen(method, getter = "endings")]
    fn endings_shim(this: &BlobPropertyBag) -> EndingTypes;
    #[cfg(feature = "EndingTypes")]
    #[wasm_bindgen(method, setter = "endings")]
    fn set_endings_shim(this: &BlobPropertyBag, val: EndingTypes);
    #[wasm_bindgen(method, getter = "type")]
    fn type__shim(this: &BlobPropertyBag) -> &str;
    #[wasm_bindgen(method, setter = "type")]
    fn set_type__shim(this: &BlobPropertyBag, val: &str);
}
#[doc = "The trait to access properties on the `BlobPropertyBag` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `BlobPropertyBag`*"]
pub trait BlobPropertyBagGetters {
    #[cfg(feature = "EndingTypes")]
    #[doc = "Get the `endings` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BlobPropertyBag`, `EndingTypes`*"]
    fn endings(&self) -> EndingTypes;
    #[doc = "Get the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BlobPropertyBag`*"]
    fn type_(&self) -> &str;
}
impl BlobPropertyBagGetters for BlobPropertyBag {
    #[cfg(feature = "EndingTypes")]
    fn endings(&self) -> EndingTypes {
        self.endings_shim()
    }
    fn type_(&self) -> &str {
        self.type__shim()
    }
}
impl BlobPropertyBag {
    #[doc = "Construct a new `BlobPropertyBag`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BlobPropertyBag`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[cfg(feature = "EndingTypes")]
    #[doc = "Change the `endings` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BlobPropertyBag`, `EndingTypes`*"]
    pub fn endings(&mut self, val: EndingTypes) -> &mut Self {
        self.set_endings_shim(val);
        self
    }
    #[doc = "Change the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BlobPropertyBag`*"]
    pub fn type_(&mut self, val: &str) -> &mut Self {
        self.set_type__shim(val);
        self
    }
}
impl Default for BlobPropertyBag {
    fn default() -> Self {
        Self::new()
    }
}
