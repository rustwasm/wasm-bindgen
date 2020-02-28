use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = IdbKeyRange , extends = :: js_sys :: Object , js_name = IDBLocaleAwareKeyRange , typescript_name = IDBLocaleAwareKeyRange ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `IdbLocaleAwareKeyRange` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBLocaleAwareKeyRange)\n\n*This API requires the following crate features to be activated: `IdbLocaleAwareKeyRange`*"]
    pub type IdbLocaleAwareKeyRange;
    # [ wasm_bindgen ( catch , static_method_of = IdbLocaleAwareKeyRange , js_class = "IDBLocaleAwareKeyRange" , js_name = bound ) ]
    #[doc = "The `bound()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBLocaleAwareKeyRange/bound)\n\n*This API requires the following crate features to be activated: `IdbLocaleAwareKeyRange`*"]
    pub fn bound(
        lower: &::wasm_bindgen::JsValue,
        upper: &::wasm_bindgen::JsValue,
    ) -> Result<IdbLocaleAwareKeyRange, JsValue>;
    # [ wasm_bindgen ( catch , static_method_of = IdbLocaleAwareKeyRange , js_class = "IDBLocaleAwareKeyRange" , js_name = bound ) ]
    #[doc = "The `bound()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBLocaleAwareKeyRange/bound)\n\n*This API requires the following crate features to be activated: `IdbLocaleAwareKeyRange`*"]
    pub fn bound_with_lower_open(
        lower: &::wasm_bindgen::JsValue,
        upper: &::wasm_bindgen::JsValue,
        lower_open: bool,
    ) -> Result<IdbLocaleAwareKeyRange, JsValue>;
    # [ wasm_bindgen ( catch , static_method_of = IdbLocaleAwareKeyRange , js_class = "IDBLocaleAwareKeyRange" , js_name = bound ) ]
    #[doc = "The `bound()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBLocaleAwareKeyRange/bound)\n\n*This API requires the following crate features to be activated: `IdbLocaleAwareKeyRange`*"]
    pub fn bound_with_lower_open_and_upper_open(
        lower: &::wasm_bindgen::JsValue,
        upper: &::wasm_bindgen::JsValue,
        lower_open: bool,
        upper_open: bool,
    ) -> Result<IdbLocaleAwareKeyRange, JsValue>;
}
