#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = FakePluginMimeEntry)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `FakePluginMimeEntry` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FakePluginMimeEntry`*"]
    pub type FakePluginMimeEntry;
    #[wasm_bindgen(method, setter = "description")]
    fn description_shim(this: &FakePluginMimeEntry, val: &str);
    #[wasm_bindgen(method, setter = "extension")]
    fn extension_shim(this: &FakePluginMimeEntry, val: &str);
    #[wasm_bindgen(method, setter = "type")]
    fn type__shim(this: &FakePluginMimeEntry, val: &str);
}
impl FakePluginMimeEntry {
    #[doc = "Construct a new `FakePluginMimeEntry`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FakePluginMimeEntry`*"]
    pub fn new(type_: &str) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.type_(type_);
        ret
    }
    #[doc = "Change the `description` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FakePluginMimeEntry`*"]
    pub fn description(&mut self, val: &str) -> &mut Self {
        self.description_shim(val);
        self
    }
    #[doc = "Change the `extension` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FakePluginMimeEntry`*"]
    pub fn extension(&mut self, val: &str) -> &mut Self {
        self.extension_shim(val);
        self
    }
    #[doc = "Change the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FakePluginMimeEntry`*"]
    pub fn type_(&mut self, val: &str) -> &mut Self {
        self.type__shim(val);
        self
    }
}
