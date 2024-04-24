#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = ShadowRootInit)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ShadowRootInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ShadowRootInit`*"]
    pub type ShadowRootInit;
    #[cfg(feature = "ShadowRootMode")]
    #[wasm_bindgen(method, getter = "mode")]
    fn mode_shim(this: &ShadowRootInit) -> ShadowRootMode;
    #[cfg(feature = "ShadowRootMode")]
    #[wasm_bindgen(method, setter = "mode")]
    fn set_mode_shim(this: &ShadowRootInit, val: ShadowRootMode);
}
#[doc = "The trait to access properties on the `ShadowRootInit` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `ShadowRootInit`*"]
pub trait ShadowRootInitGetters {
    #[cfg(feature = "ShadowRootMode")]
    #[doc = "Get the `mode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ShadowRootInit`, `ShadowRootMode`*"]
    fn mode(&self) -> ShadowRootMode;
}
impl ShadowRootInitGetters for ShadowRootInit {
    #[cfg(feature = "ShadowRootMode")]
    fn mode(&self) -> ShadowRootMode {
        self.mode_shim()
    }
}
impl ShadowRootInit {
    #[cfg(feature = "ShadowRootMode")]
    #[doc = "Construct a new `ShadowRootInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ShadowRootInit`, `ShadowRootMode`*"]
    pub fn new(mode: ShadowRootMode) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        Self::mode(&mut ret, mode);
        ret
    }
    #[cfg(feature = "ShadowRootMode")]
    #[doc = "Change the `mode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ShadowRootInit`, `ShadowRootMode`*"]
    pub fn mode(&mut self, val: ShadowRootMode) -> &mut Self {
        self.set_mode_shim(val);
        self
    }
}
