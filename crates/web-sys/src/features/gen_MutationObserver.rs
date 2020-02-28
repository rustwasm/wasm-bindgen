use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = MutationObserver , typescript_name = MutationObserver ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `MutationObserver` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MutationObserver)\n\n*This API requires the following crate features to be activated: `MutationObserver`*"]
    pub type MutationObserver;
    #[wasm_bindgen(catch, constructor)]
    #[doc = "The `new MutationObserver(..)` constructor, creating a new instance of `MutationObserver`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MutationObserver/MutationObserver)\n\n*This API requires the following crate features to be activated: `MutationObserver`*"]
    pub fn new(
        this: &MutationObserver,
        mutation_callback: &::js_sys::Function,
    ) -> Result<MutationObserver, JsValue>;
    # [ wasm_bindgen ( method , structural , js_name = disconnect ) ]
    #[doc = "The `disconnect()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MutationObserver/disconnect)\n\n*This API requires the following crate features to be activated: `MutationObserver`*"]
    pub fn disconnect(this: &MutationObserver);
    #[cfg(feature = "Node")]
    # [ wasm_bindgen ( catch , method , structural , js_name = observe ) ]
    #[doc = "The `observe()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MutationObserver/observe)\n\n*This API requires the following crate features to be activated: `MutationObserver`, `Node`*"]
    pub fn observe(this: &MutationObserver, target: &Node) -> Result<(), JsValue>;
    #[cfg(all(feature = "MutationObserverInit", feature = "Node",))]
    # [ wasm_bindgen ( catch , method , structural , js_name = observe ) ]
    #[doc = "The `observe()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MutationObserver/observe)\n\n*This API requires the following crate features to be activated: `MutationObserver`, `MutationObserverInit`, `Node`*"]
    pub fn observe_with_options(
        this: &MutationObserver,
        target: &Node,
        options: &MutationObserverInit,
    ) -> Result<(), JsValue>;
    # [ wasm_bindgen ( method , structural , js_name = takeRecords ) ]
    #[doc = "The `takeRecords()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MutationObserver/takeRecords)\n\n*This API requires the following crate features to be activated: `MutationObserver`*"]
    pub fn take_records(this: &MutationObserver) -> ::js_sys::Array;
}
