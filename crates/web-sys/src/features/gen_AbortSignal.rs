use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = EventTarget , extends = :: js_sys :: Object , js_name = AbortSignal , typescript_type = "AbortSignal" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `AbortSignal` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AbortSignal)
    ///
    ///*This API requires the following crate features to be activated: `AbortSignal`*
    pub type AbortSignal;

    # [ wasm_bindgen ( structural , method , getter , js_class = "AbortSignal" , js_name = aborted ) ]
    ///Getter for the `aborted` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AbortSignal/aborted)
    ///
    ///*This API requires the following crate features to be activated: `AbortSignal`*
    pub fn aborted(this: &AbortSignal) -> bool;

    # [ wasm_bindgen ( structural , method , getter , js_class = "AbortSignal" , js_name = onabort ) ]
    ///Getter for the `onabort` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AbortSignal/onabort)
    ///
    ///*This API requires the following crate features to be activated: `AbortSignal`*
    pub fn onabort(this: &AbortSignal) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "AbortSignal" , js_name = onabort ) ]
    ///Setter for the `onabort` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AbortSignal/onabort)
    ///
    ///*This API requires the following crate features to be activated: `AbortSignal`*
    pub fn set_onabort(this: &AbortSignal, value: Option<&::js_sys::Function>);

}
