use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = Animation , extends = EventTarget , extends = :: js_sys :: Object , js_name = CSSAnimation , typescript_type = "CSSAnimation" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `CssAnimation` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSAnimation)
    ///
    ///*This API requires the following crate features to be activated: `CssAnimation`*
    pub type CssAnimation;

    # [ wasm_bindgen ( structural , method , getter , js_class = "CSSAnimation" , js_name = animationName ) ]
    ///Getter for the `animationName` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSAnimation/animationName)
    ///
    ///*This API requires the following crate features to be activated: `CssAnimation`*
    pub fn animation_name(this: &CssAnimation) -> String;

}
