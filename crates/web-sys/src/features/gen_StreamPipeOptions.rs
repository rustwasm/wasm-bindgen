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
    #[wasm_bindgen(method, getter = "preventAbort")]
    fn prevent_abort_shim(this: &StreamPipeOptions) -> bool;
    #[wasm_bindgen(method, setter = "preventAbort")]
    fn set_prevent_abort_shim(this: &StreamPipeOptions, val: bool);
    #[wasm_bindgen(method, getter = "preventCancel")]
    fn prevent_cancel_shim(this: &StreamPipeOptions) -> bool;
    #[wasm_bindgen(method, setter = "preventCancel")]
    fn set_prevent_cancel_shim(this: &StreamPipeOptions, val: bool);
    #[wasm_bindgen(method, getter = "preventClose")]
    fn prevent_close_shim(this: &StreamPipeOptions) -> bool;
    #[wasm_bindgen(method, setter = "preventClose")]
    fn set_prevent_close_shim(this: &StreamPipeOptions, val: bool);
    #[cfg(feature = "AbortSignal")]
    #[wasm_bindgen(method, getter = "signal")]
    fn signal_shim(this: &StreamPipeOptions) -> AbortSignal;
    #[cfg(feature = "AbortSignal")]
    #[wasm_bindgen(method, setter = "signal")]
    fn set_signal_shim(this: &StreamPipeOptions, val: &AbortSignal);
}
#[doc = "The trait to access properties on the `StreamPipeOptions` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `StreamPipeOptions`*"]
pub trait StreamPipeOptionsGetters {
    #[doc = "Get the `preventAbort` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StreamPipeOptions`*"]
    fn prevent_abort(&self) -> bool;
    #[doc = "Get the `preventCancel` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StreamPipeOptions`*"]
    fn prevent_cancel(&self) -> bool;
    #[doc = "Get the `preventClose` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StreamPipeOptions`*"]
    fn prevent_close(&self) -> bool;
    #[cfg(feature = "AbortSignal")]
    #[doc = "Get the `signal` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AbortSignal`, `StreamPipeOptions`*"]
    fn signal(&self) -> AbortSignal;
}
impl StreamPipeOptionsGetters for StreamPipeOptions {
    fn prevent_abort(&self) -> bool {
        self.prevent_abort_shim()
    }
    fn prevent_cancel(&self) -> bool {
        self.prevent_cancel_shim()
    }
    fn prevent_close(&self) -> bool {
        self.prevent_close_shim()
    }
    #[cfg(feature = "AbortSignal")]
    fn signal(&self) -> AbortSignal {
        self.signal_shim()
    }
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
        self.set_prevent_abort_shim(val);
        self
    }
    #[doc = "Change the `preventCancel` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StreamPipeOptions`*"]
    pub fn prevent_cancel(&mut self, val: bool) -> &mut Self {
        self.set_prevent_cancel_shim(val);
        self
    }
    #[doc = "Change the `preventClose` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StreamPipeOptions`*"]
    pub fn prevent_close(&mut self, val: bool) -> &mut Self {
        self.set_prevent_close_shim(val);
        self
    }
    #[cfg(feature = "AbortSignal")]
    #[doc = "Change the `signal` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AbortSignal`, `StreamPipeOptions`*"]
    pub fn signal(&mut self, val: &AbortSignal) -> &mut Self {
        self.set_signal_shim(val);
        self
    }
}
impl Default for StreamPipeOptions {
    fn default() -> Self {
        Self::new()
    }
}
