#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = LockManagerSnapshot)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `LockManagerSnapshot` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `LockManagerSnapshot`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type LockManagerSnapshot;
    #[wasm_bindgen(method, setter = "held")]
    fn held_shim(this: &LockManagerSnapshot, val: &::wasm_bindgen::JsValue);
    #[wasm_bindgen(method, setter = "pending")]
    fn pending_shim(this: &LockManagerSnapshot, val: &::wasm_bindgen::JsValue);
}
#[cfg(web_sys_unstable_apis)]
impl LockManagerSnapshot {
    #[doc = "Construct a new `LockManagerSnapshot`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `LockManagerSnapshot`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `held` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `LockManagerSnapshot`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn held(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.held_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `pending` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `LockManagerSnapshot`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn pending(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.pending_shim(val);
        self
    }
}
#[cfg(web_sys_unstable_apis)]
impl Default for LockManagerSnapshot {
    fn default() -> Self {
        Self::new()
    }
}
