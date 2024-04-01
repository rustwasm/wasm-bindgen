#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = FakePluginTagInit)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `FakePluginTagInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FakePluginTagInit`*"]
    pub type FakePluginTagInit;
    #[wasm_bindgen(method, setter = "description")]
    fn description_shim(this: &FakePluginTagInit, val: &str);
    #[wasm_bindgen(method, setter = "fileName")]
    fn file_name_shim(this: &FakePluginTagInit, val: &str);
    #[wasm_bindgen(method, setter = "fullPath")]
    fn full_path_shim(this: &FakePluginTagInit, val: &str);
    #[wasm_bindgen(method, setter = "handlerURI")]
    fn handler_uri_shim(this: &FakePluginTagInit, val: &str);
    #[wasm_bindgen(method, setter = "mimeEntries")]
    fn mime_entries_shim(this: &FakePluginTagInit, val: &::wasm_bindgen::JsValue);
    #[wasm_bindgen(method, setter = "name")]
    fn name_shim(this: &FakePluginTagInit, val: &str);
    #[wasm_bindgen(method, setter = "niceName")]
    fn nice_name_shim(this: &FakePluginTagInit, val: &str);
    #[wasm_bindgen(method, setter = "sandboxScript")]
    fn sandbox_script_shim(this: &FakePluginTagInit, val: &str);
    #[wasm_bindgen(method, setter = "version")]
    fn version_shim(this: &FakePluginTagInit, val: &str);
}
impl FakePluginTagInit {
    #[doc = "Construct a new `FakePluginTagInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FakePluginTagInit`*"]
    pub fn new(handler_uri: &str, mime_entries: &::wasm_bindgen::JsValue) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.handler_uri(handler_uri);
        ret.mime_entries(mime_entries);
        ret
    }
    #[doc = "Change the `description` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FakePluginTagInit`*"]
    pub fn description(&mut self, val: &str) -> &mut Self {
        self.description_shim(val);
        self
    }
    #[doc = "Change the `fileName` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FakePluginTagInit`*"]
    pub fn file_name(&mut self, val: &str) -> &mut Self {
        self.file_name_shim(val);
        self
    }
    #[doc = "Change the `fullPath` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FakePluginTagInit`*"]
    pub fn full_path(&mut self, val: &str) -> &mut Self {
        self.full_path_shim(val);
        self
    }
    #[doc = "Change the `handlerURI` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FakePluginTagInit`*"]
    pub fn handler_uri(&mut self, val: &str) -> &mut Self {
        self.handler_uri_shim(val);
        self
    }
    #[doc = "Change the `mimeEntries` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FakePluginTagInit`*"]
    pub fn mime_entries(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.mime_entries_shim(val);
        self
    }
    #[doc = "Change the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FakePluginTagInit`*"]
    pub fn name(&mut self, val: &str) -> &mut Self {
        self.name_shim(val);
        self
    }
    #[doc = "Change the `niceName` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FakePluginTagInit`*"]
    pub fn nice_name(&mut self, val: &str) -> &mut Self {
        self.nice_name_shim(val);
        self
    }
    #[doc = "Change the `sandboxScript` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FakePluginTagInit`*"]
    pub fn sandbox_script(&mut self, val: &str) -> &mut Self {
        self.sandbox_script_shim(val);
        self
    }
    #[doc = "Change the `version` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FakePluginTagInit`*"]
    pub fn version(&mut self, val: &str) -> &mut Self {
        self.version_shim(val);
        self
    }
}
