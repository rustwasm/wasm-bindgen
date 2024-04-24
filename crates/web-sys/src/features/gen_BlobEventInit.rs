#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = BlobEventInit)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `BlobEventInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BlobEventInit`*"]
    pub type BlobEventInit;
    #[wasm_bindgen(method, getter = "bubbles")]
    fn bubbles_shim(this: &BlobEventInit) -> bool;
    #[wasm_bindgen(method, setter = "bubbles")]
    fn set_bubbles_shim(this: &BlobEventInit, val: bool);
    #[wasm_bindgen(method, getter = "cancelable")]
    fn cancelable_shim(this: &BlobEventInit) -> bool;
    #[wasm_bindgen(method, setter = "cancelable")]
    fn set_cancelable_shim(this: &BlobEventInit, val: bool);
    #[wasm_bindgen(method, getter = "composed")]
    fn composed_shim(this: &BlobEventInit) -> bool;
    #[wasm_bindgen(method, setter = "composed")]
    fn set_composed_shim(this: &BlobEventInit, val: bool);
    #[cfg(feature = "Blob")]
    #[wasm_bindgen(method, getter = "data")]
    fn data_shim(this: &BlobEventInit) -> Option<Blob>;
    #[cfg(feature = "Blob")]
    #[wasm_bindgen(method, setter = "data")]
    fn set_data_shim(this: &BlobEventInit, val: Option<&Blob>);
}
#[doc = "The trait to access properties on the `BlobEventInit` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `BlobEventInit`*"]
pub trait BlobEventInitGetters {
    #[doc = "Get the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BlobEventInit`*"]
    fn bubbles(&self) -> bool;
    #[doc = "Get the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BlobEventInit`*"]
    fn cancelable(&self) -> bool;
    #[doc = "Get the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BlobEventInit`*"]
    fn composed(&self) -> bool;
    #[cfg(feature = "Blob")]
    #[doc = "Get the `data` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Blob`, `BlobEventInit`*"]
    fn data(&self) -> Option<Blob>;
}
impl BlobEventInitGetters for BlobEventInit {
    fn bubbles(&self) -> bool {
        self.bubbles_shim()
    }
    fn cancelable(&self) -> bool {
        self.cancelable_shim()
    }
    fn composed(&self) -> bool {
        self.composed_shim()
    }
    #[cfg(feature = "Blob")]
    fn data(&self) -> Option<Blob> {
        self.data_shim()
    }
}
impl BlobEventInit {
    #[doc = "Construct a new `BlobEventInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BlobEventInit`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BlobEventInit`*"]
    pub fn bubbles(&mut self, val: bool) -> &mut Self {
        self.set_bubbles_shim(val);
        self
    }
    #[doc = "Change the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BlobEventInit`*"]
    pub fn cancelable(&mut self, val: bool) -> &mut Self {
        self.set_cancelable_shim(val);
        self
    }
    #[doc = "Change the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BlobEventInit`*"]
    pub fn composed(&mut self, val: bool) -> &mut Self {
        self.set_composed_shim(val);
        self
    }
    #[cfg(feature = "Blob")]
    #[doc = "Change the `data` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Blob`, `BlobEventInit`*"]
    pub fn data(&mut self, val: Option<&Blob>) -> &mut Self {
        self.set_data_shim(val);
        self
    }
}
impl Default for BlobEventInit {
    fn default() -> Self {
        Self::new()
    }
}
