#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = AnimationPropertyValueDetails)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `AnimationPropertyValueDetails` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnimationPropertyValueDetails`*"]
    pub type AnimationPropertyValueDetails;
    #[cfg(feature = "CompositeOperation")]
    #[wasm_bindgen(method, getter = "composite")]
    fn composite_shim(this: &AnimationPropertyValueDetails) -> CompositeOperation;
    #[cfg(feature = "CompositeOperation")]
    #[wasm_bindgen(method, setter = "composite")]
    fn set_composite_shim(this: &AnimationPropertyValueDetails, val: CompositeOperation);
    #[wasm_bindgen(method, getter = "easing")]
    fn easing_shim(this: &AnimationPropertyValueDetails) -> String;
    #[wasm_bindgen(method, setter = "easing")]
    fn set_easing_shim(this: &AnimationPropertyValueDetails, val: &str);
    #[wasm_bindgen(method, getter = "offset")]
    fn offset_shim(this: &AnimationPropertyValueDetails) -> f64;
    #[wasm_bindgen(method, setter = "offset")]
    fn set_offset_shim(this: &AnimationPropertyValueDetails, val: f64);
    #[wasm_bindgen(method, getter = "value")]
    fn value_shim(this: &AnimationPropertyValueDetails) -> String;
    #[wasm_bindgen(method, setter = "value")]
    fn set_value_shim(this: &AnimationPropertyValueDetails, val: &str);
}
#[doc = "The trait to access properties on the `AnimationPropertyValueDetails` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `AnimationPropertyValueDetails`*"]
pub trait AnimationPropertyValueDetailsGetters {
    #[cfg(feature = "CompositeOperation")]
    #[doc = "Get the `composite` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnimationPropertyValueDetails`, `CompositeOperation`*"]
    fn composite(&self) -> CompositeOperation;
    #[doc = "Get the `easing` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnimationPropertyValueDetails`*"]
    fn easing(&self) -> String;
    #[doc = "Get the `offset` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnimationPropertyValueDetails`*"]
    fn offset(&self) -> f64;
    #[doc = "Get the `value` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnimationPropertyValueDetails`*"]
    fn value(&self) -> String;
}
impl AnimationPropertyValueDetailsGetters for AnimationPropertyValueDetails {
    #[cfg(feature = "CompositeOperation")]
    fn composite(&self) -> CompositeOperation {
        self.composite_shim()
    }
    fn easing(&self) -> String {
        self.easing_shim()
    }
    fn offset(&self) -> f64 {
        self.offset_shim()
    }
    fn value(&self) -> String {
        self.value_shim()
    }
}
impl AnimationPropertyValueDetails {
    #[cfg(feature = "CompositeOperation")]
    #[doc = "Construct a new `AnimationPropertyValueDetails`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnimationPropertyValueDetails`, `CompositeOperation`*"]
    pub fn new(composite: CompositeOperation, offset: f64) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        Self::composite(&mut ret, composite);
        Self::offset(&mut ret, offset);
        ret
    }
    #[cfg(feature = "CompositeOperation")]
    #[doc = "Change the `composite` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnimationPropertyValueDetails`, `CompositeOperation`*"]
    pub fn composite(&mut self, val: CompositeOperation) -> &mut Self {
        self.set_composite_shim(val);
        self
    }
    #[doc = "Change the `easing` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnimationPropertyValueDetails`*"]
    pub fn easing(&mut self, val: &str) -> &mut Self {
        self.set_easing_shim(val);
        self
    }
    #[doc = "Change the `offset` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnimationPropertyValueDetails`*"]
    pub fn offset(&mut self, val: f64) -> &mut Self {
        self.set_offset_shim(val);
        self
    }
    #[doc = "Change the `value` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnimationPropertyValueDetails`*"]
    pub fn value(&mut self, val: &str) -> &mut Self {
        self.set_value_shim(val);
        self
    }
}
