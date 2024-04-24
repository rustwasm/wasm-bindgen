#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = AutocompleteInfo)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `AutocompleteInfo` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AutocompleteInfo`*"]
    pub type AutocompleteInfo;
    #[wasm_bindgen(method, getter = "addressType")]
    fn address_type_shim(this: &AutocompleteInfo) -> String;
    #[wasm_bindgen(method, setter = "addressType")]
    fn set_address_type_shim(this: &AutocompleteInfo, val: &str);
    #[wasm_bindgen(method, getter = "contactType")]
    fn contact_type_shim(this: &AutocompleteInfo) -> String;
    #[wasm_bindgen(method, setter = "contactType")]
    fn set_contact_type_shim(this: &AutocompleteInfo, val: &str);
    #[wasm_bindgen(method, getter = "fieldName")]
    fn field_name_shim(this: &AutocompleteInfo) -> String;
    #[wasm_bindgen(method, setter = "fieldName")]
    fn set_field_name_shim(this: &AutocompleteInfo, val: &str);
    #[wasm_bindgen(method, getter = "section")]
    fn section_shim(this: &AutocompleteInfo) -> String;
    #[wasm_bindgen(method, setter = "section")]
    fn set_section_shim(this: &AutocompleteInfo, val: &str);
}
#[doc = "The trait to access properties on the `AutocompleteInfo` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `AutocompleteInfo`*"]
pub trait AutocompleteInfoGetters {
    #[doc = "Get the `addressType` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AutocompleteInfo`*"]
    fn address_type(&self) -> String;
    #[doc = "Get the `contactType` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AutocompleteInfo`*"]
    fn contact_type(&self) -> String;
    #[doc = "Get the `fieldName` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AutocompleteInfo`*"]
    fn field_name(&self) -> String;
    #[doc = "Get the `section` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AutocompleteInfo`*"]
    fn section(&self) -> String;
}
impl AutocompleteInfoGetters for AutocompleteInfo {
    fn address_type(&self) -> String {
        self.address_type_shim()
    }
    fn contact_type(&self) -> String {
        self.contact_type_shim()
    }
    fn field_name(&self) -> String {
        self.field_name_shim()
    }
    fn section(&self) -> String {
        self.section_shim()
    }
}
impl AutocompleteInfo {
    #[doc = "Construct a new `AutocompleteInfo`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AutocompleteInfo`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `addressType` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AutocompleteInfo`*"]
    pub fn address_type(&mut self, val: &str) -> &mut Self {
        self.set_address_type_shim(val);
        self
    }
    #[doc = "Change the `contactType` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AutocompleteInfo`*"]
    pub fn contact_type(&mut self, val: &str) -> &mut Self {
        self.set_contact_type_shim(val);
        self
    }
    #[doc = "Change the `fieldName` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AutocompleteInfo`*"]
    pub fn field_name(&mut self, val: &str) -> &mut Self {
        self.set_field_name_shim(val);
        self
    }
    #[doc = "Change the `section` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AutocompleteInfo`*"]
    pub fn section(&mut self, val: &str) -> &mut Self {
        self.set_section_shim(val);
        self
    }
}
impl Default for AutocompleteInfo {
    fn default() -> Self {
        Self::new()
    }
}
