use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = EventTarget , extends = :: js_sys :: Object , js_name = SharedWorker , typescript_name = SharedWorker ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SharedWorker` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SharedWorker)\n\n*This API requires the following crate features to be activated: `SharedWorker`*"]
    pub type SharedWorker;
    # [ wasm_bindgen ( structural , method , getter , js_name = port ) ]
    #[cfg(feature = "MessagePort")]
    #[doc = "Getter for the `port` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SharedWorker/port)\n\n*This API requires the following crate features to be activated: `MessagePort`, `SharedWorker`*"]
    pub fn port(this: &SharedWorker) -> MessagePort;
    # [ wasm_bindgen ( structural , method , getter , js_name = onerror ) ]
    #[doc = "Getter for the `onerror` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SharedWorker/onerror)\n\n*This API requires the following crate features to be activated: `SharedWorker`*"]
    pub fn onerror(this: &SharedWorker) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onerror ) ]
    #[doc = "Setter for the `onerror` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SharedWorker/onerror)\n\n*This API requires the following crate features to be activated: `SharedWorker`*"]
    pub fn set_onerror(this: &SharedWorker, value: Option<::js_sys::Function>);
    #[wasm_bindgen(catch, constructor)]
    #[doc = "The `new SharedWorker(..)` constructor, creating a new instance of `SharedWorker`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SharedWorker/SharedWorker)\n\n*This API requires the following crate features to be activated: `SharedWorker`*"]
    pub fn new(this: &SharedWorker, script_url: &str) -> Result<SharedWorker, JsValue>;
    #[wasm_bindgen(catch, constructor)]
    #[doc = "The `new SharedWorker(..)` constructor, creating a new instance of `SharedWorker`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SharedWorker/SharedWorker)\n\n*This API requires the following crate features to be activated: `SharedWorker`*"]
    pub fn new_with_str(
        this: &SharedWorker,
        script_url: &str,
        options: &str,
    ) -> Result<SharedWorker, JsValue>;
    #[cfg(feature = "WorkerOptions")]
    #[wasm_bindgen(catch, constructor)]
    #[doc = "The `new SharedWorker(..)` constructor, creating a new instance of `SharedWorker`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SharedWorker/SharedWorker)\n\n*This API requires the following crate features to be activated: `SharedWorker`, `WorkerOptions`*"]
    pub fn new_with_worker_options(
        this: &SharedWorker,
        script_url: &str,
        options: &WorkerOptions,
    ) -> Result<SharedWorker, JsValue>;
}
