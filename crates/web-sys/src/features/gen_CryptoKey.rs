use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = CryptoKey , typescript_name = CryptoKey ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `CryptoKey` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CryptoKey)
    ///
    ///*This API requires the following crate features to be activated: `CryptoKey`*
    pub type CryptoKey;

    # [ wasm_bindgen ( structural , method , getter , js_class = "CryptoKey" , js_name = type ) ]
    ///Getter for the `type` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CryptoKey/type)
    ///
    ///*This API requires the following crate features to be activated: `CryptoKey`*
    pub fn type_(this: &CryptoKey) -> String;

    # [ wasm_bindgen ( structural , method , getter , js_class = "CryptoKey" , js_name = extractable ) ]
    ///Getter for the `extractable` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CryptoKey/extractable)
    ///
    ///*This API requires the following crate features to be activated: `CryptoKey`*
    pub fn extractable(this: &CryptoKey) -> bool;

    # [ wasm_bindgen ( structural , catch , method , getter , js_class = "CryptoKey" , js_name = algorithm ) ]
    ///Getter for the `algorithm` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CryptoKey/algorithm)
    ///
    ///*This API requires the following crate features to be activated: `CryptoKey`*
    pub fn algorithm(this: &CryptoKey) -> Result<::js_sys::Object, JsValue>;

    # [ wasm_bindgen ( structural , method , getter , js_class = "CryptoKey" , js_name = usages ) ]
    ///Getter for the `usages` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CryptoKey/usages)
    ///
    ///*This API requires the following crate features to be activated: `CryptoKey`*
    pub fn usages(this: &CryptoKey) -> ::js_sys::Array;

}
