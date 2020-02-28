use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = Event , extends = :: js_sys :: Object , js_name = IDBVersionChangeEvent , typescript_name = IDBVersionChangeEvent ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `IdbVersionChangeEvent` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBVersionChangeEvent)\n\n*This API requires the following crate features to be activated: `IdbVersionChangeEvent`*"]
    pub type IdbVersionChangeEvent;
    # [ wasm_bindgen ( structural , method , getter , js_name = oldVersion ) ]
    #[doc = "Getter for the `oldVersion` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBVersionChangeEvent/oldVersion)\n\n*This API requires the following crate features to be activated: `IdbVersionChangeEvent`*"]
    pub fn old_version(this: &IdbVersionChangeEvent) -> f64;
    # [ wasm_bindgen ( structural , method , getter , js_name = newVersion ) ]
    #[doc = "Getter for the `newVersion` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBVersionChangeEvent/newVersion)\n\n*This API requires the following crate features to be activated: `IdbVersionChangeEvent`*"]
    pub fn new_version(this: &IdbVersionChangeEvent) -> Option<f64>;
    #[wasm_bindgen(catch, constructor)]
    #[doc = "The `new IdbVersionChangeEvent(..)` constructor, creating a new instance of `IdbVersionChangeEvent`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBVersionChangeEvent/IDBVersionChangeEvent)\n\n*This API requires the following crate features to be activated: `IdbVersionChangeEvent`*"]
    pub fn new(this: &IdbVersionChangeEvent, type_: &str)
        -> Result<IdbVersionChangeEvent, JsValue>;
    #[cfg(feature = "IdbVersionChangeEventInit")]
    #[wasm_bindgen(catch, constructor)]
    #[doc = "The `new IdbVersionChangeEvent(..)` constructor, creating a new instance of `IdbVersionChangeEvent`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBVersionChangeEvent/IDBVersionChangeEvent)\n\n*This API requires the following crate features to be activated: `IdbVersionChangeEvent`, `IdbVersionChangeEventInit`*"]
    pub fn new_with_event_init_dict(
        this: &IdbVersionChangeEvent,
        type_: &str,
        event_init_dict: &IdbVersionChangeEventInit,
    ) -> Result<IdbVersionChangeEvent, JsValue>;
}
