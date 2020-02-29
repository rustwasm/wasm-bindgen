use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = Path2D , typescript_name = Path2D ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `Path2d` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Path2D)
    ///
    ///*This API requires the following crate features to be activated: `Path2d`*
    pub type Path2d;

    #[wasm_bindgen(catch, constructor, js_class = "Path2D")]
    ///The `new Path2d(..)` constructor, creating a new instance of `Path2d`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Path2D/Path2D)
    ///
    ///*This API requires the following crate features to be activated: `Path2d`*
    pub fn new() -> Result<Path2d, JsValue>;

    #[wasm_bindgen(catch, constructor, js_class = "Path2D")]
    ///The `new Path2d(..)` constructor, creating a new instance of `Path2d`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Path2D/Path2D)
    ///
    ///*This API requires the following crate features to be activated: `Path2d`*
    pub fn new_with_other(other: &Path2d) -> Result<Path2d, JsValue>;

    #[wasm_bindgen(catch, constructor, js_class = "Path2D")]
    ///The `new Path2d(..)` constructor, creating a new instance of `Path2d`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Path2D/Path2D)
    ///
    ///*This API requires the following crate features to be activated: `Path2d`*
    pub fn new_with_path_string(path_string: &str) -> Result<Path2d, JsValue>;

    # [ wasm_bindgen ( method , structural , js_class = "Path2D" , js_name = addPath ) ]
    ///The `addPath()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Path2D/addPath)
    ///
    ///*This API requires the following crate features to be activated: `Path2d`*
    pub fn add_path(this: &Path2d, path: &Path2d);

    #[cfg(feature = "SvgMatrix")]
    # [ wasm_bindgen ( method , structural , js_class = "Path2D" , js_name = addPath ) ]
    ///The `addPath()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Path2D/addPath)
    ///
    ///*This API requires the following crate features to be activated: `Path2d`, `SvgMatrix`*
    pub fn add_path_with_transformation(this: &Path2d, path: &Path2d, transformation: &SvgMatrix);

    # [ wasm_bindgen ( catch , method , structural , js_class = "Path2D" , js_name = arc ) ]
    ///The `arc()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Path2D/arc)
    ///
    ///*This API requires the following crate features to be activated: `Path2d`*
    pub fn arc(
        this: &Path2d,
        x: f64,
        y: f64,
        radius: f64,
        start_angle: f64,
        end_angle: f64,
    ) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Path2D" , js_name = arc ) ]
    ///The `arc()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Path2D/arc)
    ///
    ///*This API requires the following crate features to be activated: `Path2d`*
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
    ///The `arcTo()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Path2D/arcTo)
    ///
    ///*This API requires the following crate features to be activated: `Path2d`*
    pub fn arc_to(
        this: &Path2d,
        x1: f64,
        y1: f64,
        x2: f64,
        y2: f64,
        radius: f64,
    ) -> Result<(), JsValue>;

    # [ wasm_bindgen ( method , structural , js_class = "Path2D" , js_name = bezierCurveTo ) ]
    ///The `bezierCurveTo()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Path2D/bezierCurveTo)
    ///
    ///*This API requires the following crate features to be activated: `Path2d`*
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
    ///The `closePath()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Path2D/closePath)
    ///
    ///*This API requires the following crate features to be activated: `Path2d`*
    pub fn close_path(this: &Path2d);

    # [ wasm_bindgen ( catch , method , structural , js_class = "Path2D" , js_name = ellipse ) ]
    ///The `ellipse()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Path2D/ellipse)
    ///
    ///*This API requires the following crate features to be activated: `Path2d`*
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
    ///The `ellipse()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Path2D/ellipse)
    ///
    ///*This API requires the following crate features to be activated: `Path2d`*
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
    ///The `lineTo()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Path2D/lineTo)
    ///
    ///*This API requires the following crate features to be activated: `Path2d`*
    pub fn line_to(this: &Path2d, x: f64, y: f64);

    # [ wasm_bindgen ( method , structural , js_class = "Path2D" , js_name = moveTo ) ]
    ///The `moveTo()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Path2D/moveTo)
    ///
    ///*This API requires the following crate features to be activated: `Path2d`*
    pub fn move_to(this: &Path2d, x: f64, y: f64);

    # [ wasm_bindgen ( method , structural , js_class = "Path2D" , js_name = quadraticCurveTo ) ]
    ///The `quadraticCurveTo()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Path2D/quadraticCurveTo)
    ///
    ///*This API requires the following crate features to be activated: `Path2d`*
    pub fn quadratic_curve_to(this: &Path2d, cpx: f64, cpy: f64, x: f64, y: f64);

    # [ wasm_bindgen ( method , structural , js_class = "Path2D" , js_name = rect ) ]
    ///The `rect()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Path2D/rect)
    ///
    ///*This API requires the following crate features to be activated: `Path2d`*
    pub fn rect(this: &Path2d, x: f64, y: f64, w: f64, h: f64);

}
