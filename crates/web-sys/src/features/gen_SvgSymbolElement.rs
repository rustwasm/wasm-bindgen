use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = SvgElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = SVGSymbolElement , typescript_name = SVGSymbolElement ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SvgSymbolElement` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGSymbolElement)\n\n*This API requires the following crate features to be activated: `SvgSymbolElement`*"]
    pub type SvgSymbolElement;
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGSymbolElement" , js_name = viewBox ) ]
    #[cfg(feature = "SvgAnimatedRect")]
    #[doc = "Getter for the `viewBox` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGSymbolElement/viewBox)\n\n*This API requires the following crate features to be activated: `SvgAnimatedRect`, `SvgSymbolElement`*"]
    pub fn view_box(this: &SvgSymbolElement) -> SvgAnimatedRect;
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGSymbolElement" , js_name = preserveAspectRatio ) ]
    #[cfg(feature = "SvgAnimatedPreserveAspectRatio")]
    #[doc = "Getter for the `preserveAspectRatio` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGSymbolElement/preserveAspectRatio)\n\n*This API requires the following crate features to be activated: `SvgAnimatedPreserveAspectRatio`, `SvgSymbolElement`*"]
    pub fn preserve_aspect_ratio(this: &SvgSymbolElement) -> SvgAnimatedPreserveAspectRatio;
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGSymbolElement" , js_name = requiredFeatures ) ]
    #[cfg(feature = "SvgStringList")]
    #[doc = "Getter for the `requiredFeatures` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGSymbolElement/requiredFeatures)\n\n*This API requires the following crate features to be activated: `SvgStringList`, `SvgSymbolElement`*"]
    pub fn required_features(this: &SvgSymbolElement) -> SvgStringList;
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGSymbolElement" , js_name = requiredExtensions ) ]
    #[cfg(feature = "SvgStringList")]
    #[doc = "Getter for the `requiredExtensions` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGSymbolElement/requiredExtensions)\n\n*This API requires the following crate features to be activated: `SvgStringList`, `SvgSymbolElement`*"]
    pub fn required_extensions(this: &SvgSymbolElement) -> SvgStringList;
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGSymbolElement" , js_name = systemLanguage ) ]
    #[cfg(feature = "SvgStringList")]
    #[doc = "Getter for the `systemLanguage` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGSymbolElement/systemLanguage)\n\n*This API requires the following crate features to be activated: `SvgStringList`, `SvgSymbolElement`*"]
    pub fn system_language(this: &SvgSymbolElement) -> SvgStringList;
    # [ wasm_bindgen ( method , structural , js_class = "SVGSymbolElement" , js_name = hasExtension ) ]
    #[doc = "The `hasExtension()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGSymbolElement/hasExtension)\n\n*This API requires the following crate features to be activated: `SvgSymbolElement`*"]
    pub fn has_extension(this: &SvgSymbolElement, extension: &str) -> bool;
}
