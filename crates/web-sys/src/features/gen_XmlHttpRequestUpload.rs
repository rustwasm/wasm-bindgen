use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = XmlHttpRequestEventTarget , extends = EventTarget , extends = :: js_sys :: Object , js_name = XMLHttpRequestUpload , typescript_name = XMLHttpRequestUpload ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `XmlHttpRequestUpload` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequestUpload)\n\n*This API requires the following crate features to be activated: `XmlHttpRequestUpload`*"]
    pub type XmlHttpRequestUpload;
}
