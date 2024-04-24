#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = QueuingStrategy)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `QueuingStrategy` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `QueuingStrategy`*"]
    pub type QueuingStrategy;
    #[wasm_bindgen(method, getter = "highWaterMark")]
    fn high_water_mark_shim(this: &QueuingStrategy) -> f64;
    #[wasm_bindgen(method, setter = "highWaterMark")]
    fn set_high_water_mark_shim(this: &QueuingStrategy, val: f64);
    #[wasm_bindgen(method, getter = "size")]
    fn size_shim(this: &QueuingStrategy) -> ::js_sys::Function;
    #[wasm_bindgen(method, setter = "size")]
    fn set_size_shim(this: &QueuingStrategy, val: &::js_sys::Function);
}
#[doc = "The trait to access properties on the `QueuingStrategy` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `QueuingStrategy`*"]
pub trait QueuingStrategyGetters {
    #[doc = "Get the `highWaterMark` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `QueuingStrategy`*"]
    fn high_water_mark(&self) -> f64;
    #[doc = "Get the `size` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `QueuingStrategy`*"]
    fn size(&self) -> ::js_sys::Function;
}
impl QueuingStrategyGetters for QueuingStrategy {
    fn high_water_mark(&self) -> f64 {
        self.high_water_mark_shim()
    }
    fn size(&self) -> ::js_sys::Function {
        self.size_shim()
    }
}
impl QueuingStrategy {
    #[doc = "Construct a new `QueuingStrategy`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `QueuingStrategy`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `highWaterMark` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `QueuingStrategy`*"]
    pub fn high_water_mark(&mut self, val: f64) -> &mut Self {
        self.set_high_water_mark_shim(val);
        self
    }
    #[doc = "Change the `size` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `QueuingStrategy`*"]
    pub fn size(&mut self, val: &::js_sys::Function) -> &mut Self {
        self.set_size_shim(val);
        self
    }
}
impl Default for QueuingStrategy {
    fn default() -> Self {
        Self::new()
    }
}
