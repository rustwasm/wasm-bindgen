use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = FetchReadableStreamReadDataArray ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `FetchReadableStreamReadDataArray` dictionary.
    ///
    ///*This API requires the following crate features to be activated: `FetchReadableStreamReadDataArray`*
    pub type FetchReadableStreamReadDataArray;

}

impl FetchReadableStreamReadDataArray {
    ///Construct a new `FetchReadableStreamReadDataArray`.
    ///
    ///*This API requires the following crate features to be activated: `FetchReadableStreamReadDataArray`*

    pub fn new() -> Self {

        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());

        ret
    }
}
