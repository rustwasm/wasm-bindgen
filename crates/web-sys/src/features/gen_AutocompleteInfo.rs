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
    #[wasm_bindgen(method, setter = "addressType")]
    fn address_type_shim(this: &AutocompleteInfo, val: &str);
    #[wasm_bindgen(method, setter = "contactType")]
    fn contact_type_shim(this: &AutocompleteInfo, val: &str);
    #[wasm_bindgen(method, setter = "fieldName")]
    fn field_name_shim(this: &AutocompleteInfo, val: &str);
    #[wasm_bindgen(method, setter = "section")]
    fn section_shim(this: &AutocompleteInfo, val: &str);
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
        self.address_type_shim(val);
        self
    }
    #[doc = "Change the `contactType` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AutocompleteInfo`*"]
    pub fn contact_type(&mut self, val: &str) -> &mut Self {
        self.contact_type_shim(val);
        self
    }
    #[doc = "Change the `fieldName` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AutocompleteInfo`*"]
    pub fn field_name(&mut self, val: &str) -> &mut Self {
        self.field_name_shim(val);
        self
    }
    #[doc = "Change the `section` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AutocompleteInfo`*"]
    pub fn section(&mut self, val: &str) -> &mut Self {
        self.section_shim(val);
        self
    }
}
impl Default for AutocompleteInfo {
    fn default() -> Self {
        Self::new()
    }
}
