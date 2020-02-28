use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = EventTarget , extends = :: js_sys :: Object , js_name = WorkerDebuggerGlobalScope , typescript_name = WorkerDebuggerGlobalScope ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `WorkerDebuggerGlobalScope` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerDebuggerGlobalScope)\n\n*This API requires the following crate features to be activated: `WorkerDebuggerGlobalScope`*"]
    pub type WorkerDebuggerGlobalScope;
    # [ wasm_bindgen ( structural , catch , method , getter , js_name = global ) ]
    #[doc = "Getter for the `global` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerDebuggerGlobalScope/global)\n\n*This API requires the following crate features to be activated: `WorkerDebuggerGlobalScope`*"]
    pub fn global(this: &WorkerDebuggerGlobalScope) -> Result<::js_sys::Object, JsValue>;
    # [ wasm_bindgen ( structural , method , getter , js_name = onmessage ) ]
    #[doc = "Getter for the `onmessage` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerDebuggerGlobalScope/onmessage)\n\n*This API requires the following crate features to be activated: `WorkerDebuggerGlobalScope`*"]
    pub fn onmessage(this: &WorkerDebuggerGlobalScope) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onmessage ) ]
    #[doc = "Setter for the `onmessage` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerDebuggerGlobalScope/onmessage)\n\n*This API requires the following crate features to be activated: `WorkerDebuggerGlobalScope`*"]
    pub fn set_onmessage(this: &WorkerDebuggerGlobalScope, value: Option<::js_sys::Function>);
    # [ wasm_bindgen ( catch , method , structural , js_name = createSandbox ) ]
    #[doc = "The `createSandbox()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerDebuggerGlobalScope/createSandbox)\n\n*This API requires the following crate features to be activated: `WorkerDebuggerGlobalScope`*"]
    pub fn create_sandbox(
        this: &WorkerDebuggerGlobalScope,
        name: &str,
        prototype: &::js_sys::Object,
    ) -> Result<::js_sys::Object, JsValue>;
    # [ wasm_bindgen ( method , structural , js_name = dump ) ]
    #[doc = "The `dump()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerDebuggerGlobalScope/dump)\n\n*This API requires the following crate features to be activated: `WorkerDebuggerGlobalScope`*"]
    pub fn dump(this: &WorkerDebuggerGlobalScope);
    # [ wasm_bindgen ( method , structural , js_name = dump ) ]
    #[doc = "The `dump()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerDebuggerGlobalScope/dump)\n\n*This API requires the following crate features to be activated: `WorkerDebuggerGlobalScope`*"]
    pub fn dump_with_string(this: &WorkerDebuggerGlobalScope, string: &str);
    # [ wasm_bindgen ( method , structural , js_name = enterEventLoop ) ]
    #[doc = "The `enterEventLoop()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerDebuggerGlobalScope/enterEventLoop)\n\n*This API requires the following crate features to be activated: `WorkerDebuggerGlobalScope`*"]
    pub fn enter_event_loop(this: &WorkerDebuggerGlobalScope);
    # [ wasm_bindgen ( method , structural , js_name = leaveEventLoop ) ]
    #[doc = "The `leaveEventLoop()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerDebuggerGlobalScope/leaveEventLoop)\n\n*This API requires the following crate features to be activated: `WorkerDebuggerGlobalScope`*"]
    pub fn leave_event_loop(this: &WorkerDebuggerGlobalScope);
    # [ wasm_bindgen ( catch , method , structural , js_name = loadSubScript ) ]
    #[doc = "The `loadSubScript()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerDebuggerGlobalScope/loadSubScript)\n\n*This API requires the following crate features to be activated: `WorkerDebuggerGlobalScope`*"]
    pub fn load_sub_script(this: &WorkerDebuggerGlobalScope, url: &str) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = loadSubScript ) ]
    #[doc = "The `loadSubScript()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerDebuggerGlobalScope/loadSubScript)\n\n*This API requires the following crate features to be activated: `WorkerDebuggerGlobalScope`*"]
    pub fn load_sub_script_with_sandbox(
        this: &WorkerDebuggerGlobalScope,
        url: &str,
        sandbox: &::js_sys::Object,
    ) -> Result<(), JsValue>;
    # [ wasm_bindgen ( method , structural , js_name = postMessage ) ]
    #[doc = "The `postMessage()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerDebuggerGlobalScope/postMessage)\n\n*This API requires the following crate features to be activated: `WorkerDebuggerGlobalScope`*"]
    pub fn post_message(this: &WorkerDebuggerGlobalScope, message: &str);
    # [ wasm_bindgen ( method , structural , js_name = reportError ) ]
    #[doc = "The `reportError()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerDebuggerGlobalScope/reportError)\n\n*This API requires the following crate features to be activated: `WorkerDebuggerGlobalScope`*"]
    pub fn report_error(this: &WorkerDebuggerGlobalScope, message: &str);
    # [ wasm_bindgen ( catch , method , structural , js_name = retrieveConsoleEvents ) ]
    #[doc = "The `retrieveConsoleEvents()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerDebuggerGlobalScope/retrieveConsoleEvents)\n\n*This API requires the following crate features to be activated: `WorkerDebuggerGlobalScope`*"]
    pub fn retrieve_console_events(
        this: &WorkerDebuggerGlobalScope,
    ) -> Result<::js_sys::Array, JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = setConsoleEventHandler ) ]
    #[doc = "The `setConsoleEventHandler()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerDebuggerGlobalScope/setConsoleEventHandler)\n\n*This API requires the following crate features to be activated: `WorkerDebuggerGlobalScope`*"]
    pub fn set_console_event_handler(
        this: &WorkerDebuggerGlobalScope,
        handler: Option<&::js_sys::Function>,
    ) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = setImmediate ) ]
    #[doc = "The `setImmediate()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerDebuggerGlobalScope/setImmediate)\n\n*This API requires the following crate features to be activated: `WorkerDebuggerGlobalScope`*"]
    pub fn set_immediate(
        this: &WorkerDebuggerGlobalScope,
        handler: &::js_sys::Function,
    ) -> Result<(), JsValue>;
}
