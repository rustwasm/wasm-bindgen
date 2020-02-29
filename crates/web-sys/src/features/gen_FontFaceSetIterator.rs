use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( is_type_of = | _ | false , extends = :: js_sys :: Object , js_name = FontFaceSetIterator , typescript_type = "FontFaceSetIterator" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `FontFaceSetIterator` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFaceSetIterator)
    ///
    ///*This API requires the following crate features to be activated: `FontFaceSetIterator`*
    pub type FontFaceSetIterator;

    #[cfg(feature = "FontFaceSetIteratorResult")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "FontFaceSetIterator" , js_name = next ) ]
    ///The `next()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFaceSetIterator/next)
    ///
    ///*This API requires the following crate features to be activated: `FontFaceSetIterator`, `FontFaceSetIteratorResult`*
    pub fn next(this: &FontFaceSetIterator) -> Result<FontFaceSetIteratorResult, JsValue>;

}
