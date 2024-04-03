#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = ResizeObserverOptions)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ResizeObserverOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ResizeObserverOptions`*"]
    pub type ResizeObserverOptions;
    #[cfg(feature = "ResizeObserverBoxOptions")]
    #[wasm_bindgen(method, setter = "box")]
    fn box__shim(this: &ResizeObserverOptions, val: ResizeObserverBoxOptions);
}
impl ResizeObserverOptions {
    #[doc = "Construct a new `ResizeObserverOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ResizeObserverOptions`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[cfg(feature = "ResizeObserverBoxOptions")]
    #[doc = "Change the `box` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ResizeObserverBoxOptions`, `ResizeObserverOptions`*"]
    pub fn box_(&mut self, val: ResizeObserverBoxOptions) -> &mut Self {
        self.box__shim(val);
        self
    }
}
impl Default for ResizeObserverOptions {
    fn default() -> Self {
        Self::new()
    }
}
