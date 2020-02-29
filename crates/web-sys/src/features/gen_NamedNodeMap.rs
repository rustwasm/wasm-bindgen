use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = NamedNodeMap , typescript_name = NamedNodeMap ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `NamedNodeMap` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/NamedNodeMap)
    ///
    ///*This API requires the following crate features to be activated: `NamedNodeMap`*
    pub type NamedNodeMap;

    # [ wasm_bindgen ( structural , method , getter , js_class = "NamedNodeMap" , js_name = length ) ]
    ///Getter for the `length` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/NamedNodeMap/length)
    ///
    ///*This API requires the following crate features to be activated: `NamedNodeMap`*
    pub fn length(this: &NamedNodeMap) -> u32;

    #[cfg(feature = "Attr")]
    # [ wasm_bindgen ( method , structural , js_class = "NamedNodeMap" , js_name = getNamedItem ) ]
    ///The `getNamedItem()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/NamedNodeMap/getNamedItem)
    ///
    ///*This API requires the following crate features to be activated: `Attr`, `NamedNodeMap`*
    pub fn get_named_item(this: &NamedNodeMap, name: &str) -> Option<Attr>;

    #[cfg(feature = "Attr")]
    # [ wasm_bindgen ( method , structural , js_class = "NamedNodeMap" , js_name = getNamedItemNS ) ]
    ///The `getNamedItemNS()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/NamedNodeMap/getNamedItemNS)
    ///
    ///*This API requires the following crate features to be activated: `Attr`, `NamedNodeMap`*
    pub fn get_named_item_ns(
        this: &NamedNodeMap,
        namespace_uri: Option<&str>,
        local_name: &str,
    ) -> Option<Attr>;

    #[cfg(feature = "Attr")]
    # [ wasm_bindgen ( method , structural , js_class = "NamedNodeMap" , js_name = item ) ]
    ///The `item()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/NamedNodeMap/item)
    ///
    ///*This API requires the following crate features to be activated: `Attr`, `NamedNodeMap`*
    pub fn item(this: &NamedNodeMap, index: u32) -> Option<Attr>;

    #[cfg(feature = "Attr")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "NamedNodeMap" , js_name = removeNamedItem ) ]
    ///The `removeNamedItem()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/NamedNodeMap/removeNamedItem)
    ///
    ///*This API requires the following crate features to be activated: `Attr`, `NamedNodeMap`*
    pub fn remove_named_item(this: &NamedNodeMap, name: &str) -> Result<Attr, JsValue>;

    #[cfg(feature = "Attr")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "NamedNodeMap" , js_name = removeNamedItemNS ) ]
    ///The `removeNamedItemNS()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/NamedNodeMap/removeNamedItemNS)
    ///
    ///*This API requires the following crate features to be activated: `Attr`, `NamedNodeMap`*
    pub fn remove_named_item_ns(
        this: &NamedNodeMap,
        namespace_uri: Option<&str>,
        local_name: &str,
    ) -> Result<Attr, JsValue>;

    #[cfg(feature = "Attr")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "NamedNodeMap" , js_name = setNamedItem ) ]
    ///The `setNamedItem()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/NamedNodeMap/setNamedItem)
    ///
    ///*This API requires the following crate features to be activated: `Attr`, `NamedNodeMap`*
    pub fn set_named_item(this: &NamedNodeMap, arg: &Attr) -> Result<Option<Attr>, JsValue>;

    #[cfg(feature = "Attr")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "NamedNodeMap" , js_name = setNamedItemNS ) ]
    ///The `setNamedItemNS()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/NamedNodeMap/setNamedItemNS)
    ///
    ///*This API requires the following crate features to be activated: `Attr`, `NamedNodeMap`*
    pub fn set_named_item_ns(this: &NamedNodeMap, arg: &Attr) -> Result<Option<Attr>, JsValue>;

    #[cfg(feature = "Attr")]
    #[wasm_bindgen(method, structural, js_class = "NamedNodeMap", indexing_getter)]
    ///Indexing getter.
    ///
    ///
    ///
    ///*This API requires the following crate features to be activated: `Attr`, `NamedNodeMap`*
    pub fn get_with_name(this: &NamedNodeMap, name: &str) -> Option<Attr>;

    #[cfg(feature = "Attr")]
    #[wasm_bindgen(method, structural, js_class = "NamedNodeMap", indexing_getter)]
    ///Indexing getter.
    ///
    ///
    ///
    ///*This API requires the following crate features to be activated: `Attr`, `NamedNodeMap`*
    pub fn get_with_index(this: &NamedNodeMap, index: u32) -> Option<Attr>;

}
