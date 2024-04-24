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
    #[wasm_bindgen(method, getter = "encrypted")]
    fn encrypted_shim(this: &WebSocketElement) -> bool;
    #[wasm_bindgen(method, setter = "encrypted")]
    fn set_encrypted_shim(this: &WebSocketElement, val: bool);
    #[wasm_bindgen(method, getter = "hostport")]
    fn hostport_shim(this: &WebSocketElement) -> String;
    #[wasm_bindgen(method, setter = "hostport")]
    fn set_hostport_shim(this: &WebSocketElement, val: &str);
    #[wasm_bindgen(method, getter = "msgreceived")]
    fn msgreceived_shim(this: &WebSocketElement) -> u32;
    #[wasm_bindgen(method, setter = "msgreceived")]
    fn set_msgreceived_shim(this: &WebSocketElement, val: u32);
    #[wasm_bindgen(method, getter = "msgsent")]
    fn msgsent_shim(this: &WebSocketElement) -> u32;
    #[wasm_bindgen(method, setter = "msgsent")]
    fn set_msgsent_shim(this: &WebSocketElement, val: u32);
    #[wasm_bindgen(method, getter = "receivedsize")]
    fn receivedsize_shim(this: &WebSocketElement) -> f64;
    #[wasm_bindgen(method, setter = "receivedsize")]
    fn set_receivedsize_shim(this: &WebSocketElement, val: f64);
    #[wasm_bindgen(method, getter = "sentsize")]
    fn sentsize_shim(this: &WebSocketElement) -> f64;
    #[wasm_bindgen(method, setter = "sentsize")]
    fn set_sentsize_shim(this: &WebSocketElement, val: f64);
}
#[doc = "The trait to access properties on the `WebSocketElement` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `WebSocketElement`*"]
pub trait WebSocketElementGetters {
    #[doc = "Get the `encrypted` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebSocketElement`*"]
    fn encrypted(&self) -> bool;
    #[doc = "Get the `hostport` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebSocketElement`*"]
    fn hostport(&self) -> String;
    #[doc = "Get the `msgreceived` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebSocketElement`*"]
    fn msgreceived(&self) -> u32;
    #[doc = "Get the `msgsent` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebSocketElement`*"]
    fn msgsent(&self) -> u32;
    #[doc = "Get the `receivedsize` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebSocketElement`*"]
    fn receivedsize(&self) -> f64;
    #[doc = "Get the `sentsize` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebSocketElement`*"]
    fn sentsize(&self) -> f64;
}
impl WebSocketElementGetters for WebSocketElement {
    fn encrypted(&self) -> bool {
        self.encrypted_shim()
    }
    fn hostport(&self) -> String {
        self.hostport_shim()
    }
    fn msgreceived(&self) -> u32 {
        self.msgreceived_shim()
    }
    fn msgsent(&self) -> u32 {
        self.msgsent_shim()
    }
    fn receivedsize(&self) -> f64 {
        self.receivedsize_shim()
    }
    fn sentsize(&self) -> f64 {
        self.sentsize_shim()
    }
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
        self.set_encrypted_shim(val);
        self
    }
    #[doc = "Change the `hostport` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebSocketElement`*"]
    pub fn hostport(&mut self, val: &str) -> &mut Self {
        self.set_hostport_shim(val);
        self
    }
    #[doc = "Change the `msgreceived` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebSocketElement`*"]
    pub fn msgreceived(&mut self, val: u32) -> &mut Self {
        self.set_msgreceived_shim(val);
        self
    }
    #[doc = "Change the `msgsent` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebSocketElement`*"]
    pub fn msgsent(&mut self, val: u32) -> &mut Self {
        self.set_msgsent_shim(val);
        self
    }
    #[doc = "Change the `receivedsize` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebSocketElement`*"]
    pub fn receivedsize(&mut self, val: f64) -> &mut Self {
        self.set_receivedsize_shim(val);
        self
    }
    #[doc = "Change the `sentsize` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebSocketElement`*"]
    pub fn sentsize(&mut self, val: f64) -> &mut Self {
        self.set_sentsize_shim(val);
        self
    }
}
impl Default for WebSocketElement {
    fn default() -> Self {
        Self::new()
    }
}
