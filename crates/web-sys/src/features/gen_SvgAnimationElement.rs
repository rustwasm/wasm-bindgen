use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = SvgElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = SVGAnimationElement , typescript_name = SVGAnimationElement ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SvgAnimationElement` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimationElement)\n\n*This API requires the following crate features to be activated: `SvgAnimationElement`*"]
    pub type SvgAnimationElement;
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGAnimationElement" , js_name = targetElement ) ]
    #[doc = "Getter for the `targetElement` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimationElement/targetElement)\n\n*This API requires the following crate features to be activated: `SvgAnimationElement`*"]
    pub fn target_element(this: &SvgAnimationElement) -> Option<SvgElement>;
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGAnimationElement" , js_name = requiredFeatures ) ]
    #[cfg(feature = "SvgStringList")]
    #[doc = "Getter for the `requiredFeatures` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimationElement/requiredFeatures)\n\n*This API requires the following crate features to be activated: `SvgAnimationElement`, `SvgStringList`*"]
    pub fn required_features(this: &SvgAnimationElement) -> SvgStringList;
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGAnimationElement" , js_name = requiredExtensions ) ]
    #[cfg(feature = "SvgStringList")]
    #[doc = "Getter for the `requiredExtensions` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimationElement/requiredExtensions)\n\n*This API requires the following crate features to be activated: `SvgAnimationElement`, `SvgStringList`*"]
    pub fn required_extensions(this: &SvgAnimationElement) -> SvgStringList;
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGAnimationElement" , js_name = systemLanguage ) ]
    #[cfg(feature = "SvgStringList")]
    #[doc = "Getter for the `systemLanguage` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimationElement/systemLanguage)\n\n*This API requires the following crate features to be activated: `SvgAnimationElement`, `SvgStringList`*"]
    pub fn system_language(this: &SvgAnimationElement) -> SvgStringList;
    # [ wasm_bindgen ( catch , method , structural , js_class = "SVGAnimationElement" , js_name = beginElement ) ]
    #[doc = "The `beginElement()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimationElement/beginElement)\n\n*This API requires the following crate features to be activated: `SvgAnimationElement`*"]
    pub fn begin_element(this: &SvgAnimationElement) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_class = "SVGAnimationElement" , js_name = beginElementAt ) ]
    #[doc = "The `beginElementAt()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimationElement/beginElementAt)\n\n*This API requires the following crate features to be activated: `SvgAnimationElement`*"]
    pub fn begin_element_at(this: &SvgAnimationElement, offset: f32) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_class = "SVGAnimationElement" , js_name = endElement ) ]
    #[doc = "The `endElement()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimationElement/endElement)\n\n*This API requires the following crate features to be activated: `SvgAnimationElement`*"]
    pub fn end_element(this: &SvgAnimationElement) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_class = "SVGAnimationElement" , js_name = endElementAt ) ]
    #[doc = "The `endElementAt()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimationElement/endElementAt)\n\n*This API requires the following crate features to be activated: `SvgAnimationElement`*"]
    pub fn end_element_at(this: &SvgAnimationElement, offset: f32) -> Result<(), JsValue>;
    # [ wasm_bindgen ( method , structural , js_class = "SVGAnimationElement" , js_name = getCurrentTime ) ]
    #[doc = "The `getCurrentTime()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimationElement/getCurrentTime)\n\n*This API requires the following crate features to be activated: `SvgAnimationElement`*"]
    pub fn get_current_time(this: &SvgAnimationElement) -> f32;
    # [ wasm_bindgen ( catch , method , structural , js_class = "SVGAnimationElement" , js_name = getSimpleDuration ) ]
    #[doc = "The `getSimpleDuration()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimationElement/getSimpleDuration)\n\n*This API requires the following crate features to be activated: `SvgAnimationElement`*"]
    pub fn get_simple_duration(this: &SvgAnimationElement) -> Result<f32, JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_class = "SVGAnimationElement" , js_name = getStartTime ) ]
    #[doc = "The `getStartTime()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimationElement/getStartTime)\n\n*This API requires the following crate features to be activated: `SvgAnimationElement`*"]
    pub fn get_start_time(this: &SvgAnimationElement) -> Result<f32, JsValue>;
    # [ wasm_bindgen ( method , structural , js_class = "SVGAnimationElement" , js_name = hasExtension ) ]
    #[doc = "The `hasExtension()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimationElement/hasExtension)\n\n*This API requires the following crate features to be activated: `SvgAnimationElement`*"]
    pub fn has_extension(this: &SvgAnimationElement, extension: &str) -> bool;
}
