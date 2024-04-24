#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = LockInfo)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `LockInfo` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `LockInfo`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type LockInfo;
    #[wasm_bindgen(method, getter = "clientId")]
    fn client_id_shim(this: &LockInfo) -> &str;
    #[wasm_bindgen(method, setter = "clientId")]
    fn set_client_id_shim(this: &LockInfo, val: &str);
    #[cfg(feature = "LockMode")]
    #[wasm_bindgen(method, getter = "mode")]
    fn mode_shim(this: &LockInfo) -> LockMode;
    #[cfg(feature = "LockMode")]
    #[wasm_bindgen(method, setter = "mode")]
    fn set_mode_shim(this: &LockInfo, val: LockMode);
    #[wasm_bindgen(method, getter = "name")]
    fn name_shim(this: &LockInfo) -> &str;
    #[wasm_bindgen(method, setter = "name")]
    fn set_name_shim(this: &LockInfo, val: &str);
}
#[cfg(web_sys_unstable_apis)]
#[doc = "The trait to access properties on the `LockInfo` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `LockInfo`*"]
pub trait LockInfoGetters {
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `clientId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `LockInfo`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn client_id(&self) -> &str;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "LockMode")]
    #[doc = "Get the `mode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `LockInfo`, `LockMode`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn mode(&self) -> LockMode;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `LockInfo`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn name(&self) -> &str;
}
#[cfg(web_sys_unstable_apis)]
impl LockInfoGetters for LockInfo {
    #[cfg(web_sys_unstable_apis)]
    fn client_id(&self) -> &str {
        self.client_id_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "LockMode")]
    fn mode(&self) -> LockMode {
        self.mode_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn name(&self) -> &str {
        self.name_shim()
    }
}
#[cfg(web_sys_unstable_apis)]
impl LockInfo {
    #[doc = "Construct a new `LockInfo`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `LockInfo`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `clientId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `LockInfo`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn client_id(&mut self, val: &str) -> &mut Self {
        self.set_client_id_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "LockMode")]
    #[doc = "Change the `mode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `LockInfo`, `LockMode`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn mode(&mut self, val: LockMode) -> &mut Self {
        self.set_mode_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `LockInfo`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn name(&mut self, val: &str) -> &mut Self {
        self.set_name_shim(val);
        self
    }
}
#[cfg(web_sys_unstable_apis)]
impl Default for LockInfo {
    fn default() -> Self {
        Self::new()
    }
}
