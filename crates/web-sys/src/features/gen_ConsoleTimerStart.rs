#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = ConsoleTimerStart)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ConsoleTimerStart` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleTimerStart`*"]
    pub type ConsoleTimerStart;
    #[wasm_bindgen(method, getter = "name")]
    fn name_shim(this: &ConsoleTimerStart) -> &str;
    #[wasm_bindgen(method, setter = "name")]
    fn set_name_shim(this: &ConsoleTimerStart, val: &str);
}
#[doc = "The trait to access properties on the `ConsoleTimerStart` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `ConsoleTimerStart`*"]
pub trait ConsoleTimerStartGetters {
    #[doc = "Get the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleTimerStart`*"]
    fn name(&self) -> &str;
}
impl ConsoleTimerStartGetters for ConsoleTimerStart {
    fn name(&self) -> &str {
        self.name_shim()
    }
}
impl ConsoleTimerStart {
    #[doc = "Construct a new `ConsoleTimerStart`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleTimerStart`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleTimerStart`*"]
    pub fn name(&mut self, val: &str) -> &mut Self {
        self.set_name_shim(val);
        self
    }
}
impl Default for ConsoleTimerStart {
    fn default() -> Self {
        Self::new()
    }
}
