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
    #[doc = "Get the `encrypted` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebSocketElement`*"]
    #[wasm_bindgen(method, getter = "encrypted")]
    pub fn get_encrypted(this: &WebSocketElement) -> Option<bool>;
    #[wasm_bindgen(method, setter = "encrypted")]
    fn set_encrypted(this: &WebSocketElement, val: bool);
    #[doc = "Get the `hostport` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebSocketElement`*"]
    #[wasm_bindgen(method, getter = "hostport")]
    pub fn get_hostport(this: &WebSocketElement) -> Option<String>;
    #[wasm_bindgen(method, setter = "hostport")]
    fn set_hostport(this: &WebSocketElement, val: &str);
    #[doc = "Get the `msgreceived` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebSocketElement`*"]
    #[wasm_bindgen(method, getter = "msgreceived")]
    pub fn get_msgreceived(this: &WebSocketElement) -> Option<u32>;
    #[wasm_bindgen(method, setter = "msgreceived")]
    fn set_msgreceived(this: &WebSocketElement, val: u32);
    #[doc = "Get the `msgsent` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebSocketElement`*"]
    #[wasm_bindgen(method, getter = "msgsent")]
    pub fn get_msgsent(this: &WebSocketElement) -> Option<u32>;
    #[wasm_bindgen(method, setter = "msgsent")]
    fn set_msgsent(this: &WebSocketElement, val: u32);
    #[doc = "Get the `receivedsize` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebSocketElement`*"]
    #[wasm_bindgen(method, getter = "receivedsize")]
    pub fn get_receivedsize(this: &WebSocketElement) -> Option<f64>;
    #[wasm_bindgen(method, setter = "receivedsize")]
    fn set_receivedsize(this: &WebSocketElement, val: f64);
    #[doc = "Get the `sentsize` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebSocketElement`*"]
    #[wasm_bindgen(method, getter = "sentsize")]
    pub fn get_sentsize(this: &WebSocketElement) -> Option<f64>;
    #[wasm_bindgen(method, setter = "sentsize")]
    fn set_sentsize(this: &WebSocketElement, val: f64);
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
        self.set_encrypted(val);
        self
    }
    #[doc = "Change the `hostport` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebSocketElement`*"]
    pub fn hostport(&mut self, val: &str) -> &mut Self {
        self.set_hostport(val);
        self
    }
    #[doc = "Change the `msgreceived` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebSocketElement`*"]
    pub fn msgreceived(&mut self, val: u32) -> &mut Self {
        self.set_msgreceived(val);
        self
    }
    #[doc = "Change the `msgsent` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebSocketElement`*"]
    pub fn msgsent(&mut self, val: u32) -> &mut Self {
        self.set_msgsent(val);
        self
    }
    #[doc = "Change the `receivedsize` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebSocketElement`*"]
    pub fn receivedsize(&mut self, val: f64) -> &mut Self {
        self.set_receivedsize(val);
        self
    }
    #[doc = "Change the `sentsize` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebSocketElement`*"]
    pub fn sentsize(&mut self, val: f64) -> &mut Self {
        self.set_sentsize(val);
        self
    }
}
impl Default for WebSocketElement {
    fn default() -> Self {
        Self::new()
    }
}
