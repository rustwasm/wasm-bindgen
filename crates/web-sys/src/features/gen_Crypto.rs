use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = Crypto , typescript_type = "Crypto" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `Crypto` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Crypto)
    ///
    ///*This API requires the following crate features to be activated: `Crypto`*
    pub type Crypto;

    #[cfg(feature = "SubtleCrypto")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "Crypto" , js_name = subtle ) ]
    ///Getter for the `subtle` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Crypto/subtle)
    ///
    ///*This API requires the following crate features to be activated: `Crypto`, `SubtleCrypto`*
    pub fn subtle(this: &Crypto) -> SubtleCrypto;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Crypto" , js_name = getRandomValues ) ]
    ///The `getRandomValues()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Crypto/getRandomValues)
    ///
    ///*This API requires the following crate features to be activated: `Crypto`*
    pub fn get_random_values_with_array_buffer_view(
        this: &Crypto,
        array: &::js_sys::Object,
    ) -> Result<::js_sys::Object, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Crypto" , js_name = getRandomValues ) ]
    ///The `getRandomValues()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Crypto/getRandomValues)
    ///
    ///*This API requires the following crate features to be activated: `Crypto`*
    pub fn get_random_values_with_u8_array(
        this: &Crypto,
        array: &mut [u8],
    ) -> Result<::js_sys::Object, JsValue>;

}
