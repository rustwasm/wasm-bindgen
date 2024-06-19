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
    #[doc = "Get the `property` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnimationPropertyDetails`*"]
    #[wasm_bindgen(method, getter = "property")]
    pub fn get_property(this: &AnimationPropertyDetails) -> String;
    #[wasm_bindgen(method, setter = "property")]
    fn set_property(this: &AnimationPropertyDetails, val: &str);
    #[doc = "Get the `runningOnCompositor` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnimationPropertyDetails`*"]
    #[wasm_bindgen(method, getter = "runningOnCompositor")]
    pub fn get_running_on_compositor(this: &AnimationPropertyDetails) -> bool;
    #[wasm_bindgen(method, setter = "runningOnCompositor")]
    fn set_running_on_compositor(this: &AnimationPropertyDetails, val: bool);
    #[doc = "Get the `values` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnimationPropertyDetails`*"]
    #[wasm_bindgen(method, getter = "values")]
    pub fn get_values(this: &AnimationPropertyDetails) -> ::js_sys::Array;
    #[wasm_bindgen(method, setter = "values")]
    fn set_values(this: &AnimationPropertyDetails, val: &::wasm_bindgen::JsValue);
    #[doc = "Get the `warning` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnimationPropertyDetails`*"]
    #[wasm_bindgen(method, getter = "warning")]
    pub fn get_warning(this: &AnimationPropertyDetails) -> Option<String>;
    #[wasm_bindgen(method, setter = "warning")]
    fn set_warning(this: &AnimationPropertyDetails, val: &str);
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
        self.set_property(val);
        self
    }
    #[doc = "Change the `runningOnCompositor` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnimationPropertyDetails`*"]
    pub fn running_on_compositor(&mut self, val: bool) -> &mut Self {
        self.set_running_on_compositor(val);
        self
    }
    #[doc = "Change the `values` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnimationPropertyDetails`*"]
    pub fn values(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_values(val);
        self
    }
    #[doc = "Change the `warning` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnimationPropertyDetails`*"]
    pub fn warning(&mut self, val: &str) -> &mut Self {
        self.set_warning(val);
        self
    }
}
