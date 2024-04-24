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
    #[wasm_bindgen(method, getter = "description")]
    fn description_shim(this: &FakePluginTagInit) -> String;
    #[wasm_bindgen(method, setter = "description")]
    fn set_description_shim(this: &FakePluginTagInit, val: &str);
    #[wasm_bindgen(method, getter = "fileName")]
    fn file_name_shim(this: &FakePluginTagInit) -> String;
    #[wasm_bindgen(method, setter = "fileName")]
    fn set_file_name_shim(this: &FakePluginTagInit, val: &str);
    #[wasm_bindgen(method, getter = "fullPath")]
    fn full_path_shim(this: &FakePluginTagInit) -> String;
    #[wasm_bindgen(method, setter = "fullPath")]
    fn set_full_path_shim(this: &FakePluginTagInit, val: &str);
    #[wasm_bindgen(method, getter = "handlerURI")]
    fn handler_uri_shim(this: &FakePluginTagInit) -> String;
    #[wasm_bindgen(method, setter = "handlerURI")]
    fn set_handler_uri_shim(this: &FakePluginTagInit, val: &str);
    #[wasm_bindgen(method, getter = "mimeEntries")]
    fn mime_entries_shim(this: &FakePluginTagInit) -> ::js_sys::Array;
    #[wasm_bindgen(method, setter = "mimeEntries")]
    fn set_mime_entries_shim(this: &FakePluginTagInit, val: &::wasm_bindgen::JsValue);
    #[wasm_bindgen(method, getter = "name")]
    fn name_shim(this: &FakePluginTagInit) -> String;
    #[wasm_bindgen(method, setter = "name")]
    fn set_name_shim(this: &FakePluginTagInit, val: &str);
    #[wasm_bindgen(method, getter = "niceName")]
    fn nice_name_shim(this: &FakePluginTagInit) -> String;
    #[wasm_bindgen(method, setter = "niceName")]
    fn set_nice_name_shim(this: &FakePluginTagInit, val: &str);
    #[wasm_bindgen(method, getter = "sandboxScript")]
    fn sandbox_script_shim(this: &FakePluginTagInit) -> String;
    #[wasm_bindgen(method, setter = "sandboxScript")]
    fn set_sandbox_script_shim(this: &FakePluginTagInit, val: &str);
    #[wasm_bindgen(method, getter = "version")]
    fn version_shim(this: &FakePluginTagInit) -> String;
    #[wasm_bindgen(method, setter = "version")]
    fn set_version_shim(this: &FakePluginTagInit, val: &str);
}
#[doc = "The trait to access properties on the `FakePluginTagInit` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `FakePluginTagInit`*"]
pub trait FakePluginTagInitGetters {
    #[doc = "Get the `description` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FakePluginTagInit`*"]
    fn description(&self) -> String;
    #[doc = "Get the `fileName` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FakePluginTagInit`*"]
    fn file_name(&self) -> String;
    #[doc = "Get the `fullPath` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FakePluginTagInit`*"]
    fn full_path(&self) -> String;
    #[doc = "Get the `handlerURI` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FakePluginTagInit`*"]
    fn handler_uri(&self) -> String;
    #[doc = "Get the `mimeEntries` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FakePluginTagInit`*"]
    fn mime_entries(&self) -> ::js_sys::Array;
    #[doc = "Get the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FakePluginTagInit`*"]
    fn name(&self) -> String;
    #[doc = "Get the `niceName` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FakePluginTagInit`*"]
    fn nice_name(&self) -> String;
    #[doc = "Get the `sandboxScript` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FakePluginTagInit`*"]
    fn sandbox_script(&self) -> String;
    #[doc = "Get the `version` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FakePluginTagInit`*"]
    fn version(&self) -> String;
}
impl FakePluginTagInitGetters for FakePluginTagInit {
    fn description(&self) -> String {
        self.description_shim()
    }
    fn file_name(&self) -> String {
        self.file_name_shim()
    }
    fn full_path(&self) -> String {
        self.full_path_shim()
    }
    fn handler_uri(&self) -> String {
        self.handler_uri_shim()
    }
    fn mime_entries(&self) -> ::js_sys::Array {
        self.mime_entries_shim()
    }
    fn name(&self) -> String {
        self.name_shim()
    }
    fn nice_name(&self) -> String {
        self.nice_name_shim()
    }
    fn sandbox_script(&self) -> String {
        self.sandbox_script_shim()
    }
    fn version(&self) -> String {
        self.version_shim()
    }
}
impl FakePluginTagInit {
    #[doc = "Construct a new `FakePluginTagInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FakePluginTagInit`*"]
    pub fn new(handler_uri: &str, mime_entries: &::wasm_bindgen::JsValue) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        Self::handler_uri(&mut ret, handler_uri);
        Self::mime_entries(&mut ret, mime_entries);
        ret
    }
    #[doc = "Change the `description` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FakePluginTagInit`*"]
    pub fn description(&mut self, val: &str) -> &mut Self {
        self.set_description_shim(val);
        self
    }
    #[doc = "Change the `fileName` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FakePluginTagInit`*"]
    pub fn file_name(&mut self, val: &str) -> &mut Self {
        self.set_file_name_shim(val);
        self
    }
    #[doc = "Change the `fullPath` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FakePluginTagInit`*"]
    pub fn full_path(&mut self, val: &str) -> &mut Self {
        self.set_full_path_shim(val);
        self
    }
    #[doc = "Change the `handlerURI` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FakePluginTagInit`*"]
    pub fn handler_uri(&mut self, val: &str) -> &mut Self {
        self.set_handler_uri_shim(val);
        self
    }
    #[doc = "Change the `mimeEntries` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FakePluginTagInit`*"]
    pub fn mime_entries(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_mime_entries_shim(val);
        self
    }
    #[doc = "Change the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FakePluginTagInit`*"]
    pub fn name(&mut self, val: &str) -> &mut Self {
        self.set_name_shim(val);
        self
    }
    #[doc = "Change the `niceName` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FakePluginTagInit`*"]
    pub fn nice_name(&mut self, val: &str) -> &mut Self {
        self.set_nice_name_shim(val);
        self
    }
    #[doc = "Change the `sandboxScript` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FakePluginTagInit`*"]
    pub fn sandbox_script(&mut self, val: &str) -> &mut Self {
        self.set_sandbox_script_shim(val);
        self
    }
    #[doc = "Change the `version` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FakePluginTagInit`*"]
    pub fn version(&mut self, val: &str) -> &mut Self {
        self.set_version_shim(val);
        self
    }
}
