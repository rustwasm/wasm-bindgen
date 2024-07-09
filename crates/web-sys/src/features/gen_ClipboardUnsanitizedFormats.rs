#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = ClipboardUnsanitizedFormats)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ClipboardUnsanitizedFormats` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ClipboardUnsanitizedFormats`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type ClipboardUnsanitizedFormats;
    #[wasm_bindgen(method, setter = "unsanitized")]
    fn unsanitized_shim(this: &ClipboardUnsanitizedFormats, val: &::wasm_bindgen::JsValue);
}
#[cfg(web_sys_unstable_apis)]
impl ClipboardUnsanitizedFormats {
    #[doc = "Construct a new `ClipboardUnsanitizedFormats`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ClipboardUnsanitizedFormats`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `unsanitized` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ClipboardUnsanitizedFormats`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn unsanitized(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.unsanitized_shim(val);
        self
    }
}
#[cfg(web_sys_unstable_apis)]
impl Default for ClipboardUnsanitizedFormats {
    fn default() -> Self {
        Self::new()
    }
}
