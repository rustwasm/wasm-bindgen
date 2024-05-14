#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = ULongRange)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ULongRange` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ULongRange`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type ULongRange;
    #[wasm_bindgen(method, setter = "max")]
    fn max_shim(this: &ULongRange, val: u32);
    #[wasm_bindgen(method, setter = "min")]
    fn min_shim(this: &ULongRange, val: u32);
}
#[cfg(web_sys_unstable_apis)]
impl ULongRange {
    #[doc = "Construct a new `ULongRange`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ULongRange`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `max` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ULongRange`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn max(&mut self, val: u32) -> &mut Self {
        self.max_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `min` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ULongRange`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn min(&mut self, val: u32) -> &mut Self {
        self.min_shim(val);
        self
    }
}
#[cfg(web_sys_unstable_apis)]
impl Default for ULongRange {
    fn default() -> Self {
        Self::new()
    }
}
