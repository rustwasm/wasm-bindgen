use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = Event , extends = :: js_sys :: Object , js_name = CloseEvent , typescript_name = CloseEvent ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `CloseEvent` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CloseEvent)\n\n*This API requires the following crate features to be activated: `CloseEvent`*"]
    pub type CloseEvent;
    # [ wasm_bindgen ( structural , method , getter , js_name = wasClean ) ]
    #[doc = "Getter for the `wasClean` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CloseEvent/wasClean)\n\n*This API requires the following crate features to be activated: `CloseEvent`*"]
    pub fn was_clean(this: &CloseEvent) -> bool;
    # [ wasm_bindgen ( structural , method , getter , js_name = code ) ]
    #[doc = "Getter for the `code` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CloseEvent/code)\n\n*This API requires the following crate features to be activated: `CloseEvent`*"]
    pub fn code(this: &CloseEvent) -> u16;
    # [ wasm_bindgen ( structural , method , getter , js_name = reason ) ]
    #[doc = "Getter for the `reason` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CloseEvent/reason)\n\n*This API requires the following crate features to be activated: `CloseEvent`*"]
    pub fn reason(this: &CloseEvent) -> String;
    #[wasm_bindgen(catch, constructor)]
    #[doc = "The `new CloseEvent(..)` constructor, creating a new instance of `CloseEvent`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CloseEvent/CloseEvent)\n\n*This API requires the following crate features to be activated: `CloseEvent`*"]
    pub fn new(this: &CloseEvent, type_: &str) -> Result<CloseEvent, JsValue>;
    #[cfg(feature = "CloseEventInit")]
    #[wasm_bindgen(catch, constructor)]
    #[doc = "The `new CloseEvent(..)` constructor, creating a new instance of `CloseEvent`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CloseEvent/CloseEvent)\n\n*This API requires the following crate features to be activated: `CloseEvent`, `CloseEventInit`*"]
    pub fn new_with_event_init_dict(
        this: &CloseEvent,
        type_: &str,
        event_init_dict: &CloseEventInit,
    ) -> Result<CloseEvent, JsValue>;
}
