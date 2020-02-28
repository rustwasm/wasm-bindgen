use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = CSSRule , typescript_name = CSSRule ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `CssRule` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSRule)\n\n*This API requires the following crate features to be activated: `CssRule`*"]
    pub type CssRule;
    # [ wasm_bindgen ( structural , method , getter , js_name = type ) ]
    #[doc = "Getter for the `type` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSRule/type)\n\n*This API requires the following crate features to be activated: `CssRule`*"]
    pub fn type_(this: &CssRule) -> u16;
    # [ wasm_bindgen ( structural , method , getter , js_name = cssText ) ]
    #[doc = "Getter for the `cssText` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSRule/cssText)\n\n*This API requires the following crate features to be activated: `CssRule`*"]
    pub fn css_text(this: &CssRule) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = cssText ) ]
    #[doc = "Setter for the `cssText` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSRule/cssText)\n\n*This API requires the following crate features to be activated: `CssRule`*"]
    pub fn set_css_text(this: &CssRule, value: String);
    # [ wasm_bindgen ( structural , method , getter , js_name = parentRule ) ]
    #[doc = "Getter for the `parentRule` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSRule/parentRule)\n\n*This API requires the following crate features to be activated: `CssRule`*"]
    pub fn parent_rule(this: &CssRule) -> Option<CssRule>;
    # [ wasm_bindgen ( structural , method , getter , js_name = parentStyleSheet ) ]
    #[cfg(feature = "CssStyleSheet")]
    #[doc = "Getter for the `parentStyleSheet` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSRule/parentStyleSheet)\n\n*This API requires the following crate features to be activated: `CssRule`, `CssStyleSheet`*"]
    pub fn parent_style_sheet(this: &CssRule) -> Option<CssStyleSheet>;
}
impl CssRule {
    pub const STYLE_RULE: u16 = 1u64 as u16;
    pub const CHARSET_RULE: u16 = 2u64 as u16;
    pub const IMPORT_RULE: u16 = 3u64 as u16;
    pub const MEDIA_RULE: u16 = 4u64 as u16;
    pub const FONT_FACE_RULE: u16 = 5u64 as u16;
    pub const PAGE_RULE: u16 = 6u64 as u16;
    pub const NAMESPACE_RULE: u16 = 10u64 as u16;
    pub const KEYFRAMES_RULE: u16 = 7u64 as u16;
    pub const KEYFRAME_RULE: u16 = 8u64 as u16;
    pub const COUNTER_STYLE_RULE: u16 = 11u64 as u16;
    pub const SUPPORTS_RULE: u16 = 12u64 as u16;
    pub const FONT_FEATURE_VALUES_RULE: u16 = 14u64 as u16;
}
