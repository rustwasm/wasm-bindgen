#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = ProfileTimelineMarker)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ProfileTimelineMarker` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProfileTimelineMarker`*"]
    pub type ProfileTimelineMarker;
    #[wasm_bindgen(method, setter = "causeName")]
    fn cause_name_shim(this: &ProfileTimelineMarker, val: &str);
    #[wasm_bindgen(method, setter = "end")]
    fn end_shim(this: &ProfileTimelineMarker, val: f64);
    #[wasm_bindgen(method, setter = "endStack")]
    fn end_stack_shim(this: &ProfileTimelineMarker, val: Option<&::js_sys::Object>);
    #[wasm_bindgen(method, setter = "eventPhase")]
    fn event_phase_shim(this: &ProfileTimelineMarker, val: u16);
    #[wasm_bindgen(method, setter = "isAnimationOnly")]
    fn is_animation_only_shim(this: &ProfileTimelineMarker, val: bool);
    #[wasm_bindgen(method, setter = "isOffMainThread")]
    fn is_off_main_thread_shim(this: &ProfileTimelineMarker, val: bool);
    #[cfg(feature = "ProfileTimelineMessagePortOperationType")]
    #[wasm_bindgen(method, setter = "messagePortOperation")]
    fn message_port_operation_shim(
        this: &ProfileTimelineMarker,
        val: ProfileTimelineMessagePortOperationType,
    );
    #[wasm_bindgen(method, setter = "name")]
    fn name_shim(this: &ProfileTimelineMarker, val: &str);
    #[wasm_bindgen(method, setter = "processType")]
    fn process_type_shim(this: &ProfileTimelineMarker, val: u16);
    #[wasm_bindgen(method, setter = "rectangles")]
    fn rectangles_shim(this: &ProfileTimelineMarker, val: &::wasm_bindgen::JsValue);
    #[wasm_bindgen(method, setter = "stack")]
    fn stack_shim(this: &ProfileTimelineMarker, val: Option<&::js_sys::Object>);
    #[wasm_bindgen(method, setter = "start")]
    fn start_shim(this: &ProfileTimelineMarker, val: f64);
    #[wasm_bindgen(method, setter = "type")]
    fn type__shim(this: &ProfileTimelineMarker, val: &str);
    #[wasm_bindgen(method, setter = "unixTime")]
    fn unix_time_shim(this: &ProfileTimelineMarker, val: f64);
    #[cfg(feature = "ProfileTimelineWorkerOperationType")]
    #[wasm_bindgen(method, setter = "workerOperation")]
    fn worker_operation_shim(this: &ProfileTimelineMarker, val: ProfileTimelineWorkerOperationType);
}
impl ProfileTimelineMarker {
    #[doc = "Construct a new `ProfileTimelineMarker`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProfileTimelineMarker`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `causeName` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProfileTimelineMarker`*"]
    pub fn cause_name(&mut self, val: &str) -> &mut Self {
        self.cause_name_shim(val);
        self
    }
    #[doc = "Change the `end` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProfileTimelineMarker`*"]
    pub fn end(&mut self, val: f64) -> &mut Self {
        self.end_shim(val);
        self
    }
    #[doc = "Change the `endStack` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProfileTimelineMarker`*"]
    pub fn end_stack(&mut self, val: Option<&::js_sys::Object>) -> &mut Self {
        self.end_stack_shim(val);
        self
    }
    #[doc = "Change the `eventPhase` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProfileTimelineMarker`*"]
    pub fn event_phase(&mut self, val: u16) -> &mut Self {
        self.event_phase_shim(val);
        self
    }
    #[doc = "Change the `isAnimationOnly` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProfileTimelineMarker`*"]
    pub fn is_animation_only(&mut self, val: bool) -> &mut Self {
        self.is_animation_only_shim(val);
        self
    }
    #[doc = "Change the `isOffMainThread` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProfileTimelineMarker`*"]
    pub fn is_off_main_thread(&mut self, val: bool) -> &mut Self {
        self.is_off_main_thread_shim(val);
        self
    }
    #[cfg(feature = "ProfileTimelineMessagePortOperationType")]
    #[doc = "Change the `messagePortOperation` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProfileTimelineMarker`, `ProfileTimelineMessagePortOperationType`*"]
    pub fn message_port_operation(
        &mut self,
        val: ProfileTimelineMessagePortOperationType,
    ) -> &mut Self {
        self.message_port_operation_shim(val);
        self
    }
    #[doc = "Change the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProfileTimelineMarker`*"]
    pub fn name(&mut self, val: &str) -> &mut Self {
        self.name_shim(val);
        self
    }
    #[doc = "Change the `processType` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProfileTimelineMarker`*"]
    pub fn process_type(&mut self, val: u16) -> &mut Self {
        self.process_type_shim(val);
        self
    }
    #[doc = "Change the `rectangles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProfileTimelineMarker`*"]
    pub fn rectangles(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.rectangles_shim(val);
        self
    }
    #[doc = "Change the `stack` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProfileTimelineMarker`*"]
    pub fn stack(&mut self, val: Option<&::js_sys::Object>) -> &mut Self {
        self.stack_shim(val);
        self
    }
    #[doc = "Change the `start` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProfileTimelineMarker`*"]
    pub fn start(&mut self, val: f64) -> &mut Self {
        self.start_shim(val);
        self
    }
    #[doc = "Change the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProfileTimelineMarker`*"]
    pub fn type_(&mut self, val: &str) -> &mut Self {
        self.type__shim(val);
        self
    }
    #[doc = "Change the `unixTime` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProfileTimelineMarker`*"]
    pub fn unix_time(&mut self, val: f64) -> &mut Self {
        self.unix_time_shim(val);
        self
    }
    #[cfg(feature = "ProfileTimelineWorkerOperationType")]
    #[doc = "Change the `workerOperation` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProfileTimelineMarker`, `ProfileTimelineWorkerOperationType`*"]
    pub fn worker_operation(&mut self, val: ProfileTimelineWorkerOperationType) -> &mut Self {
        self.worker_operation_shim(val);
        self
    }
}
impl Default for ProfileTimelineMarker {
    fn default() -> Self {
        Self::new()
    }
}
