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
    #[doc = "Get the `l10nArgs` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `L10nElement`*"]
    #[wasm_bindgen(method, getter = "l10nArgs")]
    pub fn get_l10n_args(this: &L10nElement) -> Option<::js_sys::Object>;
    #[wasm_bindgen(method, setter = "l10nArgs")]
    fn set_l10n_args(this: &L10nElement, val: Option<&::js_sys::Object>);
    #[doc = "Get the `l10nAttrs` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `L10nElement`*"]
    #[wasm_bindgen(method, getter = "l10nAttrs")]
    pub fn get_l10n_attrs(this: &L10nElement) -> Option<String>;
    #[wasm_bindgen(method, setter = "l10nAttrs")]
    fn set_l10n_attrs(this: &L10nElement, val: Option<&str>);
    #[doc = "Get the `l10nId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `L10nElement`*"]
    #[wasm_bindgen(method, getter = "l10nId")]
    pub fn get_l10n_id(this: &L10nElement) -> String;
    #[wasm_bindgen(method, setter = "l10nId")]
    fn set_l10n_id(this: &L10nElement, val: &str);
    #[doc = "Get the `localName` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `L10nElement`*"]
    #[wasm_bindgen(method, getter = "localName")]
    pub fn get_local_name(this: &L10nElement) -> String;
    #[wasm_bindgen(method, setter = "localName")]
    fn set_local_name(this: &L10nElement, val: &str);
    #[doc = "Get the `namespaceURI` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `L10nElement`*"]
    #[wasm_bindgen(method, getter = "namespaceURI")]
    pub fn get_namespace_uri(this: &L10nElement) -> String;
    #[wasm_bindgen(method, setter = "namespaceURI")]
    fn set_namespace_uri(this: &L10nElement, val: &str);
    #[doc = "Get the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `L10nElement`*"]
    #[wasm_bindgen(method, getter = "type")]
    pub fn get_type(this: &L10nElement) -> Option<String>;
    #[wasm_bindgen(method, setter = "type")]
    fn set_type(this: &L10nElement, val: Option<&str>);
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
        self.set_l10n_args(val);
        self
    }
    #[doc = "Change the `l10nAttrs` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `L10nElement`*"]
    pub fn l10n_attrs(&mut self, val: Option<&str>) -> &mut Self {
        self.set_l10n_attrs(val);
        self
    }
    #[doc = "Change the `l10nId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `L10nElement`*"]
    pub fn l10n_id(&mut self, val: &str) -> &mut Self {
        self.set_l10n_id(val);
        self
    }
    #[doc = "Change the `localName` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `L10nElement`*"]
    pub fn local_name(&mut self, val: &str) -> &mut Self {
        self.set_local_name(val);
        self
    }
    #[doc = "Change the `namespaceURI` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `L10nElement`*"]
    pub fn namespace_uri(&mut self, val: &str) -> &mut Self {
        self.set_namespace_uri(val);
        self
    }
    #[doc = "Change the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `L10nElement`*"]
    pub fn type_(&mut self, val: Option<&str>) -> &mut Self {
        self.set_type(val);
        self
    }
}
