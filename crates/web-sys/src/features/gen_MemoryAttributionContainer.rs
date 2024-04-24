#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = MemoryAttributionContainer)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `MemoryAttributionContainer` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MemoryAttributionContainer`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type MemoryAttributionContainer;
    #[wasm_bindgen(method, getter = "id")]
    fn id_shim(this: &MemoryAttributionContainer) -> &str;
    #[wasm_bindgen(method, setter = "id")]
    fn set_id_shim(this: &MemoryAttributionContainer, val: &str);
    #[wasm_bindgen(method, getter = "src")]
    fn src_shim(this: &MemoryAttributionContainer) -> &str;
    #[wasm_bindgen(method, setter = "src")]
    fn set_src_shim(this: &MemoryAttributionContainer, val: &str);
}
#[cfg(web_sys_unstable_apis)]
#[doc = "The trait to access properties on the `MemoryAttributionContainer` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `MemoryAttributionContainer`*"]
pub trait MemoryAttributionContainerGetters {
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `id` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MemoryAttributionContainer`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn id(&self) -> &str;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `src` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MemoryAttributionContainer`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn src(&self) -> &str;
}
#[cfg(web_sys_unstable_apis)]
impl MemoryAttributionContainerGetters for MemoryAttributionContainer {
    #[cfg(web_sys_unstable_apis)]
    fn id(&self) -> &str {
        self.id_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn src(&self) -> &str {
        self.src_shim()
    }
}
#[cfg(web_sys_unstable_apis)]
impl MemoryAttributionContainer {
    #[doc = "Construct a new `MemoryAttributionContainer`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MemoryAttributionContainer`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `id` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MemoryAttributionContainer`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn id(&mut self, val: &str) -> &mut Self {
        self.set_id_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `src` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MemoryAttributionContainer`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn src(&mut self, val: &str) -> &mut Self {
        self.set_src_shim(val);
        self
    }
}
#[cfg(web_sys_unstable_apis)]
impl Default for MemoryAttributionContainer {
    fn default() -> Self {
        Self::new()
    }
}
