use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = EventTarget , extends = :: js_sys :: Object , js_name = PresentationConnectionList , typescript_name = PresentationConnectionList ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `PresentationConnectionList` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationConnectionList)
    ///
    ///*This API requires the following crate features to be activated: `PresentationConnectionList`*
    pub type PresentationConnectionList;

    # [ wasm_bindgen ( structural , method , getter , js_class = "PresentationConnectionList" , js_name = connections ) ]
    ///Getter for the `connections` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationConnectionList/connections)
    ///
    ///*This API requires the following crate features to be activated: `PresentationConnectionList`*
    pub fn connections(this: &PresentationConnectionList) -> ::js_sys::Array;

    # [ wasm_bindgen ( structural , method , getter , js_class = "PresentationConnectionList" , js_name = onconnectionavailable ) ]
    ///Getter for the `onconnectionavailable` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationConnectionList/onconnectionavailable)
    ///
    ///*This API requires the following crate features to be activated: `PresentationConnectionList`*
    pub fn onconnectionavailable(this: &PresentationConnectionList) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "PresentationConnectionList" , js_name = onconnectionavailable ) ]
    ///Setter for the `onconnectionavailable` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationConnectionList/onconnectionavailable)
    ///
    ///*This API requires the following crate features to be activated: `PresentationConnectionList`*
    pub fn set_onconnectionavailable(
        this: &PresentationConnectionList,
        value: Option<&::js_sys::Function>,
    );

}
