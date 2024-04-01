#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = WebSocketElement)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `WebSocketElement` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebSocketElement`*"]
    pub type WebSocketElement;
    #[wasm_bindgen(method, setter = "encrypted")]
    fn encrypted_shim(this: &WebSocketElement, val: bool);
    #[wasm_bindgen(method, setter = "hostport")]
    fn hostport_shim(this: &WebSocketElement, val: &str);
    #[wasm_bindgen(method, setter = "msgreceived")]
    fn msgreceived_shim(this: &WebSocketElement, val: u32);
    #[wasm_bindgen(method, setter = "msgsent")]
    fn msgsent_shim(this: &WebSocketElement, val: u32);
    #[wasm_bindgen(method, setter = "receivedsize")]
    fn receivedsize_shim(this: &WebSocketElement, val: f64);
    #[wasm_bindgen(method, setter = "sentsize")]
    fn sentsize_shim(this: &WebSocketElement, val: f64);
}
impl WebSocketElement {
    #[doc = "Construct a new `WebSocketElement`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebSocketElement`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `encrypted` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebSocketElement`*"]
    pub fn encrypted(&mut self, val: bool) -> &mut Self {
        self.encrypted_shim(val);
        self
    }
    #[doc = "Change the `hostport` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebSocketElement`*"]
    pub fn hostport(&mut self, val: &str) -> &mut Self {
        self.hostport_shim(val);
        self
    }
    #[doc = "Change the `msgreceived` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebSocketElement`*"]
    pub fn msgreceived(&mut self, val: u32) -> &mut Self {
        self.msgreceived_shim(val);
        self
    }
    #[doc = "Change the `msgsent` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebSocketElement`*"]
    pub fn msgsent(&mut self, val: u32) -> &mut Self {
        self.msgsent_shim(val);
        self
    }
    #[doc = "Change the `receivedsize` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebSocketElement`*"]
    pub fn receivedsize(&mut self, val: f64) -> &mut Self {
        self.receivedsize_shim(val);
        self
    }
    #[doc = "Change the `sentsize` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebSocketElement`*"]
    pub fn sentsize(&mut self, val: f64) -> &mut Self {
        self.sentsize_shim(val);
        self
    }
}
impl Default for WebSocketElement {
    fn default() -> Self {
        Self::new()
    }
}
