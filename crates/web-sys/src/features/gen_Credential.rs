use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = Credential , typescript_type = "Credential" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `Credential` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Credential)
    ///
    ///*This API requires the following crate features to be activated: `Credential`*
    pub type Credential;

    # [ wasm_bindgen ( structural , method , getter , js_class = "Credential" , js_name = id ) ]
    ///Getter for the `id` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Credential/id)
    ///
    ///*This API requires the following crate features to be activated: `Credential`*
    pub fn id(this: &Credential) -> String;

    # [ wasm_bindgen ( structural , method , getter , js_class = "Credential" , js_name = type ) ]
    ///Getter for the `type` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Credential/type)
    ///
    ///*This API requires the following crate features to be activated: `Credential`*
    pub fn type_(this: &Credential) -> String;

}
