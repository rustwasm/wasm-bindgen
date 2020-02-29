use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = CharacterData , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = Comment , typescript_name = Comment ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `Comment` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Comment)
    ///
    ///*This API requires the following crate features to be activated: `Comment`*
    pub type Comment;

    #[wasm_bindgen(catch, constructor, js_class = "Comment")]
    ///The `new Comment(..)` constructor, creating a new instance of `Comment`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Comment/Comment)
    ///
    ///*This API requires the following crate features to be activated: `Comment`*
    pub fn new() -> Result<Comment, JsValue>;

    #[wasm_bindgen(catch, constructor, js_class = "Comment")]
    ///The `new Comment(..)` constructor, creating a new instance of `Comment`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Comment/Comment)
    ///
    ///*This API requires the following crate features to be activated: `Comment`*
    pub fn new_with_data(data: &str) -> Result<Comment, JsValue>;

}
