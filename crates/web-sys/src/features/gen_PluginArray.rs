use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = PluginArray , typescript_name = PluginArray ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `PluginArray` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PluginArray)\n\n*This API requires the following crate features to be activated: `PluginArray`*"]
    pub type PluginArray;
    # [ wasm_bindgen ( structural , method , getter , js_name = length ) ]
    #[doc = "Getter for the `length` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PluginArray/length)\n\n*This API requires the following crate features to be activated: `PluginArray`*"]
    pub fn length(this: &PluginArray) -> u32;
    #[cfg(feature = "Plugin")]
    # [ wasm_bindgen ( method , structural , js_name = item ) ]
    #[doc = "The `item()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PluginArray/item)\n\n*This API requires the following crate features to be activated: `Plugin`, `PluginArray`*"]
    pub fn item(this: &PluginArray, index: u32) -> Option<Plugin>;
    #[cfg(feature = "Plugin")]
    # [ wasm_bindgen ( method , structural , js_name = namedItem ) ]
    #[doc = "The `namedItem()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PluginArray/namedItem)\n\n*This API requires the following crate features to be activated: `Plugin`, `PluginArray`*"]
    pub fn named_item(this: &PluginArray, name: &str) -> Option<Plugin>;
    # [ wasm_bindgen ( method , structural , js_name = refresh ) ]
    #[doc = "The `refresh()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PluginArray/refresh)\n\n*This API requires the following crate features to be activated: `PluginArray`*"]
    pub fn refresh(this: &PluginArray);
    # [ wasm_bindgen ( method , structural , js_name = refresh ) ]
    #[doc = "The `refresh()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PluginArray/refresh)\n\n*This API requires the following crate features to be activated: `PluginArray`*"]
    pub fn refresh_with_reload_documents(this: &PluginArray, reload_documents: bool);
    #[cfg(feature = "Plugin")]
    #[wasm_bindgen(method, structural, indexing_getter)]
    #[doc = "Indexing getter.\n\n\n\n*This API requires the following crate features to be activated: `Plugin`, `PluginArray`*"]
    pub fn get_with_index(this: &PluginArray, index: u32) -> Option<Plugin>;
    #[cfg(feature = "Plugin")]
    #[wasm_bindgen(method, structural, indexing_getter)]
    #[doc = "Indexing getter.\n\n\n\n*This API requires the following crate features to be activated: `Plugin`, `PluginArray`*"]
    pub fn get_with_name(this: &PluginArray, name: &str) -> Option<Plugin>;
}
