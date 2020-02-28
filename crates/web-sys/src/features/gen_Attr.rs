use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = Attr , typescript_name = Attr ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `Attr` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Attr)\n\n*This API requires the following crate features to be activated: `Attr`*"]
    pub type Attr;
    # [ wasm_bindgen ( structural , method , getter , js_name = localName ) ]
    #[doc = "Getter for the `localName` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Attr/localName)\n\n*This API requires the following crate features to be activated: `Attr`*"]
    pub fn local_name(this: &Attr) -> String;
    # [ wasm_bindgen ( structural , method , getter , js_name = value ) ]
    #[doc = "Getter for the `value` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Attr/value)\n\n*This API requires the following crate features to be activated: `Attr`*"]
    pub fn value(this: &Attr) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = value ) ]
    #[doc = "Setter for the `value` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Attr/value)\n\n*This API requires the following crate features to be activated: `Attr`*"]
    pub fn set_value(this: &Attr, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_name = name ) ]
    #[doc = "Getter for the `name` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Attr/name)\n\n*This API requires the following crate features to be activated: `Attr`*"]
    pub fn name(this: &Attr) -> String;
    # [ wasm_bindgen ( structural , method , getter , js_name = namespaceURI ) ]
    #[doc = "Getter for the `namespaceURI` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Attr/namespaceURI)\n\n*This API requires the following crate features to be activated: `Attr`*"]
    pub fn namespace_uri(this: &Attr) -> Option<String>;
    # [ wasm_bindgen ( structural , method , getter , js_name = prefix ) ]
    #[doc = "Getter for the `prefix` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Attr/prefix)\n\n*This API requires the following crate features to be activated: `Attr`*"]
    pub fn prefix(this: &Attr) -> Option<String>;
    # [ wasm_bindgen ( structural , method , getter , js_name = specified ) ]
    #[doc = "Getter for the `specified` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Attr/specified)\n\n*This API requires the following crate features to be activated: `Attr`*"]
    pub fn specified(this: &Attr) -> bool;
}
