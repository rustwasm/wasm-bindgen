use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( is_type_of = | _ | false , extends = :: js_sys :: Object , js_name = IntlUtils , typescript_name = IntlUtils ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `IntlUtils` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IntlUtils)\n\n*This API requires the following crate features to be activated: `IntlUtils`*"]
    pub type IntlUtils;
    #[cfg(feature = "DisplayNameResult")]
    # [ wasm_bindgen ( catch , method , structural , js_name = getDisplayNames ) ]
    #[doc = "The `getDisplayNames()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IntlUtils/getDisplayNames)\n\n*This API requires the following crate features to be activated: `DisplayNameResult`, `IntlUtils`*"]
    pub fn get_display_names(
        this: &IntlUtils,
        locales: &::wasm_bindgen::JsValue,
    ) -> Result<DisplayNameResult, JsValue>;
    #[cfg(all(feature = "DisplayNameOptions", feature = "DisplayNameResult",))]
    # [ wasm_bindgen ( catch , method , structural , js_name = getDisplayNames ) ]
    #[doc = "The `getDisplayNames()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IntlUtils/getDisplayNames)\n\n*This API requires the following crate features to be activated: `DisplayNameOptions`, `DisplayNameResult`, `IntlUtils`*"]
    pub fn get_display_names_with_options(
        this: &IntlUtils,
        locales: &::wasm_bindgen::JsValue,
        options: &DisplayNameOptions,
    ) -> Result<DisplayNameResult, JsValue>;
    #[cfg(feature = "LocaleInfo")]
    # [ wasm_bindgen ( catch , method , structural , js_name = getLocaleInfo ) ]
    #[doc = "The `getLocaleInfo()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IntlUtils/getLocaleInfo)\n\n*This API requires the following crate features to be activated: `IntlUtils`, `LocaleInfo`*"]
    pub fn get_locale_info(
        this: &IntlUtils,
        locales: &::wasm_bindgen::JsValue,
    ) -> Result<LocaleInfo, JsValue>;
}
