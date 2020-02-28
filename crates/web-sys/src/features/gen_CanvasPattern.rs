use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = CanvasPattern , typescript_name = CanvasPattern ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `CanvasPattern` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasPattern)\n\n*This API requires the following crate features to be activated: `CanvasPattern`*"]
    pub type CanvasPattern;
    #[cfg(feature = "SvgMatrix")]
    # [ wasm_bindgen ( method , structural , js_name = setTransform ) ]
    #[doc = "The `setTransform()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasPattern/setTransform)\n\n*This API requires the following crate features to be activated: `CanvasPattern`, `SvgMatrix`*"]
    pub fn set_transform(this: &CanvasPattern, matrix: &SvgMatrix);
}
