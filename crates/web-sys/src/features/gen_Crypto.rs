use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = Crypto , typescript_name = Crypto ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `Crypto` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Crypto)\n\n*This API requires the following crate features to be activated: `Crypto`*"]
    pub type Crypto;
    # [ wasm_bindgen ( structural , method , getter , js_name = subtle ) ]
    #[cfg(feature = "SubtleCrypto")]
    #[doc = "Getter for the `subtle` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Crypto/subtle)\n\n*This API requires the following crate features to be activated: `Crypto`, `SubtleCrypto`*"]
    pub fn subtle(this: &Crypto) -> SubtleCrypto;
    # [ wasm_bindgen ( catch , method , structural , js_name = getRandomValues ) ]
    #[doc = "The `getRandomValues()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Crypto/getRandomValues)\n\n*This API requires the following crate features to be activated: `Crypto`*"]
    pub fn get_random_values_with_array_buffer_view(
        this: &Crypto,
        array: &::js_sys::Object,
    ) -> Result<::js_sys::Object, JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = getRandomValues ) ]
    #[doc = "The `getRandomValues()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Crypto/getRandomValues)\n\n*This API requires the following crate features to be activated: `Crypto`*"]
    pub fn get_random_values_with_u8_array(
        this: &Crypto,
        array: &mut [u8],
    ) -> Result<::js_sys::Object, JsValue>;
}
