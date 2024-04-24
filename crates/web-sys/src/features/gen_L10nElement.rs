#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = L10nElement)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `L10nElement` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `L10nElement`*"]
    pub type L10nElement;
    #[wasm_bindgen(method, getter = "l10nArgs")]
    fn l10n_args_shim(this: &L10nElement) -> Option<::js_sys::Object>;
    #[wasm_bindgen(method, setter = "l10nArgs")]
    fn set_l10n_args_shim(this: &L10nElement, val: Option<&::js_sys::Object>);
    #[wasm_bindgen(method, getter = "l10nAttrs")]
    fn l10n_attrs_shim(this: &L10nElement) -> Option<String>;
    #[wasm_bindgen(method, setter = "l10nAttrs")]
    fn set_l10n_attrs_shim(this: &L10nElement, val: Option<&str>);
    #[wasm_bindgen(method, getter = "l10nId")]
    fn l10n_id_shim(this: &L10nElement) -> String;
    #[wasm_bindgen(method, setter = "l10nId")]
    fn set_l10n_id_shim(this: &L10nElement, val: &str);
    #[wasm_bindgen(method, getter = "localName")]
    fn local_name_shim(this: &L10nElement) -> String;
    #[wasm_bindgen(method, setter = "localName")]
    fn set_local_name_shim(this: &L10nElement, val: &str);
    #[wasm_bindgen(method, getter = "namespaceURI")]
    fn namespace_uri_shim(this: &L10nElement) -> String;
    #[wasm_bindgen(method, setter = "namespaceURI")]
    fn set_namespace_uri_shim(this: &L10nElement, val: &str);
    #[wasm_bindgen(method, getter = "type")]
    fn type__shim(this: &L10nElement) -> Option<String>;
    #[wasm_bindgen(method, setter = "type")]
    fn set_type__shim(this: &L10nElement, val: Option<&str>);
}
#[doc = "The trait to access properties on the `L10nElement` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `L10nElement`*"]
pub trait L10nElementGetters {
    #[doc = "Get the `l10nArgs` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `L10nElement`*"]
    fn l10n_args(&self) -> Option<::js_sys::Object>;
    #[doc = "Get the `l10nAttrs` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `L10nElement`*"]
    fn l10n_attrs(&self) -> Option<String>;
    #[doc = "Get the `l10nId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `L10nElement`*"]
    fn l10n_id(&self) -> String;
    #[doc = "Get the `localName` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `L10nElement`*"]
    fn local_name(&self) -> String;
    #[doc = "Get the `namespaceURI` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `L10nElement`*"]
    fn namespace_uri(&self) -> String;
    #[doc = "Get the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `L10nElement`*"]
    fn type_(&self) -> Option<String>;
}
impl L10nElementGetters for L10nElement {
    fn l10n_args(&self) -> Option<::js_sys::Object> {
        self.l10n_args_shim()
    }
    fn l10n_attrs(&self) -> Option<String> {
        self.l10n_attrs_shim()
    }
    fn l10n_id(&self) -> String {
        self.l10n_id_shim()
    }
    fn local_name(&self) -> String {
        self.local_name_shim()
    }
    fn namespace_uri(&self) -> String {
        self.namespace_uri_shim()
    }
    fn type_(&self) -> Option<String> {
        self.type__shim()
    }
}
impl L10nElement {
    #[doc = "Construct a new `L10nElement`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `L10nElement`*"]
    pub fn new(l10n_id: &str, local_name: &str, namespace_uri: &str) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.l10n_id(l10n_id);
        ret.local_name(local_name);
        ret.namespace_uri(namespace_uri);
        ret
    }
    #[doc = "Change the `l10nArgs` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `L10nElement`*"]
    pub fn l10n_args(&mut self, val: Option<&::js_sys::Object>) -> &mut Self {
        self.set_l10n_args_shim(val);
        self
    }
    #[doc = "Change the `l10nAttrs` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `L10nElement`*"]
    pub fn l10n_attrs(&mut self, val: Option<&str>) -> &mut Self {
        self.set_l10n_attrs_shim(val);
        self
    }
    #[doc = "Change the `l10nId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `L10nElement`*"]
    pub fn l10n_id(&mut self, val: &str) -> &mut Self {
        self.set_l10n_id_shim(val);
        self
    }
    #[doc = "Change the `localName` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `L10nElement`*"]
    pub fn local_name(&mut self, val: &str) -> &mut Self {
        self.set_local_name_shim(val);
        self
    }
    #[doc = "Change the `namespaceURI` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `L10nElement`*"]
    pub fn namespace_uri(&mut self, val: &str) -> &mut Self {
        self.set_namespace_uri_shim(val);
        self
    }
    #[doc = "Change the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `L10nElement`*"]
    pub fn type_(&mut self, val: Option<&str>) -> &mut Self {
        self.set_type__shim(val);
        self
    }
}
