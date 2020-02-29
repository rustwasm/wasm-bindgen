use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = Animation , extends = EventTarget , extends = :: js_sys :: Object , js_name = CSSTransition , typescript_type = "CSSTransition" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `CssTransition` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSTransition)
    ///
    ///*This API requires the following crate features to be activated: `CssTransition`*
    pub type CssTransition;

    # [ wasm_bindgen ( structural , method , getter , js_class = "CSSTransition" , js_name = transitionProperty ) ]
    ///Getter for the `transitionProperty` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSTransition/transitionProperty)
    ///
    ///*This API requires the following crate features to be activated: `CssTransition`*
    pub fn transition_property(this: &CssTransition) -> String;

}
