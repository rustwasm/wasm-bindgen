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
    #[wasm_bindgen(method, getter = "p1")]
    fn p1_shim(this: &DomQuadInit) -> &DomPointInit;
    #[cfg(feature = "DomPointInit")]
    #[wasm_bindgen(method, setter = "p1")]
    fn set_p1_shim(this: &DomQuadInit, val: &DomPointInit);
    #[cfg(feature = "DomPointInit")]
    #[wasm_bindgen(method, getter = "p2")]
    fn p2_shim(this: &DomQuadInit) -> &DomPointInit;
    #[cfg(feature = "DomPointInit")]
    #[wasm_bindgen(method, setter = "p2")]
    fn set_p2_shim(this: &DomQuadInit, val: &DomPointInit);
    #[cfg(feature = "DomPointInit")]
    #[wasm_bindgen(method, getter = "p3")]
    fn p3_shim(this: &DomQuadInit) -> &DomPointInit;
    #[cfg(feature = "DomPointInit")]
    #[wasm_bindgen(method, setter = "p3")]
    fn set_p3_shim(this: &DomQuadInit, val: &DomPointInit);
    #[cfg(feature = "DomPointInit")]
    #[wasm_bindgen(method, getter = "p4")]
    fn p4_shim(this: &DomQuadInit) -> &DomPointInit;
    #[cfg(feature = "DomPointInit")]
    #[wasm_bindgen(method, setter = "p4")]
    fn set_p4_shim(this: &DomQuadInit, val: &DomPointInit);
}
#[doc = "The trait to access properties on the `DomQuadInit` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `DomQuadInit`*"]
pub trait DomQuadInitGetters {
    #[cfg(feature = "DomPointInit")]
    #[doc = "Get the `p1` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomPointInit`, `DomQuadInit`*"]
    fn p1(&self) -> &DomPointInit;
    #[cfg(feature = "DomPointInit")]
    #[doc = "Get the `p2` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomPointInit`, `DomQuadInit`*"]
    fn p2(&self) -> &DomPointInit;
    #[cfg(feature = "DomPointInit")]
    #[doc = "Get the `p3` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomPointInit`, `DomQuadInit`*"]
    fn p3(&self) -> &DomPointInit;
    #[cfg(feature = "DomPointInit")]
    #[doc = "Get the `p4` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomPointInit`, `DomQuadInit`*"]
    fn p4(&self) -> &DomPointInit;
}
impl DomQuadInitGetters for DomQuadInit {
    #[cfg(feature = "DomPointInit")]
    fn p1(&self) -> &DomPointInit {
        self.p1_shim()
    }
    #[cfg(feature = "DomPointInit")]
    fn p2(&self) -> &DomPointInit {
        self.p2_shim()
    }
    #[cfg(feature = "DomPointInit")]
    fn p3(&self) -> &DomPointInit {
        self.p3_shim()
    }
    #[cfg(feature = "DomPointInit")]
    fn p4(&self) -> &DomPointInit {
        self.p4_shim()
    }
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
        self.set_p1_shim(val);
        self
    }
    #[cfg(feature = "DomPointInit")]
    #[doc = "Change the `p2` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomPointInit`, `DomQuadInit`*"]
    pub fn p2(&mut self, val: &DomPointInit) -> &mut Self {
        self.set_p2_shim(val);
        self
    }
    #[cfg(feature = "DomPointInit")]
    #[doc = "Change the `p3` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomPointInit`, `DomQuadInit`*"]
    pub fn p3(&mut self, val: &DomPointInit) -> &mut Self {
        self.set_p3_shim(val);
        self
    }
    #[cfg(feature = "DomPointInit")]
    #[doc = "Change the `p4` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomPointInit`, `DomQuadInit`*"]
    pub fn p4(&mut self, val: &DomPointInit) -> &mut Self {
        self.set_p4_shim(val);
        self
    }
}
impl Default for DomQuadInit {
    fn default() -> Self {
        Self::new()
    }
}
