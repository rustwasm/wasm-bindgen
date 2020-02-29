use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = EventTarget , extends = :: js_sys :: Object , js_name = SharedWorker , typescript_name = SharedWorker ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `SharedWorker` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SharedWorker)
    ///
    ///*This API requires the following crate features to be activated: `SharedWorker`*
    pub type SharedWorker;

    #[cfg(feature = "MessagePort")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SharedWorker" , js_name = port ) ]
    ///Getter for the `port` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SharedWorker/port)
    ///
    ///*This API requires the following crate features to be activated: `MessagePort`, `SharedWorker`*
    pub fn port(this: &SharedWorker) -> MessagePort;

    # [ wasm_bindgen ( structural , method , getter , js_class = "SharedWorker" , js_name = onerror ) ]
    ///Getter for the `onerror` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SharedWorker/onerror)
    ///
    ///*This API requires the following crate features to be activated: `SharedWorker`*
    pub fn onerror(this: &SharedWorker) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "SharedWorker" , js_name = onerror ) ]
    ///Setter for the `onerror` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SharedWorker/onerror)
    ///
    ///*This API requires the following crate features to be activated: `SharedWorker`*
    pub fn set_onerror(this: &SharedWorker, value: Option<&::js_sys::Function>);

    #[wasm_bindgen(catch, constructor, js_class = "SharedWorker")]
    ///The `new SharedWorker(..)` constructor, creating a new instance of `SharedWorker`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SharedWorker/SharedWorker)
    ///
    ///*This API requires the following crate features to be activated: `SharedWorker`*
    pub fn new(script_url: &str) -> Result<SharedWorker, JsValue>;

    #[wasm_bindgen(catch, constructor, js_class = "SharedWorker")]
    ///The `new SharedWorker(..)` constructor, creating a new instance of `SharedWorker`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SharedWorker/SharedWorker)
    ///
    ///*This API requires the following crate features to be activated: `SharedWorker`*
    pub fn new_with_str(script_url: &str, options: &str) -> Result<SharedWorker, JsValue>;

    #[cfg(feature = "WorkerOptions")]
    #[wasm_bindgen(catch, constructor, js_class = "SharedWorker")]
    ///The `new SharedWorker(..)` constructor, creating a new instance of `SharedWorker`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SharedWorker/SharedWorker)
    ///
    ///*This API requires the following crate features to be activated: `SharedWorker`, `WorkerOptions`*
    pub fn new_with_worker_options(
        script_url: &str,
        options: &WorkerOptions,
    ) -> Result<SharedWorker, JsValue>;

}
