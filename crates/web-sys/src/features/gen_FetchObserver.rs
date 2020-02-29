use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = EventTarget , extends = :: js_sys :: Object , js_name = FetchObserver , typescript_type = "FetchObserver" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `FetchObserver` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FetchObserver)
    ///
    ///*This API requires the following crate features to be activated: `FetchObserver`*
    pub type FetchObserver;

    #[cfg(feature = "FetchState")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "FetchObserver" , js_name = state ) ]
    ///Getter for the `state` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FetchObserver/state)
    ///
    ///*This API requires the following crate features to be activated: `FetchObserver`, `FetchState`*
    pub fn state(this: &FetchObserver) -> FetchState;

    # [ wasm_bindgen ( structural , method , getter , js_class = "FetchObserver" , js_name = onstatechange ) ]
    ///Getter for the `onstatechange` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FetchObserver/onstatechange)
    ///
    ///*This API requires the following crate features to be activated: `FetchObserver`*
    pub fn onstatechange(this: &FetchObserver) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "FetchObserver" , js_name = onstatechange ) ]
    ///Setter for the `onstatechange` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FetchObserver/onstatechange)
    ///
    ///*This API requires the following crate features to be activated: `FetchObserver`*
    pub fn set_onstatechange(this: &FetchObserver, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "FetchObserver" , js_name = onrequestprogress ) ]
    ///Getter for the `onrequestprogress` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FetchObserver/onrequestprogress)
    ///
    ///*This API requires the following crate features to be activated: `FetchObserver`*
    pub fn onrequestprogress(this: &FetchObserver) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "FetchObserver" , js_name = onrequestprogress ) ]
    ///Setter for the `onrequestprogress` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FetchObserver/onrequestprogress)
    ///
    ///*This API requires the following crate features to be activated: `FetchObserver`*
    pub fn set_onrequestprogress(this: &FetchObserver, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "FetchObserver" , js_name = onresponseprogress ) ]
    ///Getter for the `onresponseprogress` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FetchObserver/onresponseprogress)
    ///
    ///*This API requires the following crate features to be activated: `FetchObserver`*
    pub fn onresponseprogress(this: &FetchObserver) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "FetchObserver" , js_name = onresponseprogress ) ]
    ///Setter for the `onresponseprogress` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FetchObserver/onresponseprogress)
    ///
    ///*This API requires the following crate features to be activated: `FetchObserver`*
    pub fn set_onresponseprogress(this: &FetchObserver, value: Option<&::js_sys::Function>);

}
