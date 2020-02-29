use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = SvgGraphicsElement , extends = SvgElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = SVGTextContentElement , typescript_name = SVGTextContentElement ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `SvgTextContentElement` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGTextContentElement)
    ///
    ///*This API requires the following crate features to be activated: `SvgTextContentElement`*
    pub type SvgTextContentElement;

    #[cfg(feature = "SvgAnimatedLength")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGTextContentElement" , js_name = textLength ) ]
    ///Getter for the `textLength` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGTextContentElement/textLength)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgTextContentElement`*
    pub fn text_length(this: &SvgTextContentElement) -> SvgAnimatedLength;

    #[cfg(feature = "SvgAnimatedEnumeration")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGTextContentElement" , js_name = lengthAdjust ) ]
    ///Getter for the `lengthAdjust` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGTextContentElement/lengthAdjust)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedEnumeration`, `SvgTextContentElement`*
    pub fn length_adjust(this: &SvgTextContentElement) -> SvgAnimatedEnumeration;

    #[cfg(feature = "SvgPoint")]
    # [ wasm_bindgen ( method , structural , js_class = "SVGTextContentElement" , js_name = getCharNumAtPosition ) ]
    ///The `getCharNumAtPosition()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGTextContentElement/getCharNumAtPosition)
    ///
    ///*This API requires the following crate features to be activated: `SvgPoint`, `SvgTextContentElement`*
    pub fn get_char_num_at_position(this: &SvgTextContentElement, point: &SvgPoint) -> i32;

    # [ wasm_bindgen ( method , structural , js_class = "SVGTextContentElement" , js_name = getComputedTextLength ) ]
    ///The `getComputedTextLength()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGTextContentElement/getComputedTextLength)
    ///
    ///*This API requires the following crate features to be activated: `SvgTextContentElement`*
    pub fn get_computed_text_length(this: &SvgTextContentElement) -> f32;

    #[cfg(feature = "SvgPoint")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "SVGTextContentElement" , js_name = getEndPositionOfChar ) ]
    ///The `getEndPositionOfChar()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGTextContentElement/getEndPositionOfChar)
    ///
    ///*This API requires the following crate features to be activated: `SvgPoint`, `SvgTextContentElement`*
    pub fn get_end_position_of_char(
        this: &SvgTextContentElement,
        charnum: u32,
    ) -> Result<SvgPoint, JsValue>;

    #[cfg(feature = "SvgRect")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "SVGTextContentElement" , js_name = getExtentOfChar ) ]
    ///The `getExtentOfChar()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGTextContentElement/getExtentOfChar)
    ///
    ///*This API requires the following crate features to be activated: `SvgRect`, `SvgTextContentElement`*
    pub fn get_extent_of_char(
        this: &SvgTextContentElement,
        charnum: u32,
    ) -> Result<SvgRect, JsValue>;

    # [ wasm_bindgen ( method , structural , js_class = "SVGTextContentElement" , js_name = getNumberOfChars ) ]
    ///The `getNumberOfChars()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGTextContentElement/getNumberOfChars)
    ///
    ///*This API requires the following crate features to be activated: `SvgTextContentElement`*
    pub fn get_number_of_chars(this: &SvgTextContentElement) -> i32;

    # [ wasm_bindgen ( catch , method , structural , js_class = "SVGTextContentElement" , js_name = getRotationOfChar ) ]
    ///The `getRotationOfChar()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGTextContentElement/getRotationOfChar)
    ///
    ///*This API requires the following crate features to be activated: `SvgTextContentElement`*
    pub fn get_rotation_of_char(this: &SvgTextContentElement, charnum: u32)
        -> Result<f32, JsValue>;

    #[cfg(feature = "SvgPoint")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "SVGTextContentElement" , js_name = getStartPositionOfChar ) ]
    ///The `getStartPositionOfChar()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGTextContentElement/getStartPositionOfChar)
    ///
    ///*This API requires the following crate features to be activated: `SvgPoint`, `SvgTextContentElement`*
    pub fn get_start_position_of_char(
        this: &SvgTextContentElement,
        charnum: u32,
    ) -> Result<SvgPoint, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "SVGTextContentElement" , js_name = getSubStringLength ) ]
    ///The `getSubStringLength()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGTextContentElement/getSubStringLength)
    ///
    ///*This API requires the following crate features to be activated: `SvgTextContentElement`*
    pub fn get_sub_string_length(
        this: &SvgTextContentElement,
        charnum: u32,
        nchars: u32,
    ) -> Result<f32, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "SVGTextContentElement" , js_name = selectSubString ) ]
    ///The `selectSubString()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGTextContentElement/selectSubString)
    ///
    ///*This API requires the following crate features to be activated: `SvgTextContentElement`*
    pub fn select_sub_string(
        this: &SvgTextContentElement,
        charnum: u32,
        nchars: u32,
    ) -> Result<(), JsValue>;

}

impl SvgTextContentElement {
    ///The `SVGTextContentElement.LENGTHADJUST_UNKNOWN` const.
    ///
    ///*This API requires the following crate features to be activated: `SvgTextContentElement`*

    pub const LENGTHADJUST_UNKNOWN: u16 = 0i64 as u16;

    ///The `SVGTextContentElement.LENGTHADJUST_SPACING` const.
    ///
    ///*This API requires the following crate features to be activated: `SvgTextContentElement`*

    pub const LENGTHADJUST_SPACING: u16 = 1u64 as u16;

    ///The `SVGTextContentElement.LENGTHADJUST_SPACINGANDGLYPHS` const.
    ///
    ///*This API requires the following crate features to be activated: `SvgTextContentElement`*

    pub const LENGTHADJUST_SPACINGANDGLYPHS: u16 = 2u64 as u16;
}
