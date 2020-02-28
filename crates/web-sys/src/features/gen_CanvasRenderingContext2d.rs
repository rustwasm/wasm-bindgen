use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = CanvasRenderingContext2D , typescript_name = CanvasRenderingContext2D ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `CanvasRenderingContext2d` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    pub type CanvasRenderingContext2d;
    # [ wasm_bindgen ( structural , method , getter , js_name = canvas ) ]
    #[cfg(feature = "HtmlCanvasElement")]
    #[doc = "Getter for the `canvas` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/canvas)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `HtmlCanvasElement`*"]
    pub fn canvas(this: &CanvasRenderingContext2d) -> Option<HtmlCanvasElement>;
    # [ wasm_bindgen ( structural , method , getter , js_name = globalAlpha ) ]
    #[doc = "Getter for the `globalAlpha` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/globalAlpha)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    pub fn global_alpha(this: &CanvasRenderingContext2d) -> f64;
    # [ wasm_bindgen ( structural , method , setter , js_name = globalAlpha ) ]
    #[doc = "Setter for the `globalAlpha` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/globalAlpha)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    pub fn set_global_alpha(this: &CanvasRenderingContext2d, value: f64);
    # [ wasm_bindgen ( structural , catch , method , getter , js_name = globalCompositeOperation ) ]
    #[doc = "Getter for the `globalCompositeOperation` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/globalCompositeOperation)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    pub fn global_composite_operation(this: &CanvasRenderingContext2d) -> Result<String, JsValue>;
    # [ wasm_bindgen ( structural , catch , method , setter , js_name = globalCompositeOperation ) ]
    #[doc = "Setter for the `globalCompositeOperation` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/globalCompositeOperation)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    pub fn set_global_composite_operation(
        this: &CanvasRenderingContext2d,
        value: &str,
    ) -> Result<(), JsValue>;
    # [ wasm_bindgen ( structural , method , getter , js_name = strokeStyle ) ]
    #[doc = "Getter for the `strokeStyle` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/strokeStyle)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    pub fn stroke_style(this: &CanvasRenderingContext2d) -> ::wasm_bindgen::JsValue;
    # [ wasm_bindgen ( structural , method , setter , js_name = strokeStyle ) ]
    #[doc = "Setter for the `strokeStyle` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/strokeStyle)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    pub fn set_stroke_style(this: &CanvasRenderingContext2d, value: &::wasm_bindgen::JsValue);
    # [ wasm_bindgen ( structural , method , getter , js_name = fillStyle ) ]
    #[doc = "Getter for the `fillStyle` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/fillStyle)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    pub fn fill_style(this: &CanvasRenderingContext2d) -> ::wasm_bindgen::JsValue;
    # [ wasm_bindgen ( structural , method , setter , js_name = fillStyle ) ]
    #[doc = "Setter for the `fillStyle` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/fillStyle)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    pub fn set_fill_style(this: &CanvasRenderingContext2d, value: &::wasm_bindgen::JsValue);
    # [ wasm_bindgen ( structural , method , getter , js_name = filter ) ]
    #[doc = "Getter for the `filter` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/filter)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    pub fn filter(this: &CanvasRenderingContext2d) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = filter ) ]
    #[doc = "Setter for the `filter` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/filter)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    pub fn set_filter(this: &CanvasRenderingContext2d, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_name = imageSmoothingEnabled ) ]
    #[doc = "Getter for the `imageSmoothingEnabled` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/imageSmoothingEnabled)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    pub fn image_smoothing_enabled(this: &CanvasRenderingContext2d) -> bool;
    # [ wasm_bindgen ( structural , method , setter , js_name = imageSmoothingEnabled ) ]
    #[doc = "Setter for the `imageSmoothingEnabled` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/imageSmoothingEnabled)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    pub fn set_image_smoothing_enabled(this: &CanvasRenderingContext2d, value: bool);
    # [ wasm_bindgen ( structural , method , getter , js_name = lineWidth ) ]
    #[doc = "Getter for the `lineWidth` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/lineWidth)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    pub fn line_width(this: &CanvasRenderingContext2d) -> f64;
    # [ wasm_bindgen ( structural , method , setter , js_name = lineWidth ) ]
    #[doc = "Setter for the `lineWidth` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/lineWidth)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    pub fn set_line_width(this: &CanvasRenderingContext2d, value: f64);
    # [ wasm_bindgen ( structural , method , getter , js_name = lineCap ) ]
    #[doc = "Getter for the `lineCap` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/lineCap)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    pub fn line_cap(this: &CanvasRenderingContext2d) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = lineCap ) ]
    #[doc = "Setter for the `lineCap` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/lineCap)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    pub fn set_line_cap(this: &CanvasRenderingContext2d, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_name = lineJoin ) ]
    #[doc = "Getter for the `lineJoin` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/lineJoin)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    pub fn line_join(this: &CanvasRenderingContext2d) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = lineJoin ) ]
    #[doc = "Setter for the `lineJoin` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/lineJoin)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    pub fn set_line_join(this: &CanvasRenderingContext2d, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_name = miterLimit ) ]
    #[doc = "Getter for the `miterLimit` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/miterLimit)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    pub fn miter_limit(this: &CanvasRenderingContext2d) -> f64;
    # [ wasm_bindgen ( structural , method , setter , js_name = miterLimit ) ]
    #[doc = "Setter for the `miterLimit` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/miterLimit)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    pub fn set_miter_limit(this: &CanvasRenderingContext2d, value: f64);
    # [ wasm_bindgen ( structural , method , getter , js_name = lineDashOffset ) ]
    #[doc = "Getter for the `lineDashOffset` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/lineDashOffset)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    pub fn line_dash_offset(this: &CanvasRenderingContext2d) -> f64;
    # [ wasm_bindgen ( structural , method , setter , js_name = lineDashOffset ) ]
    #[doc = "Setter for the `lineDashOffset` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/lineDashOffset)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    pub fn set_line_dash_offset(this: &CanvasRenderingContext2d, value: f64);
    # [ wasm_bindgen ( structural , method , getter , js_name = shadowOffsetX ) ]
    #[doc = "Getter for the `shadowOffsetX` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/shadowOffsetX)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    pub fn shadow_offset_x(this: &CanvasRenderingContext2d) -> f64;
    # [ wasm_bindgen ( structural , method , setter , js_name = shadowOffsetX ) ]
    #[doc = "Setter for the `shadowOffsetX` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/shadowOffsetX)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    pub fn set_shadow_offset_x(this: &CanvasRenderingContext2d, value: f64);
    # [ wasm_bindgen ( structural , method , getter , js_name = shadowOffsetY ) ]
    #[doc = "Getter for the `shadowOffsetY` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/shadowOffsetY)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    pub fn shadow_offset_y(this: &CanvasRenderingContext2d) -> f64;
    # [ wasm_bindgen ( structural , method , setter , js_name = shadowOffsetY ) ]
    #[doc = "Setter for the `shadowOffsetY` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/shadowOffsetY)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    pub fn set_shadow_offset_y(this: &CanvasRenderingContext2d, value: f64);
    # [ wasm_bindgen ( structural , method , getter , js_name = shadowBlur ) ]
    #[doc = "Getter for the `shadowBlur` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/shadowBlur)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    pub fn shadow_blur(this: &CanvasRenderingContext2d) -> f64;
    # [ wasm_bindgen ( structural , method , setter , js_name = shadowBlur ) ]
    #[doc = "Setter for the `shadowBlur` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/shadowBlur)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    pub fn set_shadow_blur(this: &CanvasRenderingContext2d, value: f64);
    # [ wasm_bindgen ( structural , method , getter , js_name = shadowColor ) ]
    #[doc = "Getter for the `shadowColor` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/shadowColor)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    pub fn shadow_color(this: &CanvasRenderingContext2d) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = shadowColor ) ]
    #[doc = "Setter for the `shadowColor` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/shadowColor)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    pub fn set_shadow_color(this: &CanvasRenderingContext2d, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_name = font ) ]
    #[doc = "Getter for the `font` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/font)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    pub fn font(this: &CanvasRenderingContext2d) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = font ) ]
    #[doc = "Setter for the `font` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/font)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    pub fn set_font(this: &CanvasRenderingContext2d, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_name = textAlign ) ]
    #[doc = "Getter for the `textAlign` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/textAlign)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    pub fn text_align(this: &CanvasRenderingContext2d) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = textAlign ) ]
    #[doc = "Setter for the `textAlign` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/textAlign)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    pub fn set_text_align(this: &CanvasRenderingContext2d, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_name = textBaseline ) ]
    #[doc = "Getter for the `textBaseline` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/textBaseline)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    pub fn text_baseline(this: &CanvasRenderingContext2d) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = textBaseline ) ]
    #[doc = "Setter for the `textBaseline` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/textBaseline)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    pub fn set_text_baseline(this: &CanvasRenderingContext2d, value: &str);
    #[cfg(feature = "Window")]
    # [ wasm_bindgen ( catch , method , structural , js_name = drawWindow ) ]
    #[doc = "The `drawWindow()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/drawWindow)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `Window`*"]
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
    # [ wasm_bindgen ( catch , method , structural , js_name = drawWindow ) ]
    #[doc = "The `drawWindow()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/drawWindow)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `Window`*"]
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
    # [ wasm_bindgen ( catch , method , structural , js_name = drawImage ) ]
    #[doc = "The `drawImage()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/drawImage)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `HtmlImageElement`*"]
    pub fn draw_image_with_html_image_element(
        this: &CanvasRenderingContext2d,
        image: &HtmlImageElement,
        dx: f64,
        dy: f64,
    ) -> Result<(), JsValue>;
    #[cfg(feature = "SvgImageElement")]
    # [ wasm_bindgen ( catch , method , structural , js_name = drawImage ) ]
    #[doc = "The `drawImage()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/drawImage)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `SvgImageElement`*"]
    pub fn draw_image_with_svg_image_element(
        this: &CanvasRenderingContext2d,
        image: &SvgImageElement,
        dx: f64,
        dy: f64,
    ) -> Result<(), JsValue>;
    #[cfg(feature = "HtmlCanvasElement")]
    # [ wasm_bindgen ( catch , method , structural , js_name = drawImage ) ]
    #[doc = "The `drawImage()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/drawImage)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `HtmlCanvasElement`*"]
    pub fn draw_image_with_html_canvas_element(
        this: &CanvasRenderingContext2d,
        image: &HtmlCanvasElement,
        dx: f64,
        dy: f64,
    ) -> Result<(), JsValue>;
    #[cfg(feature = "HtmlVideoElement")]
    # [ wasm_bindgen ( catch , method , structural , js_name = drawImage ) ]
    #[doc = "The `drawImage()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/drawImage)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `HtmlVideoElement`*"]
    pub fn draw_image_with_html_video_element(
        this: &CanvasRenderingContext2d,
        image: &HtmlVideoElement,
        dx: f64,
        dy: f64,
    ) -> Result<(), JsValue>;
    #[cfg(feature = "ImageBitmap")]
    # [ wasm_bindgen ( catch , method , structural , js_name = drawImage ) ]
    #[doc = "The `drawImage()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/drawImage)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `ImageBitmap`*"]
    pub fn draw_image_with_image_bitmap(
        this: &CanvasRenderingContext2d,
        image: &ImageBitmap,
        dx: f64,
        dy: f64,
    ) -> Result<(), JsValue>;
    #[cfg(feature = "HtmlImageElement")]
    # [ wasm_bindgen ( catch , method , structural , js_name = drawImage ) ]
    #[doc = "The `drawImage()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/drawImage)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `HtmlImageElement`*"]
    pub fn draw_image_with_html_image_element_and_dw_and_dh(
        this: &CanvasRenderingContext2d,
        image: &HtmlImageElement,
        dx: f64,
        dy: f64,
        dw: f64,
        dh: f64,
    ) -> Result<(), JsValue>;
    #[cfg(feature = "SvgImageElement")]
    # [ wasm_bindgen ( catch , method , structural , js_name = drawImage ) ]
    #[doc = "The `drawImage()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/drawImage)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `SvgImageElement`*"]
    pub fn draw_image_with_svg_image_element_and_dw_and_dh(
        this: &CanvasRenderingContext2d,
        image: &SvgImageElement,
        dx: f64,
        dy: f64,
        dw: f64,
        dh: f64,
    ) -> Result<(), JsValue>;
    #[cfg(feature = "HtmlCanvasElement")]
    # [ wasm_bindgen ( catch , method , structural , js_name = drawImage ) ]
    #[doc = "The `drawImage()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/drawImage)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `HtmlCanvasElement`*"]
    pub fn draw_image_with_html_canvas_element_and_dw_and_dh(
        this: &CanvasRenderingContext2d,
        image: &HtmlCanvasElement,
        dx: f64,
        dy: f64,
        dw: f64,
        dh: f64,
    ) -> Result<(), JsValue>;
    #[cfg(feature = "HtmlVideoElement")]
    # [ wasm_bindgen ( catch , method , structural , js_name = drawImage ) ]
    #[doc = "The `drawImage()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/drawImage)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `HtmlVideoElement`*"]
    pub fn draw_image_with_html_video_element_and_dw_and_dh(
        this: &CanvasRenderingContext2d,
        image: &HtmlVideoElement,
        dx: f64,
        dy: f64,
        dw: f64,
        dh: f64,
    ) -> Result<(), JsValue>;
    #[cfg(feature = "ImageBitmap")]
    # [ wasm_bindgen ( catch , method , structural , js_name = drawImage ) ]
    #[doc = "The `drawImage()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/drawImage)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `ImageBitmap`*"]
    pub fn draw_image_with_image_bitmap_and_dw_and_dh(
        this: &CanvasRenderingContext2d,
        image: &ImageBitmap,
        dx: f64,
        dy: f64,
        dw: f64,
        dh: f64,
    ) -> Result<(), JsValue>;
    #[cfg(feature = "HtmlImageElement")]
    # [ wasm_bindgen ( catch , method , structural , js_name = drawImage ) ]
    #[doc = "The `drawImage()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/drawImage)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `HtmlImageElement`*"]
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
    # [ wasm_bindgen ( catch , method , structural , js_name = drawImage ) ]
    #[doc = "The `drawImage()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/drawImage)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `SvgImageElement`*"]
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
    # [ wasm_bindgen ( catch , method , structural , js_name = drawImage ) ]
    #[doc = "The `drawImage()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/drawImage)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `HtmlCanvasElement`*"]
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
    # [ wasm_bindgen ( catch , method , structural , js_name = drawImage ) ]
    #[doc = "The `drawImage()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/drawImage)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `HtmlVideoElement`*"]
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
    # [ wasm_bindgen ( catch , method , structural , js_name = drawImage ) ]
    #[doc = "The `drawImage()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/drawImage)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `ImageBitmap`*"]
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
    # [ wasm_bindgen ( method , structural , js_name = beginPath ) ]
    #[doc = "The `beginPath()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/beginPath)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    pub fn begin_path(this: &CanvasRenderingContext2d);
    # [ wasm_bindgen ( method , structural , js_name = clip ) ]
    #[doc = "The `clip()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/clip)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    pub fn clip(this: &CanvasRenderingContext2d);
    #[cfg(feature = "CanvasWindingRule")]
    # [ wasm_bindgen ( method , structural , js_name = clip ) ]
    #[doc = "The `clip()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/clip)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `CanvasWindingRule`*"]
    pub fn clip_with_canvas_winding_rule(
        this: &CanvasRenderingContext2d,
        winding: CanvasWindingRule,
    );
    #[cfg(feature = "Path2d")]
    # [ wasm_bindgen ( method , structural , js_name = clip ) ]
    #[doc = "The `clip()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/clip)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `Path2d`*"]
    pub fn clip_with_path_2d(this: &CanvasRenderingContext2d, path: &Path2d);
    #[cfg(all(feature = "CanvasWindingRule", feature = "Path2d",))]
    # [ wasm_bindgen ( method , structural , js_name = clip ) ]
    #[doc = "The `clip()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/clip)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `CanvasWindingRule`, `Path2d`*"]
    pub fn clip_with_path_2d_and_winding(
        this: &CanvasRenderingContext2d,
        path: &Path2d,
        winding: CanvasWindingRule,
    );
    # [ wasm_bindgen ( method , structural , js_name = fill ) ]
    #[doc = "The `fill()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/fill)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    pub fn fill(this: &CanvasRenderingContext2d);
    #[cfg(feature = "CanvasWindingRule")]
    # [ wasm_bindgen ( method , structural , js_name = fill ) ]
    #[doc = "The `fill()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/fill)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `CanvasWindingRule`*"]
    pub fn fill_with_canvas_winding_rule(
        this: &CanvasRenderingContext2d,
        winding: CanvasWindingRule,
    );
    #[cfg(feature = "Path2d")]
    # [ wasm_bindgen ( method , structural , js_name = fill ) ]
    #[doc = "The `fill()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/fill)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `Path2d`*"]
    pub fn fill_with_path_2d(this: &CanvasRenderingContext2d, path: &Path2d);
    #[cfg(all(feature = "CanvasWindingRule", feature = "Path2d",))]
    # [ wasm_bindgen ( method , structural , js_name = fill ) ]
    #[doc = "The `fill()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/fill)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `CanvasWindingRule`, `Path2d`*"]
    pub fn fill_with_path_2d_and_winding(
        this: &CanvasRenderingContext2d,
        path: &Path2d,
        winding: CanvasWindingRule,
    );
    # [ wasm_bindgen ( method , structural , js_name = isPointInPath ) ]
    #[doc = "The `isPointInPath()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/isPointInPath)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    pub fn is_point_in_path_with_f64(this: &CanvasRenderingContext2d, x: f64, y: f64) -> bool;
    #[cfg(feature = "CanvasWindingRule")]
    # [ wasm_bindgen ( method , structural , js_name = isPointInPath ) ]
    #[doc = "The `isPointInPath()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/isPointInPath)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `CanvasWindingRule`*"]
    pub fn is_point_in_path_with_f64_and_canvas_winding_rule(
        this: &CanvasRenderingContext2d,
        x: f64,
        y: f64,
        winding: CanvasWindingRule,
    ) -> bool;
    #[cfg(feature = "Path2d")]
    # [ wasm_bindgen ( method , structural , js_name = isPointInPath ) ]
    #[doc = "The `isPointInPath()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/isPointInPath)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `Path2d`*"]
    pub fn is_point_in_path_with_path_2d_and_f64(
        this: &CanvasRenderingContext2d,
        path: &Path2d,
        x: f64,
        y: f64,
    ) -> bool;
    #[cfg(all(feature = "CanvasWindingRule", feature = "Path2d",))]
    # [ wasm_bindgen ( method , structural , js_name = isPointInPath ) ]
    #[doc = "The `isPointInPath()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/isPointInPath)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `CanvasWindingRule`, `Path2d`*"]
    pub fn is_point_in_path_with_path_2d_and_f64_and_winding(
        this: &CanvasRenderingContext2d,
        path: &Path2d,
        x: f64,
        y: f64,
        winding: CanvasWindingRule,
    ) -> bool;
    # [ wasm_bindgen ( method , structural , js_name = isPointInStroke ) ]
    #[doc = "The `isPointInStroke()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/isPointInStroke)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    pub fn is_point_in_stroke_with_x_and_y(this: &CanvasRenderingContext2d, x: f64, y: f64)
        -> bool;
    #[cfg(feature = "Path2d")]
    # [ wasm_bindgen ( method , structural , js_name = isPointInStroke ) ]
    #[doc = "The `isPointInStroke()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/isPointInStroke)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `Path2d`*"]
    pub fn is_point_in_stroke_with_path_and_x_and_y(
        this: &CanvasRenderingContext2d,
        path: &Path2d,
        x: f64,
        y: f64,
    ) -> bool;
    # [ wasm_bindgen ( method , structural , js_name = stroke ) ]
    #[doc = "The `stroke()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/stroke)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    pub fn stroke(this: &CanvasRenderingContext2d);
    #[cfg(feature = "Path2d")]
    # [ wasm_bindgen ( method , structural , js_name = stroke ) ]
    #[doc = "The `stroke()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/stroke)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `Path2d`*"]
    pub fn stroke_with_path(this: &CanvasRenderingContext2d, path: &Path2d);
    #[cfg(feature = "CanvasGradient")]
    # [ wasm_bindgen ( method , structural , js_name = createLinearGradient ) ]
    #[doc = "The `createLinearGradient()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/createLinearGradient)\n\n*This API requires the following crate features to be activated: `CanvasGradient`, `CanvasRenderingContext2d`*"]
    pub fn create_linear_gradient(
        this: &CanvasRenderingContext2d,
        x0: f64,
        y0: f64,
        x1: f64,
        y1: f64,
    ) -> CanvasGradient;
    #[cfg(all(feature = "CanvasPattern", feature = "HtmlImageElement",))]
    # [ wasm_bindgen ( catch , method , structural , js_name = createPattern ) ]
    #[doc = "The `createPattern()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/createPattern)\n\n*This API requires the following crate features to be activated: `CanvasPattern`, `CanvasRenderingContext2d`, `HtmlImageElement`*"]
    pub fn create_pattern_with_html_image_element(
        this: &CanvasRenderingContext2d,
        image: &HtmlImageElement,
        repetition: &str,
    ) -> Result<Option<CanvasPattern>, JsValue>;
    #[cfg(all(feature = "CanvasPattern", feature = "SvgImageElement",))]
    # [ wasm_bindgen ( catch , method , structural , js_name = createPattern ) ]
    #[doc = "The `createPattern()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/createPattern)\n\n*This API requires the following crate features to be activated: `CanvasPattern`, `CanvasRenderingContext2d`, `SvgImageElement`*"]
    pub fn create_pattern_with_svg_image_element(
        this: &CanvasRenderingContext2d,
        image: &SvgImageElement,
        repetition: &str,
    ) -> Result<Option<CanvasPattern>, JsValue>;
    #[cfg(all(feature = "CanvasPattern", feature = "HtmlCanvasElement",))]
    # [ wasm_bindgen ( catch , method , structural , js_name = createPattern ) ]
    #[doc = "The `createPattern()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/createPattern)\n\n*This API requires the following crate features to be activated: `CanvasPattern`, `CanvasRenderingContext2d`, `HtmlCanvasElement`*"]
    pub fn create_pattern_with_html_canvas_element(
        this: &CanvasRenderingContext2d,
        image: &HtmlCanvasElement,
        repetition: &str,
    ) -> Result<Option<CanvasPattern>, JsValue>;
    #[cfg(all(feature = "CanvasPattern", feature = "HtmlVideoElement",))]
    # [ wasm_bindgen ( catch , method , structural , js_name = createPattern ) ]
    #[doc = "The `createPattern()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/createPattern)\n\n*This API requires the following crate features to be activated: `CanvasPattern`, `CanvasRenderingContext2d`, `HtmlVideoElement`*"]
    pub fn create_pattern_with_html_video_element(
        this: &CanvasRenderingContext2d,
        image: &HtmlVideoElement,
        repetition: &str,
    ) -> Result<Option<CanvasPattern>, JsValue>;
    #[cfg(all(feature = "CanvasPattern", feature = "ImageBitmap",))]
    # [ wasm_bindgen ( catch , method , structural , js_name = createPattern ) ]
    #[doc = "The `createPattern()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/createPattern)\n\n*This API requires the following crate features to be activated: `CanvasPattern`, `CanvasRenderingContext2d`, `ImageBitmap`*"]
    pub fn create_pattern_with_image_bitmap(
        this: &CanvasRenderingContext2d,
        image: &ImageBitmap,
        repetition: &str,
    ) -> Result<Option<CanvasPattern>, JsValue>;
    #[cfg(feature = "CanvasGradient")]
    # [ wasm_bindgen ( catch , method , structural , js_name = createRadialGradient ) ]
    #[doc = "The `createRadialGradient()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/createRadialGradient)\n\n*This API requires the following crate features to be activated: `CanvasGradient`, `CanvasRenderingContext2d`*"]
    pub fn create_radial_gradient(
        this: &CanvasRenderingContext2d,
        x0: f64,
        y0: f64,
        r0: f64,
        x1: f64,
        y1: f64,
        r1: f64,
    ) -> Result<CanvasGradient, JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = addHitRegion ) ]
    #[doc = "The `addHitRegion()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/addHitRegion)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    pub fn add_hit_region(this: &CanvasRenderingContext2d) -> Result<(), JsValue>;
    #[cfg(feature = "HitRegionOptions")]
    # [ wasm_bindgen ( catch , method , structural , js_name = addHitRegion ) ]
    #[doc = "The `addHitRegion()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/addHitRegion)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `HitRegionOptions`*"]
    pub fn add_hit_region_with_options(
        this: &CanvasRenderingContext2d,
        options: &HitRegionOptions,
    ) -> Result<(), JsValue>;
    # [ wasm_bindgen ( method , structural , js_name = clearHitRegions ) ]
    #[doc = "The `clearHitRegions()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/clearHitRegions)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    pub fn clear_hit_regions(this: &CanvasRenderingContext2d);
    # [ wasm_bindgen ( method , structural , js_name = removeHitRegion ) ]
    #[doc = "The `removeHitRegion()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/removeHitRegion)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    pub fn remove_hit_region(this: &CanvasRenderingContext2d, id: &str);
    #[cfg(feature = "ImageData")]
    # [ wasm_bindgen ( catch , method , structural , js_name = createImageData ) ]
    #[doc = "The `createImageData()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/createImageData)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `ImageData`*"]
    pub fn create_image_data_with_sw_and_sh(
        this: &CanvasRenderingContext2d,
        sw: f64,
        sh: f64,
    ) -> Result<ImageData, JsValue>;
    #[cfg(feature = "ImageData")]
    # [ wasm_bindgen ( catch , method , structural , js_name = createImageData ) ]
    #[doc = "The `createImageData()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/createImageData)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `ImageData`*"]
    pub fn create_image_data_with_imagedata(
        this: &CanvasRenderingContext2d,
        imagedata: &ImageData,
    ) -> Result<ImageData, JsValue>;
    #[cfg(feature = "ImageData")]
    # [ wasm_bindgen ( catch , method , structural , js_name = getImageData ) ]
    #[doc = "The `getImageData()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/getImageData)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `ImageData`*"]
    pub fn get_image_data(
        this: &CanvasRenderingContext2d,
        sx: f64,
        sy: f64,
        sw: f64,
        sh: f64,
    ) -> Result<ImageData, JsValue>;
    #[cfg(feature = "ImageData")]
    # [ wasm_bindgen ( catch , method , structural , js_name = putImageData ) ]
    #[doc = "The `putImageData()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/putImageData)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `ImageData`*"]
    pub fn put_image_data(
        this: &CanvasRenderingContext2d,
        imagedata: &ImageData,
        dx: f64,
        dy: f64,
    ) -> Result<(), JsValue>;
    #[cfg(feature = "ImageData")]
    # [ wasm_bindgen ( catch , method , structural , js_name = putImageData ) ]
    #[doc = "The `putImageData()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/putImageData)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `ImageData`*"]
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
    # [ wasm_bindgen ( method , structural , js_name = getLineDash ) ]
    #[doc = "The `getLineDash()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/getLineDash)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    pub fn get_line_dash(this: &CanvasRenderingContext2d) -> ::js_sys::Array;
    # [ wasm_bindgen ( catch , method , structural , js_name = setLineDash ) ]
    #[doc = "The `setLineDash()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/setLineDash)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    pub fn set_line_dash(
        this: &CanvasRenderingContext2d,
        segments: &::wasm_bindgen::JsValue,
    ) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = arc ) ]
    #[doc = "The `arc()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/arc)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    pub fn arc(
        this: &CanvasRenderingContext2d,
        x: f64,
        y: f64,
        radius: f64,
        start_angle: f64,
        end_angle: f64,
    ) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = arc ) ]
    #[doc = "The `arc()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/arc)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    pub fn arc_with_anticlockwise(
        this: &CanvasRenderingContext2d,
        x: f64,
        y: f64,
        radius: f64,
        start_angle: f64,
        end_angle: f64,
        anticlockwise: bool,
    ) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = arcTo ) ]
    #[doc = "The `arcTo()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/arcTo)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    pub fn arc_to(
        this: &CanvasRenderingContext2d,
        x1: f64,
        y1: f64,
        x2: f64,
        y2: f64,
        radius: f64,
    ) -> Result<(), JsValue>;
    # [ wasm_bindgen ( method , structural , js_name = bezierCurveTo ) ]
    #[doc = "The `bezierCurveTo()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/bezierCurveTo)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    pub fn bezier_curve_to(
        this: &CanvasRenderingContext2d,
        cp1x: f64,
        cp1y: f64,
        cp2x: f64,
        cp2y: f64,
        x: f64,
        y: f64,
    );
    # [ wasm_bindgen ( method , structural , js_name = closePath ) ]
    #[doc = "The `closePath()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/closePath)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    pub fn close_path(this: &CanvasRenderingContext2d);
    # [ wasm_bindgen ( catch , method , structural , js_name = ellipse ) ]
    #[doc = "The `ellipse()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/ellipse)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
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
    # [ wasm_bindgen ( catch , method , structural , js_name = ellipse ) ]
    #[doc = "The `ellipse()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/ellipse)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
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
    # [ wasm_bindgen ( method , structural , js_name = lineTo ) ]
    #[doc = "The `lineTo()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/lineTo)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    pub fn line_to(this: &CanvasRenderingContext2d, x: f64, y: f64);
    # [ wasm_bindgen ( method , structural , js_name = moveTo ) ]
    #[doc = "The `moveTo()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/moveTo)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    pub fn move_to(this: &CanvasRenderingContext2d, x: f64, y: f64);
    # [ wasm_bindgen ( method , structural , js_name = quadraticCurveTo ) ]
    #[doc = "The `quadraticCurveTo()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/quadraticCurveTo)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    pub fn quadratic_curve_to(this: &CanvasRenderingContext2d, cpx: f64, cpy: f64, x: f64, y: f64);
    # [ wasm_bindgen ( method , structural , js_name = rect ) ]
    #[doc = "The `rect()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/rect)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    pub fn rect(this: &CanvasRenderingContext2d, x: f64, y: f64, w: f64, h: f64);
    # [ wasm_bindgen ( method , structural , js_name = clearRect ) ]
    #[doc = "The `clearRect()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/clearRect)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    pub fn clear_rect(this: &CanvasRenderingContext2d, x: f64, y: f64, w: f64, h: f64);
    # [ wasm_bindgen ( method , structural , js_name = fillRect ) ]
    #[doc = "The `fillRect()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/fillRect)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    pub fn fill_rect(this: &CanvasRenderingContext2d, x: f64, y: f64, w: f64, h: f64);
    # [ wasm_bindgen ( method , structural , js_name = strokeRect ) ]
    #[doc = "The `strokeRect()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/strokeRect)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    pub fn stroke_rect(this: &CanvasRenderingContext2d, x: f64, y: f64, w: f64, h: f64);
    # [ wasm_bindgen ( method , structural , js_name = restore ) ]
    #[doc = "The `restore()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/restore)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    pub fn restore(this: &CanvasRenderingContext2d);
    # [ wasm_bindgen ( method , structural , js_name = save ) ]
    #[doc = "The `save()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/save)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    pub fn save(this: &CanvasRenderingContext2d);
    # [ wasm_bindgen ( catch , method , structural , js_name = fillText ) ]
    #[doc = "The `fillText()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/fillText)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    pub fn fill_text(
        this: &CanvasRenderingContext2d,
        text: &str,
        x: f64,
        y: f64,
    ) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = fillText ) ]
    #[doc = "The `fillText()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/fillText)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    pub fn fill_text_with_max_width(
        this: &CanvasRenderingContext2d,
        text: &str,
        x: f64,
        y: f64,
        max_width: f64,
    ) -> Result<(), JsValue>;
    #[cfg(feature = "TextMetrics")]
    # [ wasm_bindgen ( catch , method , structural , js_name = measureText ) ]
    #[doc = "The `measureText()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/measureText)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `TextMetrics`*"]
    pub fn measure_text(
        this: &CanvasRenderingContext2d,
        text: &str,
    ) -> Result<TextMetrics, JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = strokeText ) ]
    #[doc = "The `strokeText()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/strokeText)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    pub fn stroke_text(
        this: &CanvasRenderingContext2d,
        text: &str,
        x: f64,
        y: f64,
    ) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = strokeText ) ]
    #[doc = "The `strokeText()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/strokeText)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    pub fn stroke_text_with_max_width(
        this: &CanvasRenderingContext2d,
        text: &str,
        x: f64,
        y: f64,
        max_width: f64,
    ) -> Result<(), JsValue>;
    #[cfg(feature = "DomMatrix")]
    # [ wasm_bindgen ( catch , method , structural , js_name = getTransform ) ]
    #[doc = "The `getTransform()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/getTransform)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `DomMatrix`*"]
    pub fn get_transform(this: &CanvasRenderingContext2d) -> Result<DomMatrix, JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = resetTransform ) ]
    #[doc = "The `resetTransform()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/resetTransform)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    pub fn reset_transform(this: &CanvasRenderingContext2d) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = rotate ) ]
    #[doc = "The `rotate()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/rotate)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    pub fn rotate(this: &CanvasRenderingContext2d, angle: f64) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = scale ) ]
    #[doc = "The `scale()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/scale)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    pub fn scale(this: &CanvasRenderingContext2d, x: f64, y: f64) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = setTransform ) ]
    #[doc = "The `setTransform()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/setTransform)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    pub fn set_transform(
        this: &CanvasRenderingContext2d,
        a: f64,
        b: f64,
        c: f64,
        d: f64,
        e: f64,
        f: f64,
    ) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = transform ) ]
    #[doc = "The `transform()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/transform)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    pub fn transform(
        this: &CanvasRenderingContext2d,
        a: f64,
        b: f64,
        c: f64,
        d: f64,
        e: f64,
        f: f64,
    ) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = translate ) ]
    #[doc = "The `translate()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/translate)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    pub fn translate(this: &CanvasRenderingContext2d, x: f64, y: f64) -> Result<(), JsValue>;
    #[cfg(feature = "Element")]
    # [ wasm_bindgen ( method , structural , js_name = drawCustomFocusRing ) ]
    #[doc = "The `drawCustomFocusRing()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/drawCustomFocusRing)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `Element`*"]
    pub fn draw_custom_focus_ring(this: &CanvasRenderingContext2d, element: &Element) -> bool;
    #[cfg(feature = "Element")]
    # [ wasm_bindgen ( catch , method , structural , js_name = drawFocusIfNeeded ) ]
    #[doc = "The `drawFocusIfNeeded()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/drawFocusIfNeeded)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `Element`*"]
    pub fn draw_focus_if_needed(
        this: &CanvasRenderingContext2d,
        element: &Element,
    ) -> Result<(), JsValue>;
}
impl CanvasRenderingContext2d {
    pub const DRAWWINDOW_DRAW_CARET: u32 = 1u64 as u32;
    pub const DRAWWINDOW_DO_NOT_FLUSH: u32 = 2u64 as u32;
    pub const DRAWWINDOW_DRAW_VIEW: u32 = 4u64 as u32;
    pub const DRAWWINDOW_USE_WIDGET_LAYERS: u32 = 8u64 as u32;
    pub const DRAWWINDOW_ASYNC_DECODE_IMAGES: u32 = 16u64 as u32;
}
