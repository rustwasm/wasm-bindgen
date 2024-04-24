#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = MemoryAttribution)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `MemoryAttribution` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MemoryAttribution`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type MemoryAttribution;
    #[cfg(feature = "MemoryAttributionContainer")]
    #[wasm_bindgen(method, getter = "container")]
    fn container_shim(this: &MemoryAttribution) -> &MemoryAttributionContainer;
    #[cfg(feature = "MemoryAttributionContainer")]
    #[wasm_bindgen(method, setter = "container")]
    fn set_container_shim(this: &MemoryAttribution, val: &MemoryAttributionContainer);
    #[wasm_bindgen(method, getter = "scope")]
    fn scope_shim(this: &MemoryAttribution) -> &str;
    #[wasm_bindgen(method, setter = "scope")]
    fn set_scope_shim(this: &MemoryAttribution, val: &str);
    #[wasm_bindgen(method, getter = "url")]
    fn url_shim(this: &MemoryAttribution) -> &str;
    #[wasm_bindgen(method, setter = "url")]
    fn set_url_shim(this: &MemoryAttribution, val: &str);
}
#[cfg(web_sys_unstable_apis)]
#[doc = "The trait to access properties on the `MemoryAttribution` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `MemoryAttribution`*"]
pub trait MemoryAttributionGetters {
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "MemoryAttributionContainer")]
    #[doc = "Get the `container` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MemoryAttribution`, `MemoryAttributionContainer`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn container(&self) -> &MemoryAttributionContainer;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `scope` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MemoryAttribution`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn scope(&self) -> &str;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `url` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MemoryAttribution`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn url(&self) -> &str;
}
#[cfg(web_sys_unstable_apis)]
impl MemoryAttributionGetters for MemoryAttribution {
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "MemoryAttributionContainer")]
    fn container(&self) -> &MemoryAttributionContainer {
        self.container_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn scope(&self) -> &str {
        self.scope_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn url(&self) -> &str {
        self.url_shim()
    }
}
#[cfg(web_sys_unstable_apis)]
impl MemoryAttribution {
    #[doc = "Construct a new `MemoryAttribution`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MemoryAttribution`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "MemoryAttributionContainer")]
    #[doc = "Change the `container` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MemoryAttribution`, `MemoryAttributionContainer`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn container(&mut self, val: &MemoryAttributionContainer) -> &mut Self {
        self.set_container_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `scope` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MemoryAttribution`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn scope(&mut self, val: &str) -> &mut Self {
        self.set_scope_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `url` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MemoryAttribution`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn url(&mut self, val: &str) -> &mut Self {
        self.set_url_shim(val);
        self
    }
}
#[cfg(web_sys_unstable_apis)]
impl Default for MemoryAttribution {
    fn default() -> Self {
        Self::new()
    }
}
