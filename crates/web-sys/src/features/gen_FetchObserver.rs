use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = EventTarget , extends = :: js_sys :: Object , js_name = FetchObserver , typescript_name = FetchObserver ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `FetchObserver` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FetchObserver)\n\n*This API requires the following crate features to be activated: `FetchObserver`*"]
    pub type FetchObserver;
    # [ wasm_bindgen ( structural , method , getter , js_name = state ) ]
    #[cfg(feature = "FetchState")]
    #[doc = "Getter for the `state` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FetchObserver/state)\n\n*This API requires the following crate features to be activated: `FetchObserver`, `FetchState`*"]
    pub fn state(this: &FetchObserver) -> FetchState;
    # [ wasm_bindgen ( structural , method , getter , js_name = onstatechange ) ]
    #[doc = "Getter for the `onstatechange` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FetchObserver/onstatechange)\n\n*This API requires the following crate features to be activated: `FetchObserver`*"]
    pub fn onstatechange(this: &FetchObserver) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onstatechange ) ]
    #[doc = "Setter for the `onstatechange` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FetchObserver/onstatechange)\n\n*This API requires the following crate features to be activated: `FetchObserver`*"]
    pub fn set_onstatechange(this: &FetchObserver, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = onrequestprogress ) ]
    #[doc = "Getter for the `onrequestprogress` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FetchObserver/onrequestprogress)\n\n*This API requires the following crate features to be activated: `FetchObserver`*"]
    pub fn onrequestprogress(this: &FetchObserver) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onrequestprogress ) ]
    #[doc = "Setter for the `onrequestprogress` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FetchObserver/onrequestprogress)\n\n*This API requires the following crate features to be activated: `FetchObserver`*"]
    pub fn set_onrequestprogress(this: &FetchObserver, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = onresponseprogress ) ]
    #[doc = "Getter for the `onresponseprogress` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FetchObserver/onresponseprogress)\n\n*This API requires the following crate features to be activated: `FetchObserver`*"]
    pub fn onresponseprogress(this: &FetchObserver) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onresponseprogress ) ]
    #[doc = "Setter for the `onresponseprogress` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FetchObserver/onresponseprogress)\n\n*This API requires the following crate features to be activated: `FetchObserver`*"]
    pub fn set_onresponseprogress(this: &FetchObserver, value: Option<&::js_sys::Function>);
}
