use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = SvgElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = SVGSymbolElement , typescript_name = SVGSymbolElement ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `SvgSymbolElement` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGSymbolElement)
    ///
    ///*This API requires the following crate features to be activated: `SvgSymbolElement`*
    pub type SvgSymbolElement;

    #[cfg(feature = "SvgAnimatedRect")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGSymbolElement" , js_name = viewBox ) ]
    ///Getter for the `viewBox` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGSymbolElement/viewBox)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedRect`, `SvgSymbolElement`*
    pub fn view_box(this: &SvgSymbolElement) -> SvgAnimatedRect;

    #[cfg(feature = "SvgAnimatedPreserveAspectRatio")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGSymbolElement" , js_name = preserveAspectRatio ) ]
    ///Getter for the `preserveAspectRatio` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGSymbolElement/preserveAspectRatio)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedPreserveAspectRatio`, `SvgSymbolElement`*
    pub fn preserve_aspect_ratio(this: &SvgSymbolElement) -> SvgAnimatedPreserveAspectRatio;

    #[cfg(feature = "SvgStringList")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGSymbolElement" , js_name = requiredFeatures ) ]
    ///Getter for the `requiredFeatures` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGSymbolElement/requiredFeatures)
    ///
    ///*This API requires the following crate features to be activated: `SvgStringList`, `SvgSymbolElement`*
    pub fn required_features(this: &SvgSymbolElement) -> SvgStringList;

    #[cfg(feature = "SvgStringList")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGSymbolElement" , js_name = requiredExtensions ) ]
    ///Getter for the `requiredExtensions` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGSymbolElement/requiredExtensions)
    ///
    ///*This API requires the following crate features to be activated: `SvgStringList`, `SvgSymbolElement`*
    pub fn required_extensions(this: &SvgSymbolElement) -> SvgStringList;

    #[cfg(feature = "SvgStringList")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGSymbolElement" , js_name = systemLanguage ) ]
    ///Getter for the `systemLanguage` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGSymbolElement/systemLanguage)
    ///
    ///*This API requires the following crate features to be activated: `SvgStringList`, `SvgSymbolElement`*
    pub fn system_language(this: &SvgSymbolElement) -> SvgStringList;

    # [ wasm_bindgen ( method , structural , js_class = "SVGSymbolElement" , js_name = hasExtension ) ]
    ///The `hasExtension()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGSymbolElement/hasExtension)
    ///
    ///*This API requires the following crate features to be activated: `SvgSymbolElement`*
    pub fn has_extension(this: &SvgSymbolElement, extension: &str) -> bool;

}
