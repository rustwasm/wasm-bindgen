use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = PluginArray , typescript_type = "PluginArray" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `PluginArray` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PluginArray)
    ///
    ///*This API requires the following crate features to be activated: `PluginArray`*
    pub type PluginArray;

    # [ wasm_bindgen ( structural , method , getter , js_class = "PluginArray" , js_name = length ) ]
    ///Getter for the `length` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PluginArray/length)
    ///
    ///*This API requires the following crate features to be activated: `PluginArray`*
    pub fn length(this: &PluginArray) -> u32;

    #[cfg(feature = "Plugin")]
    # [ wasm_bindgen ( method , structural , js_class = "PluginArray" , js_name = item ) ]
    ///The `item()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PluginArray/item)
    ///
    ///*This API requires the following crate features to be activated: `Plugin`, `PluginArray`*
    pub fn item(this: &PluginArray, index: u32) -> Option<Plugin>;

    #[cfg(feature = "Plugin")]
    # [ wasm_bindgen ( method , structural , js_class = "PluginArray" , js_name = namedItem ) ]
    ///The `namedItem()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PluginArray/namedItem)
    ///
    ///*This API requires the following crate features to be activated: `Plugin`, `PluginArray`*
    pub fn named_item(this: &PluginArray, name: &str) -> Option<Plugin>;

    # [ wasm_bindgen ( method , structural , js_class = "PluginArray" , js_name = refresh ) ]
    ///The `refresh()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PluginArray/refresh)
    ///
    ///*This API requires the following crate features to be activated: `PluginArray`*
    pub fn refresh(this: &PluginArray);

    # [ wasm_bindgen ( method , structural , js_class = "PluginArray" , js_name = refresh ) ]
    ///The `refresh()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PluginArray/refresh)
    ///
    ///*This API requires the following crate features to be activated: `PluginArray`*
    pub fn refresh_with_reload_documents(this: &PluginArray, reload_documents: bool);

    #[cfg(feature = "Plugin")]
    #[wasm_bindgen(method, structural, js_class = "PluginArray", indexing_getter)]
    ///Indexing getter.
    ///
    ///
    ///
    ///*This API requires the following crate features to be activated: `Plugin`, `PluginArray`*
    pub fn get_with_index(this: &PluginArray, index: u32) -> Option<Plugin>;

    #[cfg(feature = "Plugin")]
    #[wasm_bindgen(method, structural, js_class = "PluginArray", indexing_getter)]
    ///Indexing getter.
    ///
    ///
    ///
    ///*This API requires the following crate features to be activated: `Plugin`, `PluginArray`*
    pub fn get_with_name(this: &PluginArray, name: &str) -> Option<Plugin>;

}
