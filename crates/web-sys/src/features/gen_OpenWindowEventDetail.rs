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
    #[wasm_bindgen(method, getter = "features")]
    fn features_shim(this: &OpenWindowEventDetail) -> &str;
    #[wasm_bindgen(method, setter = "features")]
    fn set_features_shim(this: &OpenWindowEventDetail, val: &str);
    #[cfg(feature = "Node")]
    #[wasm_bindgen(method, getter = "frameElement")]
    fn frame_element_shim(this: &OpenWindowEventDetail) -> Option<&Node>;
    #[cfg(feature = "Node")]
    #[wasm_bindgen(method, setter = "frameElement")]
    fn set_frame_element_shim(this: &OpenWindowEventDetail, val: Option<&Node>);
    #[wasm_bindgen(method, getter = "name")]
    fn name_shim(this: &OpenWindowEventDetail) -> &str;
    #[wasm_bindgen(method, setter = "name")]
    fn set_name_shim(this: &OpenWindowEventDetail, val: &str);
    #[wasm_bindgen(method, getter = "url")]
    fn url_shim(this: &OpenWindowEventDetail) -> &str;
    #[wasm_bindgen(method, setter = "url")]
    fn set_url_shim(this: &OpenWindowEventDetail, val: &str);
}
#[doc = "The trait to access properties on the `OpenWindowEventDetail` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `OpenWindowEventDetail`*"]
pub trait OpenWindowEventDetailGetters {
    #[doc = "Get the `features` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OpenWindowEventDetail`*"]
    fn features(&self) -> &str;
    #[cfg(feature = "Node")]
    #[doc = "Get the `frameElement` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Node`, `OpenWindowEventDetail`*"]
    fn frame_element(&self) -> Option<&Node>;
    #[doc = "Get the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OpenWindowEventDetail`*"]
    fn name(&self) -> &str;
    #[doc = "Get the `url` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OpenWindowEventDetail`*"]
    fn url(&self) -> &str;
}
impl OpenWindowEventDetailGetters for OpenWindowEventDetail {
    fn features(&self) -> &str {
        self.features_shim()
    }
    #[cfg(feature = "Node")]
    fn frame_element(&self) -> Option<&Node> {
        self.frame_element_shim()
    }
    fn name(&self) -> &str {
        self.name_shim()
    }
    fn url(&self) -> &str {
        self.url_shim()
    }
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
        self.set_features_shim(val);
        self
    }
    #[cfg(feature = "Node")]
    #[doc = "Change the `frameElement` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Node`, `OpenWindowEventDetail`*"]
    pub fn frame_element(&mut self, val: Option<&Node>) -> &mut Self {
        self.set_frame_element_shim(val);
        self
    }
    #[doc = "Change the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OpenWindowEventDetail`*"]
    pub fn name(&mut self, val: &str) -> &mut Self {
        self.set_name_shim(val);
        self
    }
    #[doc = "Change the `url` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OpenWindowEventDetail`*"]
    pub fn url(&mut self, val: &str) -> &mut Self {
        self.set_url_shim(val);
        self
    }
}
impl Default for OpenWindowEventDetail {
    fn default() -> Self {
        Self::new()
    }
}
