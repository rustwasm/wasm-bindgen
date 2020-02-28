use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = Document , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = HTMLDocument , typescript_name = HTMLDocument ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `HtmlDocument` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument)\n\n*This API requires the following crate features to be activated: `HtmlDocument`*"]
    pub type HtmlDocument;
    # [ wasm_bindgen ( structural , method , getter , js_name = domain ) ]
    #[doc = "Getter for the `domain` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/domain)\n\n*This API requires the following crate features to be activated: `HtmlDocument`*"]
    pub fn domain(this: &HtmlDocument) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = domain ) ]
    #[doc = "Setter for the `domain` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/domain)\n\n*This API requires the following crate features to be activated: `HtmlDocument`*"]
    pub fn set_domain(this: &HtmlDocument, value: String);
    # [ wasm_bindgen ( structural , catch , method , getter , js_name = cookie ) ]
    #[doc = "Getter for the `cookie` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/cookie)\n\n*This API requires the following crate features to be activated: `HtmlDocument`*"]
    pub fn cookie(this: &HtmlDocument) -> Result<String, JsValue>;
    # [ wasm_bindgen ( structural , catch , method , setter , js_name = cookie ) ]
    #[doc = "Setter for the `cookie` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/cookie)\n\n*This API requires the following crate features to be activated: `HtmlDocument`*"]
    pub fn set_cookie(this: &HtmlDocument, value: Result<String, JsValue>);
    # [ wasm_bindgen ( structural , method , getter , js_name = designMode ) ]
    #[doc = "Getter for the `designMode` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/designMode)\n\n*This API requires the following crate features to be activated: `HtmlDocument`*"]
    pub fn design_mode(this: &HtmlDocument) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = designMode ) ]
    #[doc = "Setter for the `designMode` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/designMode)\n\n*This API requires the following crate features to be activated: `HtmlDocument`*"]
    pub fn set_design_mode(this: &HtmlDocument, value: String);
    # [ wasm_bindgen ( structural , method , getter , js_name = fgColor ) ]
    #[doc = "Getter for the `fgColor` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/fgColor)\n\n*This API requires the following crate features to be activated: `HtmlDocument`*"]
    pub fn fg_color(this: &HtmlDocument) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = fgColor ) ]
    #[doc = "Setter for the `fgColor` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/fgColor)\n\n*This API requires the following crate features to be activated: `HtmlDocument`*"]
    pub fn set_fg_color(this: &HtmlDocument, value: String);
    # [ wasm_bindgen ( structural , method , getter , js_name = linkColor ) ]
    #[doc = "Getter for the `linkColor` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/linkColor)\n\n*This API requires the following crate features to be activated: `HtmlDocument`*"]
    pub fn link_color(this: &HtmlDocument) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = linkColor ) ]
    #[doc = "Setter for the `linkColor` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/linkColor)\n\n*This API requires the following crate features to be activated: `HtmlDocument`*"]
    pub fn set_link_color(this: &HtmlDocument, value: String);
    # [ wasm_bindgen ( structural , method , getter , js_name = vlinkColor ) ]
    #[doc = "Getter for the `vlinkColor` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/vlinkColor)\n\n*This API requires the following crate features to be activated: `HtmlDocument`*"]
    pub fn vlink_color(this: &HtmlDocument) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = vlinkColor ) ]
    #[doc = "Setter for the `vlinkColor` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/vlinkColor)\n\n*This API requires the following crate features to be activated: `HtmlDocument`*"]
    pub fn set_vlink_color(this: &HtmlDocument, value: String);
    # [ wasm_bindgen ( structural , method , getter , js_name = alinkColor ) ]
    #[doc = "Getter for the `alinkColor` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/alinkColor)\n\n*This API requires the following crate features to be activated: `HtmlDocument`*"]
    pub fn alink_color(this: &HtmlDocument) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = alinkColor ) ]
    #[doc = "Setter for the `alinkColor` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/alinkColor)\n\n*This API requires the following crate features to be activated: `HtmlDocument`*"]
    pub fn set_alink_color(this: &HtmlDocument, value: String);
    # [ wasm_bindgen ( structural , method , getter , js_name = bgColor ) ]
    #[doc = "Getter for the `bgColor` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/bgColor)\n\n*This API requires the following crate features to be activated: `HtmlDocument`*"]
    pub fn bg_color(this: &HtmlDocument) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = bgColor ) ]
    #[doc = "Setter for the `bgColor` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/bgColor)\n\n*This API requires the following crate features to be activated: `HtmlDocument`*"]
    pub fn set_bg_color(this: &HtmlDocument, value: String);
    # [ wasm_bindgen ( structural , method , getter , js_name = all ) ]
    #[cfg(feature = "HtmlAllCollection")]
    #[doc = "Getter for the `all` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/all)\n\n*This API requires the following crate features to be activated: `HtmlAllCollection`, `HtmlDocument`*"]
    pub fn all(this: &HtmlDocument) -> HtmlAllCollection;
    # [ wasm_bindgen ( method , structural , js_name = captureEvents ) ]
    #[doc = "The `captureEvents()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/captureEvents)\n\n*This API requires the following crate features to be activated: `HtmlDocument`*"]
    pub fn capture_events(this: &HtmlDocument);
    # [ wasm_bindgen ( method , structural , js_name = clear ) ]
    #[doc = "The `clear()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/clear)\n\n*This API requires the following crate features to be activated: `HtmlDocument`*"]
    pub fn clear(this: &HtmlDocument);
    # [ wasm_bindgen ( catch , method , structural , js_name = close ) ]
    #[doc = "The `close()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/close)\n\n*This API requires the following crate features to be activated: `HtmlDocument`*"]
    pub fn close(this: &HtmlDocument) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = execCommand ) ]
    #[doc = "The `execCommand()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/execCommand)\n\n*This API requires the following crate features to be activated: `HtmlDocument`*"]
    pub fn exec_command(this: &HtmlDocument, command_id: &str) -> Result<bool, JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = execCommand ) ]
    #[doc = "The `execCommand()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/execCommand)\n\n*This API requires the following crate features to be activated: `HtmlDocument`*"]
    pub fn exec_command_with_show_ui(
        this: &HtmlDocument,
        command_id: &str,
        show_ui: bool,
    ) -> Result<bool, JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = execCommand ) ]
    #[doc = "The `execCommand()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/execCommand)\n\n*This API requires the following crate features to be activated: `HtmlDocument`*"]
    pub fn exec_command_with_show_ui_and_value(
        this: &HtmlDocument,
        command_id: &str,
        show_ui: bool,
        value: &str,
    ) -> Result<bool, JsValue>;
    #[cfg(feature = "Document")]
    # [ wasm_bindgen ( catch , method , structural , js_name = open ) ]
    #[doc = "The `open()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/open)\n\n*This API requires the following crate features to be activated: `Document`, `HtmlDocument`*"]
    pub fn open(this: &HtmlDocument) -> Result<Document, JsValue>;
    #[cfg(feature = "Document")]
    # [ wasm_bindgen ( catch , method , structural , js_name = open ) ]
    #[doc = "The `open()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/open)\n\n*This API requires the following crate features to be activated: `Document`, `HtmlDocument`*"]
    pub fn open_with_type(this: &HtmlDocument, type_: &str) -> Result<Document, JsValue>;
    #[cfg(feature = "Document")]
    # [ wasm_bindgen ( catch , method , structural , js_name = open ) ]
    #[doc = "The `open()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/open)\n\n*This API requires the following crate features to be activated: `Document`, `HtmlDocument`*"]
    pub fn open_with_type_and_replace(
        this: &HtmlDocument,
        type_: &str,
        replace: &str,
    ) -> Result<Document, JsValue>;
    #[cfg(feature = "Window")]
    # [ wasm_bindgen ( catch , method , structural , js_name = open ) ]
    #[doc = "The `open()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/open)\n\n*This API requires the following crate features to be activated: `HtmlDocument`, `Window`*"]
    pub fn open_with_url_and_name_and_features(
        this: &HtmlDocument,
        url: &str,
        name: &str,
        features: &str,
    ) -> Result<Option<Window>, JsValue>;
    #[cfg(feature = "Window")]
    # [ wasm_bindgen ( catch , method , structural , js_name = open ) ]
    #[doc = "The `open()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/open)\n\n*This API requires the following crate features to be activated: `HtmlDocument`, `Window`*"]
    pub fn open_with_url_and_name_and_features_and_replace(
        this: &HtmlDocument,
        url: &str,
        name: &str,
        features: &str,
        replace: bool,
    ) -> Result<Option<Window>, JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = queryCommandEnabled ) ]
    #[doc = "The `queryCommandEnabled()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/queryCommandEnabled)\n\n*This API requires the following crate features to be activated: `HtmlDocument`*"]
    pub fn query_command_enabled(this: &HtmlDocument, command_id: &str) -> Result<bool, JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = queryCommandIndeterm ) ]
    #[doc = "The `queryCommandIndeterm()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/queryCommandIndeterm)\n\n*This API requires the following crate features to be activated: `HtmlDocument`*"]
    pub fn query_command_indeterm(this: &HtmlDocument, command_id: &str) -> Result<bool, JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = queryCommandState ) ]
    #[doc = "The `queryCommandState()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/queryCommandState)\n\n*This API requires the following crate features to be activated: `HtmlDocument`*"]
    pub fn query_command_state(this: &HtmlDocument, command_id: &str) -> Result<bool, JsValue>;
    # [ wasm_bindgen ( method , structural , js_name = queryCommandSupported ) ]
    #[doc = "The `queryCommandSupported()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/queryCommandSupported)\n\n*This API requires the following crate features to be activated: `HtmlDocument`*"]
    pub fn query_command_supported(this: &HtmlDocument, command_id: &str) -> bool;
    # [ wasm_bindgen ( catch , method , structural , js_name = queryCommandValue ) ]
    #[doc = "The `queryCommandValue()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/queryCommandValue)\n\n*This API requires the following crate features to be activated: `HtmlDocument`*"]
    pub fn query_command_value(this: &HtmlDocument, command_id: &str) -> Result<String, JsValue>;
    # [ wasm_bindgen ( method , structural , js_name = releaseEvents ) ]
    #[doc = "The `releaseEvents()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/releaseEvents)\n\n*This API requires the following crate features to be activated: `HtmlDocument`*"]
    pub fn release_events(this: &HtmlDocument);
    # [ wasm_bindgen ( catch , method , structural , variadic , js_name = write ) ]
    #[doc = "The `write()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/write)\n\n*This API requires the following crate features to be activated: `HtmlDocument`*"]
    pub fn write(this: &HtmlDocument, text: &str) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = write ) ]
    #[doc = "The `write()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/write)\n\n*This API requires the following crate features to be activated: `HtmlDocument`*"]
    pub fn write_0(this: &HtmlDocument) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = write ) ]
    #[doc = "The `write()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/write)\n\n*This API requires the following crate features to be activated: `HtmlDocument`*"]
    pub fn write_1(this: &HtmlDocument, text_1: &str) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = write ) ]
    #[doc = "The `write()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/write)\n\n*This API requires the following crate features to be activated: `HtmlDocument`*"]
    pub fn write_2(this: &HtmlDocument, text_1: &str, text_2: &str) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = write ) ]
    #[doc = "The `write()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/write)\n\n*This API requires the following crate features to be activated: `HtmlDocument`*"]
    pub fn write_3(
        this: &HtmlDocument,
        text_1: &str,
        text_2: &str,
        text_3: &str,
    ) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = write ) ]
    #[doc = "The `write()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/write)\n\n*This API requires the following crate features to be activated: `HtmlDocument`*"]
    pub fn write_4(
        this: &HtmlDocument,
        text_1: &str,
        text_2: &str,
        text_3: &str,
        text_4: &str,
    ) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = write ) ]
    #[doc = "The `write()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/write)\n\n*This API requires the following crate features to be activated: `HtmlDocument`*"]
    pub fn write_5(
        this: &HtmlDocument,
        text_1: &str,
        text_2: &str,
        text_3: &str,
        text_4: &str,
        text_5: &str,
    ) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = write ) ]
    #[doc = "The `write()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/write)\n\n*This API requires the following crate features to be activated: `HtmlDocument`*"]
    pub fn write_6(
        this: &HtmlDocument,
        text_1: &str,
        text_2: &str,
        text_3: &str,
        text_4: &str,
        text_5: &str,
        text_6: &str,
    ) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = write ) ]
    #[doc = "The `write()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/write)\n\n*This API requires the following crate features to be activated: `HtmlDocument`*"]
    pub fn write_7(
        this: &HtmlDocument,
        text_1: &str,
        text_2: &str,
        text_3: &str,
        text_4: &str,
        text_5: &str,
        text_6: &str,
        text_7: &str,
    ) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , variadic , js_name = writeln ) ]
    #[doc = "The `writeln()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/writeln)\n\n*This API requires the following crate features to be activated: `HtmlDocument`*"]
    pub fn writeln(this: &HtmlDocument, text: &str) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = writeln ) ]
    #[doc = "The `writeln()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/writeln)\n\n*This API requires the following crate features to be activated: `HtmlDocument`*"]
    pub fn writeln_0(this: &HtmlDocument) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = writeln ) ]
    #[doc = "The `writeln()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/writeln)\n\n*This API requires the following crate features to be activated: `HtmlDocument`*"]
    pub fn writeln_1(this: &HtmlDocument, text_1: &str) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = writeln ) ]
    #[doc = "The `writeln()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/writeln)\n\n*This API requires the following crate features to be activated: `HtmlDocument`*"]
    pub fn writeln_2(this: &HtmlDocument, text_1: &str, text_2: &str) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = writeln ) ]
    #[doc = "The `writeln()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/writeln)\n\n*This API requires the following crate features to be activated: `HtmlDocument`*"]
    pub fn writeln_3(
        this: &HtmlDocument,
        text_1: &str,
        text_2: &str,
        text_3: &str,
    ) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = writeln ) ]
    #[doc = "The `writeln()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/writeln)\n\n*This API requires the following crate features to be activated: `HtmlDocument`*"]
    pub fn writeln_4(
        this: &HtmlDocument,
        text_1: &str,
        text_2: &str,
        text_3: &str,
        text_4: &str,
    ) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = writeln ) ]
    #[doc = "The `writeln()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/writeln)\n\n*This API requires the following crate features to be activated: `HtmlDocument`*"]
    pub fn writeln_5(
        this: &HtmlDocument,
        text_1: &str,
        text_2: &str,
        text_3: &str,
        text_4: &str,
        text_5: &str,
    ) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = writeln ) ]
    #[doc = "The `writeln()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/writeln)\n\n*This API requires the following crate features to be activated: `HtmlDocument`*"]
    pub fn writeln_6(
        this: &HtmlDocument,
        text_1: &str,
        text_2: &str,
        text_3: &str,
        text_4: &str,
        text_5: &str,
        text_6: &str,
    ) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = writeln ) ]
    #[doc = "The `writeln()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/writeln)\n\n*This API requires the following crate features to be activated: `HtmlDocument`*"]
    pub fn writeln_7(
        this: &HtmlDocument,
        text_1: &str,
        text_2: &str,
        text_3: &str,
        text_4: &str,
        text_5: &str,
        text_6: &str,
        text_7: &str,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, structural, indexing_getter)]
    #[doc = "Indexing getter.\n\n\n\n*This API requires the following crate features to be activated: `HtmlDocument`*"]
    pub fn get(this: &HtmlDocument, name: &str) -> Result<::js_sys::Object, JsValue>;
}
