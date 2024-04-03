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
    #[wasm_bindgen(method, setter = "p1")]
    fn p1_shim(this: &DomQuadJson, val: &DomPoint);
    #[cfg(feature = "DomPoint")]
    #[wasm_bindgen(method, setter = "p2")]
    fn p2_shim(this: &DomQuadJson, val: &DomPoint);
    #[cfg(feature = "DomPoint")]
    #[wasm_bindgen(method, setter = "p3")]
    fn p3_shim(this: &DomQuadJson, val: &DomPoint);
    #[cfg(feature = "DomPoint")]
    #[wasm_bindgen(method, setter = "p4")]
    fn p4_shim(this: &DomQuadJson, val: &DomPoint);
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
        self.p1_shim(val);
        self
    }
    #[cfg(feature = "DomPoint")]
    #[doc = "Change the `p2` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomPoint`, `DomQuadJson`*"]
    pub fn p2(&mut self, val: &DomPoint) -> &mut Self {
        self.p2_shim(val);
        self
    }
    #[cfg(feature = "DomPoint")]
    #[doc = "Change the `p3` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomPoint`, `DomQuadJson`*"]
    pub fn p3(&mut self, val: &DomPoint) -> &mut Self {
        self.p3_shim(val);
        self
    }
    #[cfg(feature = "DomPoint")]
    #[doc = "Change the `p4` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomPoint`, `DomQuadJson`*"]
    pub fn p4(&mut self, val: &DomPoint) -> &mut Self {
        self.p4_shim(val);
        self
    }
}
impl Default for DomQuadJson {
    fn default() -> Self {
        Self::new()
    }
}
