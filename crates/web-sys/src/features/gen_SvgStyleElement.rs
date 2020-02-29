use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = SvgElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = SVGStyleElement , typescript_type = "SVGStyleElement" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `SvgStyleElement` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGStyleElement)
    ///
    ///*This API requires the following crate features to be activated: `SvgStyleElement`*
    pub type SvgStyleElement;

    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGStyleElement" , js_name = xmlspace ) ]
    ///Getter for the `xmlspace` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGStyleElement/xmlspace)
    ///
    ///*This API requires the following crate features to be activated: `SvgStyleElement`*
    pub fn xmlspace(this: &SvgStyleElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "SVGStyleElement" , js_name = xmlspace ) ]
    ///Setter for the `xmlspace` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGStyleElement/xmlspace)
    ///
    ///*This API requires the following crate features to be activated: `SvgStyleElement`*
    pub fn set_xmlspace(this: &SvgStyleElement, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGStyleElement" , js_name = type ) ]
    ///Getter for the `type` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGStyleElement/type)
    ///
    ///*This API requires the following crate features to be activated: `SvgStyleElement`*
    pub fn type_(this: &SvgStyleElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "SVGStyleElement" , js_name = type ) ]
    ///Setter for the `type` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGStyleElement/type)
    ///
    ///*This API requires the following crate features to be activated: `SvgStyleElement`*
    pub fn set_type(this: &SvgStyleElement, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGStyleElement" , js_name = media ) ]
    ///Getter for the `media` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGStyleElement/media)
    ///
    ///*This API requires the following crate features to be activated: `SvgStyleElement`*
    pub fn media(this: &SvgStyleElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "SVGStyleElement" , js_name = media ) ]
    ///Setter for the `media` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGStyleElement/media)
    ///
    ///*This API requires the following crate features to be activated: `SvgStyleElement`*
    pub fn set_media(this: &SvgStyleElement, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGStyleElement" , js_name = title ) ]
    ///Getter for the `title` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGStyleElement/title)
    ///
    ///*This API requires the following crate features to be activated: `SvgStyleElement`*
    pub fn title(this: &SvgStyleElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "SVGStyleElement" , js_name = title ) ]
    ///Setter for the `title` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGStyleElement/title)
    ///
    ///*This API requires the following crate features to be activated: `SvgStyleElement`*
    pub fn set_title(this: &SvgStyleElement, value: &str);

    #[cfg(feature = "StyleSheet")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGStyleElement" , js_name = sheet ) ]
    ///Getter for the `sheet` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGStyleElement/sheet)
    ///
    ///*This API requires the following crate features to be activated: `StyleSheet`, `SvgStyleElement`*
    pub fn sheet(this: &SvgStyleElement) -> Option<StyleSheet>;

}
