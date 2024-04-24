#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = HitRegionOptions)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `HitRegionOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HitRegionOptions`*"]
    pub type HitRegionOptions;
    #[cfg(feature = "Element")]
    #[wasm_bindgen(method, getter = "control")]
    fn control_shim(this: &HitRegionOptions) -> Option<&Element>;
    #[cfg(feature = "Element")]
    #[wasm_bindgen(method, setter = "control")]
    fn set_control_shim(this: &HitRegionOptions, val: Option<&Element>);
    #[wasm_bindgen(method, getter = "id")]
    fn id_shim(this: &HitRegionOptions) -> &str;
    #[wasm_bindgen(method, setter = "id")]
    fn set_id_shim(this: &HitRegionOptions, val: &str);
    #[cfg(feature = "Path2d")]
    #[wasm_bindgen(method, getter = "path")]
    fn path_shim(this: &HitRegionOptions) -> Option<&Path2d>;
    #[cfg(feature = "Path2d")]
    #[wasm_bindgen(method, setter = "path")]
    fn set_path_shim(this: &HitRegionOptions, val: Option<&Path2d>);
}
#[doc = "The trait to access properties on the `HitRegionOptions` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `HitRegionOptions`*"]
pub trait HitRegionOptionsGetters {
    #[cfg(feature = "Element")]
    #[doc = "Get the `control` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`, `HitRegionOptions`*"]
    fn control(&self) -> Option<&Element>;
    #[doc = "Get the `id` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HitRegionOptions`*"]
    fn id(&self) -> &str;
    #[cfg(feature = "Path2d")]
    #[doc = "Get the `path` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HitRegionOptions`, `Path2d`*"]
    fn path(&self) -> Option<&Path2d>;
}
impl HitRegionOptionsGetters for HitRegionOptions {
    #[cfg(feature = "Element")]
    fn control(&self) -> Option<&Element> {
        self.control_shim()
    }
    fn id(&self) -> &str {
        self.id_shim()
    }
    #[cfg(feature = "Path2d")]
    fn path(&self) -> Option<&Path2d> {
        self.path_shim()
    }
}
impl HitRegionOptions {
    #[doc = "Construct a new `HitRegionOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HitRegionOptions`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[cfg(feature = "Element")]
    #[doc = "Change the `control` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`, `HitRegionOptions`*"]
    pub fn control(&mut self, val: Option<&Element>) -> &mut Self {
        self.set_control_shim(val);
        self
    }
    #[doc = "Change the `id` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HitRegionOptions`*"]
    pub fn id(&mut self, val: &str) -> &mut Self {
        self.set_id_shim(val);
        self
    }
    #[cfg(feature = "Path2d")]
    #[doc = "Change the `path` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HitRegionOptions`, `Path2d`*"]
    pub fn path(&mut self, val: Option<&Path2d>) -> &mut Self {
        self.set_path_shim(val);
        self
    }
}
impl Default for HitRegionOptions {
    fn default() -> Self {
        Self::new()
    }
}
