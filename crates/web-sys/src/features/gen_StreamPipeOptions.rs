#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = StreamPipeOptions)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `StreamPipeOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StreamPipeOptions`*"]
    pub type StreamPipeOptions;
    #[wasm_bindgen(method, setter = "preventAbort")]
    fn prevent_abort_shim(this: &StreamPipeOptions, val: bool);
    #[wasm_bindgen(method, setter = "preventCancel")]
    fn prevent_cancel_shim(this: &StreamPipeOptions, val: bool);
    #[wasm_bindgen(method, setter = "preventClose")]
    fn prevent_close_shim(this: &StreamPipeOptions, val: bool);
    #[cfg(feature = "AbortSignal")]
    #[wasm_bindgen(method, setter = "signal")]
    fn signal_shim(this: &StreamPipeOptions, val: &AbortSignal);
}
impl StreamPipeOptions {
    #[doc = "Construct a new `StreamPipeOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StreamPipeOptions`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `preventAbort` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StreamPipeOptions`*"]
    pub fn prevent_abort(&mut self, val: bool) -> &mut Self {
        self.prevent_abort_shim(val);
        self
    }
    #[doc = "Change the `preventCancel` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StreamPipeOptions`*"]
    pub fn prevent_cancel(&mut self, val: bool) -> &mut Self {
        self.prevent_cancel_shim(val);
        self
    }
    #[doc = "Change the `preventClose` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StreamPipeOptions`*"]
    pub fn prevent_close(&mut self, val: bool) -> &mut Self {
        self.prevent_close_shim(val);
        self
    }
    #[cfg(feature = "AbortSignal")]
    #[doc = "Change the `signal` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AbortSignal`, `StreamPipeOptions`*"]
    pub fn signal(&mut self, val: &AbortSignal) -> &mut Self {
        self.signal_shim(val);
        self
    }
}
impl Default for StreamPipeOptions {
    fn default() -> Self {
        Self::new()
    }
}
