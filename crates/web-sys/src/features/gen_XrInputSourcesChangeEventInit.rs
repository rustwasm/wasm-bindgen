#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = XRInputSourcesChangeEventInit)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `XrInputSourcesChangeEventInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrInputSourcesChangeEventInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type XrInputSourcesChangeEventInit;
    #[wasm_bindgen(method, getter = "bubbles")]
    fn bubbles_shim(this: &XrInputSourcesChangeEventInit) -> bool;
    #[wasm_bindgen(method, setter = "bubbles")]
    fn set_bubbles_shim(this: &XrInputSourcesChangeEventInit, val: bool);
    #[wasm_bindgen(method, getter = "cancelable")]
    fn cancelable_shim(this: &XrInputSourcesChangeEventInit) -> bool;
    #[wasm_bindgen(method, setter = "cancelable")]
    fn set_cancelable_shim(this: &XrInputSourcesChangeEventInit, val: bool);
    #[wasm_bindgen(method, getter = "composed")]
    fn composed_shim(this: &XrInputSourcesChangeEventInit) -> bool;
    #[wasm_bindgen(method, setter = "composed")]
    fn set_composed_shim(this: &XrInputSourcesChangeEventInit, val: bool);
    #[wasm_bindgen(method, getter = "added")]
    fn added_shim(this: &XrInputSourcesChangeEventInit) -> ::js_sys::Array;
    #[wasm_bindgen(method, setter = "added")]
    fn set_added_shim(this: &XrInputSourcesChangeEventInit, val: &::wasm_bindgen::JsValue);
    #[wasm_bindgen(method, getter = "removed")]
    fn removed_shim(this: &XrInputSourcesChangeEventInit) -> ::js_sys::Array;
    #[wasm_bindgen(method, setter = "removed")]
    fn set_removed_shim(this: &XrInputSourcesChangeEventInit, val: &::wasm_bindgen::JsValue);
    #[cfg(feature = "XrSession")]
    #[wasm_bindgen(method, getter = "session")]
    fn session_shim(this: &XrInputSourcesChangeEventInit) -> XrSession;
    #[cfg(feature = "XrSession")]
    #[wasm_bindgen(method, setter = "session")]
    fn set_session_shim(this: &XrInputSourcesChangeEventInit, val: &XrSession);
}
#[cfg(web_sys_unstable_apis)]
#[doc = "The trait to access properties on the `XrInputSourcesChangeEventInit` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `XrInputSourcesChangeEventInit`*"]
pub trait XrInputSourcesChangeEventInitGetters {
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrInputSourcesChangeEventInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn bubbles(&self) -> bool;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrInputSourcesChangeEventInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn cancelable(&self) -> bool;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrInputSourcesChangeEventInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn composed(&self) -> bool;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `added` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrInputSourcesChangeEventInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn added(&self) -> ::js_sys::Array;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `removed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrInputSourcesChangeEventInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn removed(&self) -> ::js_sys::Array;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "XrSession")]
    #[doc = "Get the `session` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrInputSourcesChangeEventInit`, `XrSession`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn session(&self) -> XrSession;
}
#[cfg(web_sys_unstable_apis)]
impl XrInputSourcesChangeEventInitGetters for XrInputSourcesChangeEventInit {
    #[cfg(web_sys_unstable_apis)]
    fn bubbles(&self) -> bool {
        self.bubbles_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn cancelable(&self) -> bool {
        self.cancelable_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn composed(&self) -> bool {
        self.composed_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn added(&self) -> ::js_sys::Array {
        self.added_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn removed(&self) -> ::js_sys::Array {
        self.removed_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "XrSession")]
    fn session(&self) -> XrSession {
        self.session_shim()
    }
}
#[cfg(web_sys_unstable_apis)]
impl XrInputSourcesChangeEventInit {
    #[cfg(feature = "XrSession")]
    #[doc = "Construct a new `XrInputSourcesChangeEventInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrInputSourcesChangeEventInit`, `XrSession`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new(
        added: &::wasm_bindgen::JsValue,
        removed: &::wasm_bindgen::JsValue,
        session: &XrSession,
    ) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        Self::added(&mut ret, added);
        Self::removed(&mut ret, removed);
        Self::session(&mut ret, session);
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrInputSourcesChangeEventInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn bubbles(&mut self, val: bool) -> &mut Self {
        self.set_bubbles_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrInputSourcesChangeEventInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn cancelable(&mut self, val: bool) -> &mut Self {
        self.set_cancelable_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrInputSourcesChangeEventInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn composed(&mut self, val: bool) -> &mut Self {
        self.set_composed_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `added` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrInputSourcesChangeEventInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn added(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_added_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `removed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrInputSourcesChangeEventInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn removed(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_removed_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "XrSession")]
    #[doc = "Change the `session` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrInputSourcesChangeEventInit`, `XrSession`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn session(&mut self, val: &XrSession) -> &mut Self {
        self.set_session_shim(val);
        self
    }
}
