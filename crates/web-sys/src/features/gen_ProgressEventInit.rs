#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = ProgressEventInit)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ProgressEventInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProgressEventInit`*"]
    pub type ProgressEventInit;
    #[wasm_bindgen(method, getter = "bubbles")]
    fn bubbles_shim(this: &ProgressEventInit) -> bool;
    #[wasm_bindgen(method, setter = "bubbles")]
    fn set_bubbles_shim(this: &ProgressEventInit, val: bool);
    #[wasm_bindgen(method, getter = "cancelable")]
    fn cancelable_shim(this: &ProgressEventInit) -> bool;
    #[wasm_bindgen(method, setter = "cancelable")]
    fn set_cancelable_shim(this: &ProgressEventInit, val: bool);
    #[wasm_bindgen(method, getter = "composed")]
    fn composed_shim(this: &ProgressEventInit) -> bool;
    #[wasm_bindgen(method, setter = "composed")]
    fn set_composed_shim(this: &ProgressEventInit, val: bool);
    #[wasm_bindgen(method, getter = "lengthComputable")]
    fn length_computable_shim(this: &ProgressEventInit) -> bool;
    #[wasm_bindgen(method, setter = "lengthComputable")]
    fn set_length_computable_shim(this: &ProgressEventInit, val: bool);
    #[wasm_bindgen(method, getter = "loaded")]
    fn loaded_shim(this: &ProgressEventInit) -> f64;
    #[wasm_bindgen(method, setter = "loaded")]
    fn set_loaded_shim(this: &ProgressEventInit, val: f64);
    #[wasm_bindgen(method, getter = "total")]
    fn total_shim(this: &ProgressEventInit) -> f64;
    #[wasm_bindgen(method, setter = "total")]
    fn set_total_shim(this: &ProgressEventInit, val: f64);
}
#[doc = "The trait to access properties on the `ProgressEventInit` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `ProgressEventInit`*"]
pub trait ProgressEventInitGetters {
    #[doc = "Get the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProgressEventInit`*"]
    fn bubbles(&self) -> bool;
    #[doc = "Get the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProgressEventInit`*"]
    fn cancelable(&self) -> bool;
    #[doc = "Get the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProgressEventInit`*"]
    fn composed(&self) -> bool;
    #[doc = "Get the `lengthComputable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProgressEventInit`*"]
    fn length_computable(&self) -> bool;
    #[doc = "Get the `loaded` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProgressEventInit`*"]
    fn loaded(&self) -> f64;
    #[doc = "Get the `total` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProgressEventInit`*"]
    fn total(&self) -> f64;
}
impl ProgressEventInitGetters for ProgressEventInit {
    fn bubbles(&self) -> bool {
        self.bubbles_shim()
    }
    fn cancelable(&self) -> bool {
        self.cancelable_shim()
    }
    fn composed(&self) -> bool {
        self.composed_shim()
    }
    fn length_computable(&self) -> bool {
        self.length_computable_shim()
    }
    fn loaded(&self) -> f64 {
        self.loaded_shim()
    }
    fn total(&self) -> f64 {
        self.total_shim()
    }
}
impl ProgressEventInit {
    #[doc = "Construct a new `ProgressEventInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProgressEventInit`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProgressEventInit`*"]
    pub fn bubbles(&mut self, val: bool) -> &mut Self {
        self.set_bubbles_shim(val);
        self
    }
    #[doc = "Change the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProgressEventInit`*"]
    pub fn cancelable(&mut self, val: bool) -> &mut Self {
        self.set_cancelable_shim(val);
        self
    }
    #[doc = "Change the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProgressEventInit`*"]
    pub fn composed(&mut self, val: bool) -> &mut Self {
        self.set_composed_shim(val);
        self
    }
    #[doc = "Change the `lengthComputable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProgressEventInit`*"]
    pub fn length_computable(&mut self, val: bool) -> &mut Self {
        self.set_length_computable_shim(val);
        self
    }
    #[doc = "Change the `loaded` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProgressEventInit`*"]
    pub fn loaded(&mut self, val: f64) -> &mut Self {
        self.set_loaded_shim(val);
        self
    }
    #[doc = "Change the `total` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProgressEventInit`*"]
    pub fn total(&mut self, val: f64) -> &mut Self {
        self.set_total_shim(val);
        self
    }
}
impl Default for ProgressEventInit {
    fn default() -> Self {
        Self::new()
    }
}
