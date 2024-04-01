#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = DOMQuadInit)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `DomQuadInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomQuadInit`*"]
    pub type DomQuadInit;
    #[cfg(feature = "DomPointInit")]
    #[wasm_bindgen(method, setter = "p1")]
    fn p1_shim(this: &DomQuadInit, val: &DomPointInit);
    #[cfg(feature = "DomPointInit")]
    #[wasm_bindgen(method, setter = "p2")]
    fn p2_shim(this: &DomQuadInit, val: &DomPointInit);
    #[cfg(feature = "DomPointInit")]
    #[wasm_bindgen(method, setter = "p3")]
    fn p3_shim(this: &DomQuadInit, val: &DomPointInit);
    #[cfg(feature = "DomPointInit")]
    #[wasm_bindgen(method, setter = "p4")]
    fn p4_shim(this: &DomQuadInit, val: &DomPointInit);
}
impl DomQuadInit {
    #[doc = "Construct a new `DomQuadInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomQuadInit`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[cfg(feature = "DomPointInit")]
    #[doc = "Change the `p1` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomPointInit`, `DomQuadInit`*"]
    pub fn p1(&mut self, val: &DomPointInit) -> &mut Self {
        self.p1_shim(val);
        self
    }
    #[cfg(feature = "DomPointInit")]
    #[doc = "Change the `p2` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomPointInit`, `DomQuadInit`*"]
    pub fn p2(&mut self, val: &DomPointInit) -> &mut Self {
        self.p2_shim(val);
        self
    }
    #[cfg(feature = "DomPointInit")]
    #[doc = "Change the `p3` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomPointInit`, `DomQuadInit`*"]
    pub fn p3(&mut self, val: &DomPointInit) -> &mut Self {
        self.p3_shim(val);
        self
    }
    #[cfg(feature = "DomPointInit")]
    #[doc = "Change the `p4` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomPointInit`, `DomQuadInit`*"]
    pub fn p4(&mut self, val: &DomPointInit) -> &mut Self {
        self.p4_shim(val);
        self
    }
}
impl Default for DomQuadInit {
    fn default() -> Self {
        Self::new()
    }
}
