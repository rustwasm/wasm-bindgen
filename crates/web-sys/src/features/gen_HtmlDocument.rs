use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = Document , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = HTMLDocument , typescript_name = HTMLDocument ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `HtmlDocument` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument)
    ///
    ///*This API requires the following crate features to be activated: `HtmlDocument`*
    pub type HtmlDocument;

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLDocument" , js_name = domain ) ]
    ///Getter for the `domain` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/domain)
    ///
    ///*This API requires the following crate features to be activated: `HtmlDocument`*
    pub fn domain(this: &HtmlDocument) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLDocument" , js_name = domain ) ]
    ///Setter for the `domain` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/domain)
    ///
    ///*This API requires the following crate features to be activated: `HtmlDocument`*
    pub fn set_domain(this: &HtmlDocument, value: &str);

    # [ wasm_bindgen ( structural , catch , method , getter , js_class = "HTMLDocument" , js_name = cookie ) ]
    ///Getter for the `cookie` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/cookie)
    ///
    ///*This API requires the following crate features to be activated: `HtmlDocument`*
    pub fn cookie(this: &HtmlDocument) -> Result<String, JsValue>;

    # [ wasm_bindgen ( structural , catch , method , setter , js_class = "HTMLDocument" , js_name = cookie ) ]
    ///Setter for the `cookie` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/cookie)
    ///
    ///*This API requires the following crate features to be activated: `HtmlDocument`*
    pub fn set_cookie(this: &HtmlDocument, value: &str) -> Result<(), JsValue>;

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLDocument" , js_name = designMode ) ]
    ///Getter for the `designMode` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/designMode)
    ///
    ///*This API requires the following crate features to be activated: `HtmlDocument`*
    pub fn design_mode(this: &HtmlDocument) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLDocument" , js_name = designMode ) ]
    ///Setter for the `designMode` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/designMode)
    ///
    ///*This API requires the following crate features to be activated: `HtmlDocument`*
    pub fn set_design_mode(this: &HtmlDocument, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLDocument" , js_name = fgColor ) ]
    ///Getter for the `fgColor` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/fgColor)
    ///
    ///*This API requires the following crate features to be activated: `HtmlDocument`*
    pub fn fg_color(this: &HtmlDocument) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLDocument" , js_name = fgColor ) ]
    ///Setter for the `fgColor` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/fgColor)
    ///
    ///*This API requires the following crate features to be activated: `HtmlDocument`*
    pub fn set_fg_color(this: &HtmlDocument, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLDocument" , js_name = linkColor ) ]
    ///Getter for the `linkColor` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/linkColor)
    ///
    ///*This API requires the following crate features to be activated: `HtmlDocument`*
    pub fn link_color(this: &HtmlDocument) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLDocument" , js_name = linkColor ) ]
    ///Setter for the `linkColor` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/linkColor)
    ///
    ///*This API requires the following crate features to be activated: `HtmlDocument`*
    pub fn set_link_color(this: &HtmlDocument, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLDocument" , js_name = vlinkColor ) ]
    ///Getter for the `vlinkColor` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/vlinkColor)
    ///
    ///*This API requires the following crate features to be activated: `HtmlDocument`*
    pub fn vlink_color(this: &HtmlDocument) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLDocument" , js_name = vlinkColor ) ]
    ///Setter for the `vlinkColor` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/vlinkColor)
    ///
    ///*This API requires the following crate features to be activated: `HtmlDocument`*
    pub fn set_vlink_color(this: &HtmlDocument, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLDocument" , js_name = alinkColor ) ]
    ///Getter for the `alinkColor` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/alinkColor)
    ///
    ///*This API requires the following crate features to be activated: `HtmlDocument`*
    pub fn alink_color(this: &HtmlDocument) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLDocument" , js_name = alinkColor ) ]
    ///Setter for the `alinkColor` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/alinkColor)
    ///
    ///*This API requires the following crate features to be activated: `HtmlDocument`*
    pub fn set_alink_color(this: &HtmlDocument, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLDocument" , js_name = bgColor ) ]
    ///Getter for the `bgColor` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/bgColor)
    ///
    ///*This API requires the following crate features to be activated: `HtmlDocument`*
    pub fn bg_color(this: &HtmlDocument) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLDocument" , js_name = bgColor ) ]
    ///Setter for the `bgColor` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/bgColor)
    ///
    ///*This API requires the following crate features to be activated: `HtmlDocument`*
    pub fn set_bg_color(this: &HtmlDocument, value: &str);

    #[cfg(feature = "HtmlAllCollection")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLDocument" , js_name = all ) ]
    ///Getter for the `all` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/all)
    ///
    ///*This API requires the following crate features to be activated: `HtmlAllCollection`, `HtmlDocument`*
    pub fn all(this: &HtmlDocument) -> HtmlAllCollection;

    # [ wasm_bindgen ( method , structural , js_class = "HTMLDocument" , js_name = captureEvents ) ]
    ///The `captureEvents()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/captureEvents)
    ///
    ///*This API requires the following crate features to be activated: `HtmlDocument`*
    pub fn capture_events(this: &HtmlDocument);

    # [ wasm_bindgen ( method , structural , js_class = "HTMLDocument" , js_name = clear ) ]
    ///The `clear()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/clear)
    ///
    ///*This API requires the following crate features to be activated: `HtmlDocument`*
    pub fn clear(this: &HtmlDocument);

    # [ wasm_bindgen ( catch , method , structural , js_class = "HTMLDocument" , js_name = close ) ]
    ///The `close()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/close)
    ///
    ///*This API requires the following crate features to be activated: `HtmlDocument`*
    pub fn close(this: &HtmlDocument) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "HTMLDocument" , js_name = execCommand ) ]
    ///The `execCommand()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/execCommand)
    ///
    ///*This API requires the following crate features to be activated: `HtmlDocument`*
    pub fn exec_command(this: &HtmlDocument, command_id: &str) -> Result<bool, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "HTMLDocument" , js_name = execCommand ) ]
    ///The `execCommand()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/execCommand)
    ///
    ///*This API requires the following crate features to be activated: `HtmlDocument`*
    pub fn exec_command_with_show_ui(
        this: &HtmlDocument,
        command_id: &str,
        show_ui: bool,
    ) -> Result<bool, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "HTMLDocument" , js_name = execCommand ) ]
    ///The `execCommand()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/execCommand)
    ///
    ///*This API requires the following crate features to be activated: `HtmlDocument`*
    pub fn exec_command_with_show_ui_and_value(
        this: &HtmlDocument,
        command_id: &str,
        show_ui: bool,
        value: &str,
    ) -> Result<bool, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "HTMLDocument" , js_name = open ) ]
    ///The `open()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/open)
    ///
    ///*This API requires the following crate features to be activated: `HtmlDocument`*
    pub fn open(this: &HtmlDocument) -> Result<Document, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "HTMLDocument" , js_name = open ) ]
    ///The `open()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/open)
    ///
    ///*This API requires the following crate features to be activated: `HtmlDocument`*
    pub fn open_with_type(this: &HtmlDocument, type_: &str) -> Result<Document, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "HTMLDocument" , js_name = open ) ]
    ///The `open()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/open)
    ///
    ///*This API requires the following crate features to be activated: `HtmlDocument`*
    pub fn open_with_type_and_replace(
        this: &HtmlDocument,
        type_: &str,
        replace: &str,
    ) -> Result<Document, JsValue>;

    #[cfg(feature = "Window")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "HTMLDocument" , js_name = open ) ]
    ///The `open()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/open)
    ///
    ///*This API requires the following crate features to be activated: `HtmlDocument`, `Window`*
    pub fn open_with_url_and_name_and_features(
        this: &HtmlDocument,
        url: &str,
        name: &str,
        features: &str,
    ) -> Result<Option<Window>, JsValue>;

    #[cfg(feature = "Window")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "HTMLDocument" , js_name = open ) ]
    ///The `open()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/open)
    ///
    ///*This API requires the following crate features to be activated: `HtmlDocument`, `Window`*
    pub fn open_with_url_and_name_and_features_and_replace(
        this: &HtmlDocument,
        url: &str,
        name: &str,
        features: &str,
        replace: bool,
    ) -> Result<Option<Window>, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "HTMLDocument" , js_name = queryCommandEnabled ) ]
    ///The `queryCommandEnabled()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/queryCommandEnabled)
    ///
    ///*This API requires the following crate features to be activated: `HtmlDocument`*
    pub fn query_command_enabled(this: &HtmlDocument, command_id: &str) -> Result<bool, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "HTMLDocument" , js_name = queryCommandIndeterm ) ]
    ///The `queryCommandIndeterm()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/queryCommandIndeterm)
    ///
    ///*This API requires the following crate features to be activated: `HtmlDocument`*
    pub fn query_command_indeterm(this: &HtmlDocument, command_id: &str) -> Result<bool, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "HTMLDocument" , js_name = queryCommandState ) ]
    ///The `queryCommandState()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/queryCommandState)
    ///
    ///*This API requires the following crate features to be activated: `HtmlDocument`*
    pub fn query_command_state(this: &HtmlDocument, command_id: &str) -> Result<bool, JsValue>;

    # [ wasm_bindgen ( method , structural , js_class = "HTMLDocument" , js_name = queryCommandSupported ) ]
    ///The `queryCommandSupported()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/queryCommandSupported)
    ///
    ///*This API requires the following crate features to be activated: `HtmlDocument`*
    pub fn query_command_supported(this: &HtmlDocument, command_id: &str) -> bool;

    # [ wasm_bindgen ( catch , method , structural , js_class = "HTMLDocument" , js_name = queryCommandValue ) ]
    ///The `queryCommandValue()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/queryCommandValue)
    ///
    ///*This API requires the following crate features to be activated: `HtmlDocument`*
    pub fn query_command_value(this: &HtmlDocument, command_id: &str) -> Result<String, JsValue>;

    # [ wasm_bindgen ( method , structural , js_class = "HTMLDocument" , js_name = releaseEvents ) ]
    ///The `releaseEvents()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/releaseEvents)
    ///
    ///*This API requires the following crate features to be activated: `HtmlDocument`*
    pub fn release_events(this: &HtmlDocument);

    # [ wasm_bindgen ( catch , method , structural , variadic , js_class = "HTMLDocument" , js_name = write ) ]
    ///The `write()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/write)
    ///
    ///*This API requires the following crate features to be activated: `HtmlDocument`*
    pub fn write(this: &HtmlDocument, text: &::js_sys::Array) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "HTMLDocument" , js_name = write ) ]
    ///The `write()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/write)
    ///
    ///*This API requires the following crate features to be activated: `HtmlDocument`*
    pub fn write_0(this: &HtmlDocument) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "HTMLDocument" , js_name = write ) ]
    ///The `write()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/write)
    ///
    ///*This API requires the following crate features to be activated: `HtmlDocument`*
    pub fn write_1(this: &HtmlDocument, text_1: &str) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "HTMLDocument" , js_name = write ) ]
    ///The `write()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/write)
    ///
    ///*This API requires the following crate features to be activated: `HtmlDocument`*
    pub fn write_2(this: &HtmlDocument, text_1: &str, text_2: &str) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "HTMLDocument" , js_name = write ) ]
    ///The `write()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/write)
    ///
    ///*This API requires the following crate features to be activated: `HtmlDocument`*
    pub fn write_3(
        this: &HtmlDocument,
        text_1: &str,
        text_2: &str,
        text_3: &str,
    ) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "HTMLDocument" , js_name = write ) ]
    ///The `write()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/write)
    ///
    ///*This API requires the following crate features to be activated: `HtmlDocument`*
    pub fn write_4(
        this: &HtmlDocument,
        text_1: &str,
        text_2: &str,
        text_3: &str,
        text_4: &str,
    ) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "HTMLDocument" , js_name = write ) ]
    ///The `write()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/write)
    ///
    ///*This API requires the following crate features to be activated: `HtmlDocument`*
    pub fn write_5(
        this: &HtmlDocument,
        text_1: &str,
        text_2: &str,
        text_3: &str,
        text_4: &str,
        text_5: &str,
    ) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "HTMLDocument" , js_name = write ) ]
    ///The `write()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/write)
    ///
    ///*This API requires the following crate features to be activated: `HtmlDocument`*
    pub fn write_6(
        this: &HtmlDocument,
        text_1: &str,
        text_2: &str,
        text_3: &str,
        text_4: &str,
        text_5: &str,
        text_6: &str,
    ) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "HTMLDocument" , js_name = write ) ]
    ///The `write()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/write)
    ///
    ///*This API requires the following crate features to be activated: `HtmlDocument`*
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

    # [ wasm_bindgen ( catch , method , structural , variadic , js_class = "HTMLDocument" , js_name = writeln ) ]
    ///The `writeln()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/writeln)
    ///
    ///*This API requires the following crate features to be activated: `HtmlDocument`*
    pub fn writeln(this: &HtmlDocument, text: &::js_sys::Array) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "HTMLDocument" , js_name = writeln ) ]
    ///The `writeln()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/writeln)
    ///
    ///*This API requires the following crate features to be activated: `HtmlDocument`*
    pub fn writeln_0(this: &HtmlDocument) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "HTMLDocument" , js_name = writeln ) ]
    ///The `writeln()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/writeln)
    ///
    ///*This API requires the following crate features to be activated: `HtmlDocument`*
    pub fn writeln_1(this: &HtmlDocument, text_1: &str) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "HTMLDocument" , js_name = writeln ) ]
    ///The `writeln()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/writeln)
    ///
    ///*This API requires the following crate features to be activated: `HtmlDocument`*
    pub fn writeln_2(this: &HtmlDocument, text_1: &str, text_2: &str) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "HTMLDocument" , js_name = writeln ) ]
    ///The `writeln()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/writeln)
    ///
    ///*This API requires the following crate features to be activated: `HtmlDocument`*
    pub fn writeln_3(
        this: &HtmlDocument,
        text_1: &str,
        text_2: &str,
        text_3: &str,
    ) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "HTMLDocument" , js_name = writeln ) ]
    ///The `writeln()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/writeln)
    ///
    ///*This API requires the following crate features to be activated: `HtmlDocument`*
    pub fn writeln_4(
        this: &HtmlDocument,
        text_1: &str,
        text_2: &str,
        text_3: &str,
        text_4: &str,
    ) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "HTMLDocument" , js_name = writeln ) ]
    ///The `writeln()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/writeln)
    ///
    ///*This API requires the following crate features to be activated: `HtmlDocument`*
    pub fn writeln_5(
        this: &HtmlDocument,
        text_1: &str,
        text_2: &str,
        text_3: &str,
        text_4: &str,
        text_5: &str,
    ) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "HTMLDocument" , js_name = writeln ) ]
    ///The `writeln()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/writeln)
    ///
    ///*This API requires the following crate features to be activated: `HtmlDocument`*
    pub fn writeln_6(
        this: &HtmlDocument,
        text_1: &str,
        text_2: &str,
        text_3: &str,
        text_4: &str,
        text_5: &str,
        text_6: &str,
    ) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "HTMLDocument" , js_name = writeln ) ]
    ///The `writeln()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/writeln)
    ///
    ///*This API requires the following crate features to be activated: `HtmlDocument`*
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

    #[wasm_bindgen(catch, method, structural, js_class = "HTMLDocument", indexing_getter)]
    ///Indexing getter.
    ///
    ///
    ///
    ///*This API requires the following crate features to be activated: `HtmlDocument`*
    pub fn get(this: &HtmlDocument, name: &str) -> Result<::js_sys::Object, JsValue>;

}
