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
    #[doc = "Get the `description` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FakePluginMimeEntry`*"]
    #[wasm_bindgen(method, getter = "description")]
    pub fn get_description(this: &FakePluginMimeEntry) -> Option<String>;
    #[wasm_bindgen(method, setter = "description")]
    fn set_description(this: &FakePluginMimeEntry, val: &str);
    #[doc = "Get the `extension` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FakePluginMimeEntry`*"]
    #[wasm_bindgen(method, getter = "extension")]
    pub fn get_extension(this: &FakePluginMimeEntry) -> Option<String>;
    #[wasm_bindgen(method, setter = "extension")]
    fn set_extension(this: &FakePluginMimeEntry, val: &str);
    #[doc = "Get the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FakePluginMimeEntry`*"]
    #[wasm_bindgen(method, getter = "type")]
    pub fn get_type(this: &FakePluginMimeEntry) -> String;
    #[wasm_bindgen(method, setter = "type")]
    fn set_type(this: &FakePluginMimeEntry, val: &str);
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
        self.set_description(val);
        self
    }
    #[doc = "Change the `extension` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FakePluginMimeEntry`*"]
    pub fn extension(&mut self, val: &str) -> &mut Self {
        self.set_extension(val);
        self
    }
    #[doc = "Change the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FakePluginMimeEntry`*"]
    pub fn type_(&mut self, val: &str) -> &mut Self {
        self.set_type(val);
        self
    }
}
