#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = ConnStatusDict)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ConnStatusDict` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConnStatusDict`*"]
    pub type ConnStatusDict;
    #[wasm_bindgen(method, getter = "status")]
    fn status_shim(this: &ConnStatusDict) -> &str;
    #[wasm_bindgen(method, setter = "status")]
    fn set_status_shim(this: &ConnStatusDict, val: &str);
}
#[doc = "The trait to access properties on the `ConnStatusDict` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `ConnStatusDict`*"]
pub trait ConnStatusDictGetters {
    #[doc = "Get the `status` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConnStatusDict`*"]
    fn status(&self) -> &str;
}
impl ConnStatusDictGetters for ConnStatusDict {
    fn status(&self) -> &str {
        self.status_shim()
    }
}
impl ConnStatusDict {
    #[doc = "Construct a new `ConnStatusDict`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConnStatusDict`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `status` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConnStatusDict`*"]
    pub fn status(&mut self, val: &str) -> &mut Self {
        self.set_status_shim(val);
        self
    }
}
impl Default for ConnStatusDict {
    fn default() -> Self {
        Self::new()
    }
}
