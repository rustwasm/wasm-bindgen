#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = FetchEventInit)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `FetchEventInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FetchEventInit`*"]
    pub type FetchEventInit;
    #[wasm_bindgen(method, getter = "bubbles")]
    fn bubbles_shim(this: &FetchEventInit) -> bool;
    #[wasm_bindgen(method, setter = "bubbles")]
    fn set_bubbles_shim(this: &FetchEventInit, val: bool);
    #[wasm_bindgen(method, getter = "cancelable")]
    fn cancelable_shim(this: &FetchEventInit) -> bool;
    #[wasm_bindgen(method, setter = "cancelable")]
    fn set_cancelable_shim(this: &FetchEventInit, val: bool);
    #[wasm_bindgen(method, getter = "composed")]
    fn composed_shim(this: &FetchEventInit) -> bool;
    #[wasm_bindgen(method, setter = "composed")]
    fn set_composed_shim(this: &FetchEventInit, val: bool);
    #[wasm_bindgen(method, getter = "clientId")]
    fn client_id_shim(this: &FetchEventInit) -> Option<&str>;
    #[wasm_bindgen(method, setter = "clientId")]
    fn set_client_id_shim(this: &FetchEventInit, val: Option<&str>);
    #[wasm_bindgen(method, getter = "isReload")]
    fn is_reload_shim(this: &FetchEventInit) -> bool;
    #[wasm_bindgen(method, setter = "isReload")]
    fn set_is_reload_shim(this: &FetchEventInit, val: bool);
    #[cfg(feature = "Request")]
    #[wasm_bindgen(method, getter = "request")]
    fn request_shim(this: &FetchEventInit) -> &Request;
    #[cfg(feature = "Request")]
    #[wasm_bindgen(method, setter = "request")]
    fn set_request_shim(this: &FetchEventInit, val: &Request);
}
#[doc = "The trait to access properties on the `FetchEventInit` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `FetchEventInit`*"]
pub trait FetchEventInitGetters {
    #[doc = "Get the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FetchEventInit`*"]
    fn bubbles(&self) -> bool;
    #[doc = "Get the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FetchEventInit`*"]
    fn cancelable(&self) -> bool;
    #[doc = "Get the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FetchEventInit`*"]
    fn composed(&self) -> bool;
    #[doc = "Get the `clientId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FetchEventInit`*"]
    fn client_id(&self) -> Option<&str>;
    #[doc = "Get the `isReload` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FetchEventInit`*"]
    fn is_reload(&self) -> bool;
    #[cfg(feature = "Request")]
    #[doc = "Get the `request` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FetchEventInit`, `Request`*"]
    fn request(&self) -> &Request;
}
impl FetchEventInitGetters for FetchEventInit {
    fn bubbles(&self) -> bool {
        self.bubbles_shim()
    }
    fn cancelable(&self) -> bool {
        self.cancelable_shim()
    }
    fn composed(&self) -> bool {
        self.composed_shim()
    }
    fn client_id(&self) -> Option<&str> {
        self.client_id_shim()
    }
    fn is_reload(&self) -> bool {
        self.is_reload_shim()
    }
    #[cfg(feature = "Request")]
    fn request(&self) -> &Request {
        self.request_shim()
    }
}
impl FetchEventInit {
    #[cfg(feature = "Request")]
    #[doc = "Construct a new `FetchEventInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FetchEventInit`, `Request`*"]
    pub fn new(request: &Request) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.request(request);
        ret
    }
    #[doc = "Change the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FetchEventInit`*"]
    pub fn bubbles(&mut self, val: bool) -> &mut Self {
        self.set_bubbles_shim(val);
        self
    }
    #[doc = "Change the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FetchEventInit`*"]
    pub fn cancelable(&mut self, val: bool) -> &mut Self {
        self.set_cancelable_shim(val);
        self
    }
    #[doc = "Change the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FetchEventInit`*"]
    pub fn composed(&mut self, val: bool) -> &mut Self {
        self.set_composed_shim(val);
        self
    }
    #[doc = "Change the `clientId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FetchEventInit`*"]
    pub fn client_id(&mut self, val: Option<&str>) -> &mut Self {
        self.set_client_id_shim(val);
        self
    }
    #[doc = "Change the `isReload` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FetchEventInit`*"]
    pub fn is_reload(&mut self, val: bool) -> &mut Self {
        self.set_is_reload_shim(val);
        self
    }
    #[cfg(feature = "Request")]
    #[doc = "Change the `request` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FetchEventInit`, `Request`*"]
    pub fn request(&mut self, val: &Request) -> &mut Self {
        self.set_request_shim(val);
        self
    }
}
