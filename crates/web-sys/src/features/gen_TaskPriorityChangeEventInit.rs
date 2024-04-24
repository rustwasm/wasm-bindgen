#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = TaskPriorityChangeEventInit)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `TaskPriorityChangeEventInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TaskPriorityChangeEventInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type TaskPriorityChangeEventInit;
    #[wasm_bindgen(method, getter = "bubbles")]
    fn bubbles_shim(this: &TaskPriorityChangeEventInit) -> bool;
    #[wasm_bindgen(method, setter = "bubbles")]
    fn set_bubbles_shim(this: &TaskPriorityChangeEventInit, val: bool);
    #[wasm_bindgen(method, getter = "cancelable")]
    fn cancelable_shim(this: &TaskPriorityChangeEventInit) -> bool;
    #[wasm_bindgen(method, setter = "cancelable")]
    fn set_cancelable_shim(this: &TaskPriorityChangeEventInit, val: bool);
    #[wasm_bindgen(method, getter = "composed")]
    fn composed_shim(this: &TaskPriorityChangeEventInit) -> bool;
    #[wasm_bindgen(method, setter = "composed")]
    fn set_composed_shim(this: &TaskPriorityChangeEventInit, val: bool);
    #[cfg(feature = "TaskPriority")]
    #[wasm_bindgen(method, getter = "previousPriority")]
    fn previous_priority_shim(this: &TaskPriorityChangeEventInit) -> TaskPriority;
    #[cfg(feature = "TaskPriority")]
    #[wasm_bindgen(method, setter = "previousPriority")]
    fn set_previous_priority_shim(this: &TaskPriorityChangeEventInit, val: TaskPriority);
}
#[cfg(web_sys_unstable_apis)]
#[doc = "The trait to access properties on the `TaskPriorityChangeEventInit` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `TaskPriorityChangeEventInit`*"]
pub trait TaskPriorityChangeEventInitGetters {
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TaskPriorityChangeEventInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn bubbles(&self) -> bool;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TaskPriorityChangeEventInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn cancelable(&self) -> bool;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TaskPriorityChangeEventInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn composed(&self) -> bool;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "TaskPriority")]
    #[doc = "Get the `previousPriority` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TaskPriority`, `TaskPriorityChangeEventInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn previous_priority(&self) -> TaskPriority;
}
#[cfg(web_sys_unstable_apis)]
impl TaskPriorityChangeEventInitGetters for TaskPriorityChangeEventInit {
    #[cfg(web_sys_unstable_apis)]
    fn bubbles(&self) -> bool {
        self.bubbles_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn cancelable(&self) -> bool {
        self.cancelable_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn composed(&self) -> bool {
        self.composed_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "TaskPriority")]
    fn previous_priority(&self) -> TaskPriority {
        self.previous_priority_shim()
    }
}
#[cfg(web_sys_unstable_apis)]
impl TaskPriorityChangeEventInit {
    #[cfg(feature = "TaskPriority")]
    #[doc = "Construct a new `TaskPriorityChangeEventInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TaskPriority`, `TaskPriorityChangeEventInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new(previous_priority: TaskPriority) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        Self::previous_priority(&mut ret, previous_priority);
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TaskPriorityChangeEventInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn bubbles(&mut self, val: bool) -> &mut Self {
        self.set_bubbles_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TaskPriorityChangeEventInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn cancelable(&mut self, val: bool) -> &mut Self {
        self.set_cancelable_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TaskPriorityChangeEventInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn composed(&mut self, val: bool) -> &mut Self {
        self.set_composed_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "TaskPriority")]
    #[doc = "Change the `previousPriority` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TaskPriority`, `TaskPriorityChangeEventInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn previous_priority(&mut self, val: TaskPriority) -> &mut Self {
        self.set_previous_priority_shim(val);
        self
    }
}
