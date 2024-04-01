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
    #[wasm_bindgen(method, setter = "composite")]
    fn composite_shim(this: &AnimationPropertyValueDetails, val: CompositeOperation);
    #[wasm_bindgen(method, setter = "easing")]
    fn easing_shim(this: &AnimationPropertyValueDetails, val: &str);
    #[wasm_bindgen(method, setter = "offset")]
    fn offset_shim(this: &AnimationPropertyValueDetails, val: f64);
    #[wasm_bindgen(method, setter = "value")]
    fn value_shim(this: &AnimationPropertyValueDetails, val: &str);
}
impl AnimationPropertyValueDetails {
    #[cfg(feature = "CompositeOperation")]
    #[doc = "Construct a new `AnimationPropertyValueDetails`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnimationPropertyValueDetails`, `CompositeOperation`*"]
    pub fn new(composite: CompositeOperation, offset: f64) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.composite(composite);
        ret.offset(offset);
        ret
    }
    #[cfg(feature = "CompositeOperation")]
    #[doc = "Change the `composite` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnimationPropertyValueDetails`, `CompositeOperation`*"]
    pub fn composite(&mut self, val: CompositeOperation) -> &mut Self {
        self.composite_shim(val);
        self
    }
    #[doc = "Change the `easing` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnimationPropertyValueDetails`*"]
    pub fn easing(&mut self, val: &str) -> &mut Self {
        self.easing_shim(val);
        self
    }
    #[doc = "Change the `offset` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnimationPropertyValueDetails`*"]
    pub fn offset(&mut self, val: f64) -> &mut Self {
        self.offset_shim(val);
        self
    }
    #[doc = "Change the `value` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnimationPropertyValueDetails`*"]
    pub fn value(&mut self, val: &str) -> &mut Self {
        self.value_shim(val);
        self
    }
}
