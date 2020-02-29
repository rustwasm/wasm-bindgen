use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = WorkerGlobalScope , extends = EventTarget , extends = :: js_sys :: Object , js_name = SharedWorkerGlobalScope , typescript_type = "SharedWorkerGlobalScope" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `SharedWorkerGlobalScope` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SharedWorkerGlobalScope)
    ///
    ///*This API requires the following crate features to be activated: `SharedWorkerGlobalScope`*
    pub type SharedWorkerGlobalScope;

    # [ wasm_bindgen ( structural , method , getter , js_class = "SharedWorkerGlobalScope" , js_name = name ) ]
    ///Getter for the `name` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SharedWorkerGlobalScope/name)
    ///
    ///*This API requires the following crate features to be activated: `SharedWorkerGlobalScope`*
    pub fn name(this: &SharedWorkerGlobalScope) -> String;

    # [ wasm_bindgen ( structural , method , getter , js_class = "SharedWorkerGlobalScope" , js_name = onconnect ) ]
    ///Getter for the `onconnect` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SharedWorkerGlobalScope/onconnect)
    ///
    ///*This API requires the following crate features to be activated: `SharedWorkerGlobalScope`*
    pub fn onconnect(this: &SharedWorkerGlobalScope) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "SharedWorkerGlobalScope" , js_name = onconnect ) ]
    ///Setter for the `onconnect` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SharedWorkerGlobalScope/onconnect)
    ///
    ///*This API requires the following crate features to be activated: `SharedWorkerGlobalScope`*
    pub fn set_onconnect(this: &SharedWorkerGlobalScope, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( method , structural , js_class = "SharedWorkerGlobalScope" , js_name = close ) ]
    ///The `close()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SharedWorkerGlobalScope/close)
    ///
    ///*This API requires the following crate features to be activated: `SharedWorkerGlobalScope`*
    pub fn close(this: &SharedWorkerGlobalScope);

}
