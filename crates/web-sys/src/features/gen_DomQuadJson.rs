#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = DOMQuadJSON)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `DomQuadJson` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomQuadJson`*"]
    pub type DomQuadJson;
    #[cfg(feature = "DomPoint")]
    #[wasm_bindgen(method, getter = "p1")]
    fn p1_shim(this: &DomQuadJson) -> &DomPoint;
    #[cfg(feature = "DomPoint")]
    #[wasm_bindgen(method, setter = "p1")]
    fn set_p1_shim(this: &DomQuadJson, val: &DomPoint);
    #[cfg(feature = "DomPoint")]
    #[wasm_bindgen(method, getter = "p2")]
    fn p2_shim(this: &DomQuadJson) -> &DomPoint;
    #[cfg(feature = "DomPoint")]
    #[wasm_bindgen(method, setter = "p2")]
    fn set_p2_shim(this: &DomQuadJson, val: &DomPoint);
    #[cfg(feature = "DomPoint")]
    #[wasm_bindgen(method, getter = "p3")]
    fn p3_shim(this: &DomQuadJson) -> &DomPoint;
    #[cfg(feature = "DomPoint")]
    #[wasm_bindgen(method, setter = "p3")]
    fn set_p3_shim(this: &DomQuadJson, val: &DomPoint);
    #[cfg(feature = "DomPoint")]
    #[wasm_bindgen(method, getter = "p4")]
    fn p4_shim(this: &DomQuadJson) -> &DomPoint;
    #[cfg(feature = "DomPoint")]
    #[wasm_bindgen(method, setter = "p4")]
    fn set_p4_shim(this: &DomQuadJson, val: &DomPoint);
}
#[doc = "The trait to access properties on the `DomQuadJson` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `DomQuadJson`*"]
pub trait DomQuadJsonGetters {
    #[cfg(feature = "DomPoint")]
    #[doc = "Get the `p1` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomPoint`, `DomQuadJson`*"]
    fn p1(&self) -> &DomPoint;
    #[cfg(feature = "DomPoint")]
    #[doc = "Get the `p2` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomPoint`, `DomQuadJson`*"]
    fn p2(&self) -> &DomPoint;
    #[cfg(feature = "DomPoint")]
    #[doc = "Get the `p3` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomPoint`, `DomQuadJson`*"]
    fn p3(&self) -> &DomPoint;
    #[cfg(feature = "DomPoint")]
    #[doc = "Get the `p4` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomPoint`, `DomQuadJson`*"]
    fn p4(&self) -> &DomPoint;
}
impl DomQuadJsonGetters for DomQuadJson {
    #[cfg(feature = "DomPoint")]
    fn p1(&self) -> &DomPoint {
        self.p1_shim()
    }
    #[cfg(feature = "DomPoint")]
    fn p2(&self) -> &DomPoint {
        self.p2_shim()
    }
    #[cfg(feature = "DomPoint")]
    fn p3(&self) -> &DomPoint {
        self.p3_shim()
    }
    #[cfg(feature = "DomPoint")]
    fn p4(&self) -> &DomPoint {
        self.p4_shim()
    }
}
impl DomQuadJson {
    #[doc = "Construct a new `DomQuadJson`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomQuadJson`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[cfg(feature = "DomPoint")]
    #[doc = "Change the `p1` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomPoint`, `DomQuadJson`*"]
    pub fn p1(&mut self, val: &DomPoint) -> &mut Self {
        self.set_p1_shim(val);
        self
    }
    #[cfg(feature = "DomPoint")]
    #[doc = "Change the `p2` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomPoint`, `DomQuadJson`*"]
    pub fn p2(&mut self, val: &DomPoint) -> &mut Self {
        self.set_p2_shim(val);
        self
    }
    #[cfg(feature = "DomPoint")]
    #[doc = "Change the `p3` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomPoint`, `DomQuadJson`*"]
    pub fn p3(&mut self, val: &DomPoint) -> &mut Self {
        self.set_p3_shim(val);
        self
    }
    #[cfg(feature = "DomPoint")]
    #[doc = "Change the `p4` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomPoint`, `DomQuadJson`*"]
    pub fn p4(&mut self, val: &DomPoint) -> &mut Self {
        self.set_p4_shim(val);
        self
    }
}
impl Default for DomQuadJson {
    fn default() -> Self {
        Self::new()
    }
}
