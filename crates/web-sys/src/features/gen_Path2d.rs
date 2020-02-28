use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = Path2D , typescript_name = Path2D ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `Path2d` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Path2D)\n\n*This API requires the following crate features to be activated: `Path2d`*"]
    pub type Path2d;
    #[wasm_bindgen(catch, js_class = "Path2D", constructor)]
    #[doc = "The `new Path2d(..)` constructor, creating a new instance of `Path2d`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Path2D/Path2D)\n\n*This API requires the following crate features to be activated: `Path2d`*"]
    pub fn new(this: &Path2d) -> Result<Path2d, JsValue>;
    #[wasm_bindgen(catch, js_class = "Path2D", constructor)]
    #[doc = "The `new Path2d(..)` constructor, creating a new instance of `Path2d`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Path2D/Path2D)\n\n*This API requires the following crate features to be activated: `Path2d`*"]
    pub fn new_with_other(this: &Path2d, other: &Path2d) -> Result<Path2d, JsValue>;
    #[wasm_bindgen(catch, js_class = "Path2D", constructor)]
    #[doc = "The `new Path2d(..)` constructor, creating a new instance of `Path2d`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Path2D/Path2D)\n\n*This API requires the following crate features to be activated: `Path2d`*"]
    pub fn new_with_path_string(this: &Path2d, path_string: &str) -> Result<Path2d, JsValue>;
    # [ wasm_bindgen ( method , structural , js_class = "Path2D" , js_name = addPath ) ]
    #[doc = "The `addPath()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Path2D/addPath)\n\n*This API requires the following crate features to be activated: `Path2d`*"]
    pub fn add_path(this: &Path2d, path: &Path2d);
    #[cfg(feature = "SvgMatrix")]
    # [ wasm_bindgen ( method , structural , js_class = "Path2D" , js_name = addPath ) ]
    #[doc = "The `addPath()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Path2D/addPath)\n\n*This API requires the following crate features to be activated: `Path2d`, `SvgMatrix`*"]
    pub fn add_path_with_transformation(this: &Path2d, path: &Path2d, transformation: &SvgMatrix);
    # [ wasm_bindgen ( catch , method , structural , js_class = "Path2D" , js_name = arc ) ]
    #[doc = "The `arc()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Path2D/arc)\n\n*This API requires the following crate features to be activated: `Path2d`*"]
    pub fn arc(
        this: &Path2d,
        x: f64,
        y: f64,
        radius: f64,
        start_angle: f64,
        end_angle: f64,
    ) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_class = "Path2D" , js_name = arc ) ]
    #[doc = "The `arc()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Path2D/arc)\n\n*This API requires the following crate features to be activated: `Path2d`*"]
    pub fn arc_with_anticlockwise(
        this: &Path2d,
        x: f64,
        y: f64,
        radius: f64,
        start_angle: f64,
        end_angle: f64,
        anticlockwise: bool,
    ) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_class = "Path2D" , js_name = arcTo ) ]
    #[doc = "The `arcTo()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Path2D/arcTo)\n\n*This API requires the following crate features to be activated: `Path2d`*"]
    pub fn arc_to(
        this: &Path2d,
        x1: f64,
        y1: f64,
        x2: f64,
        y2: f64,
        radius: f64,
    ) -> Result<(), JsValue>;
    # [ wasm_bindgen ( method , structural , js_class = "Path2D" , js_name = bezierCurveTo ) ]
    #[doc = "The `bezierCurveTo()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Path2D/bezierCurveTo)\n\n*This API requires the following crate features to be activated: `Path2d`*"]
    pub fn bezier_curve_to(
        this: &Path2d,
        cp1x: f64,
        cp1y: f64,
        cp2x: f64,
        cp2y: f64,
        x: f64,
        y: f64,
    );
    # [ wasm_bindgen ( method , structural , js_class = "Path2D" , js_name = closePath ) ]
    #[doc = "The `closePath()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Path2D/closePath)\n\n*This API requires the following crate features to be activated: `Path2d`*"]
    pub fn close_path(this: &Path2d);
    # [ wasm_bindgen ( catch , method , structural , js_class = "Path2D" , js_name = ellipse ) ]
    #[doc = "The `ellipse()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Path2D/ellipse)\n\n*This API requires the following crate features to be activated: `Path2d`*"]
    pub fn ellipse(
        this: &Path2d,
        x: f64,
        y: f64,
        radius_x: f64,
        radius_y: f64,
        rotation: f64,
        start_angle: f64,
        end_angle: f64,
    ) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_class = "Path2D" , js_name = ellipse ) ]
    #[doc = "The `ellipse()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Path2D/ellipse)\n\n*This API requires the following crate features to be activated: `Path2d`*"]
    pub fn ellipse_with_anticlockwise(
        this: &Path2d,
        x: f64,
        y: f64,
        radius_x: f64,
        radius_y: f64,
        rotation: f64,
        start_angle: f64,
        end_angle: f64,
        anticlockwise: bool,
    ) -> Result<(), JsValue>;
    # [ wasm_bindgen ( method , structural , js_class = "Path2D" , js_name = lineTo ) ]
    #[doc = "The `lineTo()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Path2D/lineTo)\n\n*This API requires the following crate features to be activated: `Path2d`*"]
    pub fn line_to(this: &Path2d, x: f64, y: f64);
    # [ wasm_bindgen ( method , structural , js_class = "Path2D" , js_name = moveTo ) ]
    #[doc = "The `moveTo()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Path2D/moveTo)\n\n*This API requires the following crate features to be activated: `Path2d`*"]
    pub fn move_to(this: &Path2d, x: f64, y: f64);
    # [ wasm_bindgen ( method , structural , js_class = "Path2D" , js_name = quadraticCurveTo ) ]
    #[doc = "The `quadraticCurveTo()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Path2D/quadraticCurveTo)\n\n*This API requires the following crate features to be activated: `Path2d`*"]
    pub fn quadratic_curve_to(this: &Path2d, cpx: f64, cpy: f64, x: f64, y: f64);
    # [ wasm_bindgen ( method , structural , js_class = "Path2D" , js_name = rect ) ]
    #[doc = "The `rect()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Path2D/rect)\n\n*This API requires the following crate features to be activated: `Path2d`*"]
    pub fn rect(this: &Path2d, x: f64, y: f64, w: f64, h: f64);
}
