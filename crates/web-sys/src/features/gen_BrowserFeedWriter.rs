use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = BrowserFeedWriter , typescript_name = BrowserFeedWriter ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `BrowserFeedWriter` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BrowserFeedWriter)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BrowserFeedWriter`*"]
    pub type BrowserFeedWriter;
    #[wasm_bindgen(catch, js_class = "BrowserFeedWriter", constructor)]
    #[doc = "The `new BrowserFeedWriter(..)` constructor, creating a new instance of `BrowserFeedWriter`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BrowserFeedWriter/BrowserFeedWriter)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BrowserFeedWriter`*"]
    pub fn new(this: &BrowserFeedWriter) -> Result<BrowserFeedWriter, JsValue>;
    # [ wasm_bindgen ( method , structural , js_class = "BrowserFeedWriter" , js_name = close ) ]
    #[doc = "The `close()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BrowserFeedWriter/close)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BrowserFeedWriter`*"]
    pub fn close(this: &BrowserFeedWriter);
    # [ wasm_bindgen ( method , structural , js_class = "BrowserFeedWriter" , js_name = writeContent ) ]
    #[doc = "The `writeContent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BrowserFeedWriter/writeContent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BrowserFeedWriter`*"]
    pub fn write_content(this: &BrowserFeedWriter);
}
