#![allow(unused_imports)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = OffscreenCanvasRenderingContext2D , typescript_type = "OffscreenCanvasRenderingContext2D" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `OffscreenCanvasRenderingContext2D` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvasRenderingContext2D`*"]
    pub type OffscreenCanvasRenderingContext2D;
    #[cfg(feature = "OffscreenCanvas")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "OffscreenCanvasRenderingContext2D" , js_name = canvas ) ]
    #[doc = "Getter for the `canvas` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/canvas)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvasRenderingContext2D`, `OffscreenCanvas`*"]
    pub fn canvas(this: &OffscreenCanvasRenderingContext2D) -> Option<OffscreenCanvas>;
    # [ wasm_bindgen ( structural , method , getter , js_class = "OffscreenCanvasRenderingContext2D" , js_name = globalAlpha ) ]
    #[doc = "Getter for the `globalAlpha` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/globalAlpha)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvasRenderingContext2D`*"]
    pub fn global_alpha(this: &OffscreenCanvasRenderingContext2D) -> f64;
    # [ wasm_bindgen ( structural , method , setter , js_class = "OffscreenCanvasRenderingContext2D" , js_name = globalAlpha ) ]
    #[doc = "Setter for the `globalAlpha` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/globalAlpha)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvasRenderingContext2D`*"]
    pub fn set_global_alpha(this: &OffscreenCanvasRenderingContext2D, value: f64);
    # [ wasm_bindgen ( structural , catch , method , getter , js_class = "OffscreenCanvasRenderingContext2D" , js_name = globalCompositeOperation ) ]
    #[doc = "Getter for the `globalCompositeOperation` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/globalCompositeOperation)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvasRenderingContext2D`*"]
    pub fn global_composite_operation(this: &OffscreenCanvasRenderingContext2D) -> Result<String, JsValue>;
    # [ wasm_bindgen ( structural , catch , method , setter , js_class = "OffscreenCanvasRenderingContext2D" , js_name = globalCompositeOperation ) ]
    #[doc = "Setter for the `globalCompositeOperation` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/globalCompositeOperation)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvasRenderingContext2D`*"]
    pub fn set_global_composite_operation(
        this: &OffscreenCanvasRenderingContext2D,
        value: &str,
    ) -> Result<(), JsValue>;
    # [ wasm_bindgen ( structural , method , getter , js_class = "OffscreenCanvasRenderingContext2D" , js_name = strokeStyle ) ]
    #[doc = "Getter for the `strokeStyle` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/strokeStyle)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvasRenderingContext2D`*"]
    pub fn stroke_style(this: &OffscreenCanvasRenderingContext2D) -> ::wasm_bindgen::JsValue;
    # [ wasm_bindgen ( structural , method , setter , js_class = "OffscreenCanvasRenderingContext2D" , js_name = strokeStyle ) ]
    #[doc = "Setter for the `strokeStyle` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/strokeStyle)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvasRenderingContext2D`*"]
    pub fn set_stroke_style(this: &OffscreenCanvasRenderingContext2D, value: &::wasm_bindgen::JsValue);
    # [ wasm_bindgen ( structural , method , getter , js_class = "OffscreenCanvasRenderingContext2D" , js_name = fillStyle ) ]
    #[doc = "Getter for the `fillStyle` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/fillStyle)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvasRenderingContext2D`*"]
    pub fn fill_style(this: &OffscreenCanvasRenderingContext2D) -> ::wasm_bindgen::JsValue;
    # [ wasm_bindgen ( structural , method , setter , js_class = "OffscreenCanvasRenderingContext2D" , js_name = fillStyle ) ]
    #[doc = "Setter for the `fillStyle` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/fillStyle)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvasRenderingContext2D`*"]
    pub fn set_fill_style(this: &OffscreenCanvasRenderingContext2D, value: &::wasm_bindgen::JsValue);
    # [ wasm_bindgen ( structural , method , getter , js_class = "OffscreenCanvasRenderingContext2D" , js_name = filter ) ]
    #[doc = "Getter for the `filter` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/filter)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvasRenderingContext2D`*"]
    pub fn filter(this: &OffscreenCanvasRenderingContext2D) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_class = "OffscreenCanvasRenderingContext2D" , js_name = filter ) ]
    #[doc = "Setter for the `filter` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/filter)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvasRenderingContext2D`*"]
    pub fn set_filter(this: &OffscreenCanvasRenderingContext2D, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_class = "OffscreenCanvasRenderingContext2D" , js_name = imageSmoothingEnabled ) ]
    #[doc = "Getter for the `imageSmoothingEnabled` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/imageSmoothingEnabled)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvasRenderingContext2D`*"]
    pub fn image_smoothing_enabled(this: &OffscreenCanvasRenderingContext2D) -> bool;
    # [ wasm_bindgen ( structural , method , setter , js_class = "OffscreenCanvasRenderingContext2D" , js_name = imageSmoothingEnabled ) ]
    #[doc = "Setter for the `imageSmoothingEnabled` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/imageSmoothingEnabled)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvasRenderingContext2D`*"]
    pub fn set_image_smoothing_enabled(this: &OffscreenCanvasRenderingContext2D, value: bool);
    # [ wasm_bindgen ( structural , method , getter , js_class = "OffscreenCanvasRenderingContext2D" , js_name = lineWidth ) ]
    #[doc = "Getter for the `lineWidth` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/lineWidth)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvasRenderingContext2D`*"]
    pub fn line_width(this: &OffscreenCanvasRenderingContext2D) -> f64;
    # [ wasm_bindgen ( structural , method , setter , js_class = "OffscreenCanvasRenderingContext2D" , js_name = lineWidth ) ]
    #[doc = "Setter for the `lineWidth` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/lineWidth)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvasRenderingContext2D`*"]
    pub fn set_line_width(this: &OffscreenCanvasRenderingContext2D, value: f64);
    # [ wasm_bindgen ( structural , method , getter , js_class = "OffscreenCanvasRenderingContext2D" , js_name = lineCap ) ]
    #[doc = "Getter for the `lineCap` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/lineCap)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvasRenderingContext2D`*"]
    pub fn line_cap(this: &OffscreenCanvasRenderingContext2D) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_class = "OffscreenCanvasRenderingContext2D" , js_name = lineCap ) ]
    #[doc = "Setter for the `lineCap` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/lineCap)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvasRenderingContext2D`*"]
    pub fn set_line_cap(this: &OffscreenCanvasRenderingContext2D, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_class = "OffscreenCanvasRenderingContext2D" , js_name = lineJoin ) ]
    #[doc = "Getter for the `lineJoin` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/lineJoin)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvasRenderingContext2D`*"]
    pub fn line_join(this: &OffscreenCanvasRenderingContext2D) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_class = "OffscreenCanvasRenderingContext2D" , js_name = lineJoin ) ]
    #[doc = "Setter for the `lineJoin` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/lineJoin)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvasRenderingContext2D`*"]
    pub fn set_line_join(this: &OffscreenCanvasRenderingContext2D, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_class = "OffscreenCanvasRenderingContext2D" , js_name = miterLimit ) ]
    #[doc = "Getter for the `miterLimit` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/miterLimit)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvasRenderingContext2D`*"]
    pub fn miter_limit(this: &OffscreenCanvasRenderingContext2D) -> f64;
    # [ wasm_bindgen ( structural , method , setter , js_class = "OffscreenCanvasRenderingContext2D" , js_name = miterLimit ) ]
    #[doc = "Setter for the `miterLimit` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/miterLimit)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvasRenderingContext2D`*"]
    pub fn set_miter_limit(this: &OffscreenCanvasRenderingContext2D, value: f64);
    # [ wasm_bindgen ( structural , method , getter , js_class = "OffscreenCanvasRenderingContext2D" , js_name = lineDashOffset ) ]
    #[doc = "Getter for the `lineDashOffset` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/lineDashOffset)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvasRenderingContext2D`*"]
    pub fn line_dash_offset(this: &OffscreenCanvasRenderingContext2D) -> f64;
    # [ wasm_bindgen ( structural , method , setter , js_class = "OffscreenCanvasRenderingContext2D" , js_name = lineDashOffset ) ]
    #[doc = "Setter for the `lineDashOffset` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/lineDashOffset)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvasRenderingContext2D`*"]
    pub fn set_line_dash_offset(this: &OffscreenCanvasRenderingContext2D, value: f64);
    # [ wasm_bindgen ( structural , method , getter , js_class = "OffscreenCanvasRenderingContext2D" , js_name = shadowOffsetX ) ]
    #[doc = "Getter for the `shadowOffsetX` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/shadowOffsetX)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvasRenderingContext2D`*"]
    pub fn shadow_offset_x(this: &OffscreenCanvasRenderingContext2D) -> f64;
    # [ wasm_bindgen ( structural , method , setter , js_class = "OffscreenCanvasRenderingContext2D" , js_name = shadowOffsetX ) ]
    #[doc = "Setter for the `shadowOffsetX` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/shadowOffsetX)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvasRenderingContext2D`*"]
    pub fn set_shadow_offset_x(this: &OffscreenCanvasRenderingContext2D, value: f64);
    # [ wasm_bindgen ( structural , method , getter , js_class = "OffscreenCanvasRenderingContext2D" , js_name = shadowOffsetY ) ]
    #[doc = "Getter for the `shadowOffsetY` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/shadowOffsetY)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvasRenderingContext2D`*"]
    pub fn shadow_offset_y(this: &OffscreenCanvasRenderingContext2D) -> f64;
    # [ wasm_bindgen ( structural , method , setter , js_class = "OffscreenCanvasRenderingContext2D" , js_name = shadowOffsetY ) ]
    #[doc = "Setter for the `shadowOffsetY` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/shadowOffsetY)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvasRenderingContext2D`*"]
    pub fn set_shadow_offset_y(this: &OffscreenCanvasRenderingContext2D, value: f64);
    # [ wasm_bindgen ( structural , method , getter , js_class = "OffscreenCanvasRenderingContext2D" , js_name = shadowBlur ) ]
    #[doc = "Getter for the `shadowBlur` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/shadowBlur)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvasRenderingContext2D`*"]
    pub fn shadow_blur(this: &OffscreenCanvasRenderingContext2D) -> f64;
    # [ wasm_bindgen ( structural , method , setter , js_class = "OffscreenCanvasRenderingContext2D" , js_name = shadowBlur ) ]
    #[doc = "Setter for the `shadowBlur` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/shadowBlur)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvasRenderingContext2D`*"]
    pub fn set_shadow_blur(this: &OffscreenCanvasRenderingContext2D, value: f64);
    # [ wasm_bindgen ( structural , method , getter , js_class = "OffscreenCanvasRenderingContext2D" , js_name = shadowColor ) ]
    #[doc = "Getter for the `shadowColor` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/shadowColor)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvasRenderingContext2D`*"]
    pub fn shadow_color(this: &OffscreenCanvasRenderingContext2D) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_class = "OffscreenCanvasRenderingContext2D" , js_name = shadowColor ) ]
    #[doc = "Setter for the `shadowColor` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/shadowColor)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvasRenderingContext2D`*"]
    pub fn set_shadow_color(this: &OffscreenCanvasRenderingContext2D, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_class = "OffscreenCanvasRenderingContext2D" , js_name = font ) ]
    #[doc = "Getter for the `font` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/font)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvasRenderingContext2D`*"]
    pub fn font(this: &OffscreenCanvasRenderingContext2D) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_class = "OffscreenCanvasRenderingContext2D" , js_name = font ) ]
    #[doc = "Setter for the `font` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/font)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvasRenderingContext2D`*"]
    pub fn set_font(this: &OffscreenCanvasRenderingContext2D, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_class = "OffscreenCanvasRenderingContext2D" , js_name = textAlign ) ]
    #[doc = "Getter for the `textAlign` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/textAlign)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvasRenderingContext2D`*"]
    pub fn text_align(this: &OffscreenCanvasRenderingContext2D) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_class = "OffscreenCanvasRenderingContext2D" , js_name = textAlign ) ]
    #[doc = "Setter for the `textAlign` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/textAlign)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvasRenderingContext2D`*"]
    pub fn set_text_align(this: &OffscreenCanvasRenderingContext2D, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_class = "OffscreenCanvasRenderingContext2D" , js_name = textBaseline ) ]
    #[doc = "Getter for the `textBaseline` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/textBaseline)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvasRenderingContext2D`*"]
    pub fn text_baseline(this: &OffscreenCanvasRenderingContext2D) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_class = "OffscreenCanvasRenderingContext2D" , js_name = textBaseline ) ]
    #[doc = "Setter for the `textBaseline` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/textBaseline)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvasRenderingContext2D`*"]
    pub fn set_text_baseline(this: &OffscreenCanvasRenderingContext2D, value: &str);
    #[cfg(feature = "HtmlImageElement")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "OffscreenCanvasRenderingContext2D" , js_name = drawImage ) ]
    #[doc = "The `drawImage()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/drawImage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvasRenderingContext2D`, `HtmlImageElement`*"]
    pub fn draw_image_with_html_image_element(
        this: &OffscreenCanvasRenderingContext2D,
        image: &HtmlImageElement,
        dx: f64,
        dy: f64,
    ) -> Result<(), JsValue>;
    #[cfg(feature = "SvgImageElement")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "OffscreenCanvasRenderingContext2D" , js_name = drawImage ) ]
    #[doc = "The `drawImage()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/drawImage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvasRenderingContext2D`, `SvgImageElement`*"]
    pub fn draw_image_with_svg_image_element(
        this: &OffscreenCanvasRenderingContext2D,
        image: &SvgImageElement,
        dx: f64,
        dy: f64,
    ) -> Result<(), JsValue>;
    #[cfg(feature = "OffscreenCanvas")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "OffscreenCanvasRenderingContext2D" , js_name = drawImage ) ]
    #[doc = "The `drawImage()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/drawImage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvasRenderingContext2D`, `OffscreenCanvas`*"]
    pub fn draw_image_with_html_canvas_element(
        this: &OffscreenCanvasRenderingContext2D,
        image: &OffscreenCanvas,
        dx: f64,
        dy: f64,
    ) -> Result<(), JsValue>;
    #[cfg(feature = "HtmlVideoElement")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "OffscreenCanvasRenderingContext2D" , js_name = drawImage ) ]
    #[doc = "The `drawImage()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/drawImage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvasRenderingContext2D`, `HtmlVideoElement`*"]
    pub fn draw_image_with_html_video_element(
        this: &OffscreenCanvasRenderingContext2D,
        image: &HtmlVideoElement,
        dx: f64,
        dy: f64,
    ) -> Result<(), JsValue>;
    #[cfg(feature = "ImageBitmap")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "OffscreenCanvasRenderingContext2D" , js_name = drawImage ) ]
    #[doc = "The `drawImage()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/drawImage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvasRenderingContext2D`, `ImageBitmap`*"]
    pub fn draw_image_with_image_bitmap(
        this: &OffscreenCanvasRenderingContext2D,
        image: &ImageBitmap,
        dx: f64,
        dy: f64,
    ) -> Result<(), JsValue>;
    #[cfg(feature = "HtmlImageElement")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "OffscreenCanvasRenderingContext2D" , js_name = drawImage ) ]
    #[doc = "The `drawImage()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/drawImage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvasRenderingContext2D`, `HtmlImageElement`*"]
    pub fn draw_image_with_html_image_element_and_dw_and_dh(
        this: &OffscreenCanvasRenderingContext2D,
        image: &HtmlImageElement,
        dx: f64,
        dy: f64,
        dw: f64,
        dh: f64,
    ) -> Result<(), JsValue>;
    #[cfg(feature = "SvgImageElement")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "OffscreenCanvasRenderingContext2D" , js_name = drawImage ) ]
    #[doc = "The `drawImage()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/drawImage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvasRenderingContext2D`, `SvgImageElement`*"]
    pub fn draw_image_with_svg_image_element_and_dw_and_dh(
        this: &OffscreenCanvasRenderingContext2D,
        image: &SvgImageElement,
        dx: f64,
        dy: f64,
        dw: f64,
        dh: f64,
    ) -> Result<(), JsValue>;
    #[cfg(feature = "OffscreenCanvas")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "OffscreenCanvasRenderingContext2D" , js_name = drawImage ) ]
    #[doc = "The `drawImage()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/drawImage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvasRenderingContext2D`, `OffscreenCanvas`*"]
    pub fn draw_image_with_html_canvas_element_and_dw_and_dh(
        this: &OffscreenCanvasRenderingContext2D,
        image: &OffscreenCanvas,
        dx: f64,
        dy: f64,
        dw: f64,
        dh: f64,
    ) -> Result<(), JsValue>;
    #[cfg(feature = "HtmlVideoElement")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "OffscreenCanvasRenderingContext2D" , js_name = drawImage ) ]
    #[doc = "The `drawImage()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/drawImage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvasRenderingContext2D`, `HtmlVideoElement`*"]
    pub fn draw_image_with_html_video_element_and_dw_and_dh(
        this: &OffscreenCanvasRenderingContext2D,
        image: &HtmlVideoElement,
        dx: f64,
        dy: f64,
        dw: f64,
        dh: f64,
    ) -> Result<(), JsValue>;
    #[cfg(feature = "ImageBitmap")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "OffscreenCanvasRenderingContext2D" , js_name = drawImage ) ]
    #[doc = "The `drawImage()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/drawImage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvasRenderingContext2D`, `ImageBitmap`*"]
    pub fn draw_image_with_image_bitmap_and_dw_and_dh(
        this: &OffscreenCanvasRenderingContext2D,
        image: &ImageBitmap,
        dx: f64,
        dy: f64,
        dw: f64,
        dh: f64,
    ) -> Result<(), JsValue>;
    #[cfg(feature = "HtmlImageElement")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "OffscreenCanvasRenderingContext2D" , js_name = drawImage ) ]
    #[doc = "The `drawImage()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/drawImage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvasRenderingContext2D`, `HtmlImageElement`*"]
    pub fn draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
        this: &OffscreenCanvasRenderingContext2D,
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
    # [ wasm_bindgen ( catch , method , structural , js_class = "OffscreenCanvasRenderingContext2D" , js_name = drawImage ) ]
    #[doc = "The `drawImage()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/drawImage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvasRenderingContext2D`, `SvgImageElement`*"]
    pub fn draw_image_with_svg_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
        this: &OffscreenCanvasRenderingContext2D,
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
    #[cfg(feature = "OffscreenCanvas")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "OffscreenCanvasRenderingContext2D" , js_name = drawImage ) ]
    #[doc = "The `drawImage()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/drawImage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvasRenderingContext2D`, `OffscreenCanvas`*"]
    pub fn draw_image_with_html_canvas_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
        this: &OffscreenCanvasRenderingContext2D,
        image: &OffscreenCanvas,
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
    # [ wasm_bindgen ( catch , method , structural , js_class = "OffscreenCanvasRenderingContext2D" , js_name = drawImage ) ]
    #[doc = "The `drawImage()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/drawImage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvasRenderingContext2D`, `HtmlVideoElement`*"]
    pub fn draw_image_with_html_video_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
        this: &OffscreenCanvasRenderingContext2D,
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
    # [ wasm_bindgen ( catch , method , structural , js_class = "OffscreenCanvasRenderingContext2D" , js_name = drawImage ) ]
    #[doc = "The `drawImage()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/drawImage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvasRenderingContext2D`, `ImageBitmap`*"]
    pub fn draw_image_with_image_bitmap_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
        this: &OffscreenCanvasRenderingContext2D,
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
    # [ wasm_bindgen ( method , structural , js_class = "OffscreenCanvasRenderingContext2D" , js_name = beginPath ) ]
    #[doc = "The `beginPath()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/beginPath)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvasRenderingContext2D`*"]
    pub fn begin_path(this: &OffscreenCanvasRenderingContext2D);
    # [ wasm_bindgen ( method , structural , js_class = "OffscreenCanvasRenderingContext2D" , js_name = clip ) ]
    #[doc = "The `clip()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/clip)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvasRenderingContext2D`*"]
    pub fn clip(this: &OffscreenCanvasRenderingContext2D);
    #[cfg(feature = "CanvasWindingRule")]
    # [ wasm_bindgen ( method , structural , js_class = "OffscreenCanvasRenderingContext2D" , js_name = clip ) ]
    #[doc = "The `clip()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/clip)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvasRenderingContext2D`, `CanvasWindingRule`*"]
    pub fn clip_with_canvas_winding_rule(
        this: &OffscreenCanvasRenderingContext2D,
        winding: CanvasWindingRule,
    );
    #[cfg(feature = "Path2d")]
    # [ wasm_bindgen ( method , structural , js_class = "OffscreenCanvasRenderingContext2D" , js_name = clip ) ]
    #[doc = "The `clip()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/clip)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvasRenderingContext2D`, `Path2d`*"]
    pub fn clip_with_path_2d(this: &OffscreenCanvasRenderingContext2D, path: &Path2d);
    #[cfg(all(feature = "CanvasWindingRule", feature = "Path2d",))]
    # [ wasm_bindgen ( method , structural , js_class = "OffscreenCanvasRenderingContext2D" , js_name = clip ) ]
    #[doc = "The `clip()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/clip)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvasRenderingContext2D`, `CanvasWindingRule`, `Path2d`*"]
    pub fn clip_with_path_2d_and_winding(
        this: &OffscreenCanvasRenderingContext2D,
        path: &Path2d,
        winding: CanvasWindingRule,
    );
    # [ wasm_bindgen ( method , structural , js_class = "OffscreenCanvasRenderingContext2D" , js_name = fill ) ]
    #[doc = "The `fill()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/fill)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvasRenderingContext2D`*"]
    pub fn fill(this: &OffscreenCanvasRenderingContext2D);
    #[cfg(feature = "CanvasWindingRule")]
    # [ wasm_bindgen ( method , structural , js_class = "OffscreenCanvasRenderingContext2D" , js_name = fill ) ]
    #[doc = "The `fill()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/fill)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvasRenderingContext2D`, `CanvasWindingRule`*"]
    pub fn fill_with_canvas_winding_rule(
        this: &OffscreenCanvasRenderingContext2D,
        winding: CanvasWindingRule,
    );
    #[cfg(feature = "Path2d")]
    # [ wasm_bindgen ( method , structural , js_class = "OffscreenCanvasRenderingContext2D" , js_name = fill ) ]
    #[doc = "The `fill()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/fill)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvasRenderingContext2D`, `Path2d`*"]
    pub fn fill_with_path_2d(this: &OffscreenCanvasRenderingContext2D, path: &Path2d);
    #[cfg(all(feature = "CanvasWindingRule", feature = "Path2d",))]
    # [ wasm_bindgen ( method , structural , js_class = "OffscreenCanvasRenderingContext2D" , js_name = fill ) ]
    #[doc = "The `fill()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/fill)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvasRenderingContext2D`, `CanvasWindingRule`, `Path2d`*"]
    pub fn fill_with_path_2d_and_winding(
        this: &OffscreenCanvasRenderingContext2D,
        path: &Path2d,
        winding: CanvasWindingRule,
    );
    # [ wasm_bindgen ( method , structural , js_class = "OffscreenCanvasRenderingContext2D" , js_name = isPointInPath ) ]
    #[doc = "The `isPointInPath()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/isPointInPath)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvasRenderingContext2D`*"]
    pub fn is_point_in_path_with_f64(this: &OffscreenCanvasRenderingContext2D, x: f64, y: f64) -> bool;
    #[cfg(feature = "CanvasWindingRule")]
    # [ wasm_bindgen ( method , structural , js_class = "OffscreenCanvasRenderingContext2D" , js_name = isPointInPath ) ]
    #[doc = "The `isPointInPath()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/isPointInPath)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvasRenderingContext2D`, `CanvasWindingRule`*"]
    pub fn is_point_in_path_with_f64_and_canvas_winding_rule(
        this: &OffscreenCanvasRenderingContext2D,
        x: f64,
        y: f64,
        winding: CanvasWindingRule,
    ) -> bool;
    #[cfg(feature = "Path2d")]
    # [ wasm_bindgen ( method , structural , js_class = "OffscreenCanvasRenderingContext2D" , js_name = isPointInPath ) ]
    #[doc = "The `isPointInPath()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/isPointInPath)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvasRenderingContext2D`, `Path2d`*"]
    pub fn is_point_in_path_with_path_2d_and_f64(
        this: &OffscreenCanvasRenderingContext2D,
        path: &Path2d,
        x: f64,
        y: f64,
    ) -> bool;
    #[cfg(all(feature = "CanvasWindingRule", feature = "Path2d",))]
    # [ wasm_bindgen ( method , structural , js_class = "OffscreenCanvasRenderingContext2D" , js_name = isPointInPath ) ]
    #[doc = "The `isPointInPath()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/isPointInPath)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvasRenderingContext2D`, `CanvasWindingRule`, `Path2d`*"]
    pub fn is_point_in_path_with_path_2d_and_f64_and_winding(
        this: &OffscreenCanvasRenderingContext2D,
        path: &Path2d,
        x: f64,
        y: f64,
        winding: CanvasWindingRule,
    ) -> bool;
    # [ wasm_bindgen ( method , structural , js_class = "OffscreenCanvasRenderingContext2D" , js_name = isPointInStroke ) ]
    #[doc = "The `isPointInStroke()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/isPointInStroke)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvasRenderingContext2D`*"]
    pub fn is_point_in_stroke_with_x_and_y(this: &OffscreenCanvasRenderingContext2D, x: f64, y: f64)
                                           -> bool;
    #[cfg(feature = "Path2d")]
    # [ wasm_bindgen ( method , structural , js_class = "OffscreenCanvasRenderingContext2D" , js_name = isPointInStroke ) ]
    #[doc = "The `isPointInStroke()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/isPointInStroke)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvasRenderingContext2D`, `Path2d`*"]
    pub fn is_point_in_stroke_with_path_and_x_and_y(
        this: &OffscreenCanvasRenderingContext2D,
        path: &Path2d,
        x: f64,
        y: f64,
    ) -> bool;
    # [ wasm_bindgen ( method , structural , js_class = "OffscreenCanvasRenderingContext2D" , js_name = stroke ) ]
    #[doc = "The `stroke()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/stroke)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvasRenderingContext2D`*"]
    pub fn stroke(this: &OffscreenCanvasRenderingContext2D);
    #[cfg(feature = "Path2d")]
    # [ wasm_bindgen ( method , structural , js_class = "OffscreenCanvasRenderingContext2D" , js_name = stroke ) ]
    #[doc = "The `stroke()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/stroke)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvasRenderingContext2D`, `Path2d`*"]
    pub fn stroke_with_path(this: &OffscreenCanvasRenderingContext2D, path: &Path2d);
    #[cfg(feature = "CanvasGradient")]
    # [ wasm_bindgen ( method , structural , js_class = "OffscreenCanvasRenderingContext2D" , js_name = createLinearGradient ) ]
    #[doc = "The `createLinearGradient()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/createLinearGradient)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasGradient`, `OffscreenCanvasRenderingContext2D`*"]
    pub fn create_linear_gradient(
        this: &OffscreenCanvasRenderingContext2D,
        x0: f64,
        y0: f64,
        x1: f64,
        y1: f64,
    ) -> CanvasGradient;
    #[cfg(all(feature = "CanvasPattern", feature = "HtmlImageElement",))]
    # [ wasm_bindgen ( catch , method , structural , js_class = "OffscreenCanvasRenderingContext2D" , js_name = createPattern ) ]
    #[doc = "The `createPattern()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/createPattern)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasPattern`, `OffscreenCanvasRenderingContext2D`, `HtmlImageElement`*"]
    pub fn create_pattern_with_html_image_element(
        this: &OffscreenCanvasRenderingContext2D,
        image: &HtmlImageElement,
        repetition: &str,
    ) -> Result<Option<CanvasPattern>, JsValue>;
    #[cfg(all(feature = "CanvasPattern", feature = "SvgImageElement",))]
    # [ wasm_bindgen ( catch , method , structural , js_class = "OffscreenCanvasRenderingContext2D" , js_name = createPattern ) ]
    #[doc = "The `createPattern()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/createPattern)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasPattern`, `OffscreenCanvasRenderingContext2D`, `SvgImageElement`*"]
    pub fn create_pattern_with_svg_image_element(
        this: &OffscreenCanvasRenderingContext2D,
        image: &SvgImageElement,
        repetition: &str,
    ) -> Result<Option<CanvasPattern>, JsValue>;
    #[cfg(all(feature = "CanvasPattern", feature = "OffscreenCanvas",))]
    # [ wasm_bindgen ( catch , method , structural , js_class = "OffscreenCanvasRenderingContext2D" , js_name = createPattern ) ]
    #[doc = "The `createPattern()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/createPattern)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasPattern`, `OffscreenCanvasRenderingContext2D`, `OffscreenCanvas`*"]
    pub fn create_pattern_with_html_canvas_element(
        this: &OffscreenCanvasRenderingContext2D,
        image: &OffscreenCanvas,
        repetition: &str,
    ) -> Result<Option<CanvasPattern>, JsValue>;
    #[cfg(all(feature = "CanvasPattern", feature = "HtmlVideoElement",))]
    # [ wasm_bindgen ( catch , method , structural , js_class = "OffscreenCanvasRenderingContext2D" , js_name = createPattern ) ]
    #[doc = "The `createPattern()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/createPattern)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasPattern`, `OffscreenCanvasRenderingContext2D`, `HtmlVideoElement`*"]
    pub fn create_pattern_with_html_video_element(
        this: &OffscreenCanvasRenderingContext2D,
        image: &HtmlVideoElement,
        repetition: &str,
    ) -> Result<Option<CanvasPattern>, JsValue>;
    #[cfg(all(feature = "CanvasPattern", feature = "ImageBitmap",))]
    # [ wasm_bindgen ( catch , method , structural , js_class = "OffscreenCanvasRenderingContext2D" , js_name = createPattern ) ]
    #[doc = "The `createPattern()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/createPattern)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasPattern`, `OffscreenCanvasRenderingContext2D`, `ImageBitmap`*"]
    pub fn create_pattern_with_image_bitmap(
        this: &OffscreenCanvasRenderingContext2D,
        image: &ImageBitmap,
        repetition: &str,
    ) -> Result<Option<CanvasPattern>, JsValue>;
    #[cfg(feature = "CanvasGradient")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "OffscreenCanvasRenderingContext2D" , js_name = createRadialGradient ) ]
    #[doc = "The `createRadialGradient()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/createRadialGradient)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasGradient`, `OffscreenCanvasRenderingContext2D`*"]
    pub fn create_radial_gradient(
        this: &OffscreenCanvasRenderingContext2D,
        x0: f64,
        y0: f64,
        r0: f64,
        x1: f64,
        y1: f64,
        r1: f64,
    ) -> Result<CanvasGradient, JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_class = "OffscreenCanvasRenderingContext2D" , js_name = addHitRegion ) ]
    #[doc = "The `addHitRegion()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/addHitRegion)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvasRenderingContext2D`*"]
    pub fn add_hit_region(this: &OffscreenCanvasRenderingContext2D) -> Result<(), JsValue>;
    #[cfg(feature = "HitRegionOptions")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "OffscreenCanvasRenderingContext2D" , js_name = addHitRegion ) ]
    #[doc = "The `addHitRegion()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/addHitRegion)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvasRenderingContext2D`, `HitRegionOptions`*"]
    pub fn add_hit_region_with_options(
        this: &OffscreenCanvasRenderingContext2D,
        options: &HitRegionOptions,
    ) -> Result<(), JsValue>;
    # [ wasm_bindgen ( method , structural , js_class = "OffscreenCanvasRenderingContext2D" , js_name = clearHitRegions ) ]
    #[doc = "The `clearHitRegions()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/clearHitRegions)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvasRenderingContext2D`*"]
    pub fn clear_hit_regions(this: &OffscreenCanvasRenderingContext2D);
    # [ wasm_bindgen ( method , structural , js_class = "OffscreenCanvasRenderingContext2D" , js_name = removeHitRegion ) ]
    #[doc = "The `removeHitRegion()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/removeHitRegion)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvasRenderingContext2D`*"]
    pub fn remove_hit_region(this: &OffscreenCanvasRenderingContext2D, id: &str);
    #[cfg(feature = "ImageData")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "OffscreenCanvasRenderingContext2D" , js_name = createImageData ) ]
    #[doc = "The `createImageData()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/createImageData)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvasRenderingContext2D`, `ImageData`*"]
    pub fn create_image_data_with_sw_and_sh(
        this: &OffscreenCanvasRenderingContext2D,
        sw: f64,
        sh: f64,
    ) -> Result<ImageData, JsValue>;
    #[cfg(feature = "ImageData")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "OffscreenCanvasRenderingContext2D" , js_name = createImageData ) ]
    #[doc = "The `createImageData()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/createImageData)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvasRenderingContext2D`, `ImageData`*"]
    pub fn create_image_data_with_imagedata(
        this: &OffscreenCanvasRenderingContext2D,
        imagedata: &ImageData,
    ) -> Result<ImageData, JsValue>;
    #[cfg(feature = "ImageData")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "OffscreenCanvasRenderingContext2D" , js_name = getImageData ) ]
    #[doc = "The `getImageData()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/getImageData)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvasRenderingContext2D`, `ImageData`*"]
    pub fn get_image_data(
        this: &OffscreenCanvasRenderingContext2D,
        sx: f64,
        sy: f64,
        sw: f64,
        sh: f64,
    ) -> Result<ImageData, JsValue>;
    #[cfg(feature = "ImageData")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "OffscreenCanvasRenderingContext2D" , js_name = putImageData ) ]
    #[doc = "The `putImageData()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/putImageData)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvasRenderingContext2D`, `ImageData`*"]
    pub fn put_image_data(
        this: &OffscreenCanvasRenderingContext2D,
        imagedata: &ImageData,
        dx: f64,
        dy: f64,
    ) -> Result<(), JsValue>;
    #[cfg(feature = "ImageData")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "OffscreenCanvasRenderingContext2D" , js_name = putImageData ) ]
    #[doc = "The `putImageData()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/putImageData)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvasRenderingContext2D`, `ImageData`*"]
    pub fn put_image_data_with_dirty_x_and_dirty_y_and_dirty_width_and_dirty_height(
        this: &OffscreenCanvasRenderingContext2D,
        imagedata: &ImageData,
        dx: f64,
        dy: f64,
        dirty_x: f64,
        dirty_y: f64,
        dirty_width: f64,
        dirty_height: f64,
    ) -> Result<(), JsValue>;
    # [ wasm_bindgen ( method , structural , js_class = "OffscreenCanvasRenderingContext2D" , js_name = getLineDash ) ]
    #[doc = "The `getLineDash()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/getLineDash)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvasRenderingContext2D`*"]
    pub fn get_line_dash(this: &OffscreenCanvasRenderingContext2D) -> ::js_sys::Array;
    # [ wasm_bindgen ( catch , method , structural , js_class = "OffscreenCanvasRenderingContext2D" , js_name = setLineDash ) ]
    #[doc = "The `setLineDash()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/setLineDash)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvasRenderingContext2D`*"]
    pub fn set_line_dash(
        this: &OffscreenCanvasRenderingContext2D,
        segments: &::wasm_bindgen::JsValue,
    ) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_class = "OffscreenCanvasRenderingContext2D" , js_name = arc ) ]
    #[doc = "The `arc()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/arc)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvasRenderingContext2D`*"]
    pub fn arc(
        this: &OffscreenCanvasRenderingContext2D,
        x: f64,
        y: f64,
        radius: f64,
        start_angle: f64,
        end_angle: f64,
    ) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_class = "OffscreenCanvasRenderingContext2D" , js_name = arc ) ]
    #[doc = "The `arc()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/arc)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvasRenderingContext2D`*"]
    pub fn arc_with_anticlockwise(
        this: &OffscreenCanvasRenderingContext2D,
        x: f64,
        y: f64,
        radius: f64,
        start_angle: f64,
        end_angle: f64,
        anticlockwise: bool,
    ) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_class = "OffscreenCanvasRenderingContext2D" , js_name = arcTo ) ]
    #[doc = "The `arcTo()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/arcTo)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvasRenderingContext2D`*"]
    pub fn arc_to(
        this: &OffscreenCanvasRenderingContext2D,
        x1: f64,
        y1: f64,
        x2: f64,
        y2: f64,
        radius: f64,
    ) -> Result<(), JsValue>;
    # [ wasm_bindgen ( method , structural , js_class = "OffscreenCanvasRenderingContext2D" , js_name = bezierCurveTo ) ]
    #[doc = "The `bezierCurveTo()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/bezierCurveTo)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvasRenderingContext2D`*"]
    pub fn bezier_curve_to(
        this: &OffscreenCanvasRenderingContext2D,
        cp1x: f64,
        cp1y: f64,
        cp2x: f64,
        cp2y: f64,
        x: f64,
        y: f64,
    );
    # [ wasm_bindgen ( method , structural , js_class = "OffscreenCanvasRenderingContext2D" , js_name = closePath ) ]
    #[doc = "The `closePath()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/closePath)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvasRenderingContext2D`*"]
    pub fn close_path(this: &OffscreenCanvasRenderingContext2D);
    # [ wasm_bindgen ( catch , method , structural , js_class = "OffscreenCanvasRenderingContext2D" , js_name = ellipse ) ]
    #[doc = "The `ellipse()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/ellipse)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvasRenderingContext2D`*"]
    pub fn ellipse(
        this: &OffscreenCanvasRenderingContext2D,
        x: f64,
        y: f64,
        radius_x: f64,
        radius_y: f64,
        rotation: f64,
        start_angle: f64,
        end_angle: f64,
    ) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_class = "OffscreenCanvasRenderingContext2D" , js_name = ellipse ) ]
    #[doc = "The `ellipse()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/ellipse)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvasRenderingContext2D`*"]
    pub fn ellipse_with_anticlockwise(
        this: &OffscreenCanvasRenderingContext2D,
        x: f64,
        y: f64,
        radius_x: f64,
        radius_y: f64,
        rotation: f64,
        start_angle: f64,
        end_angle: f64,
        anticlockwise: bool,
    ) -> Result<(), JsValue>;
    # [ wasm_bindgen ( method , structural , js_class = "OffscreenCanvasRenderingContext2D" , js_name = lineTo ) ]
    #[doc = "The `lineTo()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/lineTo)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvasRenderingContext2D`*"]
    pub fn line_to(this: &OffscreenCanvasRenderingContext2D, x: f64, y: f64);
    # [ wasm_bindgen ( method , structural , js_class = "OffscreenCanvasRenderingContext2D" , js_name = moveTo ) ]
    #[doc = "The `moveTo()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/moveTo)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvasRenderingContext2D`*"]
    pub fn move_to(this: &OffscreenCanvasRenderingContext2D, x: f64, y: f64);
    # [ wasm_bindgen ( method , structural , js_class = "OffscreenCanvasRenderingContext2D" , js_name = quadraticCurveTo ) ]
    #[doc = "The `quadraticCurveTo()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/quadraticCurveTo)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvasRenderingContext2D`*"]
    pub fn quadratic_curve_to(this: &OffscreenCanvasRenderingContext2D, cpx: f64, cpy: f64, x: f64, y: f64);
    # [ wasm_bindgen ( method , structural , js_class = "OffscreenCanvasRenderingContext2D" , js_name = rect ) ]
    #[doc = "The `rect()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/rect)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvasRenderingContext2D`*"]
    pub fn rect(this: &OffscreenCanvasRenderingContext2D, x: f64, y: f64, w: f64, h: f64);
    # [ wasm_bindgen ( method , structural , js_class = "OffscreenCanvasRenderingContext2D" , js_name = clearRect ) ]
    #[doc = "The `clearRect()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/clearRect)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvasRenderingContext2D`*"]
    pub fn clear_rect(this: &OffscreenCanvasRenderingContext2D, x: f64, y: f64, w: f64, h: f64);
    # [ wasm_bindgen ( method , structural , js_class = "OffscreenCanvasRenderingContext2D" , js_name = fillRect ) ]
    #[doc = "The `fillRect()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/fillRect)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvasRenderingContext2D`*"]
    pub fn fill_rect(this: &OffscreenCanvasRenderingContext2D, x: f64, y: f64, w: f64, h: f64);
    # [ wasm_bindgen ( method , structural , js_class = "OffscreenCanvasRenderingContext2D" , js_name = strokeRect ) ]
    #[doc = "The `strokeRect()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/strokeRect)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvasRenderingContext2D`*"]
    pub fn stroke_rect(this: &OffscreenCanvasRenderingContext2D, x: f64, y: f64, w: f64, h: f64);
    # [ wasm_bindgen ( method , structural , js_class = "OffscreenCanvasRenderingContext2D" , js_name = restore ) ]
    #[doc = "The `restore()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/restore)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvasRenderingContext2D`*"]
    pub fn restore(this: &OffscreenCanvasRenderingContext2D);
    # [ wasm_bindgen ( method , structural , js_class = "OffscreenCanvasRenderingContext2D" , js_name = save ) ]
    #[doc = "The `save()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/save)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvasRenderingContext2D`*"]
    pub fn save(this: &OffscreenCanvasRenderingContext2D);
    # [ wasm_bindgen ( catch , method , structural , js_class = "OffscreenCanvasRenderingContext2D" , js_name = fillText ) ]
    #[doc = "The `fillText()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/fillText)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvasRenderingContext2D`*"]
    pub fn fill_text(
        this: &OffscreenCanvasRenderingContext2D,
        text: &str,
        x: f64,
        y: f64,
    ) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_class = "OffscreenCanvasRenderingContext2D" , js_name = fillText ) ]
    #[doc = "The `fillText()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/fillText)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvasRenderingContext2D`*"]
    pub fn fill_text_with_max_width(
        this: &OffscreenCanvasRenderingContext2D,
        text: &str,
        x: f64,
        y: f64,
        max_width: f64,
    ) -> Result<(), JsValue>;
    #[cfg(feature = "TextMetrics")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "OffscreenCanvasRenderingContext2D" , js_name = measureText ) ]
    #[doc = "The `measureText()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/measureText)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvasRenderingContext2D`, `TextMetrics`*"]
    pub fn measure_text(
        this: &OffscreenCanvasRenderingContext2D,
        text: &str,
    ) -> Result<TextMetrics, JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_class = "OffscreenCanvasRenderingContext2D" , js_name = strokeText ) ]
    #[doc = "The `strokeText()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/strokeText)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvasRenderingContext2D`*"]
    pub fn stroke_text(
        this: &OffscreenCanvasRenderingContext2D,
        text: &str,
        x: f64,
        y: f64,
    ) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_class = "OffscreenCanvasRenderingContext2D" , js_name = strokeText ) ]
    #[doc = "The `strokeText()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/strokeText)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvasRenderingContext2D`*"]
    pub fn stroke_text_with_max_width(
        this: &OffscreenCanvasRenderingContext2D,
        text: &str,
        x: f64,
        y: f64,
        max_width: f64,
    ) -> Result<(), JsValue>;
    #[cfg(feature = "DomMatrix")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "OffscreenCanvasRenderingContext2D" , js_name = getTransform ) ]
    #[doc = "The `getTransform()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/getTransform)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvasRenderingContext2D`, `DomMatrix`*"]
    pub fn get_transform(this: &OffscreenCanvasRenderingContext2D) -> Result<DomMatrix, JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_class = "OffscreenCanvasRenderingContext2D" , js_name = resetTransform ) ]
    #[doc = "The `resetTransform()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/resetTransform)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvasRenderingContext2D`*"]
    pub fn reset_transform(this: &OffscreenCanvasRenderingContext2D) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_class = "OffscreenCanvasRenderingContext2D" , js_name = rotate ) ]
    #[doc = "The `rotate()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/rotate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvasRenderingContext2D`*"]
    pub fn rotate(this: &OffscreenCanvasRenderingContext2D, angle: f64) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_class = "OffscreenCanvasRenderingContext2D" , js_name = scale ) ]
    #[doc = "The `scale()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/scale)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvasRenderingContext2D`*"]
    pub fn scale(this: &OffscreenCanvasRenderingContext2D, x: f64, y: f64) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_class = "OffscreenCanvasRenderingContext2D" , js_name = setTransform ) ]
    #[doc = "The `setTransform()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/setTransform)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvasRenderingContext2D`*"]
    pub fn set_transform(
        this: &OffscreenCanvasRenderingContext2D,
        a: f64,
        b: f64,
        c: f64,
        d: f64,
        e: f64,
        f: f64,
    ) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_class = "OffscreenCanvasRenderingContext2D" , js_name = transform ) ]
    #[doc = "The `transform()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/transform)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvasRenderingContext2D`*"]
    pub fn transform(
        this: &OffscreenCanvasRenderingContext2D,
        a: f64,
        b: f64,
        c: f64,
        d: f64,
        e: f64,
        f: f64,
    ) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_class = "OffscreenCanvasRenderingContext2D" , js_name = translate ) ]
    #[doc = "The `translate()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/translate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvasRenderingContext2D`*"]
    pub fn translate(this: &OffscreenCanvasRenderingContext2D, x: f64, y: f64) -> Result<(), JsValue>;
    #[cfg(feature = "Element")]
    # [ wasm_bindgen ( method , structural , js_class = "OffscreenCanvasRenderingContext2D" , js_name = drawCustomFocusRing ) ]
    #[doc = "The `drawCustomFocusRing()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/drawCustomFocusRing)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvasRenderingContext2D`, `Element`*"]
    pub fn draw_custom_focus_ring(this: &OffscreenCanvasRenderingContext2D, element: &Element) -> bool;
    #[cfg(feature = "Element")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "OffscreenCanvasRenderingContext2D" , js_name = drawFocusIfNeeded ) ]
    #[doc = "The `drawFocusIfNeeded()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/drawFocusIfNeeded)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvasRenderingContext2D`, `Element`*"]
    pub fn draw_focus_if_needed(
        this: &OffscreenCanvasRenderingContext2D,
        element: &Element,
    ) -> Result<(), JsValue>;
}
impl OffscreenCanvasRenderingContext2D {
    #[doc = "The `OffscreenCanvasRenderingContext2D.DRAWWINDOW_DRAW_CARET` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvasRenderingContext2D`*"]
    pub const DRAWWINDOW_DRAW_CARET: u32 = 1u64 as u32;
    #[doc = "The `OffscreenCanvasRenderingContext2D.DRAWWINDOW_DO_NOT_FLUSH` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvasRenderingContext2D`*"]
    pub const DRAWWINDOW_DO_NOT_FLUSH: u32 = 2u64 as u32;
    #[doc = "The `OffscreenCanvasRenderingContext2D.DRAWWINDOW_DRAW_VIEW` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvasRenderingContext2D`*"]
    pub const DRAWWINDOW_DRAW_VIEW: u32 = 4u64 as u32;
    #[doc = "The `OffscreenCanvasRenderingContext2D.DRAWWINDOW_USE_WIDGET_LAYERS` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvasRenderingContext2D`*"]
    pub const DRAWWINDOW_USE_WIDGET_LAYERS: u32 = 8u64 as u32;
    #[doc = "The `OffscreenCanvasRenderingContext2D.DRAWWINDOW_ASYNC_DECODE_IMAGES` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvasRenderingContext2D`*"]
    pub const DRAWWINDOW_ASYNC_DECODE_IMAGES: u32 = 16u64 as u32;
}
