use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = IdbRequest , extends = EventTarget , extends = :: js_sys :: Object , js_name = IDBOpenDBRequest , typescript_name = IDBOpenDBRequest ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `IdbOpenDbRequest` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBOpenDBRequest)\n\n*This API requires the following crate features to be activated: `IdbOpenDbRequest`*"]
    pub type IdbOpenDbRequest;
    # [ wasm_bindgen ( structural , method , getter , js_name = onblocked ) ]
    #[doc = "Getter for the `onblocked` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBOpenDBRequest/onblocked)\n\n*This API requires the following crate features to be activated: `IdbOpenDbRequest`*"]
    pub fn onblocked(this: &IdbOpenDbRequest) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onblocked ) ]
    #[doc = "Setter for the `onblocked` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBOpenDBRequest/onblocked)\n\n*This API requires the following crate features to be activated: `IdbOpenDbRequest`*"]
    pub fn set_onblocked(this: &IdbOpenDbRequest, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = onupgradeneeded ) ]
    #[doc = "Getter for the `onupgradeneeded` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBOpenDBRequest/onupgradeneeded)\n\n*This API requires the following crate features to be activated: `IdbOpenDbRequest`*"]
    pub fn onupgradeneeded(this: &IdbOpenDbRequest) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onupgradeneeded ) ]
    #[doc = "Setter for the `onupgradeneeded` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBOpenDBRequest/onupgradeneeded)\n\n*This API requires the following crate features to be activated: `IdbOpenDbRequest`*"]
    pub fn set_onupgradeneeded(this: &IdbOpenDbRequest, value: Option<&::js_sys::Function>);
}
