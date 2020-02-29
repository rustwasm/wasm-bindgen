use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = Plugin , typescript_type = "Plugin" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `Plugin` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Plugin)
    ///
    ///*This API requires the following crate features to be activated: `Plugin`*
    pub type Plugin;

    # [ wasm_bindgen ( structural , method , getter , js_class = "Plugin" , js_name = description ) ]
    ///Getter for the `description` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Plugin/description)
    ///
    ///*This API requires the following crate features to be activated: `Plugin`*
    pub fn description(this: &Plugin) -> String;

    # [ wasm_bindgen ( structural , method , getter , js_class = "Plugin" , js_name = filename ) ]
    ///Getter for the `filename` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Plugin/filename)
    ///
    ///*This API requires the following crate features to be activated: `Plugin`*
    pub fn filename(this: &Plugin) -> String;

    # [ wasm_bindgen ( structural , method , getter , js_class = "Plugin" , js_name = version ) ]
    ///Getter for the `version` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Plugin/version)
    ///
    ///*This API requires the following crate features to be activated: `Plugin`*
    pub fn version(this: &Plugin) -> String;

    # [ wasm_bindgen ( structural , method , getter , js_class = "Plugin" , js_name = name ) ]
    ///Getter for the `name` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Plugin/name)
    ///
    ///*This API requires the following crate features to be activated: `Plugin`*
    pub fn name(this: &Plugin) -> String;

    # [ wasm_bindgen ( structural , method , getter , js_class = "Plugin" , js_name = length ) ]
    ///Getter for the `length` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Plugin/length)
    ///
    ///*This API requires the following crate features to be activated: `Plugin`*
    pub fn length(this: &Plugin) -> u32;

    #[cfg(feature = "MimeType")]
    # [ wasm_bindgen ( method , structural , js_class = "Plugin" , js_name = item ) ]
    ///The `item()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Plugin/item)
    ///
    ///*This API requires the following crate features to be activated: `MimeType`, `Plugin`*
    pub fn item(this: &Plugin, index: u32) -> Option<MimeType>;

    #[cfg(feature = "MimeType")]
    # [ wasm_bindgen ( method , structural , js_class = "Plugin" , js_name = namedItem ) ]
    ///The `namedItem()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Plugin/namedItem)
    ///
    ///*This API requires the following crate features to be activated: `MimeType`, `Plugin`*
    pub fn named_item(this: &Plugin, name: &str) -> Option<MimeType>;

    #[cfg(feature = "MimeType")]
    #[wasm_bindgen(method, structural, js_class = "Plugin", indexing_getter)]
    ///Indexing getter.
    ///
    ///
    ///
    ///*This API requires the following crate features to be activated: `MimeType`, `Plugin`*
    pub fn get_with_index(this: &Plugin, index: u32) -> Option<MimeType>;

    #[cfg(feature = "MimeType")]
    #[wasm_bindgen(method, structural, js_class = "Plugin", indexing_getter)]
    ///Indexing getter.
    ///
    ///
    ///
    ///*This API requires the following crate features to be activated: `MimeType`, `Plugin`*
    pub fn get_with_name(this: &Plugin, name: &str) -> Option<MimeType>;

}
