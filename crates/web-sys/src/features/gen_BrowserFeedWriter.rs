use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = BrowserFeedWriter , typescript_type = "BrowserFeedWriter" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `BrowserFeedWriter` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BrowserFeedWriter)
    ///
    ///*This API requires the following crate features to be activated: `BrowserFeedWriter`*
    pub type BrowserFeedWriter;

    #[wasm_bindgen(catch, constructor, js_class = "BrowserFeedWriter")]
    ///The `new BrowserFeedWriter(..)` constructor, creating a new instance of `BrowserFeedWriter`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BrowserFeedWriter/BrowserFeedWriter)
    ///
    ///*This API requires the following crate features to be activated: `BrowserFeedWriter`*
    pub fn new() -> Result<BrowserFeedWriter, JsValue>;

    # [ wasm_bindgen ( method , structural , js_class = "BrowserFeedWriter" , js_name = close ) ]
    ///The `close()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BrowserFeedWriter/close)
    ///
    ///*This API requires the following crate features to be activated: `BrowserFeedWriter`*
    pub fn close(this: &BrowserFeedWriter);

    # [ wasm_bindgen ( method , structural , js_class = "BrowserFeedWriter" , js_name = writeContent ) ]
    ///The `writeContent()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BrowserFeedWriter/writeContent)
    ///
    ///*This API requires the following crate features to be activated: `BrowserFeedWriter`*
    pub fn write_content(this: &BrowserFeedWriter);

}
