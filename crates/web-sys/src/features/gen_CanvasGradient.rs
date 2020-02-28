use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = CanvasGradient , typescript_name = CanvasGradient ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `CanvasGradient` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasGradient)\n\n*This API requires the following crate features to be activated: `CanvasGradient`*"]
    pub type CanvasGradient;
    # [ wasm_bindgen ( catch , method , structural , js_class = "CanvasGradient" , js_name = addColorStop ) ]
    #[doc = "The `addColorStop()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasGradient/addColorStop)\n\n*This API requires the following crate features to be activated: `CanvasGradient`*"]
    pub fn add_color_stop(this: &CanvasGradient, offset: f32, color: &str) -> Result<(), JsValue>;
}
