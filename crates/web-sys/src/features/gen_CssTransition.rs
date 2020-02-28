use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = Animation , extends = EventTarget , extends = :: js_sys :: Object , js_name = CSSTransition , typescript_name = CSSTransition ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `CssTransition` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSTransition)\n\n*This API requires the following crate features to be activated: `CssTransition`*"]
    pub type CssTransition;
    # [ wasm_bindgen ( structural , method , getter , js_name = transitionProperty ) ]
    #[doc = "Getter for the `transitionProperty` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSTransition/transitionProperty)\n\n*This API requires the following crate features to be activated: `CssTransition`*"]
    pub fn transition_property(this: &CssTransition) -> String;
}
