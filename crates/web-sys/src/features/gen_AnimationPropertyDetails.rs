#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = AnimationPropertyDetails)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `AnimationPropertyDetails` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnimationPropertyDetails`*"]
    pub type AnimationPropertyDetails;
    #[wasm_bindgen(method, setter = "property")]
    fn property_shim(this: &AnimationPropertyDetails, val: &str);
    #[wasm_bindgen(method, setter = "runningOnCompositor")]
    fn running_on_compositor_shim(this: &AnimationPropertyDetails, val: bool);
    #[wasm_bindgen(method, setter = "values")]
    fn values_shim(this: &AnimationPropertyDetails, val: &::wasm_bindgen::JsValue);
    #[wasm_bindgen(method, setter = "warning")]
    fn warning_shim(this: &AnimationPropertyDetails, val: &str);
}
impl AnimationPropertyDetails {
    #[doc = "Construct a new `AnimationPropertyDetails`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnimationPropertyDetails`*"]
    pub fn new(
        property: &str,
        running_on_compositor: bool,
        values: &::wasm_bindgen::JsValue,
    ) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.property(property);
        ret.running_on_compositor(running_on_compositor);
        ret.values(values);
        ret
    }
    #[doc = "Change the `property` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnimationPropertyDetails`*"]
    pub fn property(&mut self, val: &str) -> &mut Self {
        self.property_shim(val);
        self
    }
    #[doc = "Change the `runningOnCompositor` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnimationPropertyDetails`*"]
    pub fn running_on_compositor(&mut self, val: bool) -> &mut Self {
        self.running_on_compositor_shim(val);
        self
    }
    #[doc = "Change the `values` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnimationPropertyDetails`*"]
    pub fn values(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.values_shim(val);
        self
    }
    #[doc = "Change the `warning` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnimationPropertyDetails`*"]
    pub fn warning(&mut self, val: &str) -> &mut Self {
        self.warning_shim(val);
        self
    }
}
