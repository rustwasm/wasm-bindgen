use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = CanvasRenderingContext2D , typescript_type = "CanvasRenderingContext2D" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `CanvasRenderingContext2d` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D)
    ///
    ///*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*
    pub type CanvasRenderingContext2d;

    #[cfg(feature = "HtmlCanvasElement")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "CanvasRenderingContext2D" , js_name = canvas ) ]
    ///Getter for the `canvas` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/canvas)
    ///
    ///*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `HtmlCanvasElement`*
    pub fn canvas(this: &CanvasRenderingContext2d) -> Option<HtmlCanvasElement>;

    # [ wasm_bindgen ( structural , method , getter , js_class = "CanvasRenderingContext2D" , js_name = globalAlpha ) ]
    ///Getter for the `globalAlpha` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/globalAlpha)
    ///
    ///*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*
    pub fn global_alpha(this: &CanvasRenderingContext2d) -> f64;

    # [ wasm_bindgen ( structural , method , setter , js_class = "CanvasRenderingContext2D" , js_name = globalAlpha ) ]
    ///Setter for the `globalAlpha` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/globalAlpha)
    ///
    ///*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*
    pub fn set_global_alpha(this: &CanvasRenderingContext2d, value: f64);

    # [ wasm_bindgen ( structural , catch , method , getter , js_class = "CanvasRenderingContext2D" , js_name = globalCompositeOperation ) ]
    ///Getter for the `globalCompositeOperation` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/globalCompositeOperation)
    ///
    ///*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*
    pub fn global_composite_operation(this: &CanvasRenderingContext2d) -> Result<String, JsValue>;

    # [ wasm_bindgen ( structural , catch , method , setter , js_class = "CanvasRenderingContext2D" , js_name = globalCompositeOperation ) ]
    ///Setter for the `globalCompositeOperation` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/globalCompositeOperation)
    ///
    ///*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*
    pub fn set_global_composite_operation(
        this: &CanvasRenderingContext2d,
        value: &str,
    ) -> Result<(), JsValue>;

    # [ wasm_bindgen ( structural , method , getter , js_class = "CanvasRenderingContext2D" , js_name = strokeStyle ) ]
    ///Getter for the `strokeStyle` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/strokeStyle)
    ///
    ///*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*
    pub fn stroke_style(this: &CanvasRenderingContext2d) -> ::wasm_bindgen::JsValue;

    # [ wasm_bindgen ( structural , method , setter , js_class = "CanvasRenderingContext2D" , js_name = strokeStyle ) ]
    ///Setter for the `strokeStyle` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/strokeStyle)
    ///
    ///*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*
    pub fn set_stroke_style(this: &CanvasRenderingContext2d, value: &::wasm_bindgen::JsValue);

    # [ wasm_bindgen ( structural , method , getter , js_class = "CanvasRenderingContext2D" , js_name = fillStyle ) ]
    ///Getter for the `fillStyle` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/fillStyle)
    ///
    ///*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*
    pub fn fill_style(this: &CanvasRenderingContext2d) -> ::wasm_bindgen::JsValue;

    # [ wasm_bindgen ( structural , method , setter , js_class = "CanvasRenderingContext2D" , js_name = fillStyle ) ]
    ///Setter for the `fillStyle` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/fillStyle)
    ///
    ///*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*
    pub fn set_fill_style(this: &CanvasRenderingContext2d, value: &::wasm_bindgen::JsValue);

    # [ wasm_bindgen ( structural , method , getter , js_class = "CanvasRenderingContext2D" , js_name = filter ) ]
    ///Getter for the `filter` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/filter)
    ///
    ///*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*
    pub fn filter(this: &CanvasRenderingContext2d) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "CanvasRenderingContext2D" , js_name = filter ) ]
    ///Setter for the `filter` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/filter)
    ///
    ///*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*
    pub fn set_filter(this: &CanvasRenderingContext2d, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "CanvasRenderingContext2D" , js_name = imageSmoothingEnabled ) ]
    ///Getter for the `imageSmoothingEnabled` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/imageSmoothingEnabled)
    ///
    ///*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*
    pub fn image_smoothing_enabled(this: &CanvasRenderingContext2d) -> bool;

    # [ wasm_bindgen ( structural , method , setter , js_class = "CanvasRenderingContext2D" , js_name = imageSmoothingEnabled ) ]
    ///Setter for the `imageSmoothingEnabled` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/imageSmoothingEnabled)
    ///
    ///*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*
    pub fn set_image_smoothing_enabled(this: &CanvasRenderingContext2d, value: bool);

    # [ wasm_bindgen ( structural , method , getter , js_class = "CanvasRenderingContext2D" , js_name = lineWidth ) ]
    ///Getter for the `lineWidth` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/lineWidth)
    ///
    ///*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*
    pub fn line_width(this: &CanvasRenderingContext2d) -> f64;

    # [ wasm_bindgen ( structural , method , setter , js_class = "CanvasRenderingContext2D" , js_name = lineWidth ) ]
    ///Setter for the `lineWidth` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/lineWidth)
    ///
    ///*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*
    pub fn set_line_width(this: &CanvasRenderingContext2d, value: f64);

    # [ wasm_bindgen ( structural , method , getter , js_class = "CanvasRenderingContext2D" , js_name = lineCap ) ]
    ///Getter for the `lineCap` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/lineCap)
    ///
    ///*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*
    pub fn line_cap(this: &CanvasRenderingContext2d) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "CanvasRenderingContext2D" , js_name = lineCap ) ]
    ///Setter for the `lineCap` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/lineCap)
    ///
    ///*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*
    pub fn set_line_cap(this: &CanvasRenderingContext2d, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "CanvasRenderingContext2D" , js_name = lineJoin ) ]
    ///Getter for the `lineJoin` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/lineJoin)
    ///
    ///*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*
    pub fn line_join(this: &CanvasRenderingContext2d) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "CanvasRenderingContext2D" , js_name = lineJoin ) ]
    ///Setter for the `lineJoin` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/lineJoin)
    ///
    ///*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*
    pub fn set_line_join(this: &CanvasRenderingContext2d, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "CanvasRenderingContext2D" , js_name = miterLimit ) ]
    ///Getter for the `miterLimit` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/miterLimit)
    ///
    ///*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*
    pub fn miter_limit(this: &CanvasRenderingContext2d) -> f64;

    # [ wasm_bindgen ( structural , method , setter , js_class = "CanvasRenderingContext2D" , js_name = miterLimit ) ]
    ///Setter for the `miterLimit` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/miterLimit)
    ///
    ///*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*
    pub fn set_miter_limit(this: &CanvasRenderingContext2d, value: f64);

    # [ wasm_bindgen ( structural , method , getter , js_class = "CanvasRenderingContext2D" , js_name = lineDashOffset ) ]
    ///Getter for the `lineDashOffset` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/lineDashOffset)
    ///
    ///*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*
    pub fn line_dash_offset(this: &CanvasRenderingContext2d) -> f64;

    # [ wasm_bindgen ( structural , method , setter , js_class = "CanvasRenderingContext2D" , js_name = lineDashOffset ) ]
    ///Setter for the `lineDashOffset` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/lineDashOffset)
    ///
    ///*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*
    pub fn set_line_dash_offset(this: &CanvasRenderingContext2d, value: f64);

    # [ wasm_bindgen ( structural , method , getter , js_class = "CanvasRenderingContext2D" , js_name = shadowOffsetX ) ]
    ///Getter for the `shadowOffsetX` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/shadowOffsetX)
    ///
    ///*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*
    pub fn shadow_offset_x(this: &CanvasRenderingContext2d) -> f64;

    # [ wasm_bindgen ( structural , method , setter , js_class = "CanvasRenderingContext2D" , js_name = shadowOffsetX ) ]
    ///Setter for the `shadowOffsetX` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/shadowOffsetX)
    ///
    ///*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*
    pub fn set_shadow_offset_x(this: &CanvasRenderingContext2d, value: f64);

    # [ wasm_bindgen ( structural , method , getter , js_class = "CanvasRenderingContext2D" , js_name = shadowOffsetY ) ]
    ///Getter for the `shadowOffsetY` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/shadowOffsetY)
    ///
    ///*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*
    pub fn shadow_offset_y(this: &CanvasRenderingContext2d) -> f64;

    # [ wasm_bindgen ( structural , method , setter , js_class = "CanvasRenderingContext2D" , js_name = shadowOffsetY ) ]
    ///Setter for the `shadowOffsetY` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/shadowOffsetY)
    ///
    ///*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*
    pub fn set_shadow_offset_y(this: &CanvasRenderingContext2d, value: f64);

    # [ wasm_bindgen ( structural , method , getter , js_class = "CanvasRenderingContext2D" , js_name = shadowBlur ) ]
    ///Getter for the `shadowBlur` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/shadowBlur)
    ///
    ///*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*
    pub fn shadow_blur(this: &CanvasRenderingContext2d) -> f64;

    # [ wasm_bindgen ( structural , method , setter , js_class = "CanvasRenderingContext2D" , js_name = shadowBlur ) ]
    ///Setter for the `shadowBlur` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/shadowBlur)
    ///
    ///*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*
    pub fn set_shadow_blur(this: &CanvasRenderingContext2d, value: f64);

    # [ wasm_bindgen ( structural , method , getter , js_class = "CanvasRenderingContext2D" , js_name = shadowColor ) ]
    ///Getter for the `shadowColor` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/shadowColor)
    ///
    ///*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*
    pub fn shadow_color(this: &CanvasRenderingContext2d) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "CanvasRenderingContext2D" , js_name = shadowColor ) ]
    ///Setter for the `shadowColor` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/shadowColor)
    ///
    ///*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*
    pub fn set_shadow_color(this: &CanvasRenderingContext2d, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "CanvasRenderingContext2D" , js_name = font ) ]
    ///Getter for the `font` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/font)
    ///
    ///*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*
    pub fn font(this: &CanvasRenderingContext2d) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "CanvasRenderingContext2D" , js_name = font ) ]
    ///Setter for the `font` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/font)
    ///
    ///*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*
    pub fn set_font(this: &CanvasRenderingContext2d, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "CanvasRenderingContext2D" , js_name = textAlign ) ]
    ///Getter for the `textAlign` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/textAlign)
    ///
    ///*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*
    pub fn text_align(this: &CanvasRenderingContext2d) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "CanvasRenderingContext2D" , js_name = textAlign ) ]
    ///Setter for the `textAlign` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/textAlign)
    ///
    ///*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*
    pub fn set_text_align(this: &CanvasRenderingContext2d, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "CanvasRenderingContext2D" , js_name = textBaseline ) ]
    ///Getter for the `textBaseline` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/textBaseline)
    ///
    ///*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*
    pub fn text_baseline(this: &CanvasRenderingContext2d) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "CanvasRenderingContext2D" , js_name = textBaseline ) ]
    ///Setter for the `textBaseline` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/textBaseline)
    ///
    ///*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*
    pub fn set_text_baseline(this: &CanvasRenderingContext2d, value: &str);

    #[cfg(feature = "Window")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "CanvasRenderingContext2D" , js_name = drawWindow ) ]
    ///The `drawWindow()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/drawWindow)
    ///
    ///*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `Window`*
    pub fn draw_window(
        this: &CanvasRenderingContext2d,
        window: &Window,
        x: f64,
        y: f64,
        w: f64,
        h: f64,
        bg_color: &str,
    ) -> Result<(), JsValue>;

    #[cfg(feature = "Window")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "CanvasRenderingContext2D" , js_name = drawWindow ) ]
    ///The `drawWindow()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/drawWindow)
    ///
    ///*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `Window`*
    pub fn draw_window_with_flags(
        this: &CanvasRenderingContext2d,
        window: &Window,
        x: f64,
        y: f64,
        w: f64,
        h: f64,
        bg_color: &str,
        flags: u32,
    ) -> Result<(), JsValue>;

    #[cfg(feature = "HtmlImageElement")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "CanvasRenderingContext2D" , js_name = drawImage ) ]
    ///The `drawImage()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/drawImage)
    ///
    ///*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `HtmlImageElement`*
    pub fn draw_image_with_html_image_element(
        this: &CanvasRenderingContext2d,
        image: &HtmlImageElement,
        dx: f64,
        dy: f64,
    ) -> Result<(), JsValue>;

    #[cfg(feature = "SvgImageElement")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "CanvasRenderingContext2D" , js_name = drawImage ) ]
    ///The `drawImage()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/drawImage)
    ///
    ///*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `SvgImageElement`*
    pub fn draw_image_with_svg_image_element(
        this: &CanvasRenderingContext2d,
        image: &SvgImageElement,
        dx: f64,
        dy: f64,
    ) -> Result<(), JsValue>;

    #[cfg(feature = "HtmlCanvasElement")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "CanvasRenderingContext2D" , js_name = drawImage ) ]
    ///The `drawImage()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/drawImage)
    ///
    ///*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `HtmlCanvasElement`*
    pub fn draw_image_with_html_canvas_element(
        this: &CanvasRenderingContext2d,
        image: &HtmlCanvasElement,
        dx: f64,
        dy: f64,
    ) -> Result<(), JsValue>;

    #[cfg(feature = "HtmlVideoElement")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "CanvasRenderingContext2D" , js_name = drawImage ) ]
    ///The `drawImage()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/drawImage)
    ///
    ///*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `HtmlVideoElement`*
    pub fn draw_image_with_html_video_element(
        this: &CanvasRenderingContext2d,
        image: &HtmlVideoElement,
        dx: f64,
        dy: f64,
    ) -> Result<(), JsValue>;

    #[cfg(feature = "ImageBitmap")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "CanvasRenderingContext2D" , js_name = drawImage ) ]
    ///The `drawImage()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/drawImage)
    ///
    ///*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `ImageBitmap`*
    pub fn draw_image_with_image_bitmap(
        this: &CanvasRenderingContext2d,
        image: &ImageBitmap,
        dx: f64,
        dy: f64,
    ) -> Result<(), JsValue>;

    #[cfg(feature = "HtmlImageElement")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "CanvasRenderingContext2D" , js_name = drawImage ) ]
    ///The `drawImage()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/drawImage)
    ///
    ///*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `HtmlImageElement`*
    pub fn draw_image_with_html_image_element_and_dw_and_dh(
        this: &CanvasRenderingContext2d,
        image: &HtmlImageElement,
        dx: f64,
        dy: f64,
        dw: f64,
        dh: f64,
    ) -> Result<(), JsValue>;

    #[cfg(feature = "SvgImageElement")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "CanvasRenderingContext2D" , js_name = drawImage ) ]
    ///The `drawImage()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/drawImage)
    ///
    ///*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `SvgImageElement`*
    pub fn draw_image_with_svg_image_element_and_dw_and_dh(
        this: &CanvasRenderingContext2d,
        image: &SvgImageElement,
        dx: f64,
        dy: f64,
        dw: f64,
        dh: f64,
    ) -> Result<(), JsValue>;

    #[cfg(feature = "HtmlCanvasElement")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "CanvasRenderingContext2D" , js_name = drawImage ) ]
    ///The `drawImage()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/drawImage)
    ///
    ///*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `HtmlCanvasElement`*
    pub fn draw_image_with_html_canvas_element_and_dw_and_dh(
        this: &CanvasRenderingContext2d,
        image: &HtmlCanvasElement,
        dx: f64,
        dy: f64,
        dw: f64,
        dh: f64,
    ) -> Result<(), JsValue>;

    #[cfg(feature = "HtmlVideoElement")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "CanvasRenderingContext2D" , js_name = drawImage ) ]
    ///The `drawImage()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/drawImage)
    ///
    ///*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `HtmlVideoElement`*
    pub fn draw_image_with_html_video_element_and_dw_and_dh(
        this: &CanvasRenderingContext2d,
        image: &HtmlVideoElement,
        dx: f64,
        dy: f64,
        dw: f64,
        dh: f64,
    ) -> Result<(), JsValue>;

    #[cfg(feature = "ImageBitmap")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "CanvasRenderingContext2D" , js_name = drawImage ) ]
    ///The `drawImage()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/drawImage)
    ///
    ///*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `ImageBitmap`*
    pub fn draw_image_with_image_bitmap_and_dw_and_dh(
        this: &CanvasRenderingContext2d,
        image: &ImageBitmap,
        dx: f64,
        dy: f64,
        dw: f64,
        dh: f64,
    ) -> Result<(), JsValue>;

    #[cfg(feature = "HtmlImageElement")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "CanvasRenderingContext2D" , js_name = drawImage ) ]
    ///The `drawImage()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/drawImage)
    ///
    ///*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `HtmlImageElement`*
    pub fn draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
        this: &CanvasRenderingContext2d,
        image: &HtmlImageElement,
        sx: f64,
        sy: f64,
        sw: f64,
        sh: f64,
        dx: f64,
        dy: f64,
        dw: f64,
        dh: f64,
    ) -> Result<(), JsValue>;

    #[cfg(feature = "SvgImageElement")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "CanvasRenderingContext2D" , js_name = drawImage ) ]
    ///The `drawImage()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/drawImage)
    ///
    ///*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `SvgImageElement`*
    pub fn draw_image_with_svg_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
        this: &CanvasRenderingContext2d,
        image: &SvgImageElement,
        sx: f64,
        sy: f64,
        sw: f64,
        sh: f64,
        dx: f64,
        dy: f64,
        dw: f64,
        dh: f64,
    ) -> Result<(), JsValue>;

    #[cfg(feature = "HtmlCanvasElement")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "CanvasRenderingContext2D" , js_name = drawImage ) ]
    ///The `drawImage()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/drawImage)
    ///
    ///*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `HtmlCanvasElement`*
    pub fn draw_image_with_html_canvas_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
        this: &CanvasRenderingContext2d,
        image: &HtmlCanvasElement,
        sx: f64,
        sy: f64,
        sw: f64,
        sh: f64,
        dx: f64,
        dy: f64,
        dw: f64,
        dh: f64,
    ) -> Result<(), JsValue>;

    #[cfg(feature = "HtmlVideoElement")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "CanvasRenderingContext2D" , js_name = drawImage ) ]
    ///The `drawImage()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/drawImage)
    ///
    ///*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `HtmlVideoElement`*
    pub fn draw_image_with_html_video_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
        this: &CanvasRenderingContext2d,
        image: &HtmlVideoElement,
        sx: f64,
        sy: f64,
        sw: f64,
        sh: f64,
        dx: f64,
        dy: f64,
        dw: f64,
        dh: f64,
    ) -> Result<(), JsValue>;

    #[cfg(feature = "ImageBitmap")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "CanvasRenderingContext2D" , js_name = drawImage ) ]
    ///The `drawImage()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/drawImage)
    ///
    ///*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `ImageBitmap`*
    pub fn draw_image_with_image_bitmap_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
        this: &CanvasRenderingContext2d,
        image: &ImageBitmap,
        sx: f64,
        sy: f64,
        sw: f64,
        sh: f64,
        dx: f64,
        dy: f64,
        dw: f64,
        dh: f64,
    ) -> Result<(), JsValue>;

    # [ wasm_bindgen ( method , structural , js_class = "CanvasRenderingContext2D" , js_name = beginPath ) ]
    ///The `beginPath()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/beginPath)
    ///
    ///*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*
    pub fn begin_path(this: &CanvasRenderingContext2d);

    # [ wasm_bindgen ( method , structural , js_class = "CanvasRenderingContext2D" , js_name = clip ) ]
    ///The `clip()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/clip)
    ///
    ///*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*
    pub fn clip(this: &CanvasRenderingContext2d);

    #[cfg(feature = "CanvasWindingRule")]
    # [ wasm_bindgen ( method , structural , js_class = "CanvasRenderingContext2D" , js_name = clip ) ]
    ///The `clip()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/clip)
    ///
    ///*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `CanvasWindingRule`*
    pub fn clip_with_canvas_winding_rule(
        this: &CanvasRenderingContext2d,
        winding: CanvasWindingRule,
    );

    #[cfg(feature = "Path2d")]
    # [ wasm_bindgen ( method , structural , js_class = "CanvasRenderingContext2D" , js_name = clip ) ]
    ///The `clip()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/clip)
    ///
    ///*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `Path2d`*
    pub fn clip_with_path_2d(this: &CanvasRenderingContext2d, path: &Path2d);

    #[cfg(all(feature = "CanvasWindingRule", feature = "Path2d",))]
    # [ wasm_bindgen ( method , structural , js_class = "CanvasRenderingContext2D" , js_name = clip ) ]
    ///The `clip()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/clip)
    ///
    ///*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `CanvasWindingRule`, `Path2d`*
    pub fn clip_with_path_2d_and_winding(
        this: &CanvasRenderingContext2d,
        path: &Path2d,
        winding: CanvasWindingRule,
    );

    # [ wasm_bindgen ( method , structural , js_class = "CanvasRenderingContext2D" , js_name = fill ) ]
    ///The `fill()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/fill)
    ///
    ///*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*
    pub fn fill(this: &CanvasRenderingContext2d);

    #[cfg(feature = "CanvasWindingRule")]
    # [ wasm_bindgen ( method , structural , js_class = "CanvasRenderingContext2D" , js_name = fill ) ]
    ///The `fill()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/fill)
    ///
    ///*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `CanvasWindingRule`*
    pub fn fill_with_canvas_winding_rule(
        this: &CanvasRenderingContext2d,
        winding: CanvasWindingRule,
    );

    #[cfg(feature = "Path2d")]
    # [ wasm_bindgen ( method , structural , js_class = "CanvasRenderingContext2D" , js_name = fill ) ]
    ///The `fill()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/fill)
    ///
    ///*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `Path2d`*
    pub fn fill_with_path_2d(this: &CanvasRenderingContext2d, path: &Path2d);

    #[cfg(all(feature = "CanvasWindingRule", feature = "Path2d",))]
    # [ wasm_bindgen ( method , structural , js_class = "CanvasRenderingContext2D" , js_name = fill ) ]
    ///The `fill()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/fill)
    ///
    ///*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `CanvasWindingRule`, `Path2d`*
    pub fn fill_with_path_2d_and_winding(
        this: &CanvasRenderingContext2d,
        path: &Path2d,
        winding: CanvasWindingRule,
    );

    # [ wasm_bindgen ( method , structural , js_class = "CanvasRenderingContext2D" , js_name = isPointInPath ) ]
    ///The `isPointInPath()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/isPointInPath)
    ///
    ///*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*
    pub fn is_point_in_path_with_f64(this: &CanvasRenderingContext2d, x: f64, y: f64) -> bool;

    #[cfg(feature = "CanvasWindingRule")]
    # [ wasm_bindgen ( method , structural , js_class = "CanvasRenderingContext2D" , js_name = isPointInPath ) ]
    ///The `isPointInPath()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/isPointInPath)
    ///
    ///*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `CanvasWindingRule`*
    pub fn is_point_in_path_with_f64_and_canvas_winding_rule(
        this: &CanvasRenderingContext2d,
        x: f64,
        y: f64,
        winding: CanvasWindingRule,
    ) -> bool;

    #[cfg(feature = "Path2d")]
    # [ wasm_bindgen ( method , structural , js_class = "CanvasRenderingContext2D" , js_name = isPointInPath ) ]
    ///The `isPointInPath()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/isPointInPath)
    ///
    ///*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `Path2d`*
    pub fn is_point_in_path_with_path_2d_and_f64(
        this: &CanvasRenderingContext2d,
        path: &Path2d,
        x: f64,
        y: f64,
    ) -> bool;

    #[cfg(all(feature = "CanvasWindingRule", feature = "Path2d",))]
    # [ wasm_bindgen ( method , structural , js_class = "CanvasRenderingContext2D" , js_name = isPointInPath ) ]
    ///The `isPointInPath()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/isPointInPath)
    ///
    ///*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `CanvasWindingRule`, `Path2d`*
    pub fn is_point_in_path_with_path_2d_and_f64_and_winding(
        this: &CanvasRenderingContext2d,
        path: &Path2d,
        x: f64,
        y: f64,
        winding: CanvasWindingRule,
    ) -> bool;

    # [ wasm_bindgen ( method , structural , js_class = "CanvasRenderingContext2D" , js_name = isPointInStroke ) ]
    ///The `isPointInStroke()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/isPointInStroke)
    ///
    ///*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*
    pub fn is_point_in_stroke_with_x_and_y(this: &CanvasRenderingContext2d, x: f64, y: f64)
        -> bool;

    #[cfg(feature = "Path2d")]
    # [ wasm_bindgen ( method , structural , js_class = "CanvasRenderingContext2D" , js_name = isPointInStroke ) ]
    ///The `isPointInStroke()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/isPointInStroke)
    ///
    ///*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `Path2d`*
    pub fn is_point_in_stroke_with_path_and_x_and_y(
        this: &CanvasRenderingContext2d,
        path: &Path2d,
        x: f64,
        y: f64,
    ) -> bool;

    # [ wasm_bindgen ( method , structural , js_class = "CanvasRenderingContext2D" , js_name = stroke ) ]
    ///The `stroke()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/stroke)
    ///
    ///*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*
    pub fn stroke(this: &CanvasRenderingContext2d);

    #[cfg(feature = "Path2d")]
    # [ wasm_bindgen ( method , structural , js_class = "CanvasRenderingContext2D" , js_name = stroke ) ]
    ///The `stroke()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/stroke)
    ///
    ///*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `Path2d`*
    pub fn stroke_with_path(this: &CanvasRenderingContext2d, path: &Path2d);

    #[cfg(feature = "CanvasGradient")]
    # [ wasm_bindgen ( method , structural , js_class = "CanvasRenderingContext2D" , js_name = createLinearGradient ) ]
    ///The `createLinearGradient()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/createLinearGradient)
    ///
    ///*This API requires the following crate features to be activated: `CanvasGradient`, `CanvasRenderingContext2d`*
    pub fn create_linear_gradient(
        this: &CanvasRenderingContext2d,
        x0: f64,
        y0: f64,
        x1: f64,
        y1: f64,
    ) -> CanvasGradient;

    #[cfg(all(feature = "CanvasPattern", feature = "HtmlImageElement",))]
    # [ wasm_bindgen ( catch , method , structural , js_class = "CanvasRenderingContext2D" , js_name = createPattern ) ]
    ///The `createPattern()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/createPattern)
    ///
    ///*This API requires the following crate features to be activated: `CanvasPattern`, `CanvasRenderingContext2d`, `HtmlImageElement`*
    pub fn create_pattern_with_html_image_element(
        this: &CanvasRenderingContext2d,
        image: &HtmlImageElement,
        repetition: &str,
    ) -> Result<Option<CanvasPattern>, JsValue>;

    #[cfg(all(feature = "CanvasPattern", feature = "SvgImageElement",))]
    # [ wasm_bindgen ( catch , method , structural , js_class = "CanvasRenderingContext2D" , js_name = createPattern ) ]
    ///The `createPattern()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/createPattern)
    ///
    ///*This API requires the following crate features to be activated: `CanvasPattern`, `CanvasRenderingContext2d`, `SvgImageElement`*
    pub fn create_pattern_with_svg_image_element(
        this: &CanvasRenderingContext2d,
        image: &SvgImageElement,
        repetition: &str,
    ) -> Result<Option<CanvasPattern>, JsValue>;

    #[cfg(all(feature = "CanvasPattern", feature = "HtmlCanvasElement",))]
    # [ wasm_bindgen ( catch , method , structural , js_class = "CanvasRenderingContext2D" , js_name = createPattern ) ]
    ///The `createPattern()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/createPattern)
    ///
    ///*This API requires the following crate features to be activated: `CanvasPattern`, `CanvasRenderingContext2d`, `HtmlCanvasElement`*
    pub fn create_pattern_with_html_canvas_element(
        this: &CanvasRenderingContext2d,
        image: &HtmlCanvasElement,
        repetition: &str,
    ) -> Result<Option<CanvasPattern>, JsValue>;

    #[cfg(all(feature = "CanvasPattern", feature = "HtmlVideoElement",))]
    # [ wasm_bindgen ( catch , method , structural , js_class = "CanvasRenderingContext2D" , js_name = createPattern ) ]
    ///The `createPattern()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/createPattern)
    ///
    ///*This API requires the following crate features to be activated: `CanvasPattern`, `CanvasRenderingContext2d`, `HtmlVideoElement`*
    pub fn create_pattern_with_html_video_element(
        this: &CanvasRenderingContext2d,
        image: &HtmlVideoElement,
        repetition: &str,
    ) -> Result<Option<CanvasPattern>, JsValue>;

    #[cfg(all(feature = "CanvasPattern", feature = "ImageBitmap",))]
    # [ wasm_bindgen ( catch , method , structural , js_class = "CanvasRenderingContext2D" , js_name = createPattern ) ]
    ///The `createPattern()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/createPattern)
    ///
    ///*This API requires the following crate features to be activated: `CanvasPattern`, `CanvasRenderingContext2d`, `ImageBitmap`*
    pub fn create_pattern_with_image_bitmap(
        this: &CanvasRenderingContext2d,
        image: &ImageBitmap,
        repetition: &str,
    ) -> Result<Option<CanvasPattern>, JsValue>;

    #[cfg(feature = "CanvasGradient")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "CanvasRenderingContext2D" , js_name = createRadialGradient ) ]
    ///The `createRadialGradient()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/createRadialGradient)
    ///
    ///*This API requires the following crate features to be activated: `CanvasGradient`, `CanvasRenderingContext2d`*
    pub fn create_radial_gradient(
        this: &CanvasRenderingContext2d,
        x0: f64,
        y0: f64,
        r0: f64,
        x1: f64,
        y1: f64,
        r1: f64,
    ) -> Result<CanvasGradient, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "CanvasRenderingContext2D" , js_name = addHitRegion ) ]
    ///The `addHitRegion()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/addHitRegion)
    ///
    ///*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*
    pub fn add_hit_region(this: &CanvasRenderingContext2d) -> Result<(), JsValue>;

    #[cfg(feature = "HitRegionOptions")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "CanvasRenderingContext2D" , js_name = addHitRegion ) ]
    ///The `addHitRegion()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/addHitRegion)
    ///
    ///*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `HitRegionOptions`*
    pub fn add_hit_region_with_options(
        this: &CanvasRenderingContext2d,
        options: &HitRegionOptions,
    ) -> Result<(), JsValue>;

    # [ wasm_bindgen ( method , structural , js_class = "CanvasRenderingContext2D" , js_name = clearHitRegions ) ]
    ///The `clearHitRegions()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/clearHitRegions)
    ///
    ///*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*
    pub fn clear_hit_regions(this: &CanvasRenderingContext2d);

    # [ wasm_bindgen ( method , structural , js_class = "CanvasRenderingContext2D" , js_name = removeHitRegion ) ]
    ///The `removeHitRegion()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/removeHitRegion)
    ///
    ///*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*
    pub fn remove_hit_region(this: &CanvasRenderingContext2d, id: &str);

    #[cfg(feature = "ImageData")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "CanvasRenderingContext2D" , js_name = createImageData ) ]
    ///The `createImageData()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/createImageData)
    ///
    ///*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `ImageData`*
    pub fn create_image_data_with_sw_and_sh(
        this: &CanvasRenderingContext2d,
        sw: f64,
        sh: f64,
    ) -> Result<ImageData, JsValue>;

    #[cfg(feature = "ImageData")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "CanvasRenderingContext2D" , js_name = createImageData ) ]
    ///The `createImageData()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/createImageData)
    ///
    ///*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `ImageData`*
    pub fn create_image_data_with_imagedata(
        this: &CanvasRenderingContext2d,
        imagedata: &ImageData,
    ) -> Result<ImageData, JsValue>;

    #[cfg(feature = "ImageData")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "CanvasRenderingContext2D" , js_name = getImageData ) ]
    ///The `getImageData()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/getImageData)
    ///
    ///*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `ImageData`*
    pub fn get_image_data(
        this: &CanvasRenderingContext2d,
        sx: f64,
        sy: f64,
        sw: f64,
        sh: f64,
    ) -> Result<ImageData, JsValue>;

    #[cfg(feature = "ImageData")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "CanvasRenderingContext2D" , js_name = putImageData ) ]
    ///The `putImageData()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/putImageData)
    ///
    ///*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `ImageData`*
    pub fn put_image_data(
        this: &CanvasRenderingContext2d,
        imagedata: &ImageData,
        dx: f64,
        dy: f64,
    ) -> Result<(), JsValue>;

    #[cfg(feature = "ImageData")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "CanvasRenderingContext2D" , js_name = putImageData ) ]
    ///The `putImageData()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/putImageData)
    ///
    ///*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `ImageData`*
    pub fn put_image_data_with_dirty_x_and_dirty_y_and_dirty_width_and_dirty_height(
        this: &CanvasRenderingContext2d,
        imagedata: &ImageData,
        dx: f64,
        dy: f64,
        dirty_x: f64,
        dirty_y: f64,
        dirty_width: f64,
        dirty_height: f64,
    ) -> Result<(), JsValue>;

    # [ wasm_bindgen ( method , structural , js_class = "CanvasRenderingContext2D" , js_name = getLineDash ) ]
    ///The `getLineDash()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/getLineDash)
    ///
    ///*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*
    pub fn get_line_dash(this: &CanvasRenderingContext2d) -> ::js_sys::Array;

    # [ wasm_bindgen ( catch , method , structural , js_class = "CanvasRenderingContext2D" , js_name = setLineDash ) ]
    ///The `setLineDash()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/setLineDash)
    ///
    ///*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*
    pub fn set_line_dash(
        this: &CanvasRenderingContext2d,
        segments: &::wasm_bindgen::JsValue,
    ) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "CanvasRenderingContext2D" , js_name = arc ) ]
    ///The `arc()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/arc)
    ///
    ///*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*
    pub fn arc(
        this: &CanvasRenderingContext2d,
        x: f64,
        y: f64,
        radius: f64,
        start_angle: f64,
        end_angle: f64,
    ) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "CanvasRenderingContext2D" , js_name = arc ) ]
    ///The `arc()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/arc)
    ///
    ///*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*
    pub fn arc_with_anticlockwise(
        this: &CanvasRenderingContext2d,
        x: f64,
        y: f64,
        radius: f64,
        start_angle: f64,
        end_angle: f64,
        anticlockwise: bool,
    ) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "CanvasRenderingContext2D" , js_name = arcTo ) ]
    ///The `arcTo()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/arcTo)
    ///
    ///*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*
    pub fn arc_to(
        this: &CanvasRenderingContext2d,
        x1: f64,
        y1: f64,
        x2: f64,
        y2: f64,
        radius: f64,
    ) -> Result<(), JsValue>;

    # [ wasm_bindgen ( method , structural , js_class = "CanvasRenderingContext2D" , js_name = bezierCurveTo ) ]
    ///The `bezierCurveTo()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/bezierCurveTo)
    ///
    ///*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*
    pub fn bezier_curve_to(
        this: &CanvasRenderingContext2d,
        cp1x: f64,
        cp1y: f64,
        cp2x: f64,
        cp2y: f64,
        x: f64,
        y: f64,
    );

    # [ wasm_bindgen ( method , structural , js_class = "CanvasRenderingContext2D" , js_name = closePath ) ]
    ///The `closePath()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/closePath)
    ///
    ///*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*
    pub fn close_path(this: &CanvasRenderingContext2d);

    # [ wasm_bindgen ( catch , method , structural , js_class = "CanvasRenderingContext2D" , js_name = ellipse ) ]
    ///The `ellipse()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/ellipse)
    ///
    ///*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*
    pub fn ellipse(
        this: &CanvasRenderingContext2d,
        x: f64,
        y: f64,
        radius_x: f64,
        radius_y: f64,
        rotation: f64,
        start_angle: f64,
        end_angle: f64,
    ) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "CanvasRenderingContext2D" , js_name = ellipse ) ]
    ///The `ellipse()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/ellipse)
    ///
    ///*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*
    pub fn ellipse_with_anticlockwise(
        this: &CanvasRenderingContext2d,
        x: f64,
        y: f64,
        radius_x: f64,
        radius_y: f64,
        rotation: f64,
        start_angle: f64,
        end_angle: f64,
        anticlockwise: bool,
    ) -> Result<(), JsValue>;

    # [ wasm_bindgen ( method , structural , js_class = "CanvasRenderingContext2D" , js_name = lineTo ) ]
    ///The `lineTo()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/lineTo)
    ///
    ///*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*
    pub fn line_to(this: &CanvasRenderingContext2d, x: f64, y: f64);

    # [ wasm_bindgen ( method , structural , js_class = "CanvasRenderingContext2D" , js_name = moveTo ) ]
    ///The `moveTo()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/moveTo)
    ///
    ///*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*
    pub fn move_to(this: &CanvasRenderingContext2d, x: f64, y: f64);

    # [ wasm_bindgen ( method , structural , js_class = "CanvasRenderingContext2D" , js_name = quadraticCurveTo ) ]
    ///The `quadraticCurveTo()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/quadraticCurveTo)
    ///
    ///*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*
    pub fn quadratic_curve_to(this: &CanvasRenderingContext2d, cpx: f64, cpy: f64, x: f64, y: f64);

    # [ wasm_bindgen ( method , structural , js_class = "CanvasRenderingContext2D" , js_name = rect ) ]
    ///The `rect()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/rect)
    ///
    ///*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*
    pub fn rect(this: &CanvasRenderingContext2d, x: f64, y: f64, w: f64, h: f64);

    # [ wasm_bindgen ( method , structural , js_class = "CanvasRenderingContext2D" , js_name = clearRect ) ]
    ///The `clearRect()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/clearRect)
    ///
    ///*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*
    pub fn clear_rect(this: &CanvasRenderingContext2d, x: f64, y: f64, w: f64, h: f64);

    # [ wasm_bindgen ( method , structural , js_class = "CanvasRenderingContext2D" , js_name = fillRect ) ]
    ///The `fillRect()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/fillRect)
    ///
    ///*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*
    pub fn fill_rect(this: &CanvasRenderingContext2d, x: f64, y: f64, w: f64, h: f64);

    # [ wasm_bindgen ( method , structural , js_class = "CanvasRenderingContext2D" , js_name = strokeRect ) ]
    ///The `strokeRect()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/strokeRect)
    ///
    ///*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*
    pub fn stroke_rect(this: &CanvasRenderingContext2d, x: f64, y: f64, w: f64, h: f64);

    # [ wasm_bindgen ( method , structural , js_class = "CanvasRenderingContext2D" , js_name = restore ) ]
    ///The `restore()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/restore)
    ///
    ///*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*
    pub fn restore(this: &CanvasRenderingContext2d);

    # [ wasm_bindgen ( method , structural , js_class = "CanvasRenderingContext2D" , js_name = save ) ]
    ///The `save()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/save)
    ///
    ///*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*
    pub fn save(this: &CanvasRenderingContext2d);

    # [ wasm_bindgen ( catch , method , structural , js_class = "CanvasRenderingContext2D" , js_name = fillText ) ]
    ///The `fillText()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/fillText)
    ///
    ///*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*
    pub fn fill_text(
        this: &CanvasRenderingContext2d,
        text: &str,
        x: f64,
        y: f64,
    ) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "CanvasRenderingContext2D" , js_name = fillText ) ]
    ///The `fillText()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/fillText)
    ///
    ///*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*
    pub fn fill_text_with_max_width(
        this: &CanvasRenderingContext2d,
        text: &str,
        x: f64,
        y: f64,
        max_width: f64,
    ) -> Result<(), JsValue>;

    #[cfg(feature = "TextMetrics")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "CanvasRenderingContext2D" , js_name = measureText ) ]
    ///The `measureText()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/measureText)
    ///
    ///*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `TextMetrics`*
    pub fn measure_text(
        this: &CanvasRenderingContext2d,
        text: &str,
    ) -> Result<TextMetrics, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "CanvasRenderingContext2D" , js_name = strokeText ) ]
    ///The `strokeText()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/strokeText)
    ///
    ///*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*
    pub fn stroke_text(
        this: &CanvasRenderingContext2d,
        text: &str,
        x: f64,
        y: f64,
    ) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "CanvasRenderingContext2D" , js_name = strokeText ) ]
    ///The `strokeText()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/strokeText)
    ///
    ///*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*
    pub fn stroke_text_with_max_width(
        this: &CanvasRenderingContext2d,
        text: &str,
        x: f64,
        y: f64,
        max_width: f64,
    ) -> Result<(), JsValue>;

    #[cfg(feature = "DomMatrix")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "CanvasRenderingContext2D" , js_name = getTransform ) ]
    ///The `getTransform()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/getTransform)
    ///
    ///*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `DomMatrix`*
    pub fn get_transform(this: &CanvasRenderingContext2d) -> Result<DomMatrix, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "CanvasRenderingContext2D" , js_name = resetTransform ) ]
    ///The `resetTransform()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/resetTransform)
    ///
    ///*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*
    pub fn reset_transform(this: &CanvasRenderingContext2d) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "CanvasRenderingContext2D" , js_name = rotate ) ]
    ///The `rotate()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/rotate)
    ///
    ///*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*
    pub fn rotate(this: &CanvasRenderingContext2d, angle: f64) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "CanvasRenderingContext2D" , js_name = scale ) ]
    ///The `scale()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/scale)
    ///
    ///*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*
    pub fn scale(this: &CanvasRenderingContext2d, x: f64, y: f64) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "CanvasRenderingContext2D" , js_name = setTransform ) ]
    ///The `setTransform()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/setTransform)
    ///
    ///*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*
    pub fn set_transform(
        this: &CanvasRenderingContext2d,
        a: f64,
        b: f64,
        c: f64,
        d: f64,
        e: f64,
        f: f64,
    ) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "CanvasRenderingContext2D" , js_name = transform ) ]
    ///The `transform()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/transform)
    ///
    ///*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*
    pub fn transform(
        this: &CanvasRenderingContext2d,
        a: f64,
        b: f64,
        c: f64,
        d: f64,
        e: f64,
        f: f64,
    ) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "CanvasRenderingContext2D" , js_name = translate ) ]
    ///The `translate()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/translate)
    ///
    ///*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*
    pub fn translate(this: &CanvasRenderingContext2d, x: f64, y: f64) -> Result<(), JsValue>;

    #[cfg(feature = "Element")]
    # [ wasm_bindgen ( method , structural , js_class = "CanvasRenderingContext2D" , js_name = drawCustomFocusRing ) ]
    ///The `drawCustomFocusRing()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/drawCustomFocusRing)
    ///
    ///*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `Element`*
    pub fn draw_custom_focus_ring(this: &CanvasRenderingContext2d, element: &Element) -> bool;

    #[cfg(feature = "Element")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "CanvasRenderingContext2D" , js_name = drawFocusIfNeeded ) ]
    ///The `drawFocusIfNeeded()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/drawFocusIfNeeded)
    ///
    ///*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `Element`*
    pub fn draw_focus_if_needed(
        this: &CanvasRenderingContext2d,
        element: &Element,
    ) -> Result<(), JsValue>;

}

impl CanvasRenderingContext2d {
    ///The `CanvasRenderingContext2D.DRAWWINDOW_DRAW_CARET` const.
    ///
    ///*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*

    pub const DRAWWINDOW_DRAW_CARET: u32 = 1u64 as u32;

    ///The `CanvasRenderingContext2D.DRAWWINDOW_DO_NOT_FLUSH` const.
    ///
    ///*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*

    pub const DRAWWINDOW_DO_NOT_FLUSH: u32 = 2u64 as u32;

    ///The `CanvasRenderingContext2D.DRAWWINDOW_DRAW_VIEW` const.
    ///
    ///*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*

    pub const DRAWWINDOW_DRAW_VIEW: u32 = 4u64 as u32;

    ///The `CanvasRenderingContext2D.DRAWWINDOW_USE_WIDGET_LAYERS` const.
    ///
    ///*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*

    pub const DRAWWINDOW_USE_WIDGET_LAYERS: u32 = 8u64 as u32;

    ///The `CanvasRenderingContext2D.DRAWWINDOW_ASYNC_DECODE_IMAGES` const.
    ///
    ///*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*

    pub const DRAWWINDOW_ASYNC_DECODE_IMAGES: u32 = 16u64 as u32;
}
