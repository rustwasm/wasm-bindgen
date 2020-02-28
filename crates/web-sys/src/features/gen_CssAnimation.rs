use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = Animation , extends = EventTarget , extends = :: js_sys :: Object , js_name = CSSAnimation , typescript_name = CSSAnimation ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `CssAnimation` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSAnimation)\n\n*This API requires the following crate features to be activated: `CssAnimation`*"]
    pub type CssAnimation;
    # [ wasm_bindgen ( structural , method , getter , js_name = animationName ) ]
    #[doc = "Getter for the `animationName` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSAnimation/animationName)\n\n*This API requires the following crate features to be activated: `CssAnimation`*"]
    pub fn animation_name(this: &CssAnimation) -> String;
}
