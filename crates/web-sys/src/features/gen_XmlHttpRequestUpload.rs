use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = XmlHttpRequestEventTarget , extends = EventTarget , extends = :: js_sys :: Object , js_name = XMLHttpRequestUpload , typescript_name = XMLHttpRequestUpload ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `XmlHttpRequestUpload` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequestUpload)
    ///
    ///*This API requires the following crate features to be activated: `XmlHttpRequestUpload`*
    pub type XmlHttpRequestUpload;

}
