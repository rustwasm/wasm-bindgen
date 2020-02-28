pub mod css {
    use super::super::*;
    use wasm_bindgen::prelude::*;
    #[wasm_bindgen]
    extern "C" {
        # [ wasm_bindgen ( js_namespace = CSS , js_name = escape ) ]
        #[doc = "The `CSS.escape()` function.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSS/escape)\n\n*This API requires the following crate features to be activated: `css`*"]
        pub fn escape(ident: &str) -> String;
        # [ wasm_bindgen ( catch , js_namespace = CSS , js_name = supports ) ]
        #[doc = "The `CSS.supports()` function.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSS/supports)\n\n*This API requires the following crate features to be activated: `css`*"]
        pub fn supports_with_value(property: &str, value: &str) -> Result<bool, JsValue>;
        # [ wasm_bindgen ( catch , js_namespace = CSS , js_name = supports ) ]
        #[doc = "The `CSS.supports()` function.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSS/supports)\n\n*This API requires the following crate features to be activated: `css`*"]
        pub fn supports(condition_text: &str) -> Result<bool, JsValue>;
    }
}
