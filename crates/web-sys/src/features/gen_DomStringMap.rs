use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = DOMStringMap , typescript_type = "DOMStringMap" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `DomStringMap` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMStringMap)
    ///
    ///*This API requires the following crate features to be activated: `DomStringMap`*
    pub type DomStringMap;

    #[wasm_bindgen(method, structural, js_class = "DOMStringMap", indexing_getter)]
    ///Indexing getter.
    ///
    ///
    ///
    ///*This API requires the following crate features to be activated: `DomStringMap`*
    pub fn get(this: &DomStringMap, name: &str) -> Option<String>;

    #[wasm_bindgen(catch, method, structural, js_class = "DOMStringMap", indexing_setter)]
    ///Indexing setter.
    ///
    ///
    ///
    ///*This API requires the following crate features to be activated: `DomStringMap`*
    pub fn set(this: &DomStringMap, name: &str, value: &str) -> Result<(), JsValue>;

    #[wasm_bindgen(method, structural, js_class = "DOMStringMap", indexing_deleter)]
    ///Indexing deleter.
    ///
    ///
    ///
    ///*This API requires the following crate features to be activated: `DomStringMap`*
    pub fn delete(this: &DomStringMap, name: &str);

}
