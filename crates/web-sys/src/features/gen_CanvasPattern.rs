use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = CanvasPattern , typescript_type = "CanvasPattern" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `CanvasPattern` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasPattern)
    ///
    ///*This API requires the following crate features to be activated: `CanvasPattern`*
    pub type CanvasPattern;

    #[cfg(feature = "SvgMatrix")]
    # [ wasm_bindgen ( method , structural , js_class = "CanvasPattern" , js_name = setTransform ) ]
    ///The `setTransform()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasPattern/setTransform)
    ///
    ///*This API requires the following crate features to be activated: `CanvasPattern`, `SvgMatrix`*
    pub fn set_transform(this: &CanvasPattern, matrix: &SvgMatrix);

}
