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
    #[wasm_bindgen(method, getter = "readable")]
    fn readable_shim(this: &ReadableWritablePair) -> &ReadableStream;
    #[cfg(feature = "ReadableStream")]
    #[wasm_bindgen(method, setter = "readable")]
    fn set_readable_shim(this: &ReadableWritablePair, val: &ReadableStream);
    #[cfg(feature = "WritableStream")]
    #[wasm_bindgen(method, getter = "writable")]
    fn writable_shim(this: &ReadableWritablePair) -> &WritableStream;
    #[cfg(feature = "WritableStream")]
    #[wasm_bindgen(method, setter = "writable")]
    fn set_writable_shim(this: &ReadableWritablePair, val: &WritableStream);
}
#[doc = "The trait to access properties on the `ReadableWritablePair` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `ReadableWritablePair`*"]
pub trait ReadableWritablePairGetters {
    #[cfg(feature = "ReadableStream")]
    #[doc = "Get the `readable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableStream`, `ReadableWritablePair`*"]
    fn readable(&self) -> &ReadableStream;
    #[cfg(feature = "WritableStream")]
    #[doc = "Get the `writable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableWritablePair`, `WritableStream`*"]
    fn writable(&self) -> &WritableStream;
}
impl ReadableWritablePairGetters for ReadableWritablePair {
    #[cfg(feature = "ReadableStream")]
    fn readable(&self) -> &ReadableStream {
        self.readable_shim()
    }
    #[cfg(feature = "WritableStream")]
    fn writable(&self) -> &WritableStream {
        self.writable_shim()
    }
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
        self.set_readable_shim(val);
        self
    }
    #[cfg(feature = "WritableStream")]
    #[doc = "Change the `writable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableWritablePair`, `WritableStream`*"]
    pub fn writable(&mut self, val: &WritableStream) -> &mut Self {
        self.set_writable_shim(val);
        self
    }
}
