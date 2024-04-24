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
    #[wasm_bindgen(method, getter = "causeName")]
    fn cause_name_shim(this: &ProfileTimelineMarker) -> String;
    #[wasm_bindgen(method, setter = "causeName")]
    fn set_cause_name_shim(this: &ProfileTimelineMarker, val: &str);
    #[wasm_bindgen(method, getter = "end")]
    fn end_shim(this: &ProfileTimelineMarker) -> f64;
    #[wasm_bindgen(method, setter = "end")]
    fn set_end_shim(this: &ProfileTimelineMarker, val: f64);
    #[wasm_bindgen(method, getter = "endStack")]
    fn end_stack_shim(this: &ProfileTimelineMarker) -> Option<::js_sys::Object>;
    #[wasm_bindgen(method, setter = "endStack")]
    fn set_end_stack_shim(this: &ProfileTimelineMarker, val: Option<&::js_sys::Object>);
    #[wasm_bindgen(method, getter = "eventPhase")]
    fn event_phase_shim(this: &ProfileTimelineMarker) -> u16;
    #[wasm_bindgen(method, setter = "eventPhase")]
    fn set_event_phase_shim(this: &ProfileTimelineMarker, val: u16);
    #[wasm_bindgen(method, getter = "isAnimationOnly")]
    fn is_animation_only_shim(this: &ProfileTimelineMarker) -> bool;
    #[wasm_bindgen(method, setter = "isAnimationOnly")]
    fn set_is_animation_only_shim(this: &ProfileTimelineMarker, val: bool);
    #[wasm_bindgen(method, getter = "isOffMainThread")]
    fn is_off_main_thread_shim(this: &ProfileTimelineMarker) -> bool;
    #[wasm_bindgen(method, setter = "isOffMainThread")]
    fn set_is_off_main_thread_shim(this: &ProfileTimelineMarker, val: bool);
    #[cfg(feature = "ProfileTimelineMessagePortOperationType")]
    #[wasm_bindgen(method, getter = "messagePortOperation")]
    fn message_port_operation_shim(
        this: &ProfileTimelineMarker,
    ) -> ProfileTimelineMessagePortOperationType;
    #[cfg(feature = "ProfileTimelineMessagePortOperationType")]
    #[wasm_bindgen(method, setter = "messagePortOperation")]
    fn set_message_port_operation_shim(
        this: &ProfileTimelineMarker,
        val: ProfileTimelineMessagePortOperationType,
    );
    #[wasm_bindgen(method, getter = "name")]
    fn name_shim(this: &ProfileTimelineMarker) -> String;
    #[wasm_bindgen(method, setter = "name")]
    fn set_name_shim(this: &ProfileTimelineMarker, val: &str);
    #[wasm_bindgen(method, getter = "processType")]
    fn process_type_shim(this: &ProfileTimelineMarker) -> u16;
    #[wasm_bindgen(method, setter = "processType")]
    fn set_process_type_shim(this: &ProfileTimelineMarker, val: u16);
    #[wasm_bindgen(method, getter = "rectangles")]
    fn rectangles_shim(this: &ProfileTimelineMarker) -> ::js_sys::Array;
    #[wasm_bindgen(method, setter = "rectangles")]
    fn set_rectangles_shim(this: &ProfileTimelineMarker, val: &::wasm_bindgen::JsValue);
    #[wasm_bindgen(method, getter = "stack")]
    fn stack_shim(this: &ProfileTimelineMarker) -> Option<::js_sys::Object>;
    #[wasm_bindgen(method, setter = "stack")]
    fn set_stack_shim(this: &ProfileTimelineMarker, val: Option<&::js_sys::Object>);
    #[wasm_bindgen(method, getter = "start")]
    fn start_shim(this: &ProfileTimelineMarker) -> f64;
    #[wasm_bindgen(method, setter = "start")]
    fn set_start_shim(this: &ProfileTimelineMarker, val: f64);
    #[wasm_bindgen(method, getter = "type")]
    fn type__shim(this: &ProfileTimelineMarker) -> String;
    #[wasm_bindgen(method, setter = "type")]
    fn set_type__shim(this: &ProfileTimelineMarker, val: &str);
    #[wasm_bindgen(method, getter = "unixTime")]
    fn unix_time_shim(this: &ProfileTimelineMarker) -> f64;
    #[wasm_bindgen(method, setter = "unixTime")]
    fn set_unix_time_shim(this: &ProfileTimelineMarker, val: f64);
    #[cfg(feature = "ProfileTimelineWorkerOperationType")]
    #[wasm_bindgen(method, getter = "workerOperation")]
    fn worker_operation_shim(this: &ProfileTimelineMarker) -> ProfileTimelineWorkerOperationType;
    #[cfg(feature = "ProfileTimelineWorkerOperationType")]
    #[wasm_bindgen(method, setter = "workerOperation")]
    fn set_worker_operation_shim(
        this: &ProfileTimelineMarker,
        val: ProfileTimelineWorkerOperationType,
    );
}
#[doc = "The trait to access properties on the `ProfileTimelineMarker` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `ProfileTimelineMarker`*"]
pub trait ProfileTimelineMarkerGetters {
    #[doc = "Get the `causeName` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProfileTimelineMarker`*"]
    fn cause_name(&self) -> String;
    #[doc = "Get the `end` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProfileTimelineMarker`*"]
    fn end(&self) -> f64;
    #[doc = "Get the `endStack` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProfileTimelineMarker`*"]
    fn end_stack(&self) -> Option<::js_sys::Object>;
    #[doc = "Get the `eventPhase` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProfileTimelineMarker`*"]
    fn event_phase(&self) -> u16;
    #[doc = "Get the `isAnimationOnly` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProfileTimelineMarker`*"]
    fn is_animation_only(&self) -> bool;
    #[doc = "Get the `isOffMainThread` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProfileTimelineMarker`*"]
    fn is_off_main_thread(&self) -> bool;
    #[cfg(feature = "ProfileTimelineMessagePortOperationType")]
    #[doc = "Get the `messagePortOperation` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProfileTimelineMarker`, `ProfileTimelineMessagePortOperationType`*"]
    fn message_port_operation(&self) -> ProfileTimelineMessagePortOperationType;
    #[doc = "Get the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProfileTimelineMarker`*"]
    fn name(&self) -> String;
    #[doc = "Get the `processType` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProfileTimelineMarker`*"]
    fn process_type(&self) -> u16;
    #[doc = "Get the `rectangles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProfileTimelineMarker`*"]
    fn rectangles(&self) -> ::js_sys::Array;
    #[doc = "Get the `stack` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProfileTimelineMarker`*"]
    fn stack(&self) -> Option<::js_sys::Object>;
    #[doc = "Get the `start` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProfileTimelineMarker`*"]
    fn start(&self) -> f64;
    #[doc = "Get the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProfileTimelineMarker`*"]
    fn type_(&self) -> String;
    #[doc = "Get the `unixTime` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProfileTimelineMarker`*"]
    fn unix_time(&self) -> f64;
    #[cfg(feature = "ProfileTimelineWorkerOperationType")]
    #[doc = "Get the `workerOperation` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProfileTimelineMarker`, `ProfileTimelineWorkerOperationType`*"]
    fn worker_operation(&self) -> ProfileTimelineWorkerOperationType;
}
impl ProfileTimelineMarkerGetters for ProfileTimelineMarker {
    fn cause_name(&self) -> String {
        self.cause_name_shim()
    }
    fn end(&self) -> f64 {
        self.end_shim()
    }
    fn end_stack(&self) -> Option<::js_sys::Object> {
        self.end_stack_shim()
    }
    fn event_phase(&self) -> u16 {
        self.event_phase_shim()
    }
    fn is_animation_only(&self) -> bool {
        self.is_animation_only_shim()
    }
    fn is_off_main_thread(&self) -> bool {
        self.is_off_main_thread_shim()
    }
    #[cfg(feature = "ProfileTimelineMessagePortOperationType")]
    fn message_port_operation(&self) -> ProfileTimelineMessagePortOperationType {
        self.message_port_operation_shim()
    }
    fn name(&self) -> String {
        self.name_shim()
    }
    fn process_type(&self) -> u16 {
        self.process_type_shim()
    }
    fn rectangles(&self) -> ::js_sys::Array {
        self.rectangles_shim()
    }
    fn stack(&self) -> Option<::js_sys::Object> {
        self.stack_shim()
    }
    fn start(&self) -> f64 {
        self.start_shim()
    }
    fn type_(&self) -> String {
        self.type__shim()
    }
    fn unix_time(&self) -> f64 {
        self.unix_time_shim()
    }
    #[cfg(feature = "ProfileTimelineWorkerOperationType")]
    fn worker_operation(&self) -> ProfileTimelineWorkerOperationType {
        self.worker_operation_shim()
    }
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
        self.set_cause_name_shim(val);
        self
    }
    #[doc = "Change the `end` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProfileTimelineMarker`*"]
    pub fn end(&mut self, val: f64) -> &mut Self {
        self.set_end_shim(val);
        self
    }
    #[doc = "Change the `endStack` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProfileTimelineMarker`*"]
    pub fn end_stack(&mut self, val: Option<&::js_sys::Object>) -> &mut Self {
        self.set_end_stack_shim(val);
        self
    }
    #[doc = "Change the `eventPhase` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProfileTimelineMarker`*"]
    pub fn event_phase(&mut self, val: u16) -> &mut Self {
        self.set_event_phase_shim(val);
        self
    }
    #[doc = "Change the `isAnimationOnly` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProfileTimelineMarker`*"]
    pub fn is_animation_only(&mut self, val: bool) -> &mut Self {
        self.set_is_animation_only_shim(val);
        self
    }
    #[doc = "Change the `isOffMainThread` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProfileTimelineMarker`*"]
    pub fn is_off_main_thread(&mut self, val: bool) -> &mut Self {
        self.set_is_off_main_thread_shim(val);
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
        self.set_message_port_operation_shim(val);
        self
    }
    #[doc = "Change the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProfileTimelineMarker`*"]
    pub fn name(&mut self, val: &str) -> &mut Self {
        self.set_name_shim(val);
        self
    }
    #[doc = "Change the `processType` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProfileTimelineMarker`*"]
    pub fn process_type(&mut self, val: u16) -> &mut Self {
        self.set_process_type_shim(val);
        self
    }
    #[doc = "Change the `rectangles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProfileTimelineMarker`*"]
    pub fn rectangles(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_rectangles_shim(val);
        self
    }
    #[doc = "Change the `stack` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProfileTimelineMarker`*"]
    pub fn stack(&mut self, val: Option<&::js_sys::Object>) -> &mut Self {
        self.set_stack_shim(val);
        self
    }
    #[doc = "Change the `start` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProfileTimelineMarker`*"]
    pub fn start(&mut self, val: f64) -> &mut Self {
        self.set_start_shim(val);
        self
    }
    #[doc = "Change the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProfileTimelineMarker`*"]
    pub fn type_(&mut self, val: &str) -> &mut Self {
        self.set_type__shim(val);
        self
    }
    #[doc = "Change the `unixTime` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProfileTimelineMarker`*"]
    pub fn unix_time(&mut self, val: f64) -> &mut Self {
        self.set_unix_time_shim(val);
        self
    }
    #[cfg(feature = "ProfileTimelineWorkerOperationType")]
    #[doc = "Change the `workerOperation` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProfileTimelineMarker`, `ProfileTimelineWorkerOperationType`*"]
    pub fn worker_operation(&mut self, val: ProfileTimelineWorkerOperationType) -> &mut Self {
        self.set_worker_operation_shim(val);
        self
    }
}
impl Default for ProfileTimelineMarker {
    fn default() -> Self {
        Self::new()
    }
}
