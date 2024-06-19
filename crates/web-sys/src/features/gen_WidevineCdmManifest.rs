#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = WidevineCDMManifest)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `WidevineCdmManifest` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WidevineCdmManifest`*"]
    pub type WidevineCdmManifest;
    #[doc = "Get the `description` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WidevineCdmManifest`*"]
    #[wasm_bindgen(method, getter = "description")]
    pub fn get_description(this: &WidevineCdmManifest) -> String;
    #[wasm_bindgen(method, setter = "description")]
    fn set_description(this: &WidevineCdmManifest, val: &str);
    #[doc = "Get the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WidevineCdmManifest`*"]
    #[wasm_bindgen(method, getter = "name")]
    pub fn get_name(this: &WidevineCdmManifest) -> String;
    #[wasm_bindgen(method, setter = "name")]
    fn set_name(this: &WidevineCdmManifest, val: &str);
    #[doc = "Get the `version` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WidevineCdmManifest`*"]
    #[wasm_bindgen(method, getter = "version")]
    pub fn get_version(this: &WidevineCdmManifest) -> String;
    #[wasm_bindgen(method, setter = "version")]
    fn set_version(this: &WidevineCdmManifest, val: &str);
    #[doc = "Get the `x-cdm-codecs` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WidevineCdmManifest`*"]
    #[wasm_bindgen(method, getter = "x-cdm-codecs")]
    pub fn get_x_cdm_codecs(this: &WidevineCdmManifest) -> String;
    #[wasm_bindgen(method, setter = "x-cdm-codecs")]
    fn set_x_cdm_codecs(this: &WidevineCdmManifest, val: &str);
    #[doc = "Get the `x-cdm-host-versions` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WidevineCdmManifest`*"]
    #[wasm_bindgen(method, getter = "x-cdm-host-versions")]
    pub fn get_x_cdm_host_versions(this: &WidevineCdmManifest) -> String;
    #[wasm_bindgen(method, setter = "x-cdm-host-versions")]
    fn set_x_cdm_host_versions(this: &WidevineCdmManifest, val: &str);
    #[doc = "Get the `x-cdm-interface-versions` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WidevineCdmManifest`*"]
    #[wasm_bindgen(method, getter = "x-cdm-interface-versions")]
    pub fn get_x_cdm_interface_versions(this: &WidevineCdmManifest) -> String;
    #[wasm_bindgen(method, setter = "x-cdm-interface-versions")]
    fn set_x_cdm_interface_versions(this: &WidevineCdmManifest, val: &str);
    #[doc = "Get the `x-cdm-module-versions` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WidevineCdmManifest`*"]
    #[wasm_bindgen(method, getter = "x-cdm-module-versions")]
    pub fn get_x_cdm_module_versions(this: &WidevineCdmManifest) -> String;
    #[wasm_bindgen(method, setter = "x-cdm-module-versions")]
    fn set_x_cdm_module_versions(this: &WidevineCdmManifest, val: &str);
}
impl WidevineCdmManifest {
    #[doc = "Construct a new `WidevineCdmManifest`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WidevineCdmManifest`*"]
    pub fn new(
        description: &str,
        name: &str,
        version: &str,
        x_cdm_codecs: &str,
        x_cdm_host_versions: &str,
        x_cdm_interface_versions: &str,
        x_cdm_module_versions: &str,
    ) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.description(description);
        ret.name(name);
        ret.version(version);
        ret.x_cdm_codecs(x_cdm_codecs);
        ret.x_cdm_host_versions(x_cdm_host_versions);
        ret.x_cdm_interface_versions(x_cdm_interface_versions);
        ret.x_cdm_module_versions(x_cdm_module_versions);
        ret
    }
    #[doc = "Change the `description` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WidevineCdmManifest`*"]
    pub fn description(&mut self, val: &str) -> &mut Self {
        self.set_description(val);
        self
    }
    #[doc = "Change the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WidevineCdmManifest`*"]
    pub fn name(&mut self, val: &str) -> &mut Self {
        self.set_name(val);
        self
    }
    #[doc = "Change the `version` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WidevineCdmManifest`*"]
    pub fn version(&mut self, val: &str) -> &mut Self {
        self.set_version(val);
        self
    }
    #[doc = "Change the `x-cdm-codecs` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WidevineCdmManifest`*"]
    pub fn x_cdm_codecs(&mut self, val: &str) -> &mut Self {
        self.set_x_cdm_codecs(val);
        self
    }
    #[doc = "Change the `x-cdm-host-versions` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WidevineCdmManifest`*"]
    pub fn x_cdm_host_versions(&mut self, val: &str) -> &mut Self {
        self.set_x_cdm_host_versions(val);
        self
    }
    #[doc = "Change the `x-cdm-interface-versions` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WidevineCdmManifest`*"]
    pub fn x_cdm_interface_versions(&mut self, val: &str) -> &mut Self {
        self.set_x_cdm_interface_versions(val);
        self
    }
    #[doc = "Change the `x-cdm-module-versions` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WidevineCdmManifest`*"]
    pub fn x_cdm_module_versions(&mut self, val: &str) -> &mut Self {
        self.set_x_cdm_module_versions(val);
        self
    }
}
