use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = SubtleCrypto , typescript_name = SubtleCrypto ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `SubtleCrypto` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SubtleCrypto)
    ///
    ///*This API requires the following crate features to be activated: `SubtleCrypto`*
    pub type SubtleCrypto;

    #[cfg(feature = "CryptoKey")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "SubtleCrypto" , js_name = decrypt ) ]
    ///The `decrypt()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SubtleCrypto/decrypt)
    ///
    ///*This API requires the following crate features to be activated: `CryptoKey`, `SubtleCrypto`*
    pub fn decrypt_with_object_and_buffer_source(
        this: &SubtleCrypto,
        algorithm: &::js_sys::Object,
        key: &CryptoKey,
        data: &::js_sys::Object,
    ) -> Result<::js_sys::Promise, JsValue>;

    #[cfg(feature = "CryptoKey")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "SubtleCrypto" , js_name = decrypt ) ]
    ///The `decrypt()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SubtleCrypto/decrypt)
    ///
    ///*This API requires the following crate features to be activated: `CryptoKey`, `SubtleCrypto`*
    pub fn decrypt_with_str_and_buffer_source(
        this: &SubtleCrypto,
        algorithm: &str,
        key: &CryptoKey,
        data: &::js_sys::Object,
    ) -> Result<::js_sys::Promise, JsValue>;

    #[cfg(feature = "CryptoKey")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "SubtleCrypto" , js_name = decrypt ) ]
    ///The `decrypt()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SubtleCrypto/decrypt)
    ///
    ///*This API requires the following crate features to be activated: `CryptoKey`, `SubtleCrypto`*
    pub fn decrypt_with_object_and_u8_array(
        this: &SubtleCrypto,
        algorithm: &::js_sys::Object,
        key: &CryptoKey,
        data: &mut [u8],
    ) -> Result<::js_sys::Promise, JsValue>;

    #[cfg(feature = "CryptoKey")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "SubtleCrypto" , js_name = decrypt ) ]
    ///The `decrypt()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SubtleCrypto/decrypt)
    ///
    ///*This API requires the following crate features to be activated: `CryptoKey`, `SubtleCrypto`*
    pub fn decrypt_with_str_and_u8_array(
        this: &SubtleCrypto,
        algorithm: &str,
        key: &CryptoKey,
        data: &mut [u8],
    ) -> Result<::js_sys::Promise, JsValue>;

    #[cfg(feature = "CryptoKey")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "SubtleCrypto" , js_name = deriveBits ) ]
    ///The `deriveBits()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SubtleCrypto/deriveBits)
    ///
    ///*This API requires the following crate features to be activated: `CryptoKey`, `SubtleCrypto`*
    pub fn derive_bits_with_object(
        this: &SubtleCrypto,
        algorithm: &::js_sys::Object,
        base_key: &CryptoKey,
        length: u32,
    ) -> Result<::js_sys::Promise, JsValue>;

    #[cfg(feature = "CryptoKey")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "SubtleCrypto" , js_name = deriveBits ) ]
    ///The `deriveBits()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SubtleCrypto/deriveBits)
    ///
    ///*This API requires the following crate features to be activated: `CryptoKey`, `SubtleCrypto`*
    pub fn derive_bits_with_str(
        this: &SubtleCrypto,
        algorithm: &str,
        base_key: &CryptoKey,
        length: u32,
    ) -> Result<::js_sys::Promise, JsValue>;

    #[cfg(feature = "CryptoKey")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "SubtleCrypto" , js_name = deriveKey ) ]
    ///The `deriveKey()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SubtleCrypto/deriveKey)
    ///
    ///*This API requires the following crate features to be activated: `CryptoKey`, `SubtleCrypto`*
    pub fn derive_key_with_object_and_object(
        this: &SubtleCrypto,
        algorithm: &::js_sys::Object,
        base_key: &CryptoKey,
        derived_key_type: &::js_sys::Object,
        extractable: bool,
        key_usages: &::wasm_bindgen::JsValue,
    ) -> Result<::js_sys::Promise, JsValue>;

    #[cfg(feature = "CryptoKey")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "SubtleCrypto" , js_name = deriveKey ) ]
    ///The `deriveKey()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SubtleCrypto/deriveKey)
    ///
    ///*This API requires the following crate features to be activated: `CryptoKey`, `SubtleCrypto`*
    pub fn derive_key_with_str_and_object(
        this: &SubtleCrypto,
        algorithm: &str,
        base_key: &CryptoKey,
        derived_key_type: &::js_sys::Object,
        extractable: bool,
        key_usages: &::wasm_bindgen::JsValue,
    ) -> Result<::js_sys::Promise, JsValue>;

    #[cfg(feature = "CryptoKey")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "SubtleCrypto" , js_name = deriveKey ) ]
    ///The `deriveKey()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SubtleCrypto/deriveKey)
    ///
    ///*This API requires the following crate features to be activated: `CryptoKey`, `SubtleCrypto`*
    pub fn derive_key_with_object_and_str(
        this: &SubtleCrypto,
        algorithm: &::js_sys::Object,
        base_key: &CryptoKey,
        derived_key_type: &str,
        extractable: bool,
        key_usages: &::wasm_bindgen::JsValue,
    ) -> Result<::js_sys::Promise, JsValue>;

    #[cfg(feature = "CryptoKey")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "SubtleCrypto" , js_name = deriveKey ) ]
    ///The `deriveKey()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SubtleCrypto/deriveKey)
    ///
    ///*This API requires the following crate features to be activated: `CryptoKey`, `SubtleCrypto`*
    pub fn derive_key_with_str_and_str(
        this: &SubtleCrypto,
        algorithm: &str,
        base_key: &CryptoKey,
        derived_key_type: &str,
        extractable: bool,
        key_usages: &::wasm_bindgen::JsValue,
    ) -> Result<::js_sys::Promise, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "SubtleCrypto" , js_name = digest ) ]
    ///The `digest()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SubtleCrypto/digest)
    ///
    ///*This API requires the following crate features to be activated: `SubtleCrypto`*
    pub fn digest_with_object_and_buffer_source(
        this: &SubtleCrypto,
        algorithm: &::js_sys::Object,
        data: &::js_sys::Object,
    ) -> Result<::js_sys::Promise, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "SubtleCrypto" , js_name = digest ) ]
    ///The `digest()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SubtleCrypto/digest)
    ///
    ///*This API requires the following crate features to be activated: `SubtleCrypto`*
    pub fn digest_with_str_and_buffer_source(
        this: &SubtleCrypto,
        algorithm: &str,
        data: &::js_sys::Object,
    ) -> Result<::js_sys::Promise, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "SubtleCrypto" , js_name = digest ) ]
    ///The `digest()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SubtleCrypto/digest)
    ///
    ///*This API requires the following crate features to be activated: `SubtleCrypto`*
    pub fn digest_with_object_and_u8_array(
        this: &SubtleCrypto,
        algorithm: &::js_sys::Object,
        data: &mut [u8],
    ) -> Result<::js_sys::Promise, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "SubtleCrypto" , js_name = digest ) ]
    ///The `digest()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SubtleCrypto/digest)
    ///
    ///*This API requires the following crate features to be activated: `SubtleCrypto`*
    pub fn digest_with_str_and_u8_array(
        this: &SubtleCrypto,
        algorithm: &str,
        data: &mut [u8],
    ) -> Result<::js_sys::Promise, JsValue>;

    #[cfg(feature = "CryptoKey")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "SubtleCrypto" , js_name = encrypt ) ]
    ///The `encrypt()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SubtleCrypto/encrypt)
    ///
    ///*This API requires the following crate features to be activated: `CryptoKey`, `SubtleCrypto`*
    pub fn encrypt_with_object_and_buffer_source(
        this: &SubtleCrypto,
        algorithm: &::js_sys::Object,
        key: &CryptoKey,
        data: &::js_sys::Object,
    ) -> Result<::js_sys::Promise, JsValue>;

    #[cfg(feature = "CryptoKey")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "SubtleCrypto" , js_name = encrypt ) ]
    ///The `encrypt()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SubtleCrypto/encrypt)
    ///
    ///*This API requires the following crate features to be activated: `CryptoKey`, `SubtleCrypto`*
    pub fn encrypt_with_str_and_buffer_source(
        this: &SubtleCrypto,
        algorithm: &str,
        key: &CryptoKey,
        data: &::js_sys::Object,
    ) -> Result<::js_sys::Promise, JsValue>;

    #[cfg(feature = "CryptoKey")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "SubtleCrypto" , js_name = encrypt ) ]
    ///The `encrypt()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SubtleCrypto/encrypt)
    ///
    ///*This API requires the following crate features to be activated: `CryptoKey`, `SubtleCrypto`*
    pub fn encrypt_with_object_and_u8_array(
        this: &SubtleCrypto,
        algorithm: &::js_sys::Object,
        key: &CryptoKey,
        data: &mut [u8],
    ) -> Result<::js_sys::Promise, JsValue>;

    #[cfg(feature = "CryptoKey")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "SubtleCrypto" , js_name = encrypt ) ]
    ///The `encrypt()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SubtleCrypto/encrypt)
    ///
    ///*This API requires the following crate features to be activated: `CryptoKey`, `SubtleCrypto`*
    pub fn encrypt_with_str_and_u8_array(
        this: &SubtleCrypto,
        algorithm: &str,
        key: &CryptoKey,
        data: &mut [u8],
    ) -> Result<::js_sys::Promise, JsValue>;

    #[cfg(feature = "CryptoKey")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "SubtleCrypto" , js_name = exportKey ) ]
    ///The `exportKey()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SubtleCrypto/exportKey)
    ///
    ///*This API requires the following crate features to be activated: `CryptoKey`, `SubtleCrypto`*
    pub fn export_key(
        this: &SubtleCrypto,
        format: &str,
        key: &CryptoKey,
    ) -> Result<::js_sys::Promise, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "SubtleCrypto" , js_name = generateKey ) ]
    ///The `generateKey()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SubtleCrypto/generateKey)
    ///
    ///*This API requires the following crate features to be activated: `SubtleCrypto`*
    pub fn generate_key_with_object(
        this: &SubtleCrypto,
        algorithm: &::js_sys::Object,
        extractable: bool,
        key_usages: &::wasm_bindgen::JsValue,
    ) -> Result<::js_sys::Promise, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "SubtleCrypto" , js_name = generateKey ) ]
    ///The `generateKey()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SubtleCrypto/generateKey)
    ///
    ///*This API requires the following crate features to be activated: `SubtleCrypto`*
    pub fn generate_key_with_str(
        this: &SubtleCrypto,
        algorithm: &str,
        extractable: bool,
        key_usages: &::wasm_bindgen::JsValue,
    ) -> Result<::js_sys::Promise, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "SubtleCrypto" , js_name = importKey ) ]
    ///The `importKey()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SubtleCrypto/importKey)
    ///
    ///*This API requires the following crate features to be activated: `SubtleCrypto`*
    pub fn import_key_with_object(
        this: &SubtleCrypto,
        format: &str,
        key_data: &::js_sys::Object,
        algorithm: &::js_sys::Object,
        extractable: bool,
        key_usages: &::wasm_bindgen::JsValue,
    ) -> Result<::js_sys::Promise, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "SubtleCrypto" , js_name = importKey ) ]
    ///The `importKey()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SubtleCrypto/importKey)
    ///
    ///*This API requires the following crate features to be activated: `SubtleCrypto`*
    pub fn import_key_with_str(
        this: &SubtleCrypto,
        format: &str,
        key_data: &::js_sys::Object,
        algorithm: &str,
        extractable: bool,
        key_usages: &::wasm_bindgen::JsValue,
    ) -> Result<::js_sys::Promise, JsValue>;

    #[cfg(feature = "CryptoKey")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "SubtleCrypto" , js_name = sign ) ]
    ///The `sign()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SubtleCrypto/sign)
    ///
    ///*This API requires the following crate features to be activated: `CryptoKey`, `SubtleCrypto`*
    pub fn sign_with_object_and_buffer_source(
        this: &SubtleCrypto,
        algorithm: &::js_sys::Object,
        key: &CryptoKey,
        data: &::js_sys::Object,
    ) -> Result<::js_sys::Promise, JsValue>;

    #[cfg(feature = "CryptoKey")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "SubtleCrypto" , js_name = sign ) ]
    ///The `sign()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SubtleCrypto/sign)
    ///
    ///*This API requires the following crate features to be activated: `CryptoKey`, `SubtleCrypto`*
    pub fn sign_with_str_and_buffer_source(
        this: &SubtleCrypto,
        algorithm: &str,
        key: &CryptoKey,
        data: &::js_sys::Object,
    ) -> Result<::js_sys::Promise, JsValue>;

    #[cfg(feature = "CryptoKey")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "SubtleCrypto" , js_name = sign ) ]
    ///The `sign()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SubtleCrypto/sign)
    ///
    ///*This API requires the following crate features to be activated: `CryptoKey`, `SubtleCrypto`*
    pub fn sign_with_object_and_u8_array(
        this: &SubtleCrypto,
        algorithm: &::js_sys::Object,
        key: &CryptoKey,
        data: &mut [u8],
    ) -> Result<::js_sys::Promise, JsValue>;

    #[cfg(feature = "CryptoKey")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "SubtleCrypto" , js_name = sign ) ]
    ///The `sign()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SubtleCrypto/sign)
    ///
    ///*This API requires the following crate features to be activated: `CryptoKey`, `SubtleCrypto`*
    pub fn sign_with_str_and_u8_array(
        this: &SubtleCrypto,
        algorithm: &str,
        key: &CryptoKey,
        data: &mut [u8],
    ) -> Result<::js_sys::Promise, JsValue>;

    #[cfg(feature = "CryptoKey")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "SubtleCrypto" , js_name = unwrapKey ) ]
    ///The `unwrapKey()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SubtleCrypto/unwrapKey)
    ///
    ///*This API requires the following crate features to be activated: `CryptoKey`, `SubtleCrypto`*
    pub fn unwrap_key_with_buffer_source_and_object_and_object(
        this: &SubtleCrypto,
        format: &str,
        wrapped_key: &::js_sys::Object,
        unwrapping_key: &CryptoKey,
        unwrap_algorithm: &::js_sys::Object,
        unwrapped_key_algorithm: &::js_sys::Object,
        extractable: bool,
        key_usages: &::wasm_bindgen::JsValue,
    ) -> Result<::js_sys::Promise, JsValue>;

    #[cfg(feature = "CryptoKey")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "SubtleCrypto" , js_name = unwrapKey ) ]
    ///The `unwrapKey()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SubtleCrypto/unwrapKey)
    ///
    ///*This API requires the following crate features to be activated: `CryptoKey`, `SubtleCrypto`*
    pub fn unwrap_key_with_u8_array_and_object_and_object(
        this: &SubtleCrypto,
        format: &str,
        wrapped_key: &mut [u8],
        unwrapping_key: &CryptoKey,
        unwrap_algorithm: &::js_sys::Object,
        unwrapped_key_algorithm: &::js_sys::Object,
        extractable: bool,
        key_usages: &::wasm_bindgen::JsValue,
    ) -> Result<::js_sys::Promise, JsValue>;

    #[cfg(feature = "CryptoKey")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "SubtleCrypto" , js_name = unwrapKey ) ]
    ///The `unwrapKey()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SubtleCrypto/unwrapKey)
    ///
    ///*This API requires the following crate features to be activated: `CryptoKey`, `SubtleCrypto`*
    pub fn unwrap_key_with_buffer_source_and_str_and_object(
        this: &SubtleCrypto,
        format: &str,
        wrapped_key: &::js_sys::Object,
        unwrapping_key: &CryptoKey,
        unwrap_algorithm: &str,
        unwrapped_key_algorithm: &::js_sys::Object,
        extractable: bool,
        key_usages: &::wasm_bindgen::JsValue,
    ) -> Result<::js_sys::Promise, JsValue>;

    #[cfg(feature = "CryptoKey")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "SubtleCrypto" , js_name = unwrapKey ) ]
    ///The `unwrapKey()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SubtleCrypto/unwrapKey)
    ///
    ///*This API requires the following crate features to be activated: `CryptoKey`, `SubtleCrypto`*
    pub fn unwrap_key_with_u8_array_and_str_and_object(
        this: &SubtleCrypto,
        format: &str,
        wrapped_key: &mut [u8],
        unwrapping_key: &CryptoKey,
        unwrap_algorithm: &str,
        unwrapped_key_algorithm: &::js_sys::Object,
        extractable: bool,
        key_usages: &::wasm_bindgen::JsValue,
    ) -> Result<::js_sys::Promise, JsValue>;

    #[cfg(feature = "CryptoKey")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "SubtleCrypto" , js_name = unwrapKey ) ]
    ///The `unwrapKey()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SubtleCrypto/unwrapKey)
    ///
    ///*This API requires the following crate features to be activated: `CryptoKey`, `SubtleCrypto`*
    pub fn unwrap_key_with_buffer_source_and_object_and_str(
        this: &SubtleCrypto,
        format: &str,
        wrapped_key: &::js_sys::Object,
        unwrapping_key: &CryptoKey,
        unwrap_algorithm: &::js_sys::Object,
        unwrapped_key_algorithm: &str,
        extractable: bool,
        key_usages: &::wasm_bindgen::JsValue,
    ) -> Result<::js_sys::Promise, JsValue>;

    #[cfg(feature = "CryptoKey")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "SubtleCrypto" , js_name = unwrapKey ) ]
    ///The `unwrapKey()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SubtleCrypto/unwrapKey)
    ///
    ///*This API requires the following crate features to be activated: `CryptoKey`, `SubtleCrypto`*
    pub fn unwrap_key_with_u8_array_and_object_and_str(
        this: &SubtleCrypto,
        format: &str,
        wrapped_key: &mut [u8],
        unwrapping_key: &CryptoKey,
        unwrap_algorithm: &::js_sys::Object,
        unwrapped_key_algorithm: &str,
        extractable: bool,
        key_usages: &::wasm_bindgen::JsValue,
    ) -> Result<::js_sys::Promise, JsValue>;

    #[cfg(feature = "CryptoKey")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "SubtleCrypto" , js_name = unwrapKey ) ]
    ///The `unwrapKey()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SubtleCrypto/unwrapKey)
    ///
    ///*This API requires the following crate features to be activated: `CryptoKey`, `SubtleCrypto`*
    pub fn unwrap_key_with_buffer_source_and_str_and_str(
        this: &SubtleCrypto,
        format: &str,
        wrapped_key: &::js_sys::Object,
        unwrapping_key: &CryptoKey,
        unwrap_algorithm: &str,
        unwrapped_key_algorithm: &str,
        extractable: bool,
        key_usages: &::wasm_bindgen::JsValue,
    ) -> Result<::js_sys::Promise, JsValue>;

    #[cfg(feature = "CryptoKey")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "SubtleCrypto" , js_name = unwrapKey ) ]
    ///The `unwrapKey()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SubtleCrypto/unwrapKey)
    ///
    ///*This API requires the following crate features to be activated: `CryptoKey`, `SubtleCrypto`*
    pub fn unwrap_key_with_u8_array_and_str_and_str(
        this: &SubtleCrypto,
        format: &str,
        wrapped_key: &mut [u8],
        unwrapping_key: &CryptoKey,
        unwrap_algorithm: &str,
        unwrapped_key_algorithm: &str,
        extractable: bool,
        key_usages: &::wasm_bindgen::JsValue,
    ) -> Result<::js_sys::Promise, JsValue>;

    #[cfg(feature = "CryptoKey")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "SubtleCrypto" , js_name = verify ) ]
    ///The `verify()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SubtleCrypto/verify)
    ///
    ///*This API requires the following crate features to be activated: `CryptoKey`, `SubtleCrypto`*
    pub fn verify_with_object_and_buffer_source_and_buffer_source(
        this: &SubtleCrypto,
        algorithm: &::js_sys::Object,
        key: &CryptoKey,
        signature: &::js_sys::Object,
        data: &::js_sys::Object,
    ) -> Result<::js_sys::Promise, JsValue>;

    #[cfg(feature = "CryptoKey")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "SubtleCrypto" , js_name = verify ) ]
    ///The `verify()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SubtleCrypto/verify)
    ///
    ///*This API requires the following crate features to be activated: `CryptoKey`, `SubtleCrypto`*
    pub fn verify_with_str_and_buffer_source_and_buffer_source(
        this: &SubtleCrypto,
        algorithm: &str,
        key: &CryptoKey,
        signature: &::js_sys::Object,
        data: &::js_sys::Object,
    ) -> Result<::js_sys::Promise, JsValue>;

    #[cfg(feature = "CryptoKey")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "SubtleCrypto" , js_name = verify ) ]
    ///The `verify()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SubtleCrypto/verify)
    ///
    ///*This API requires the following crate features to be activated: `CryptoKey`, `SubtleCrypto`*
    pub fn verify_with_object_and_u8_array_and_buffer_source(
        this: &SubtleCrypto,
        algorithm: &::js_sys::Object,
        key: &CryptoKey,
        signature: &mut [u8],
        data: &::js_sys::Object,
    ) -> Result<::js_sys::Promise, JsValue>;

    #[cfg(feature = "CryptoKey")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "SubtleCrypto" , js_name = verify ) ]
    ///The `verify()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SubtleCrypto/verify)
    ///
    ///*This API requires the following crate features to be activated: `CryptoKey`, `SubtleCrypto`*
    pub fn verify_with_str_and_u8_array_and_buffer_source(
        this: &SubtleCrypto,
        algorithm: &str,
        key: &CryptoKey,
        signature: &mut [u8],
        data: &::js_sys::Object,
    ) -> Result<::js_sys::Promise, JsValue>;

    #[cfg(feature = "CryptoKey")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "SubtleCrypto" , js_name = verify ) ]
    ///The `verify()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SubtleCrypto/verify)
    ///
    ///*This API requires the following crate features to be activated: `CryptoKey`, `SubtleCrypto`*
    pub fn verify_with_object_and_buffer_source_and_u8_array(
        this: &SubtleCrypto,
        algorithm: &::js_sys::Object,
        key: &CryptoKey,
        signature: &::js_sys::Object,
        data: &mut [u8],
    ) -> Result<::js_sys::Promise, JsValue>;

    #[cfg(feature = "CryptoKey")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "SubtleCrypto" , js_name = verify ) ]
    ///The `verify()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SubtleCrypto/verify)
    ///
    ///*This API requires the following crate features to be activated: `CryptoKey`, `SubtleCrypto`*
    pub fn verify_with_str_and_buffer_source_and_u8_array(
        this: &SubtleCrypto,
        algorithm: &str,
        key: &CryptoKey,
        signature: &::js_sys::Object,
        data: &mut [u8],
    ) -> Result<::js_sys::Promise, JsValue>;

    #[cfg(feature = "CryptoKey")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "SubtleCrypto" , js_name = verify ) ]
    ///The `verify()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SubtleCrypto/verify)
    ///
    ///*This API requires the following crate features to be activated: `CryptoKey`, `SubtleCrypto`*
    pub fn verify_with_object_and_u8_array_and_u8_array(
        this: &SubtleCrypto,
        algorithm: &::js_sys::Object,
        key: &CryptoKey,
        signature: &mut [u8],
        data: &mut [u8],
    ) -> Result<::js_sys::Promise, JsValue>;

    #[cfg(feature = "CryptoKey")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "SubtleCrypto" , js_name = verify ) ]
    ///The `verify()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SubtleCrypto/verify)
    ///
    ///*This API requires the following crate features to be activated: `CryptoKey`, `SubtleCrypto`*
    pub fn verify_with_str_and_u8_array_and_u8_array(
        this: &SubtleCrypto,
        algorithm: &str,
        key: &CryptoKey,
        signature: &mut [u8],
        data: &mut [u8],
    ) -> Result<::js_sys::Promise, JsValue>;

    #[cfg(feature = "CryptoKey")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "SubtleCrypto" , js_name = wrapKey ) ]
    ///The `wrapKey()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SubtleCrypto/wrapKey)
    ///
    ///*This API requires the following crate features to be activated: `CryptoKey`, `SubtleCrypto`*
    pub fn wrap_key_with_object(
        this: &SubtleCrypto,
        format: &str,
        key: &CryptoKey,
        wrapping_key: &CryptoKey,
        wrap_algorithm: &::js_sys::Object,
    ) -> Result<::js_sys::Promise, JsValue>;

    #[cfg(feature = "CryptoKey")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "SubtleCrypto" , js_name = wrapKey ) ]
    ///The `wrapKey()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SubtleCrypto/wrapKey)
    ///
    ///*This API requires the following crate features to be activated: `CryptoKey`, `SubtleCrypto`*
    pub fn wrap_key_with_str(
        this: &SubtleCrypto,
        format: &str,
        key: &CryptoKey,
        wrapping_key: &CryptoKey,
        wrap_algorithm: &str,
    ) -> Result<::js_sys::Promise, JsValue>;

}
