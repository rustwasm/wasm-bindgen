#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = ScrollViewChangeEventInit)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ScrollViewChangeEventInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ScrollViewChangeEventInit`*"]
    pub type ScrollViewChangeEventInit;
    #[wasm_bindgen(method, getter = "bubbles")]
    fn bubbles_shim(this: &ScrollViewChangeEventInit) -> bool;
    #[wasm_bindgen(method, setter = "bubbles")]
    fn set_bubbles_shim(this: &ScrollViewChangeEventInit, val: bool);
    #[wasm_bindgen(method, getter = "cancelable")]
    fn cancelable_shim(this: &ScrollViewChangeEventInit) -> bool;
    #[wasm_bindgen(method, setter = "cancelable")]
    fn set_cancelable_shim(this: &ScrollViewChangeEventInit, val: bool);
    #[wasm_bindgen(method, getter = "composed")]
    fn composed_shim(this: &ScrollViewChangeEventInit) -> bool;
    #[wasm_bindgen(method, setter = "composed")]
    fn set_composed_shim(this: &ScrollViewChangeEventInit, val: bool);
    #[cfg(feature = "ScrollState")]
    #[wasm_bindgen(method, getter = "state")]
    fn state_shim(this: &ScrollViewChangeEventInit) -> ScrollState;
    #[cfg(feature = "ScrollState")]
    #[wasm_bindgen(method, setter = "state")]
    fn set_state_shim(this: &ScrollViewChangeEventInit, val: ScrollState);
}
#[doc = "The trait to access properties on the `ScrollViewChangeEventInit` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `ScrollViewChangeEventInit`*"]
pub trait ScrollViewChangeEventInitGetters {
    #[doc = "Get the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ScrollViewChangeEventInit`*"]
    fn bubbles(&self) -> bool;
    #[doc = "Get the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ScrollViewChangeEventInit`*"]
    fn cancelable(&self) -> bool;
    #[doc = "Get the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ScrollViewChangeEventInit`*"]
    fn composed(&self) -> bool;
    #[cfg(feature = "ScrollState")]
    #[doc = "Get the `state` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ScrollState`, `ScrollViewChangeEventInit`*"]
    fn state(&self) -> ScrollState;
}
impl ScrollViewChangeEventInitGetters for ScrollViewChangeEventInit {
    fn bubbles(&self) -> bool {
        self.bubbles_shim()
    }
    fn cancelable(&self) -> bool {
        self.cancelable_shim()
    }
    fn composed(&self) -> bool {
        self.composed_shim()
    }
    #[cfg(feature = "ScrollState")]
    fn state(&self) -> ScrollState {
        self.state_shim()
    }
}
impl ScrollViewChangeEventInit {
    #[doc = "Construct a new `ScrollViewChangeEventInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ScrollViewChangeEventInit`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ScrollViewChangeEventInit`*"]
    pub fn bubbles(&mut self, val: bool) -> &mut Self {
        self.set_bubbles_shim(val);
        self
    }
    #[doc = "Change the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ScrollViewChangeEventInit`*"]
    pub fn cancelable(&mut self, val: bool) -> &mut Self {
        self.set_cancelable_shim(val);
        self
    }
    #[doc = "Change the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ScrollViewChangeEventInit`*"]
    pub fn composed(&mut self, val: bool) -> &mut Self {
        self.set_composed_shim(val);
        self
    }
    #[cfg(feature = "ScrollState")]
    #[doc = "Change the `state` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ScrollState`, `ScrollViewChangeEventInit`*"]
    pub fn state(&mut self, val: ScrollState) -> &mut Self {
        self.set_state_shim(val);
        self
    }
}
impl Default for ScrollViewChangeEventInit {
    fn default() -> Self {
        Self::new()
    }
}
