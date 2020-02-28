use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = Credential , typescript_name = Credential ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `Credential` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Credential)\n\n*This API requires the following crate features to be activated: `Credential`*"]
    pub type Credential;
    # [ wasm_bindgen ( structural , method , getter , js_class = "Credential" , js_name = id ) ]
    #[doc = "Getter for the `id` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Credential/id)\n\n*This API requires the following crate features to be activated: `Credential`*"]
    pub fn id(this: &Credential) -> String;
    # [ wasm_bindgen ( structural , method , getter , js_class = "Credential" , js_name = type ) ]
    #[doc = "Getter for the `type` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Credential/type)\n\n*This API requires the following crate features to be activated: `Credential`*"]
    pub fn type_(this: &Credential) -> String;
}
