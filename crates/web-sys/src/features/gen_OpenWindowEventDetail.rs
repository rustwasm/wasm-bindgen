#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = OpenWindowEventDetail)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `OpenWindowEventDetail` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OpenWindowEventDetail`*"]
    pub type OpenWindowEventDetail;
    #[wasm_bindgen(method, setter = "features")]
    fn features_shim(this: &OpenWindowEventDetail, val: &str);
    #[cfg(feature = "Node")]
    #[wasm_bindgen(method, setter = "frameElement")]
    fn frame_element_shim(this: &OpenWindowEventDetail, val: Option<&Node>);
    #[wasm_bindgen(method, setter = "name")]
    fn name_shim(this: &OpenWindowEventDetail, val: &str);
    #[wasm_bindgen(method, setter = "url")]
    fn url_shim(this: &OpenWindowEventDetail, val: &str);
}
impl OpenWindowEventDetail {
    #[doc = "Construct a new `OpenWindowEventDetail`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OpenWindowEventDetail`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `features` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OpenWindowEventDetail`*"]
    pub fn features(&mut self, val: &str) -> &mut Self {
        self.features_shim(val);
        self
    }
    #[cfg(feature = "Node")]
    #[doc = "Change the `frameElement` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Node`, `OpenWindowEventDetail`*"]
    pub fn frame_element(&mut self, val: Option<&Node>) -> &mut Self {
        self.frame_element_shim(val);
        self
    }
    #[doc = "Change the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OpenWindowEventDetail`*"]
    pub fn name(&mut self, val: &str) -> &mut Self {
        self.name_shim(val);
        self
    }
    #[doc = "Change the `url` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OpenWindowEventDetail`*"]
    pub fn url(&mut self, val: &str) -> &mut Self {
        self.url_shim(val);
        self
    }
}
impl Default for OpenWindowEventDetail {
    fn default() -> Self {
        Self::new()
    }
}
