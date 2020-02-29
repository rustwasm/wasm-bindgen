use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = NodeList , extends = :: js_sys :: Object , js_name = RadioNodeList , typescript_type = "RadioNodeList" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `RadioNodeList` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RadioNodeList)
    ///
    ///*This API requires the following crate features to be activated: `RadioNodeList`*
    pub type RadioNodeList;

    # [ wasm_bindgen ( structural , method , getter , js_class = "RadioNodeList" , js_name = value ) ]
    ///Getter for the `value` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RadioNodeList/value)
    ///
    ///*This API requires the following crate features to be activated: `RadioNodeList`*
    pub fn value(this: &RadioNodeList) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "RadioNodeList" , js_name = value ) ]
    ///Setter for the `value` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RadioNodeList/value)
    ///
    ///*This API requires the following crate features to be activated: `RadioNodeList`*
    pub fn set_value(this: &RadioNodeList, value: &str);

}
