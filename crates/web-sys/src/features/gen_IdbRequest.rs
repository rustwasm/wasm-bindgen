use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = EventTarget , extends = :: js_sys :: Object , js_name = IDBRequest , typescript_name = IDBRequest ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `IdbRequest` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBRequest)
    ///
    ///*This API requires the following crate features to be activated: `IdbRequest`*
    pub type IdbRequest;

    # [ wasm_bindgen ( structural , catch , method , getter , js_class = "IDBRequest" , js_name = result ) ]
    ///Getter for the `result` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBRequest/result)
    ///
    ///*This API requires the following crate features to be activated: `IdbRequest`*
    pub fn result(this: &IdbRequest) -> Result<::wasm_bindgen::JsValue, JsValue>;

    #[cfg(feature = "DomException")]
    # [ wasm_bindgen ( structural , catch , method , getter , js_class = "IDBRequest" , js_name = error ) ]
    ///Getter for the `error` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBRequest/error)
    ///
    ///*This API requires the following crate features to be activated: `DomException`, `IdbRequest`*
    pub fn error(this: &IdbRequest) -> Result<Option<DomException>, JsValue>;

    # [ wasm_bindgen ( structural , method , getter , js_class = "IDBRequest" , js_name = source ) ]
    ///Getter for the `source` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBRequest/source)
    ///
    ///*This API requires the following crate features to be activated: `IdbRequest`*
    pub fn source(this: &IdbRequest) -> Option<::js_sys::Object>;

    #[cfg(feature = "IdbTransaction")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "IDBRequest" , js_name = transaction ) ]
    ///Getter for the `transaction` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBRequest/transaction)
    ///
    ///*This API requires the following crate features to be activated: `IdbRequest`, `IdbTransaction`*
    pub fn transaction(this: &IdbRequest) -> Option<IdbTransaction>;

    #[cfg(feature = "IdbRequestReadyState")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "IDBRequest" , js_name = readyState ) ]
    ///Getter for the `readyState` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBRequest/readyState)
    ///
    ///*This API requires the following crate features to be activated: `IdbRequest`, `IdbRequestReadyState`*
    pub fn ready_state(this: &IdbRequest) -> IdbRequestReadyState;

    # [ wasm_bindgen ( structural , method , getter , js_class = "IDBRequest" , js_name = onsuccess ) ]
    ///Getter for the `onsuccess` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBRequest/onsuccess)
    ///
    ///*This API requires the following crate features to be activated: `IdbRequest`*
    pub fn onsuccess(this: &IdbRequest) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "IDBRequest" , js_name = onsuccess ) ]
    ///Setter for the `onsuccess` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBRequest/onsuccess)
    ///
    ///*This API requires the following crate features to be activated: `IdbRequest`*
    pub fn set_onsuccess(this: &IdbRequest, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "IDBRequest" , js_name = onerror ) ]
    ///Getter for the `onerror` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBRequest/onerror)
    ///
    ///*This API requires the following crate features to be activated: `IdbRequest`*
    pub fn onerror(this: &IdbRequest) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "IDBRequest" , js_name = onerror ) ]
    ///Setter for the `onerror` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBRequest/onerror)
    ///
    ///*This API requires the following crate features to be activated: `IdbRequest`*
    pub fn set_onerror(this: &IdbRequest, value: Option<&::js_sys::Function>);

}
