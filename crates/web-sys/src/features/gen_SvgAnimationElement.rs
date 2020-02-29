use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = SvgElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = SVGAnimationElement , typescript_name = SVGAnimationElement ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `SvgAnimationElement` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimationElement)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimationElement`*
    pub type SvgAnimationElement;

    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGAnimationElement" , js_name = targetElement ) ]
    ///Getter for the `targetElement` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimationElement/targetElement)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimationElement`*
    pub fn target_element(this: &SvgAnimationElement) -> Option<SvgElement>;

    #[cfg(feature = "SvgStringList")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGAnimationElement" , js_name = requiredFeatures ) ]
    ///Getter for the `requiredFeatures` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimationElement/requiredFeatures)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimationElement`, `SvgStringList`*
    pub fn required_features(this: &SvgAnimationElement) -> SvgStringList;

    #[cfg(feature = "SvgStringList")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGAnimationElement" , js_name = requiredExtensions ) ]
    ///Getter for the `requiredExtensions` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimationElement/requiredExtensions)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimationElement`, `SvgStringList`*
    pub fn required_extensions(this: &SvgAnimationElement) -> SvgStringList;

    #[cfg(feature = "SvgStringList")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGAnimationElement" , js_name = systemLanguage ) ]
    ///Getter for the `systemLanguage` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimationElement/systemLanguage)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimationElement`, `SvgStringList`*
    pub fn system_language(this: &SvgAnimationElement) -> SvgStringList;

    # [ wasm_bindgen ( catch , method , structural , js_class = "SVGAnimationElement" , js_name = beginElement ) ]
    ///The `beginElement()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimationElement/beginElement)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimationElement`*
    pub fn begin_element(this: &SvgAnimationElement) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "SVGAnimationElement" , js_name = beginElementAt ) ]
    ///The `beginElementAt()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimationElement/beginElementAt)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimationElement`*
    pub fn begin_element_at(this: &SvgAnimationElement, offset: f32) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "SVGAnimationElement" , js_name = endElement ) ]
    ///The `endElement()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimationElement/endElement)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimationElement`*
    pub fn end_element(this: &SvgAnimationElement) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "SVGAnimationElement" , js_name = endElementAt ) ]
    ///The `endElementAt()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimationElement/endElementAt)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimationElement`*
    pub fn end_element_at(this: &SvgAnimationElement, offset: f32) -> Result<(), JsValue>;

    # [ wasm_bindgen ( method , structural , js_class = "SVGAnimationElement" , js_name = getCurrentTime ) ]
    ///The `getCurrentTime()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimationElement/getCurrentTime)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimationElement`*
    pub fn get_current_time(this: &SvgAnimationElement) -> f32;

    # [ wasm_bindgen ( catch , method , structural , js_class = "SVGAnimationElement" , js_name = getSimpleDuration ) ]
    ///The `getSimpleDuration()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimationElement/getSimpleDuration)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimationElement`*
    pub fn get_simple_duration(this: &SvgAnimationElement) -> Result<f32, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "SVGAnimationElement" , js_name = getStartTime ) ]
    ///The `getStartTime()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimationElement/getStartTime)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimationElement`*
    pub fn get_start_time(this: &SvgAnimationElement) -> Result<f32, JsValue>;

    # [ wasm_bindgen ( method , structural , js_class = "SVGAnimationElement" , js_name = hasExtension ) ]
    ///The `hasExtension()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimationElement/hasExtension)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimationElement`*
    pub fn has_extension(this: &SvgAnimationElement, extension: &str) -> bool;

}
