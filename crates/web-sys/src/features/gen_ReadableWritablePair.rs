#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = ReadableWritablePair)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ReadableWritablePair` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableWritablePair`*"]
    pub type ReadableWritablePair;
    #[cfg(feature = "ReadableStream")]
    #[wasm_bindgen(method, setter = "readable")]
    fn readable_shim(this: &ReadableWritablePair, val: &ReadableStream);
    #[cfg(feature = "WritableStream")]
    #[wasm_bindgen(method, setter = "writable")]
    fn writable_shim(this: &ReadableWritablePair, val: &WritableStream);
}
impl ReadableWritablePair {
    #[cfg(all(feature = "ReadableStream", feature = "WritableStream",))]
    #[doc = "Construct a new `ReadableWritablePair`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableStream`, `ReadableWritablePair`, `WritableStream`*"]
    pub fn new(readable: &ReadableStream, writable: &WritableStream) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.readable(readable);
        ret.writable(writable);
        ret
    }
    #[cfg(feature = "ReadableStream")]
    #[doc = "Change the `readable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableStream`, `ReadableWritablePair`*"]
    pub fn readable(&mut self, val: &ReadableStream) -> &mut Self {
        self.readable_shim(val);
        self
    }
    #[cfg(feature = "WritableStream")]
    #[doc = "Change the `writable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableWritablePair`, `WritableStream`*"]
    pub fn writable(&mut self, val: &WritableStream) -> &mut Self {
        self.writable_shim(val);
        self
    }
}
