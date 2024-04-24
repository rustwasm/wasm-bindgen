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
    #[wasm_bindgen(method, getter = "description")]
    fn description_shim(this: &WidevineCdmManifest) -> String;
    #[wasm_bindgen(method, setter = "description")]
    fn set_description_shim(this: &WidevineCdmManifest, val: &str);
    #[wasm_bindgen(method, getter = "name")]
    fn name_shim(this: &WidevineCdmManifest) -> String;
    #[wasm_bindgen(method, setter = "name")]
    fn set_name_shim(this: &WidevineCdmManifest, val: &str);
    #[wasm_bindgen(method, getter = "version")]
    fn version_shim(this: &WidevineCdmManifest) -> String;
    #[wasm_bindgen(method, setter = "version")]
    fn set_version_shim(this: &WidevineCdmManifest, val: &str);
    #[wasm_bindgen(method, getter = "x-cdm-codecs")]
    fn x_cdm_codecs_shim(this: &WidevineCdmManifest) -> String;
    #[wasm_bindgen(method, setter = "x-cdm-codecs")]
    fn set_x_cdm_codecs_shim(this: &WidevineCdmManifest, val: &str);
    #[wasm_bindgen(method, getter = "x-cdm-host-versions")]
    fn x_cdm_host_versions_shim(this: &WidevineCdmManifest) -> String;
    #[wasm_bindgen(method, setter = "x-cdm-host-versions")]
    fn set_x_cdm_host_versions_shim(this: &WidevineCdmManifest, val: &str);
    #[wasm_bindgen(method, getter = "x-cdm-interface-versions")]
    fn x_cdm_interface_versions_shim(this: &WidevineCdmManifest) -> String;
    #[wasm_bindgen(method, setter = "x-cdm-interface-versions")]
    fn set_x_cdm_interface_versions_shim(this: &WidevineCdmManifest, val: &str);
    #[wasm_bindgen(method, getter = "x-cdm-module-versions")]
    fn x_cdm_module_versions_shim(this: &WidevineCdmManifest) -> String;
    #[wasm_bindgen(method, setter = "x-cdm-module-versions")]
    fn set_x_cdm_module_versions_shim(this: &WidevineCdmManifest, val: &str);
}
#[doc = "The trait to access properties on the `WidevineCdmManifest` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `WidevineCdmManifest`*"]
pub trait WidevineCdmManifestGetters {
    #[doc = "Get the `description` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WidevineCdmManifest`*"]
    fn description(&self) -> String;
    #[doc = "Get the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WidevineCdmManifest`*"]
    fn name(&self) -> String;
    #[doc = "Get the `version` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WidevineCdmManifest`*"]
    fn version(&self) -> String;
    #[doc = "Get the `x-cdm-codecs` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WidevineCdmManifest`*"]
    fn x_cdm_codecs(&self) -> String;
    #[doc = "Get the `x-cdm-host-versions` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WidevineCdmManifest`*"]
    fn x_cdm_host_versions(&self) -> String;
    #[doc = "Get the `x-cdm-interface-versions` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WidevineCdmManifest`*"]
    fn x_cdm_interface_versions(&self) -> String;
    #[doc = "Get the `x-cdm-module-versions` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WidevineCdmManifest`*"]
    fn x_cdm_module_versions(&self) -> String;
}
impl WidevineCdmManifestGetters for WidevineCdmManifest {
    fn description(&self) -> String {
        self.description_shim()
    }
    fn name(&self) -> String {
        self.name_shim()
    }
    fn version(&self) -> String {
        self.version_shim()
    }
    fn x_cdm_codecs(&self) -> String {
        self.x_cdm_codecs_shim()
    }
    fn x_cdm_host_versions(&self) -> String {
        self.x_cdm_host_versions_shim()
    }
    fn x_cdm_interface_versions(&self) -> String {
        self.x_cdm_interface_versions_shim()
    }
    fn x_cdm_module_versions(&self) -> String {
        self.x_cdm_module_versions_shim()
    }
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
        Self::description(&mut ret, description);
        Self::name(&mut ret, name);
        Self::version(&mut ret, version);
        Self::x_cdm_codecs(&mut ret, x_cdm_codecs);
        Self::x_cdm_host_versions(&mut ret, x_cdm_host_versions);
        Self::x_cdm_interface_versions(&mut ret, x_cdm_interface_versions);
        Self::x_cdm_module_versions(&mut ret, x_cdm_module_versions);
        ret
    }
    #[doc = "Change the `description` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WidevineCdmManifest`*"]
    pub fn description(&mut self, val: &str) -> &mut Self {
        self.set_description_shim(val);
        self
    }
    #[doc = "Change the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WidevineCdmManifest`*"]
    pub fn name(&mut self, val: &str) -> &mut Self {
        self.set_name_shim(val);
        self
    }
    #[doc = "Change the `version` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WidevineCdmManifest`*"]
    pub fn version(&mut self, val: &str) -> &mut Self {
        self.set_version_shim(val);
        self
    }
    #[doc = "Change the `x-cdm-codecs` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WidevineCdmManifest`*"]
    pub fn x_cdm_codecs(&mut self, val: &str) -> &mut Self {
        self.set_x_cdm_codecs_shim(val);
        self
    }
    #[doc = "Change the `x-cdm-host-versions` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WidevineCdmManifest`*"]
    pub fn x_cdm_host_versions(&mut self, val: &str) -> &mut Self {
        self.set_x_cdm_host_versions_shim(val);
        self
    }
    #[doc = "Change the `x-cdm-interface-versions` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WidevineCdmManifest`*"]
    pub fn x_cdm_interface_versions(&mut self, val: &str) -> &mut Self {
        self.set_x_cdm_interface_versions_shim(val);
        self
    }
    #[doc = "Change the `x-cdm-module-versions` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WidevineCdmManifest`*"]
    pub fn x_cdm_module_versions(&mut self, val: &str) -> &mut Self {
        self.set_x_cdm_module_versions_shim(val);
        self
    }
}
