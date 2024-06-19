#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = ConsoleTimerLogOrEnd)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ConsoleTimerLogOrEnd` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleTimerLogOrEnd`*"]
    pub type ConsoleTimerLogOrEnd;
    #[doc = "Get the `duration` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleTimerLogOrEnd`*"]
    #[wasm_bindgen(method, getter = "duration")]
    pub fn get_duration(this: &ConsoleTimerLogOrEnd) -> Option<f64>;
    #[wasm_bindgen(method, setter = "duration")]
    fn set_duration(this: &ConsoleTimerLogOrEnd, val: f64);
    #[doc = "Get the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleTimerLogOrEnd`*"]
    #[wasm_bindgen(method, getter = "name")]
    pub fn get_name(this: &ConsoleTimerLogOrEnd) -> Option<String>;
    #[wasm_bindgen(method, setter = "name")]
    fn set_name(this: &ConsoleTimerLogOrEnd, val: &str);
}
impl ConsoleTimerLogOrEnd {
    #[doc = "Construct a new `ConsoleTimerLogOrEnd`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleTimerLogOrEnd`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `duration` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleTimerLogOrEnd`*"]
    pub fn duration(&mut self, val: f64) -> &mut Self {
        self.set_duration(val);
        self
    }
    #[doc = "Change the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleTimerLogOrEnd`*"]
    pub fn name(&mut self, val: &str) -> &mut Self {
        self.set_name(val);
        self
    }
}
impl Default for ConsoleTimerLogOrEnd {
    fn default() -> Self {
        Self::new()
    }
}
