use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = EventTarget , extends = :: js_sys :: Object , js_name = Screen , typescript_type = "Screen" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `Screen` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Screen)
    ///
    ///*This API requires the following crate features to be activated: `Screen`*
    pub type Screen;

    # [ wasm_bindgen ( structural , catch , method , getter , js_class = "Screen" , js_name = availWidth ) ]
    ///Getter for the `availWidth` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Screen/availWidth)
    ///
    ///*This API requires the following crate features to be activated: `Screen`*
    pub fn avail_width(this: &Screen) -> Result<i32, JsValue>;

    # [ wasm_bindgen ( structural , catch , method , getter , js_class = "Screen" , js_name = availHeight ) ]
    ///Getter for the `availHeight` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Screen/availHeight)
    ///
    ///*This API requires the following crate features to be activated: `Screen`*
    pub fn avail_height(this: &Screen) -> Result<i32, JsValue>;

    # [ wasm_bindgen ( structural , catch , method , getter , js_class = "Screen" , js_name = width ) ]
    ///Getter for the `width` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Screen/width)
    ///
    ///*This API requires the following crate features to be activated: `Screen`*
    pub fn width(this: &Screen) -> Result<i32, JsValue>;

    # [ wasm_bindgen ( structural , catch , method , getter , js_class = "Screen" , js_name = height ) ]
    ///Getter for the `height` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Screen/height)
    ///
    ///*This API requires the following crate features to be activated: `Screen`*
    pub fn height(this: &Screen) -> Result<i32, JsValue>;

    # [ wasm_bindgen ( structural , catch , method , getter , js_class = "Screen" , js_name = colorDepth ) ]
    ///Getter for the `colorDepth` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Screen/colorDepth)
    ///
    ///*This API requires the following crate features to be activated: `Screen`*
    pub fn color_depth(this: &Screen) -> Result<i32, JsValue>;

    # [ wasm_bindgen ( structural , catch , method , getter , js_class = "Screen" , js_name = pixelDepth ) ]
    ///Getter for the `pixelDepth` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Screen/pixelDepth)
    ///
    ///*This API requires the following crate features to be activated: `Screen`*
    pub fn pixel_depth(this: &Screen) -> Result<i32, JsValue>;

    # [ wasm_bindgen ( structural , catch , method , getter , js_class = "Screen" , js_name = top ) ]
    ///Getter for the `top` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Screen/top)
    ///
    ///*This API requires the following crate features to be activated: `Screen`*
    pub fn top(this: &Screen) -> Result<i32, JsValue>;

    # [ wasm_bindgen ( structural , catch , method , getter , js_class = "Screen" , js_name = left ) ]
    ///Getter for the `left` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Screen/left)
    ///
    ///*This API requires the following crate features to be activated: `Screen`*
    pub fn left(this: &Screen) -> Result<i32, JsValue>;

    # [ wasm_bindgen ( structural , catch , method , getter , js_class = "Screen" , js_name = availTop ) ]
    ///Getter for the `availTop` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Screen/availTop)
    ///
    ///*This API requires the following crate features to be activated: `Screen`*
    pub fn avail_top(this: &Screen) -> Result<i32, JsValue>;

    # [ wasm_bindgen ( structural , catch , method , getter , js_class = "Screen" , js_name = availLeft ) ]
    ///Getter for the `availLeft` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Screen/availLeft)
    ///
    ///*This API requires the following crate features to be activated: `Screen`*
    pub fn avail_left(this: &Screen) -> Result<i32, JsValue>;

    #[cfg(feature = "ScreenOrientation")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "Screen" , js_name = orientation ) ]
    ///Getter for the `orientation` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Screen/orientation)
    ///
    ///*This API requires the following crate features to be activated: `Screen`, `ScreenOrientation`*
    pub fn orientation(this: &Screen) -> ScreenOrientation;

    #[cfg(feature = "ScreenColorGamut")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "Screen" , js_name = colorGamut ) ]
    ///Getter for the `colorGamut` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Screen/colorGamut)
    ///
    ///*This API requires the following crate features to be activated: `Screen`, `ScreenColorGamut`*
    pub fn color_gamut(this: &Screen) -> ScreenColorGamut;

    #[cfg(feature = "ScreenLuminance")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "Screen" , js_name = luminance ) ]
    ///Getter for the `luminance` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Screen/luminance)
    ///
    ///*This API requires the following crate features to be activated: `Screen`, `ScreenLuminance`*
    pub fn luminance(this: &Screen) -> Option<ScreenLuminance>;

    # [ wasm_bindgen ( structural , method , getter , js_class = "Screen" , js_name = onchange ) ]
    ///Getter for the `onchange` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Screen/onchange)
    ///
    ///*This API requires the following crate features to be activated: `Screen`*
    pub fn onchange(this: &Screen) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Screen" , js_name = onchange ) ]
    ///Setter for the `onchange` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Screen/onchange)
    ///
    ///*This API requires the following crate features to be activated: `Screen`*
    pub fn set_onchange(this: &Screen, value: Option<&::js_sys::Function>);

}
