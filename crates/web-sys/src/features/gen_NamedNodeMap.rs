use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = NamedNodeMap , typescript_name = NamedNodeMap ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `NamedNodeMap` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/NamedNodeMap)\n\n*This API requires the following crate features to be activated: `NamedNodeMap`*"]
    pub type NamedNodeMap;
    # [ wasm_bindgen ( structural , method , getter , js_class = "NamedNodeMap" , js_name = length ) ]
    #[doc = "Getter for the `length` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/NamedNodeMap/length)\n\n*This API requires the following crate features to be activated: `NamedNodeMap`*"]
    pub fn length(this: &NamedNodeMap) -> u32;
    #[cfg(feature = "Attr")]
    # [ wasm_bindgen ( method , structural , js_class = "NamedNodeMap" , js_name = getNamedItem ) ]
    #[doc = "The `getNamedItem()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/NamedNodeMap/getNamedItem)\n\n*This API requires the following crate features to be activated: `Attr`, `NamedNodeMap`*"]
    pub fn get_named_item(this: &NamedNodeMap, name: &str) -> Option<Attr>;
    #[cfg(feature = "Attr")]
    # [ wasm_bindgen ( method , structural , js_class = "NamedNodeMap" , js_name = getNamedItemNS ) ]
    #[doc = "The `getNamedItemNS()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/NamedNodeMap/getNamedItemNS)\n\n*This API requires the following crate features to be activated: `Attr`, `NamedNodeMap`*"]
    pub fn get_named_item_ns(
        this: &NamedNodeMap,
        namespace_uri: Option<&str>,
        local_name: &str,
    ) -> Option<Attr>;
    #[cfg(feature = "Attr")]
    # [ wasm_bindgen ( method , structural , js_class = "NamedNodeMap" , js_name = item ) ]
    #[doc = "The `item()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/NamedNodeMap/item)\n\n*This API requires the following crate features to be activated: `Attr`, `NamedNodeMap`*"]
    pub fn item(this: &NamedNodeMap, index: u32) -> Option<Attr>;
    #[cfg(feature = "Attr")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "NamedNodeMap" , js_name = removeNamedItem ) ]
    #[doc = "The `removeNamedItem()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/NamedNodeMap/removeNamedItem)\n\n*This API requires the following crate features to be activated: `Attr`, `NamedNodeMap`*"]
    pub fn remove_named_item(this: &NamedNodeMap, name: &str) -> Result<Attr, JsValue>;
    #[cfg(feature = "Attr")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "NamedNodeMap" , js_name = removeNamedItemNS ) ]
    #[doc = "The `removeNamedItemNS()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/NamedNodeMap/removeNamedItemNS)\n\n*This API requires the following crate features to be activated: `Attr`, `NamedNodeMap`*"]
    pub fn remove_named_item_ns(
        this: &NamedNodeMap,
        namespace_uri: Option<&str>,
        local_name: &str,
    ) -> Result<Attr, JsValue>;
    #[cfg(feature = "Attr")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "NamedNodeMap" , js_name = setNamedItem ) ]
    #[doc = "The `setNamedItem()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/NamedNodeMap/setNamedItem)\n\n*This API requires the following crate features to be activated: `Attr`, `NamedNodeMap`*"]
    pub fn set_named_item(this: &NamedNodeMap, arg: &Attr) -> Result<Option<Attr>, JsValue>;
    #[cfg(feature = "Attr")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "NamedNodeMap" , js_name = setNamedItemNS ) ]
    #[doc = "The `setNamedItemNS()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/NamedNodeMap/setNamedItemNS)\n\n*This API requires the following crate features to be activated: `Attr`, `NamedNodeMap`*"]
    pub fn set_named_item_ns(this: &NamedNodeMap, arg: &Attr) -> Result<Option<Attr>, JsValue>;
    #[cfg(feature = "Attr")]
    #[wasm_bindgen(method, structural, js_class = "NamedNodeMap", indexing_getter)]
    #[doc = "Indexing getter.\n\n\n\n*This API requires the following crate features to be activated: `Attr`, `NamedNodeMap`*"]
    pub fn get_with_name(this: &NamedNodeMap, name: &str) -> Option<Attr>;
    #[cfg(feature = "Attr")]
    #[wasm_bindgen(method, structural, js_class = "NamedNodeMap", indexing_getter)]
    #[doc = "Indexing getter.\n\n\n\n*This API requires the following crate features to be activated: `Attr`, `NamedNodeMap`*"]
    pub fn get_with_index(this: &NamedNodeMap, index: u32) -> Option<Attr>;
}
