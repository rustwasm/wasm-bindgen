#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = MediaStreamEventInit)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `MediaStreamEventInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStreamEventInit`*"]
    pub type MediaStreamEventInit;
    #[doc = "Get the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStreamEventInit`*"]
    #[wasm_bindgen(method, getter = "bubbles")]
    pub fn get_bubbles(this: &MediaStreamEventInit) -> Option<bool>;
    #[wasm_bindgen(method, setter = "bubbles")]
    fn set_bubbles(this: &MediaStreamEventInit, val: bool);
    #[doc = "Get the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStreamEventInit`*"]
    #[wasm_bindgen(method, getter = "cancelable")]
    pub fn get_cancelable(this: &MediaStreamEventInit) -> Option<bool>;
    #[wasm_bindgen(method, setter = "cancelable")]
    fn set_cancelable(this: &MediaStreamEventInit, val: bool);
    #[doc = "Get the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStreamEventInit`*"]
    #[wasm_bindgen(method, getter = "composed")]
    pub fn get_composed(this: &MediaStreamEventInit) -> Option<bool>;
    #[wasm_bindgen(method, setter = "composed")]
    fn set_composed(this: &MediaStreamEventInit, val: bool);
    #[cfg(feature = "MediaStream")]
    #[doc = "Get the `stream` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStream`, `MediaStreamEventInit`*"]
    #[wasm_bindgen(method, getter = "stream")]
    pub fn get_stream(this: &MediaStreamEventInit) -> Option<MediaStream>;
    #[cfg(feature = "MediaStream")]
    #[wasm_bindgen(method, setter = "stream")]
    fn set_stream(this: &MediaStreamEventInit, val: Option<&MediaStream>);
}
impl MediaStreamEventInit {
    #[doc = "Construct a new `MediaStreamEventInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStreamEventInit`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStreamEventInit`*"]
    pub fn bubbles(&mut self, val: bool) -> &mut Self {
        self.set_bubbles(val);
        self
    }
    #[doc = "Change the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStreamEventInit`*"]
    pub fn cancelable(&mut self, val: bool) -> &mut Self {
        self.set_cancelable(val);
        self
    }
    #[doc = "Change the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStreamEventInit`*"]
    pub fn composed(&mut self, val: bool) -> &mut Self {
        self.set_composed(val);
        self
    }
    #[cfg(feature = "MediaStream")]
    #[doc = "Change the `stream` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStream`, `MediaStreamEventInit`*"]
    pub fn stream(&mut self, val: Option<&MediaStream>) -> &mut Self {
        self.set_stream(val);
        self
    }
}
impl Default for MediaStreamEventInit {
    fn default() -> Self {
        Self::new()
    }
}
