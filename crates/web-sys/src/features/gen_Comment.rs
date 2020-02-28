use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = CharacterData , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = Comment , typescript_name = Comment ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `Comment` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Comment)\n\n*This API requires the following crate features to be activated: `Comment`*"]
    pub type Comment;
    #[wasm_bindgen(catch, js_class = "Comment", constructor)]
    #[doc = "The `new Comment(..)` constructor, creating a new instance of `Comment`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Comment/Comment)\n\n*This API requires the following crate features to be activated: `Comment`*"]
    pub fn new(this: &Comment) -> Result<Comment, JsValue>;
    #[wasm_bindgen(catch, js_class = "Comment", constructor)]
    #[doc = "The `new Comment(..)` constructor, creating a new instance of `Comment`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Comment/Comment)\n\n*This API requires the following crate features to be activated: `Comment`*"]
    pub fn new_with_data(this: &Comment, data: &str) -> Result<Comment, JsValue>;
}
