use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = MutationObserver , typescript_type = "MutationObserver" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `MutationObserver` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MutationObserver)
    ///
    ///*This API requires the following crate features to be activated: `MutationObserver`*
    pub type MutationObserver;

    #[wasm_bindgen(catch, constructor, js_class = "MutationObserver")]
    ///The `new MutationObserver(..)` constructor, creating a new instance of `MutationObserver`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MutationObserver/MutationObserver)
    ///
    ///*This API requires the following crate features to be activated: `MutationObserver`*
    pub fn new(mutation_callback: &::js_sys::Function) -> Result<MutationObserver, JsValue>;

    # [ wasm_bindgen ( method , structural , js_class = "MutationObserver" , js_name = disconnect ) ]
    ///The `disconnect()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MutationObserver/disconnect)
    ///
    ///*This API requires the following crate features to be activated: `MutationObserver`*
    pub fn disconnect(this: &MutationObserver);

    #[cfg(feature = "Node")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "MutationObserver" , js_name = observe ) ]
    ///The `observe()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MutationObserver/observe)
    ///
    ///*This API requires the following crate features to be activated: `MutationObserver`, `Node`*
    pub fn observe(this: &MutationObserver, target: &Node) -> Result<(), JsValue>;

    #[cfg(all(feature = "MutationObserverInit", feature = "Node",))]
    # [ wasm_bindgen ( catch , method , structural , js_class = "MutationObserver" , js_name = observe ) ]
    ///The `observe()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MutationObserver/observe)
    ///
    ///*This API requires the following crate features to be activated: `MutationObserver`, `MutationObserverInit`, `Node`*
    pub fn observe_with_options(
        this: &MutationObserver,
        target: &Node,
        options: &MutationObserverInit,
    ) -> Result<(), JsValue>;

    # [ wasm_bindgen ( method , structural , js_class = "MutationObserver" , js_name = takeRecords ) ]
    ///The `takeRecords()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MutationObserver/takeRecords)
    ///
    ///*This API requires the following crate features to be activated: `MutationObserver`*
    pub fn take_records(this: &MutationObserver) -> ::js_sys::Array;

}
