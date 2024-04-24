#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = NativeOSFileReadOptions)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `NativeOsFileReadOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NativeOsFileReadOptions`*"]
    pub type NativeOsFileReadOptions;
    #[wasm_bindgen(method, getter = "bytes")]
    fn bytes_shim(this: &NativeOsFileReadOptions) -> Option<f64>;
    #[wasm_bindgen(method, setter = "bytes")]
    fn set_bytes_shim(this: &NativeOsFileReadOptions, val: Option<f64>);
    #[wasm_bindgen(method, getter = "encoding")]
    fn encoding_shim(this: &NativeOsFileReadOptions) -> Option<&str>;
    #[wasm_bindgen(method, setter = "encoding")]
    fn set_encoding_shim(this: &NativeOsFileReadOptions, val: Option<&str>);
}
#[doc = "The trait to access properties on the `NativeOsFileReadOptions` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `NativeOsFileReadOptions`*"]
pub trait NativeOsFileReadOptionsGetters {
    #[doc = "Get the `bytes` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NativeOsFileReadOptions`*"]
    fn bytes(&self) -> Option<f64>;
    #[doc = "Get the `encoding` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NativeOsFileReadOptions`*"]
    fn encoding(&self) -> Option<&str>;
}
impl NativeOsFileReadOptionsGetters for NativeOsFileReadOptions {
    fn bytes(&self) -> Option<f64> {
        self.bytes_shim()
    }
    fn encoding(&self) -> Option<&str> {
        self.encoding_shim()
    }
}
impl NativeOsFileReadOptions {
    #[doc = "Construct a new `NativeOsFileReadOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NativeOsFileReadOptions`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `bytes` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NativeOsFileReadOptions`*"]
    pub fn bytes(&mut self, val: Option<f64>) -> &mut Self {
        self.set_bytes_shim(val);
        self
    }
    #[doc = "Change the `encoding` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NativeOsFileReadOptions`*"]
    pub fn encoding(&mut self, val: Option<&str>) -> &mut Self {
        self.set_encoding_shim(val);
        self
    }
}
impl Default for NativeOsFileReadOptions {
    fn default() -> Self {
        Self::new()
    }
}
