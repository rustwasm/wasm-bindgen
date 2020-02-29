use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = SvgElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = SVGGraphicsElement , typescript_name = SVGGraphicsElement ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `SvgGraphicsElement` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGGraphicsElement)
    ///
    ///*This API requires the following crate features to be activated: `SvgGraphicsElement`*
    pub type SvgGraphicsElement;

    #[cfg(feature = "SvgAnimatedTransformList")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGGraphicsElement" , js_name = transform ) ]
    ///Getter for the `transform` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGGraphicsElement/transform)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedTransformList`, `SvgGraphicsElement`*
    pub fn transform(this: &SvgGraphicsElement) -> SvgAnimatedTransformList;

    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGGraphicsElement" , js_name = nearestViewportElement ) ]
    ///Getter for the `nearestViewportElement` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGGraphicsElement/nearestViewportElement)
    ///
    ///*This API requires the following crate features to be activated: `SvgGraphicsElement`*
    pub fn nearest_viewport_element(this: &SvgGraphicsElement) -> Option<SvgElement>;

    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGGraphicsElement" , js_name = farthestViewportElement ) ]
    ///Getter for the `farthestViewportElement` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGGraphicsElement/farthestViewportElement)
    ///
    ///*This API requires the following crate features to be activated: `SvgGraphicsElement`*
    pub fn farthest_viewport_element(this: &SvgGraphicsElement) -> Option<SvgElement>;

    #[cfg(feature = "SvgStringList")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGGraphicsElement" , js_name = requiredFeatures ) ]
    ///Getter for the `requiredFeatures` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGGraphicsElement/requiredFeatures)
    ///
    ///*This API requires the following crate features to be activated: `SvgGraphicsElement`, `SvgStringList`*
    pub fn required_features(this: &SvgGraphicsElement) -> SvgStringList;

    #[cfg(feature = "SvgStringList")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGGraphicsElement" , js_name = requiredExtensions ) ]
    ///Getter for the `requiredExtensions` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGGraphicsElement/requiredExtensions)
    ///
    ///*This API requires the following crate features to be activated: `SvgGraphicsElement`, `SvgStringList`*
    pub fn required_extensions(this: &SvgGraphicsElement) -> SvgStringList;

    #[cfg(feature = "SvgStringList")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGGraphicsElement" , js_name = systemLanguage ) ]
    ///Getter for the `systemLanguage` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGGraphicsElement/systemLanguage)
    ///
    ///*This API requires the following crate features to be activated: `SvgGraphicsElement`, `SvgStringList`*
    pub fn system_language(this: &SvgGraphicsElement) -> SvgStringList;

    #[cfg(feature = "SvgRect")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "SVGGraphicsElement" , js_name = getBBox ) ]
    ///The `getBBox()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGGraphicsElement/getBBox)
    ///
    ///*This API requires the following crate features to be activated: `SvgGraphicsElement`, `SvgRect`*
    pub fn get_b_box(this: &SvgGraphicsElement) -> Result<SvgRect, JsValue>;

    #[cfg(all(feature = "SvgBoundingBoxOptions", feature = "SvgRect",))]
    # [ wasm_bindgen ( catch , method , structural , js_class = "SVGGraphicsElement" , js_name = getBBox ) ]
    ///The `getBBox()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGGraphicsElement/getBBox)
    ///
    ///*This API requires the following crate features to be activated: `SvgBoundingBoxOptions`, `SvgGraphicsElement`, `SvgRect`*
    pub fn get_b_box_with_a_options(
        this: &SvgGraphicsElement,
        a_options: &SvgBoundingBoxOptions,
    ) -> Result<SvgRect, JsValue>;

    #[cfg(feature = "SvgMatrix")]
    # [ wasm_bindgen ( method , structural , js_class = "SVGGraphicsElement" , js_name = getCTM ) ]
    ///The `getCTM()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGGraphicsElement/getCTM)
    ///
    ///*This API requires the following crate features to be activated: `SvgGraphicsElement`, `SvgMatrix`*
    pub fn get_ctm(this: &SvgGraphicsElement) -> Option<SvgMatrix>;

    #[cfg(feature = "SvgMatrix")]
    # [ wasm_bindgen ( method , structural , js_class = "SVGGraphicsElement" , js_name = getScreenCTM ) ]
    ///The `getScreenCTM()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGGraphicsElement/getScreenCTM)
    ///
    ///*This API requires the following crate features to be activated: `SvgGraphicsElement`, `SvgMatrix`*
    pub fn get_screen_ctm(this: &SvgGraphicsElement) -> Option<SvgMatrix>;

    #[cfg(feature = "SvgMatrix")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "SVGGraphicsElement" , js_name = getTransformToElement ) ]
    ///The `getTransformToElement()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGGraphicsElement/getTransformToElement)
    ///
    ///*This API requires the following crate features to be activated: `SvgGraphicsElement`, `SvgMatrix`*
    pub fn get_transform_to_element(
        this: &SvgGraphicsElement,
        element: &SvgGraphicsElement,
    ) -> Result<SvgMatrix, JsValue>;

    # [ wasm_bindgen ( method , structural , js_class = "SVGGraphicsElement" , js_name = hasExtension ) ]
    ///The `hasExtension()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGGraphicsElement/hasExtension)
    ///
    ///*This API requires the following crate features to be activated: `SvgGraphicsElement`*
    pub fn has_extension(this: &SvgGraphicsElement, extension: &str) -> bool;

}
