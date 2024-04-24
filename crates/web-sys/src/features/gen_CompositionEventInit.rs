#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = CompositionEventInit)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `CompositionEventInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CompositionEventInit`*"]
    pub type CompositionEventInit;
    #[wasm_bindgen(method, getter = "bubbles")]
    fn bubbles_shim(this: &CompositionEventInit) -> bool;
    #[wasm_bindgen(method, setter = "bubbles")]
    fn set_bubbles_shim(this: &CompositionEventInit, val: bool);
    #[wasm_bindgen(method, getter = "cancelable")]
    fn cancelable_shim(this: &CompositionEventInit) -> bool;
    #[wasm_bindgen(method, setter = "cancelable")]
    fn set_cancelable_shim(this: &CompositionEventInit, val: bool);
    #[wasm_bindgen(method, getter = "composed")]
    fn composed_shim(this: &CompositionEventInit) -> bool;
    #[wasm_bindgen(method, setter = "composed")]
    fn set_composed_shim(this: &CompositionEventInit, val: bool);
    #[wasm_bindgen(method, getter = "detail")]
    fn detail_shim(this: &CompositionEventInit) -> i32;
    #[wasm_bindgen(method, setter = "detail")]
    fn set_detail_shim(this: &CompositionEventInit, val: i32);
    #[cfg(feature = "Window")]
    #[wasm_bindgen(method, getter = "view")]
    fn view_shim(this: &CompositionEventInit) -> Option<&Window>;
    #[cfg(feature = "Window")]
    #[wasm_bindgen(method, setter = "view")]
    fn set_view_shim(this: &CompositionEventInit, val: Option<&Window>);
    #[wasm_bindgen(method, getter = "data")]
    fn data_shim(this: &CompositionEventInit) -> &str;
    #[wasm_bindgen(method, setter = "data")]
    fn set_data_shim(this: &CompositionEventInit, val: &str);
}
#[doc = "The trait to access properties on the `CompositionEventInit` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `CompositionEventInit`*"]
pub trait CompositionEventInitGetters {
    #[doc = "Get the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CompositionEventInit`*"]
    fn bubbles(&self) -> bool;
    #[doc = "Get the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CompositionEventInit`*"]
    fn cancelable(&self) -> bool;
    #[doc = "Get the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CompositionEventInit`*"]
    fn composed(&self) -> bool;
    #[doc = "Get the `detail` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CompositionEventInit`*"]
    fn detail(&self) -> i32;
    #[cfg(feature = "Window")]
    #[doc = "Get the `view` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CompositionEventInit`, `Window`*"]
    fn view(&self) -> Option<&Window>;
    #[doc = "Get the `data` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CompositionEventInit`*"]
    fn data(&self) -> &str;
}
impl CompositionEventInitGetters for CompositionEventInit {
    fn bubbles(&self) -> bool {
        self.bubbles_shim()
    }
    fn cancelable(&self) -> bool {
        self.cancelable_shim()
    }
    fn composed(&self) -> bool {
        self.composed_shim()
    }
    fn detail(&self) -> i32 {
        self.detail_shim()
    }
    #[cfg(feature = "Window")]
    fn view(&self) -> Option<&Window> {
        self.view_shim()
    }
    fn data(&self) -> &str {
        self.data_shim()
    }
}
impl CompositionEventInit {
    #[doc = "Construct a new `CompositionEventInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CompositionEventInit`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CompositionEventInit`*"]
    pub fn bubbles(&mut self, val: bool) -> &mut Self {
        self.set_bubbles_shim(val);
        self
    }
    #[doc = "Change the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CompositionEventInit`*"]
    pub fn cancelable(&mut self, val: bool) -> &mut Self {
        self.set_cancelable_shim(val);
        self
    }
    #[doc = "Change the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CompositionEventInit`*"]
    pub fn composed(&mut self, val: bool) -> &mut Self {
        self.set_composed_shim(val);
        self
    }
    #[doc = "Change the `detail` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CompositionEventInit`*"]
    pub fn detail(&mut self, val: i32) -> &mut Self {
        self.set_detail_shim(val);
        self
    }
    #[cfg(feature = "Window")]
    #[doc = "Change the `view` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CompositionEventInit`, `Window`*"]
    pub fn view(&mut self, val: Option<&Window>) -> &mut Self {
        self.set_view_shim(val);
        self
    }
    #[doc = "Change the `data` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CompositionEventInit`*"]
    pub fn data(&mut self, val: &str) -> &mut Self {
        self.set_data_shim(val);
        self
    }
}
impl Default for CompositionEventInit {
    fn default() -> Self {
        Self::new()
    }
}
