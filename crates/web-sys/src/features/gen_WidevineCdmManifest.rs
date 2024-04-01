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
    #[wasm_bindgen(method, setter = "description")]
    fn description_shim(this: &WidevineCdmManifest, val: &str);
    #[wasm_bindgen(method, setter = "name")]
    fn name_shim(this: &WidevineCdmManifest, val: &str);
    #[wasm_bindgen(method, setter = "version")]
    fn version_shim(this: &WidevineCdmManifest, val: &str);
    #[wasm_bindgen(method, setter = "x-cdm-codecs")]
    fn x_cdm_codecs_shim(this: &WidevineCdmManifest, val: &str);
    #[wasm_bindgen(method, setter = "x-cdm-host-versions")]
    fn x_cdm_host_versions_shim(this: &WidevineCdmManifest, val: &str);
    #[wasm_bindgen(method, setter = "x-cdm-interface-versions")]
    fn x_cdm_interface_versions_shim(this: &WidevineCdmManifest, val: &str);
    #[wasm_bindgen(method, setter = "x-cdm-module-versions")]
    fn x_cdm_module_versions_shim(this: &WidevineCdmManifest, val: &str);
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
        self.description_shim(val);
        self
    }
    #[doc = "Change the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WidevineCdmManifest`*"]
    pub fn name(&mut self, val: &str) -> &mut Self {
        self.name_shim(val);
        self
    }
    #[doc = "Change the `version` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WidevineCdmManifest`*"]
    pub fn version(&mut self, val: &str) -> &mut Self {
        self.version_shim(val);
        self
    }
    #[doc = "Change the `x-cdm-codecs` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WidevineCdmManifest`*"]
    pub fn x_cdm_codecs(&mut self, val: &str) -> &mut Self {
        self.x_cdm_codecs_shim(val);
        self
    }
    #[doc = "Change the `x-cdm-host-versions` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WidevineCdmManifest`*"]
    pub fn x_cdm_host_versions(&mut self, val: &str) -> &mut Self {
        self.x_cdm_host_versions_shim(val);
        self
    }
    #[doc = "Change the `x-cdm-interface-versions` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WidevineCdmManifest`*"]
    pub fn x_cdm_interface_versions(&mut self, val: &str) -> &mut Self {
        self.x_cdm_interface_versions_shim(val);
        self
    }
    #[doc = "Change the `x-cdm-module-versions` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WidevineCdmManifest`*"]
    pub fn x_cdm_module_versions(&mut self, val: &str) -> &mut Self {
        self.x_cdm_module_versions_shim(val);
        self
    }
}
