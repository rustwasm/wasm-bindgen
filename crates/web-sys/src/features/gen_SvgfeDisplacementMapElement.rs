use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = SvgElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = SVGFEDisplacementMapElement , typescript_name = SVGFEDisplacementMapElement ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `SvgfeDisplacementMapElement` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEDisplacementMapElement)
    ///
    ///*This API requires the following crate features to be activated: `SvgfeDisplacementMapElement`*
    pub type SvgfeDisplacementMapElement;

    #[cfg(feature = "SvgAnimatedString")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGFEDisplacementMapElement" , js_name = in1 ) ]
    ///Getter for the `in1` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEDisplacementMapElement/in1)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedString`, `SvgfeDisplacementMapElement`*
    pub fn in1(this: &SvgfeDisplacementMapElement) -> SvgAnimatedString;

    #[cfg(feature = "SvgAnimatedString")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGFEDisplacementMapElement" , js_name = in2 ) ]
    ///Getter for the `in2` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEDisplacementMapElement/in2)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedString`, `SvgfeDisplacementMapElement`*
    pub fn in2(this: &SvgfeDisplacementMapElement) -> SvgAnimatedString;

    #[cfg(feature = "SvgAnimatedNumber")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGFEDisplacementMapElement" , js_name = scale ) ]
    ///Getter for the `scale` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEDisplacementMapElement/scale)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedNumber`, `SvgfeDisplacementMapElement`*
    pub fn scale(this: &SvgfeDisplacementMapElement) -> SvgAnimatedNumber;

    #[cfg(feature = "SvgAnimatedEnumeration")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGFEDisplacementMapElement" , js_name = xChannelSelector ) ]
    ///Getter for the `xChannelSelector` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEDisplacementMapElement/xChannelSelector)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedEnumeration`, `SvgfeDisplacementMapElement`*
    pub fn x_channel_selector(this: &SvgfeDisplacementMapElement) -> SvgAnimatedEnumeration;

    #[cfg(feature = "SvgAnimatedEnumeration")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGFEDisplacementMapElement" , js_name = yChannelSelector ) ]
    ///Getter for the `yChannelSelector` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEDisplacementMapElement/yChannelSelector)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedEnumeration`, `SvgfeDisplacementMapElement`*
    pub fn y_channel_selector(this: &SvgfeDisplacementMapElement) -> SvgAnimatedEnumeration;

    #[cfg(feature = "SvgAnimatedLength")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGFEDisplacementMapElement" , js_name = x ) ]
    ///Getter for the `x` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEDisplacementMapElement/x)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgfeDisplacementMapElement`*
    pub fn x(this: &SvgfeDisplacementMapElement) -> SvgAnimatedLength;

    #[cfg(feature = "SvgAnimatedLength")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGFEDisplacementMapElement" , js_name = y ) ]
    ///Getter for the `y` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEDisplacementMapElement/y)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgfeDisplacementMapElement`*
    pub fn y(this: &SvgfeDisplacementMapElement) -> SvgAnimatedLength;

    #[cfg(feature = "SvgAnimatedLength")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGFEDisplacementMapElement" , js_name = width ) ]
    ///Getter for the `width` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEDisplacementMapElement/width)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgfeDisplacementMapElement`*
    pub fn width(this: &SvgfeDisplacementMapElement) -> SvgAnimatedLength;

    #[cfg(feature = "SvgAnimatedLength")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGFEDisplacementMapElement" , js_name = height ) ]
    ///Getter for the `height` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEDisplacementMapElement/height)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgfeDisplacementMapElement`*
    pub fn height(this: &SvgfeDisplacementMapElement) -> SvgAnimatedLength;

    #[cfg(feature = "SvgAnimatedString")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGFEDisplacementMapElement" , js_name = result ) ]
    ///Getter for the `result` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEDisplacementMapElement/result)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedString`, `SvgfeDisplacementMapElement`*
    pub fn result(this: &SvgfeDisplacementMapElement) -> SvgAnimatedString;

}

impl SvgfeDisplacementMapElement {
    ///The `SVGFEDisplacementMapElement.SVG_CHANNEL_UNKNOWN` const.
    ///
    ///*This API requires the following crate features to be activated: `SvgfeDisplacementMapElement`*

    pub const SVG_CHANNEL_UNKNOWN: u16 = 0i64 as u16;

    ///The `SVGFEDisplacementMapElement.SVG_CHANNEL_R` const.
    ///
    ///*This API requires the following crate features to be activated: `SvgfeDisplacementMapElement`*

    pub const SVG_CHANNEL_R: u16 = 1u64 as u16;

    ///The `SVGFEDisplacementMapElement.SVG_CHANNEL_G` const.
    ///
    ///*This API requires the following crate features to be activated: `SvgfeDisplacementMapElement`*

    pub const SVG_CHANNEL_G: u16 = 2u64 as u16;

    ///The `SVGFEDisplacementMapElement.SVG_CHANNEL_B` const.
    ///
    ///*This API requires the following crate features to be activated: `SvgfeDisplacementMapElement`*

    pub const SVG_CHANNEL_B: u16 = 3u64 as u16;

    ///The `SVGFEDisplacementMapElement.SVG_CHANNEL_A` const.
    ///
    ///*This API requires the following crate features to be activated: `SvgfeDisplacementMapElement`*

    pub const SVG_CHANNEL_A: u16 = 4u64 as u16;
}
