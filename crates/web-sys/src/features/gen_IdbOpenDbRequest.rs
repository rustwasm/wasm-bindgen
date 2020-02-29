use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = IdbRequest , extends = EventTarget , extends = :: js_sys :: Object , js_name = IDBOpenDBRequest , typescript_type = "IDBOpenDBRequest" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `IdbOpenDbRequest` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBOpenDBRequest)
    ///
    ///*This API requires the following crate features to be activated: `IdbOpenDbRequest`*
    pub type IdbOpenDbRequest;

    # [ wasm_bindgen ( structural , method , getter , js_class = "IDBOpenDBRequest" , js_name = onblocked ) ]
    ///Getter for the `onblocked` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBOpenDBRequest/onblocked)
    ///
    ///*This API requires the following crate features to be activated: `IdbOpenDbRequest`*
    pub fn onblocked(this: &IdbOpenDbRequest) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "IDBOpenDBRequest" , js_name = onblocked ) ]
    ///Setter for the `onblocked` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBOpenDBRequest/onblocked)
    ///
    ///*This API requires the following crate features to be activated: `IdbOpenDbRequest`*
    pub fn set_onblocked(this: &IdbOpenDbRequest, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "IDBOpenDBRequest" , js_name = onupgradeneeded ) ]
    ///Getter for the `onupgradeneeded` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBOpenDBRequest/onupgradeneeded)
    ///
    ///*This API requires the following crate features to be activated: `IdbOpenDbRequest`*
    pub fn onupgradeneeded(this: &IdbOpenDbRequest) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "IDBOpenDBRequest" , js_name = onupgradeneeded ) ]
    ///Setter for the `onupgradeneeded` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBOpenDBRequest/onupgradeneeded)
    ///
    ///*This API requires the following crate features to be activated: `IdbOpenDbRequest`*
    pub fn set_onupgradeneeded(this: &IdbOpenDbRequest, value: Option<&::js_sys::Function>);

}
