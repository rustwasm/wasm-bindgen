use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = SvgElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = SVGScriptElement , typescript_name = SVGScriptElement ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `SvgScriptElement` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGScriptElement)
    ///
    ///*This API requires the following crate features to be activated: `SvgScriptElement`*
    pub type SvgScriptElement;

    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGScriptElement" , js_name = type ) ]
    ///Getter for the `type` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGScriptElement/type)
    ///
    ///*This API requires the following crate features to be activated: `SvgScriptElement`*
    pub fn type_(this: &SvgScriptElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "SVGScriptElement" , js_name = type ) ]
    ///Setter for the `type` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGScriptElement/type)
    ///
    ///*This API requires the following crate features to be activated: `SvgScriptElement`*
    pub fn set_type(this: &SvgScriptElement, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGScriptElement" , js_name = crossOrigin ) ]
    ///Getter for the `crossOrigin` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGScriptElement/crossOrigin)
    ///
    ///*This API requires the following crate features to be activated: `SvgScriptElement`*
    pub fn cross_origin(this: &SvgScriptElement) -> Option<String>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "SVGScriptElement" , js_name = crossOrigin ) ]
    ///Setter for the `crossOrigin` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGScriptElement/crossOrigin)
    ///
    ///*This API requires the following crate features to be activated: `SvgScriptElement`*
    pub fn set_cross_origin(this: &SvgScriptElement, value: Option<&str>);

    #[cfg(feature = "SvgAnimatedString")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGScriptElement" , js_name = href ) ]
    ///Getter for the `href` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGScriptElement/href)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedString`, `SvgScriptElement`*
    pub fn href(this: &SvgScriptElement) -> SvgAnimatedString;

}
