use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = EventTarget , extends = :: js_sys :: Object , js_name = WorkerDebuggerGlobalScope , typescript_type = "WorkerDebuggerGlobalScope" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `WorkerDebuggerGlobalScope` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerDebuggerGlobalScope)
    ///
    ///*This API requires the following crate features to be activated: `WorkerDebuggerGlobalScope`*
    pub type WorkerDebuggerGlobalScope;

    # [ wasm_bindgen ( structural , catch , method , getter , js_class = "WorkerDebuggerGlobalScope" , js_name = global ) ]
    ///Getter for the `global` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerDebuggerGlobalScope/global)
    ///
    ///*This API requires the following crate features to be activated: `WorkerDebuggerGlobalScope`*
    pub fn global(this: &WorkerDebuggerGlobalScope) -> Result<::js_sys::Object, JsValue>;

    # [ wasm_bindgen ( structural , method , getter , js_class = "WorkerDebuggerGlobalScope" , js_name = onmessage ) ]
    ///Getter for the `onmessage` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerDebuggerGlobalScope/onmessage)
    ///
    ///*This API requires the following crate features to be activated: `WorkerDebuggerGlobalScope`*
    pub fn onmessage(this: &WorkerDebuggerGlobalScope) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "WorkerDebuggerGlobalScope" , js_name = onmessage ) ]
    ///Setter for the `onmessage` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerDebuggerGlobalScope/onmessage)
    ///
    ///*This API requires the following crate features to be activated: `WorkerDebuggerGlobalScope`*
    pub fn set_onmessage(this: &WorkerDebuggerGlobalScope, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( catch , method , structural , js_class = "WorkerDebuggerGlobalScope" , js_name = createSandbox ) ]
    ///The `createSandbox()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerDebuggerGlobalScope/createSandbox)
    ///
    ///*This API requires the following crate features to be activated: `WorkerDebuggerGlobalScope`*
    pub fn create_sandbox(
        this: &WorkerDebuggerGlobalScope,
        name: &str,
        prototype: &::js_sys::Object,
    ) -> Result<::js_sys::Object, JsValue>;

    # [ wasm_bindgen ( method , structural , js_class = "WorkerDebuggerGlobalScope" , js_name = dump ) ]
    ///The `dump()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerDebuggerGlobalScope/dump)
    ///
    ///*This API requires the following crate features to be activated: `WorkerDebuggerGlobalScope`*
    pub fn dump(this: &WorkerDebuggerGlobalScope);

    # [ wasm_bindgen ( method , structural , js_class = "WorkerDebuggerGlobalScope" , js_name = dump ) ]
    ///The `dump()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerDebuggerGlobalScope/dump)
    ///
    ///*This API requires the following crate features to be activated: `WorkerDebuggerGlobalScope`*
    pub fn dump_with_string(this: &WorkerDebuggerGlobalScope, string: &str);

    # [ wasm_bindgen ( method , structural , js_class = "WorkerDebuggerGlobalScope" , js_name = enterEventLoop ) ]
    ///The `enterEventLoop()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerDebuggerGlobalScope/enterEventLoop)
    ///
    ///*This API requires the following crate features to be activated: `WorkerDebuggerGlobalScope`*
    pub fn enter_event_loop(this: &WorkerDebuggerGlobalScope);

    # [ wasm_bindgen ( method , structural , js_class = "WorkerDebuggerGlobalScope" , js_name = leaveEventLoop ) ]
    ///The `leaveEventLoop()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerDebuggerGlobalScope/leaveEventLoop)
    ///
    ///*This API requires the following crate features to be activated: `WorkerDebuggerGlobalScope`*
    pub fn leave_event_loop(this: &WorkerDebuggerGlobalScope);

    # [ wasm_bindgen ( catch , method , structural , js_class = "WorkerDebuggerGlobalScope" , js_name = loadSubScript ) ]
    ///The `loadSubScript()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerDebuggerGlobalScope/loadSubScript)
    ///
    ///*This API requires the following crate features to be activated: `WorkerDebuggerGlobalScope`*
    pub fn load_sub_script(this: &WorkerDebuggerGlobalScope, url: &str) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "WorkerDebuggerGlobalScope" , js_name = loadSubScript ) ]
    ///The `loadSubScript()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerDebuggerGlobalScope/loadSubScript)
    ///
    ///*This API requires the following crate features to be activated: `WorkerDebuggerGlobalScope`*
    pub fn load_sub_script_with_sandbox(
        this: &WorkerDebuggerGlobalScope,
        url: &str,
        sandbox: &::js_sys::Object,
    ) -> Result<(), JsValue>;

    # [ wasm_bindgen ( method , structural , js_class = "WorkerDebuggerGlobalScope" , js_name = postMessage ) ]
    ///The `postMessage()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerDebuggerGlobalScope/postMessage)
    ///
    ///*This API requires the following crate features to be activated: `WorkerDebuggerGlobalScope`*
    pub fn post_message(this: &WorkerDebuggerGlobalScope, message: &str);

    # [ wasm_bindgen ( method , structural , js_class = "WorkerDebuggerGlobalScope" , js_name = reportError ) ]
    ///The `reportError()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerDebuggerGlobalScope/reportError)
    ///
    ///*This API requires the following crate features to be activated: `WorkerDebuggerGlobalScope`*
    pub fn report_error(this: &WorkerDebuggerGlobalScope, message: &str);

    # [ wasm_bindgen ( catch , method , structural , js_class = "WorkerDebuggerGlobalScope" , js_name = retrieveConsoleEvents ) ]
    ///The `retrieveConsoleEvents()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerDebuggerGlobalScope/retrieveConsoleEvents)
    ///
    ///*This API requires the following crate features to be activated: `WorkerDebuggerGlobalScope`*
    pub fn retrieve_console_events(
        this: &WorkerDebuggerGlobalScope,
    ) -> Result<::js_sys::Array, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "WorkerDebuggerGlobalScope" , js_name = setConsoleEventHandler ) ]
    ///The `setConsoleEventHandler()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerDebuggerGlobalScope/setConsoleEventHandler)
    ///
    ///*This API requires the following crate features to be activated: `WorkerDebuggerGlobalScope`*
    pub fn set_console_event_handler(
        this: &WorkerDebuggerGlobalScope,
        handler: Option<&::js_sys::Function>,
    ) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "WorkerDebuggerGlobalScope" , js_name = setImmediate ) ]
    ///The `setImmediate()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerDebuggerGlobalScope/setImmediate)
    ///
    ///*This API requires the following crate features to be activated: `WorkerDebuggerGlobalScope`*
    pub fn set_immediate(
        this: &WorkerDebuggerGlobalScope,
        handler: &::js_sys::Function,
    ) -> Result<(), JsValue>;

}
