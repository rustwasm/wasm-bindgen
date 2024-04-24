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
    #[wasm_bindgen(method, getter = "box")]
    fn box__shim(this: &ResizeObserverOptions) -> ResizeObserverBoxOptions;
    #[cfg(feature = "ResizeObserverBoxOptions")]
    #[wasm_bindgen(method, setter = "box")]
    fn set_box__shim(this: &ResizeObserverOptions, val: ResizeObserverBoxOptions);
}
#[doc = "The trait to access properties on the `ResizeObserverOptions` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `ResizeObserverOptions`*"]
pub trait ResizeObserverOptionsGetters {
    #[cfg(feature = "ResizeObserverBoxOptions")]
    #[doc = "Get the `box` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ResizeObserverBoxOptions`, `ResizeObserverOptions`*"]
    fn box_(&self) -> ResizeObserverBoxOptions;
}
impl ResizeObserverOptionsGetters for ResizeObserverOptions {
    #[cfg(feature = "ResizeObserverBoxOptions")]
    fn box_(&self) -> ResizeObserverBoxOptions {
        self.box__shim()
    }
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
        self.set_box__shim(val);
        self
    }
}
impl Default for ResizeObserverOptions {
    fn default() -> Self {
        Self::new()
    }
}
