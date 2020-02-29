use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = Document , typescript_name = Document ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `Document` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub type Document;

    #[cfg(feature = "DomImplementation")]
    # [ wasm_bindgen ( structural , catch , method , getter , js_class = "Document" , js_name = implementation ) ]
    ///Getter for the `implementation` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/implementation)
    ///
    ///*This API requires the following crate features to be activated: `Document`, `DomImplementation`*
    pub fn implementation(this: &Document) -> Result<DomImplementation, JsValue>;

    # [ wasm_bindgen ( structural , catch , method , getter , js_class = "Document" , js_name = URL ) ]
    ///Getter for the `URL` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/URL)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn url(this: &Document) -> Result<String, JsValue>;

    # [ wasm_bindgen ( structural , catch , method , getter , js_class = "Document" , js_name = documentURI ) ]
    ///Getter for the `documentURI` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/documentURI)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn document_uri(this: &Document) -> Result<String, JsValue>;

    # [ wasm_bindgen ( structural , method , getter , js_class = "Document" , js_name = compatMode ) ]
    ///Getter for the `compatMode` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/compatMode)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn compat_mode(this: &Document) -> String;

    # [ wasm_bindgen ( structural , method , getter , js_class = "Document" , js_name = characterSet ) ]
    ///Getter for the `characterSet` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/characterSet)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn character_set(this: &Document) -> String;

    # [ wasm_bindgen ( structural , method , getter , js_class = "Document" , js_name = charset ) ]
    ///Getter for the `charset` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/charset)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn charset(this: &Document) -> String;

    # [ wasm_bindgen ( structural , method , getter , js_class = "Document" , js_name = inputEncoding ) ]
    ///Getter for the `inputEncoding` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/inputEncoding)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn input_encoding(this: &Document) -> String;

    # [ wasm_bindgen ( structural , method , getter , js_class = "Document" , js_name = contentType ) ]
    ///Getter for the `contentType` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/contentType)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn content_type(this: &Document) -> String;

    #[cfg(feature = "DocumentType")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "Document" , js_name = doctype ) ]
    ///Getter for the `doctype` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/doctype)
    ///
    ///*This API requires the following crate features to be activated: `Document`, `DocumentType`*
    pub fn doctype(this: &Document) -> Option<DocumentType>;

    #[cfg(feature = "Element")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "Document" , js_name = documentElement ) ]
    ///Getter for the `documentElement` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/documentElement)
    ///
    ///*This API requires the following crate features to be activated: `Document`, `Element`*
    pub fn document_element(this: &Document) -> Option<Element>;

    #[cfg(feature = "Location")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "Document" , js_name = location ) ]
    ///Getter for the `location` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/location)
    ///
    ///*This API requires the following crate features to be activated: `Document`, `Location`*
    pub fn location(this: &Document) -> Option<Location>;

    # [ wasm_bindgen ( structural , method , getter , js_class = "Document" , js_name = referrer ) ]
    ///Getter for the `referrer` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/referrer)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn referrer(this: &Document) -> String;

    # [ wasm_bindgen ( structural , method , getter , js_class = "Document" , js_name = lastModified ) ]
    ///Getter for the `lastModified` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/lastModified)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn last_modified(this: &Document) -> String;

    # [ wasm_bindgen ( structural , method , getter , js_class = "Document" , js_name = readyState ) ]
    ///Getter for the `readyState` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/readyState)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn ready_state(this: &Document) -> String;

    # [ wasm_bindgen ( structural , method , getter , js_class = "Document" , js_name = title ) ]
    ///Getter for the `title` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/title)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn title(this: &Document) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Document" , js_name = title ) ]
    ///Setter for the `title` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/title)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn set_title(this: &Document, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "Document" , js_name = dir ) ]
    ///Getter for the `dir` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/dir)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn dir(this: &Document) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Document" , js_name = dir ) ]
    ///Setter for the `dir` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/dir)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn set_dir(this: &Document, value: &str);

    #[cfg(feature = "HtmlElement")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "Document" , js_name = body ) ]
    ///Getter for the `body` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/body)
    ///
    ///*This API requires the following crate features to be activated: `Document`, `HtmlElement`*
    pub fn body(this: &Document) -> Option<HtmlElement>;

    #[cfg(feature = "HtmlElement")]
    # [ wasm_bindgen ( structural , method , setter , js_class = "Document" , js_name = body ) ]
    ///Setter for the `body` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/body)
    ///
    ///*This API requires the following crate features to be activated: `Document`, `HtmlElement`*
    pub fn set_body(this: &Document, value: Option<&HtmlElement>);

    #[cfg(feature = "HtmlHeadElement")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "Document" , js_name = head ) ]
    ///Getter for the `head` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/head)
    ///
    ///*This API requires the following crate features to be activated: `Document`, `HtmlHeadElement`*
    pub fn head(this: &Document) -> Option<HtmlHeadElement>;

    #[cfg(feature = "HtmlCollection")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "Document" , js_name = images ) ]
    ///Getter for the `images` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/images)
    ///
    ///*This API requires the following crate features to be activated: `Document`, `HtmlCollection`*
    pub fn images(this: &Document) -> HtmlCollection;

    #[cfg(feature = "HtmlCollection")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "Document" , js_name = embeds ) ]
    ///Getter for the `embeds` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/embeds)
    ///
    ///*This API requires the following crate features to be activated: `Document`, `HtmlCollection`*
    pub fn embeds(this: &Document) -> HtmlCollection;

    #[cfg(feature = "HtmlCollection")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "Document" , js_name = plugins ) ]
    ///Getter for the `plugins` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/plugins)
    ///
    ///*This API requires the following crate features to be activated: `Document`, `HtmlCollection`*
    pub fn plugins(this: &Document) -> HtmlCollection;

    #[cfg(feature = "HtmlCollection")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "Document" , js_name = links ) ]
    ///Getter for the `links` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/links)
    ///
    ///*This API requires the following crate features to be activated: `Document`, `HtmlCollection`*
    pub fn links(this: &Document) -> HtmlCollection;

    #[cfg(feature = "HtmlCollection")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "Document" , js_name = forms ) ]
    ///Getter for the `forms` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/forms)
    ///
    ///*This API requires the following crate features to be activated: `Document`, `HtmlCollection`*
    pub fn forms(this: &Document) -> HtmlCollection;

    #[cfg(feature = "HtmlCollection")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "Document" , js_name = scripts ) ]
    ///Getter for the `scripts` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/scripts)
    ///
    ///*This API requires the following crate features to be activated: `Document`, `HtmlCollection`*
    pub fn scripts(this: &Document) -> HtmlCollection;

    #[cfg(feature = "Window")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "Document" , js_name = defaultView ) ]
    ///Getter for the `defaultView` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/defaultView)
    ///
    ///*This API requires the following crate features to be activated: `Document`, `Window`*
    pub fn default_view(this: &Document) -> Option<Window>;

    # [ wasm_bindgen ( structural , method , getter , js_class = "Document" , js_name = onreadystatechange ) ]
    ///Getter for the `onreadystatechange` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onreadystatechange)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn onreadystatechange(this: &Document) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Document" , js_name = onreadystatechange ) ]
    ///Setter for the `onreadystatechange` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onreadystatechange)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn set_onreadystatechange(this: &Document, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "Document" , js_name = onbeforescriptexecute ) ]
    ///Getter for the `onbeforescriptexecute` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onbeforescriptexecute)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn onbeforescriptexecute(this: &Document) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Document" , js_name = onbeforescriptexecute ) ]
    ///Setter for the `onbeforescriptexecute` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onbeforescriptexecute)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn set_onbeforescriptexecute(this: &Document, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "Document" , js_name = onafterscriptexecute ) ]
    ///Getter for the `onafterscriptexecute` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onafterscriptexecute)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn onafterscriptexecute(this: &Document) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Document" , js_name = onafterscriptexecute ) ]
    ///Setter for the `onafterscriptexecute` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onafterscriptexecute)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn set_onafterscriptexecute(this: &Document, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "Document" , js_name = onselectionchange ) ]
    ///Getter for the `onselectionchange` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onselectionchange)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn onselectionchange(this: &Document) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Document" , js_name = onselectionchange ) ]
    ///Setter for the `onselectionchange` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onselectionchange)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn set_onselectionchange(this: &Document, value: Option<&::js_sys::Function>);

    #[cfg(feature = "Element")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "Document" , js_name = currentScript ) ]
    ///Getter for the `currentScript` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/currentScript)
    ///
    ///*This API requires the following crate features to be activated: `Document`, `Element`*
    pub fn current_script(this: &Document) -> Option<Element>;

    #[cfg(feature = "HtmlCollection")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "Document" , js_name = anchors ) ]
    ///Getter for the `anchors` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/anchors)
    ///
    ///*This API requires the following crate features to be activated: `Document`, `HtmlCollection`*
    pub fn anchors(this: &Document) -> HtmlCollection;

    #[cfg(feature = "HtmlCollection")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "Document" , js_name = applets ) ]
    ///Getter for the `applets` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/applets)
    ///
    ///*This API requires the following crate features to be activated: `Document`, `HtmlCollection`*
    pub fn applets(this: &Document) -> HtmlCollection;

    # [ wasm_bindgen ( structural , method , getter , js_class = "Document" , js_name = fullscreen ) ]
    ///Getter for the `fullscreen` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/fullscreen)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn fullscreen(this: &Document) -> bool;

    # [ wasm_bindgen ( structural , method , getter , js_class = "Document" , js_name = fullscreenEnabled ) ]
    ///Getter for the `fullscreenEnabled` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/fullscreenEnabled)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn fullscreen_enabled(this: &Document) -> bool;

    # [ wasm_bindgen ( structural , method , getter , js_class = "Document" , js_name = onfullscreenchange ) ]
    ///Getter for the `onfullscreenchange` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onfullscreenchange)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn onfullscreenchange(this: &Document) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Document" , js_name = onfullscreenchange ) ]
    ///Setter for the `onfullscreenchange` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onfullscreenchange)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn set_onfullscreenchange(this: &Document, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "Document" , js_name = onfullscreenerror ) ]
    ///Getter for the `onfullscreenerror` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onfullscreenerror)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn onfullscreenerror(this: &Document) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Document" , js_name = onfullscreenerror ) ]
    ///Setter for the `onfullscreenerror` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onfullscreenerror)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn set_onfullscreenerror(this: &Document, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "Document" , js_name = onpointerlockchange ) ]
    ///Getter for the `onpointerlockchange` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onpointerlockchange)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn onpointerlockchange(this: &Document) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Document" , js_name = onpointerlockchange ) ]
    ///Setter for the `onpointerlockchange` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onpointerlockchange)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn set_onpointerlockchange(this: &Document, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "Document" , js_name = onpointerlockerror ) ]
    ///Getter for the `onpointerlockerror` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onpointerlockerror)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn onpointerlockerror(this: &Document) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Document" , js_name = onpointerlockerror ) ]
    ///Setter for the `onpointerlockerror` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onpointerlockerror)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn set_onpointerlockerror(this: &Document, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "Document" , js_name = hidden ) ]
    ///Getter for the `hidden` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/hidden)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn hidden(this: &Document) -> bool;

    #[cfg(feature = "VisibilityState")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "Document" , js_name = visibilityState ) ]
    ///Getter for the `visibilityState` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/visibilityState)
    ///
    ///*This API requires the following crate features to be activated: `Document`, `VisibilityState`*
    pub fn visibility_state(this: &Document) -> VisibilityState;

    # [ wasm_bindgen ( structural , method , getter , js_class = "Document" , js_name = onvisibilitychange ) ]
    ///Getter for the `onvisibilitychange` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onvisibilitychange)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn onvisibilitychange(this: &Document) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Document" , js_name = onvisibilitychange ) ]
    ///Setter for the `onvisibilitychange` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onvisibilitychange)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn set_onvisibilitychange(this: &Document, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "Document" , js_name = selectedStyleSheetSet ) ]
    ///Getter for the `selectedStyleSheetSet` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/selectedStyleSheetSet)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn selected_style_sheet_set(this: &Document) -> Option<String>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Document" , js_name = selectedStyleSheetSet ) ]
    ///Setter for the `selectedStyleSheetSet` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/selectedStyleSheetSet)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn set_selected_style_sheet_set(this: &Document, value: Option<&str>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "Document" , js_name = lastStyleSheetSet ) ]
    ///Getter for the `lastStyleSheetSet` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/lastStyleSheetSet)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn last_style_sheet_set(this: &Document) -> Option<String>;

    # [ wasm_bindgen ( structural , method , getter , js_class = "Document" , js_name = preferredStyleSheetSet ) ]
    ///Getter for the `preferredStyleSheetSet` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/preferredStyleSheetSet)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn preferred_style_sheet_set(this: &Document) -> Option<String>;

    #[cfg(feature = "DomStringList")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "Document" , js_name = styleSheetSets ) ]
    ///Getter for the `styleSheetSets` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/styleSheetSets)
    ///
    ///*This API requires the following crate features to be activated: `Document`, `DomStringList`*
    pub fn style_sheet_sets(this: &Document) -> DomStringList;

    #[cfg(feature = "Element")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "Document" , js_name = scrollingElement ) ]
    ///Getter for the `scrollingElement` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/scrollingElement)
    ///
    ///*This API requires the following crate features to be activated: `Document`, `Element`*
    pub fn scrolling_element(this: &Document) -> Option<Element>;

    #[cfg(feature = "DocumentTimeline")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "Document" , js_name = timeline ) ]
    ///Getter for the `timeline` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/timeline)
    ///
    ///*This API requires the following crate features to be activated: `Document`, `DocumentTimeline`*
    pub fn timeline(this: &Document) -> DocumentTimeline;

    #[cfg(feature = "SvgsvgElement")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "Document" , js_name = rootElement ) ]
    ///Getter for the `rootElement` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/rootElement)
    ///
    ///*This API requires the following crate features to be activated: `Document`, `SvgsvgElement`*
    pub fn root_element(this: &Document) -> Option<SvgsvgElement>;

    # [ wasm_bindgen ( structural , method , getter , js_class = "Document" , js_name = oncopy ) ]
    ///Getter for the `oncopy` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/oncopy)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn oncopy(this: &Document) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Document" , js_name = oncopy ) ]
    ///Setter for the `oncopy` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/oncopy)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn set_oncopy(this: &Document, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "Document" , js_name = oncut ) ]
    ///Getter for the `oncut` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/oncut)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn oncut(this: &Document) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Document" , js_name = oncut ) ]
    ///Setter for the `oncut` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/oncut)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn set_oncut(this: &Document, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "Document" , js_name = onpaste ) ]
    ///Getter for the `onpaste` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onpaste)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn onpaste(this: &Document) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Document" , js_name = onpaste ) ]
    ///Setter for the `onpaste` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onpaste)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn set_onpaste(this: &Document, value: Option<&::js_sys::Function>);

    #[cfg(feature = "Element")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "Document" , js_name = activeElement ) ]
    ///Getter for the `activeElement` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/activeElement)
    ///
    ///*This API requires the following crate features to be activated: `Document`, `Element`*
    pub fn active_element(this: &Document) -> Option<Element>;

    #[cfg(feature = "StyleSheetList")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "Document" , js_name = styleSheets ) ]
    ///Getter for the `styleSheets` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/styleSheets)
    ///
    ///*This API requires the following crate features to be activated: `Document`, `StyleSheetList`*
    pub fn style_sheets(this: &Document) -> StyleSheetList;

    #[cfg(feature = "Element")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "Document" , js_name = pointerLockElement ) ]
    ///Getter for the `pointerLockElement` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/pointerLockElement)
    ///
    ///*This API requires the following crate features to be activated: `Document`, `Element`*
    pub fn pointer_lock_element(this: &Document) -> Option<Element>;

    #[cfg(feature = "Element")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "Document" , js_name = fullscreenElement ) ]
    ///Getter for the `fullscreenElement` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/fullscreenElement)
    ///
    ///*This API requires the following crate features to be activated: `Document`, `Element`*
    pub fn fullscreen_element(this: &Document) -> Option<Element>;

    #[cfg(feature = "FontFaceSet")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "Document" , js_name = fonts ) ]
    ///Getter for the `fonts` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/fonts)
    ///
    ///*This API requires the following crate features to be activated: `Document`, `FontFaceSet`*
    pub fn fonts(this: &Document) -> FontFaceSet;

    # [ wasm_bindgen ( structural , method , getter , js_class = "Document" , js_name = onabort ) ]
    ///Getter for the `onabort` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onabort)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn onabort(this: &Document) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Document" , js_name = onabort ) ]
    ///Setter for the `onabort` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onabort)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn set_onabort(this: &Document, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "Document" , js_name = onblur ) ]
    ///Getter for the `onblur` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onblur)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn onblur(this: &Document) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Document" , js_name = onblur ) ]
    ///Setter for the `onblur` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onblur)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn set_onblur(this: &Document, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "Document" , js_name = onfocus ) ]
    ///Getter for the `onfocus` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onfocus)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn onfocus(this: &Document) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Document" , js_name = onfocus ) ]
    ///Setter for the `onfocus` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onfocus)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn set_onfocus(this: &Document, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "Document" , js_name = onauxclick ) ]
    ///Getter for the `onauxclick` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onauxclick)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn onauxclick(this: &Document) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Document" , js_name = onauxclick ) ]
    ///Setter for the `onauxclick` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onauxclick)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn set_onauxclick(this: &Document, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "Document" , js_name = oncanplay ) ]
    ///Getter for the `oncanplay` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/oncanplay)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn oncanplay(this: &Document) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Document" , js_name = oncanplay ) ]
    ///Setter for the `oncanplay` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/oncanplay)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn set_oncanplay(this: &Document, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "Document" , js_name = oncanplaythrough ) ]
    ///Getter for the `oncanplaythrough` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/oncanplaythrough)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn oncanplaythrough(this: &Document) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Document" , js_name = oncanplaythrough ) ]
    ///Setter for the `oncanplaythrough` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/oncanplaythrough)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn set_oncanplaythrough(this: &Document, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "Document" , js_name = onchange ) ]
    ///Getter for the `onchange` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onchange)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn onchange(this: &Document) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Document" , js_name = onchange ) ]
    ///Setter for the `onchange` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onchange)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn set_onchange(this: &Document, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "Document" , js_name = onclick ) ]
    ///Getter for the `onclick` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onclick)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn onclick(this: &Document) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Document" , js_name = onclick ) ]
    ///Setter for the `onclick` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onclick)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn set_onclick(this: &Document, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "Document" , js_name = onclose ) ]
    ///Getter for the `onclose` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onclose)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn onclose(this: &Document) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Document" , js_name = onclose ) ]
    ///Setter for the `onclose` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onclose)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn set_onclose(this: &Document, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "Document" , js_name = oncontextmenu ) ]
    ///Getter for the `oncontextmenu` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/oncontextmenu)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn oncontextmenu(this: &Document) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Document" , js_name = oncontextmenu ) ]
    ///Setter for the `oncontextmenu` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/oncontextmenu)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn set_oncontextmenu(this: &Document, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "Document" , js_name = ondblclick ) ]
    ///Getter for the `ondblclick` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/ondblclick)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn ondblclick(this: &Document) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Document" , js_name = ondblclick ) ]
    ///Setter for the `ondblclick` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/ondblclick)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn set_ondblclick(this: &Document, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "Document" , js_name = ondrag ) ]
    ///Getter for the `ondrag` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/ondrag)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn ondrag(this: &Document) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Document" , js_name = ondrag ) ]
    ///Setter for the `ondrag` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/ondrag)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn set_ondrag(this: &Document, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "Document" , js_name = ondragend ) ]
    ///Getter for the `ondragend` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/ondragend)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn ondragend(this: &Document) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Document" , js_name = ondragend ) ]
    ///Setter for the `ondragend` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/ondragend)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn set_ondragend(this: &Document, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "Document" , js_name = ondragenter ) ]
    ///Getter for the `ondragenter` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/ondragenter)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn ondragenter(this: &Document) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Document" , js_name = ondragenter ) ]
    ///Setter for the `ondragenter` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/ondragenter)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn set_ondragenter(this: &Document, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "Document" , js_name = ondragexit ) ]
    ///Getter for the `ondragexit` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/ondragexit)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn ondragexit(this: &Document) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Document" , js_name = ondragexit ) ]
    ///Setter for the `ondragexit` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/ondragexit)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn set_ondragexit(this: &Document, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "Document" , js_name = ondragleave ) ]
    ///Getter for the `ondragleave` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/ondragleave)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn ondragleave(this: &Document) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Document" , js_name = ondragleave ) ]
    ///Setter for the `ondragleave` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/ondragleave)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn set_ondragleave(this: &Document, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "Document" , js_name = ondragover ) ]
    ///Getter for the `ondragover` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/ondragover)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn ondragover(this: &Document) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Document" , js_name = ondragover ) ]
    ///Setter for the `ondragover` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/ondragover)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn set_ondragover(this: &Document, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "Document" , js_name = ondragstart ) ]
    ///Getter for the `ondragstart` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/ondragstart)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn ondragstart(this: &Document) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Document" , js_name = ondragstart ) ]
    ///Setter for the `ondragstart` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/ondragstart)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn set_ondragstart(this: &Document, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "Document" , js_name = ondrop ) ]
    ///Getter for the `ondrop` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/ondrop)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn ondrop(this: &Document) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Document" , js_name = ondrop ) ]
    ///Setter for the `ondrop` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/ondrop)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn set_ondrop(this: &Document, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "Document" , js_name = ondurationchange ) ]
    ///Getter for the `ondurationchange` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/ondurationchange)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn ondurationchange(this: &Document) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Document" , js_name = ondurationchange ) ]
    ///Setter for the `ondurationchange` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/ondurationchange)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn set_ondurationchange(this: &Document, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "Document" , js_name = onemptied ) ]
    ///Getter for the `onemptied` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onemptied)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn onemptied(this: &Document) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Document" , js_name = onemptied ) ]
    ///Setter for the `onemptied` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onemptied)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn set_onemptied(this: &Document, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "Document" , js_name = onended ) ]
    ///Getter for the `onended` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onended)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn onended(this: &Document) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Document" , js_name = onended ) ]
    ///Setter for the `onended` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onended)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn set_onended(this: &Document, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "Document" , js_name = oninput ) ]
    ///Getter for the `oninput` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/oninput)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn oninput(this: &Document) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Document" , js_name = oninput ) ]
    ///Setter for the `oninput` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/oninput)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn set_oninput(this: &Document, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "Document" , js_name = oninvalid ) ]
    ///Getter for the `oninvalid` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/oninvalid)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn oninvalid(this: &Document) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Document" , js_name = oninvalid ) ]
    ///Setter for the `oninvalid` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/oninvalid)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn set_oninvalid(this: &Document, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "Document" , js_name = onkeydown ) ]
    ///Getter for the `onkeydown` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onkeydown)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn onkeydown(this: &Document) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Document" , js_name = onkeydown ) ]
    ///Setter for the `onkeydown` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onkeydown)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn set_onkeydown(this: &Document, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "Document" , js_name = onkeypress ) ]
    ///Getter for the `onkeypress` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onkeypress)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn onkeypress(this: &Document) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Document" , js_name = onkeypress ) ]
    ///Setter for the `onkeypress` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onkeypress)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn set_onkeypress(this: &Document, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "Document" , js_name = onkeyup ) ]
    ///Getter for the `onkeyup` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onkeyup)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn onkeyup(this: &Document) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Document" , js_name = onkeyup ) ]
    ///Setter for the `onkeyup` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onkeyup)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn set_onkeyup(this: &Document, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "Document" , js_name = onload ) ]
    ///Getter for the `onload` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onload)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn onload(this: &Document) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Document" , js_name = onload ) ]
    ///Setter for the `onload` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onload)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn set_onload(this: &Document, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "Document" , js_name = onloadeddata ) ]
    ///Getter for the `onloadeddata` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onloadeddata)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn onloadeddata(this: &Document) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Document" , js_name = onloadeddata ) ]
    ///Setter for the `onloadeddata` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onloadeddata)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn set_onloadeddata(this: &Document, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "Document" , js_name = onloadedmetadata ) ]
    ///Getter for the `onloadedmetadata` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onloadedmetadata)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn onloadedmetadata(this: &Document) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Document" , js_name = onloadedmetadata ) ]
    ///Setter for the `onloadedmetadata` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onloadedmetadata)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn set_onloadedmetadata(this: &Document, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "Document" , js_name = onloadend ) ]
    ///Getter for the `onloadend` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onloadend)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn onloadend(this: &Document) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Document" , js_name = onloadend ) ]
    ///Setter for the `onloadend` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onloadend)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn set_onloadend(this: &Document, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "Document" , js_name = onloadstart ) ]
    ///Getter for the `onloadstart` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onloadstart)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn onloadstart(this: &Document) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Document" , js_name = onloadstart ) ]
    ///Setter for the `onloadstart` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onloadstart)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn set_onloadstart(this: &Document, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "Document" , js_name = onmousedown ) ]
    ///Getter for the `onmousedown` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onmousedown)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn onmousedown(this: &Document) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Document" , js_name = onmousedown ) ]
    ///Setter for the `onmousedown` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onmousedown)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn set_onmousedown(this: &Document, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "Document" , js_name = onmouseenter ) ]
    ///Getter for the `onmouseenter` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onmouseenter)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn onmouseenter(this: &Document) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Document" , js_name = onmouseenter ) ]
    ///Setter for the `onmouseenter` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onmouseenter)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn set_onmouseenter(this: &Document, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "Document" , js_name = onmouseleave ) ]
    ///Getter for the `onmouseleave` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onmouseleave)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn onmouseleave(this: &Document) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Document" , js_name = onmouseleave ) ]
    ///Setter for the `onmouseleave` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onmouseleave)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn set_onmouseleave(this: &Document, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "Document" , js_name = onmousemove ) ]
    ///Getter for the `onmousemove` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onmousemove)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn onmousemove(this: &Document) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Document" , js_name = onmousemove ) ]
    ///Setter for the `onmousemove` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onmousemove)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn set_onmousemove(this: &Document, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "Document" , js_name = onmouseout ) ]
    ///Getter for the `onmouseout` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onmouseout)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn onmouseout(this: &Document) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Document" , js_name = onmouseout ) ]
    ///Setter for the `onmouseout` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onmouseout)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn set_onmouseout(this: &Document, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "Document" , js_name = onmouseover ) ]
    ///Getter for the `onmouseover` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onmouseover)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn onmouseover(this: &Document) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Document" , js_name = onmouseover ) ]
    ///Setter for the `onmouseover` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onmouseover)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn set_onmouseover(this: &Document, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "Document" , js_name = onmouseup ) ]
    ///Getter for the `onmouseup` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onmouseup)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn onmouseup(this: &Document) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Document" , js_name = onmouseup ) ]
    ///Setter for the `onmouseup` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onmouseup)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn set_onmouseup(this: &Document, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "Document" , js_name = onwheel ) ]
    ///Getter for the `onwheel` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onwheel)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn onwheel(this: &Document) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Document" , js_name = onwheel ) ]
    ///Setter for the `onwheel` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onwheel)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn set_onwheel(this: &Document, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "Document" , js_name = onpause ) ]
    ///Getter for the `onpause` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onpause)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn onpause(this: &Document) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Document" , js_name = onpause ) ]
    ///Setter for the `onpause` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onpause)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn set_onpause(this: &Document, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "Document" , js_name = onplay ) ]
    ///Getter for the `onplay` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onplay)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn onplay(this: &Document) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Document" , js_name = onplay ) ]
    ///Setter for the `onplay` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onplay)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn set_onplay(this: &Document, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "Document" , js_name = onplaying ) ]
    ///Getter for the `onplaying` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onplaying)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn onplaying(this: &Document) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Document" , js_name = onplaying ) ]
    ///Setter for the `onplaying` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onplaying)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn set_onplaying(this: &Document, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "Document" , js_name = onprogress ) ]
    ///Getter for the `onprogress` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onprogress)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn onprogress(this: &Document) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Document" , js_name = onprogress ) ]
    ///Setter for the `onprogress` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onprogress)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn set_onprogress(this: &Document, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "Document" , js_name = onratechange ) ]
    ///Getter for the `onratechange` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onratechange)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn onratechange(this: &Document) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Document" , js_name = onratechange ) ]
    ///Setter for the `onratechange` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onratechange)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn set_onratechange(this: &Document, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "Document" , js_name = onreset ) ]
    ///Getter for the `onreset` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onreset)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn onreset(this: &Document) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Document" , js_name = onreset ) ]
    ///Setter for the `onreset` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onreset)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn set_onreset(this: &Document, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "Document" , js_name = onresize ) ]
    ///Getter for the `onresize` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onresize)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn onresize(this: &Document) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Document" , js_name = onresize ) ]
    ///Setter for the `onresize` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onresize)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn set_onresize(this: &Document, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "Document" , js_name = onscroll ) ]
    ///Getter for the `onscroll` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onscroll)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn onscroll(this: &Document) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Document" , js_name = onscroll ) ]
    ///Setter for the `onscroll` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onscroll)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn set_onscroll(this: &Document, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "Document" , js_name = onseeked ) ]
    ///Getter for the `onseeked` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onseeked)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn onseeked(this: &Document) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Document" , js_name = onseeked ) ]
    ///Setter for the `onseeked` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onseeked)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn set_onseeked(this: &Document, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "Document" , js_name = onseeking ) ]
    ///Getter for the `onseeking` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onseeking)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn onseeking(this: &Document) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Document" , js_name = onseeking ) ]
    ///Setter for the `onseeking` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onseeking)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn set_onseeking(this: &Document, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "Document" , js_name = onselect ) ]
    ///Getter for the `onselect` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onselect)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn onselect(this: &Document) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Document" , js_name = onselect ) ]
    ///Setter for the `onselect` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onselect)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn set_onselect(this: &Document, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "Document" , js_name = onshow ) ]
    ///Getter for the `onshow` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onshow)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn onshow(this: &Document) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Document" , js_name = onshow ) ]
    ///Setter for the `onshow` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onshow)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn set_onshow(this: &Document, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "Document" , js_name = onstalled ) ]
    ///Getter for the `onstalled` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onstalled)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn onstalled(this: &Document) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Document" , js_name = onstalled ) ]
    ///Setter for the `onstalled` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onstalled)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn set_onstalled(this: &Document, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "Document" , js_name = onsubmit ) ]
    ///Getter for the `onsubmit` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onsubmit)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn onsubmit(this: &Document) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Document" , js_name = onsubmit ) ]
    ///Setter for the `onsubmit` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onsubmit)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn set_onsubmit(this: &Document, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "Document" , js_name = onsuspend ) ]
    ///Getter for the `onsuspend` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onsuspend)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn onsuspend(this: &Document) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Document" , js_name = onsuspend ) ]
    ///Setter for the `onsuspend` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onsuspend)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn set_onsuspend(this: &Document, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "Document" , js_name = ontimeupdate ) ]
    ///Getter for the `ontimeupdate` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/ontimeupdate)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn ontimeupdate(this: &Document) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Document" , js_name = ontimeupdate ) ]
    ///Setter for the `ontimeupdate` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/ontimeupdate)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn set_ontimeupdate(this: &Document, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "Document" , js_name = onvolumechange ) ]
    ///Getter for the `onvolumechange` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onvolumechange)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn onvolumechange(this: &Document) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Document" , js_name = onvolumechange ) ]
    ///Setter for the `onvolumechange` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onvolumechange)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn set_onvolumechange(this: &Document, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "Document" , js_name = onwaiting ) ]
    ///Getter for the `onwaiting` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onwaiting)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn onwaiting(this: &Document) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Document" , js_name = onwaiting ) ]
    ///Setter for the `onwaiting` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onwaiting)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn set_onwaiting(this: &Document, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "Document" , js_name = onselectstart ) ]
    ///Getter for the `onselectstart` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onselectstart)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn onselectstart(this: &Document) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Document" , js_name = onselectstart ) ]
    ///Setter for the `onselectstart` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onselectstart)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn set_onselectstart(this: &Document, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "Document" , js_name = ontoggle ) ]
    ///Getter for the `ontoggle` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/ontoggle)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn ontoggle(this: &Document) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Document" , js_name = ontoggle ) ]
    ///Setter for the `ontoggle` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/ontoggle)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn set_ontoggle(this: &Document, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "Document" , js_name = onpointercancel ) ]
    ///Getter for the `onpointercancel` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onpointercancel)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn onpointercancel(this: &Document) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Document" , js_name = onpointercancel ) ]
    ///Setter for the `onpointercancel` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onpointercancel)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn set_onpointercancel(this: &Document, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "Document" , js_name = onpointerdown ) ]
    ///Getter for the `onpointerdown` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onpointerdown)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn onpointerdown(this: &Document) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Document" , js_name = onpointerdown ) ]
    ///Setter for the `onpointerdown` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onpointerdown)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn set_onpointerdown(this: &Document, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "Document" , js_name = onpointerup ) ]
    ///Getter for the `onpointerup` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onpointerup)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn onpointerup(this: &Document) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Document" , js_name = onpointerup ) ]
    ///Setter for the `onpointerup` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onpointerup)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn set_onpointerup(this: &Document, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "Document" , js_name = onpointermove ) ]
    ///Getter for the `onpointermove` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onpointermove)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn onpointermove(this: &Document) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Document" , js_name = onpointermove ) ]
    ///Setter for the `onpointermove` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onpointermove)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn set_onpointermove(this: &Document, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "Document" , js_name = onpointerout ) ]
    ///Getter for the `onpointerout` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onpointerout)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn onpointerout(this: &Document) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Document" , js_name = onpointerout ) ]
    ///Setter for the `onpointerout` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onpointerout)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn set_onpointerout(this: &Document, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "Document" , js_name = onpointerover ) ]
    ///Getter for the `onpointerover` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onpointerover)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn onpointerover(this: &Document) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Document" , js_name = onpointerover ) ]
    ///Setter for the `onpointerover` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onpointerover)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn set_onpointerover(this: &Document, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "Document" , js_name = onpointerenter ) ]
    ///Getter for the `onpointerenter` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onpointerenter)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn onpointerenter(this: &Document) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Document" , js_name = onpointerenter ) ]
    ///Setter for the `onpointerenter` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onpointerenter)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn set_onpointerenter(this: &Document, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "Document" , js_name = onpointerleave ) ]
    ///Getter for the `onpointerleave` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onpointerleave)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn onpointerleave(this: &Document) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Document" , js_name = onpointerleave ) ]
    ///Setter for the `onpointerleave` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onpointerleave)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn set_onpointerleave(this: &Document, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "Document" , js_name = ongotpointercapture ) ]
    ///Getter for the `ongotpointercapture` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/ongotpointercapture)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn ongotpointercapture(this: &Document) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Document" , js_name = ongotpointercapture ) ]
    ///Setter for the `ongotpointercapture` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/ongotpointercapture)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn set_ongotpointercapture(this: &Document, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "Document" , js_name = onlostpointercapture ) ]
    ///Getter for the `onlostpointercapture` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onlostpointercapture)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn onlostpointercapture(this: &Document) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Document" , js_name = onlostpointercapture ) ]
    ///Setter for the `onlostpointercapture` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onlostpointercapture)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn set_onlostpointercapture(this: &Document, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "Document" , js_name = onanimationcancel ) ]
    ///Getter for the `onanimationcancel` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onanimationcancel)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn onanimationcancel(this: &Document) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Document" , js_name = onanimationcancel ) ]
    ///Setter for the `onanimationcancel` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onanimationcancel)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn set_onanimationcancel(this: &Document, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "Document" , js_name = onanimationend ) ]
    ///Getter for the `onanimationend` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onanimationend)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn onanimationend(this: &Document) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Document" , js_name = onanimationend ) ]
    ///Setter for the `onanimationend` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onanimationend)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn set_onanimationend(this: &Document, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "Document" , js_name = onanimationiteration ) ]
    ///Getter for the `onanimationiteration` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onanimationiteration)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn onanimationiteration(this: &Document) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Document" , js_name = onanimationiteration ) ]
    ///Setter for the `onanimationiteration` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onanimationiteration)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn set_onanimationiteration(this: &Document, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "Document" , js_name = onanimationstart ) ]
    ///Getter for the `onanimationstart` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onanimationstart)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn onanimationstart(this: &Document) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Document" , js_name = onanimationstart ) ]
    ///Setter for the `onanimationstart` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onanimationstart)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn set_onanimationstart(this: &Document, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "Document" , js_name = ontransitioncancel ) ]
    ///Getter for the `ontransitioncancel` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/ontransitioncancel)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn ontransitioncancel(this: &Document) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Document" , js_name = ontransitioncancel ) ]
    ///Setter for the `ontransitioncancel` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/ontransitioncancel)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn set_ontransitioncancel(this: &Document, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "Document" , js_name = ontransitionend ) ]
    ///Getter for the `ontransitionend` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/ontransitionend)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn ontransitionend(this: &Document) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Document" , js_name = ontransitionend ) ]
    ///Setter for the `ontransitionend` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/ontransitionend)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn set_ontransitionend(this: &Document, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "Document" , js_name = ontransitionrun ) ]
    ///Getter for the `ontransitionrun` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/ontransitionrun)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn ontransitionrun(this: &Document) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Document" , js_name = ontransitionrun ) ]
    ///Setter for the `ontransitionrun` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/ontransitionrun)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn set_ontransitionrun(this: &Document, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "Document" , js_name = ontransitionstart ) ]
    ///Getter for the `ontransitionstart` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/ontransitionstart)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn ontransitionstart(this: &Document) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Document" , js_name = ontransitionstart ) ]
    ///Setter for the `ontransitionstart` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/ontransitionstart)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn set_ontransitionstart(this: &Document, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "Document" , js_name = onwebkitanimationend ) ]
    ///Getter for the `onwebkitanimationend` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onwebkitanimationend)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn onwebkitanimationend(this: &Document) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Document" , js_name = onwebkitanimationend ) ]
    ///Setter for the `onwebkitanimationend` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onwebkitanimationend)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn set_onwebkitanimationend(this: &Document, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "Document" , js_name = onwebkitanimationiteration ) ]
    ///Getter for the `onwebkitanimationiteration` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onwebkitanimationiteration)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn onwebkitanimationiteration(this: &Document) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Document" , js_name = onwebkitanimationiteration ) ]
    ///Setter for the `onwebkitanimationiteration` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onwebkitanimationiteration)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn set_onwebkitanimationiteration(this: &Document, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "Document" , js_name = onwebkitanimationstart ) ]
    ///Getter for the `onwebkitanimationstart` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onwebkitanimationstart)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn onwebkitanimationstart(this: &Document) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Document" , js_name = onwebkitanimationstart ) ]
    ///Setter for the `onwebkitanimationstart` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onwebkitanimationstart)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn set_onwebkitanimationstart(this: &Document, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "Document" , js_name = onwebkittransitionend ) ]
    ///Getter for the `onwebkittransitionend` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onwebkittransitionend)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn onwebkittransitionend(this: &Document) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Document" , js_name = onwebkittransitionend ) ]
    ///Setter for the `onwebkittransitionend` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onwebkittransitionend)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn set_onwebkittransitionend(this: &Document, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "Document" , js_name = onerror ) ]
    ///Getter for the `onerror` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onerror)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn onerror(this: &Document) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Document" , js_name = onerror ) ]
    ///Setter for the `onerror` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onerror)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn set_onerror(this: &Document, value: Option<&::js_sys::Function>);

    #[cfg(feature = "HtmlCollection")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "Document" , js_name = children ) ]
    ///Getter for the `children` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/children)
    ///
    ///*This API requires the following crate features to be activated: `Document`, `HtmlCollection`*
    pub fn children(this: &Document) -> HtmlCollection;

    #[cfg(feature = "Element")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "Document" , js_name = firstElementChild ) ]
    ///Getter for the `firstElementChild` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/firstElementChild)
    ///
    ///*This API requires the following crate features to be activated: `Document`, `Element`*
    pub fn first_element_child(this: &Document) -> Option<Element>;

    #[cfg(feature = "Element")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "Document" , js_name = lastElementChild ) ]
    ///Getter for the `lastElementChild` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/lastElementChild)
    ///
    ///*This API requires the following crate features to be activated: `Document`, `Element`*
    pub fn last_element_child(this: &Document) -> Option<Element>;

    # [ wasm_bindgen ( structural , method , getter , js_class = "Document" , js_name = childElementCount ) ]
    ///Getter for the `childElementCount` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/childElementCount)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn child_element_count(this: &Document) -> u32;

    # [ wasm_bindgen ( structural , method , getter , js_class = "Document" , js_name = ontouchstart ) ]
    ///Getter for the `ontouchstart` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/ontouchstart)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn ontouchstart(this: &Document) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Document" , js_name = ontouchstart ) ]
    ///Setter for the `ontouchstart` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/ontouchstart)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn set_ontouchstart(this: &Document, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "Document" , js_name = ontouchend ) ]
    ///Getter for the `ontouchend` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/ontouchend)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn ontouchend(this: &Document) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Document" , js_name = ontouchend ) ]
    ///Setter for the `ontouchend` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/ontouchend)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn set_ontouchend(this: &Document, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "Document" , js_name = ontouchmove ) ]
    ///Getter for the `ontouchmove` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/ontouchmove)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn ontouchmove(this: &Document) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Document" , js_name = ontouchmove ) ]
    ///Setter for the `ontouchmove` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/ontouchmove)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn set_ontouchmove(this: &Document, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "Document" , js_name = ontouchcancel ) ]
    ///Getter for the `ontouchcancel` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/ontouchcancel)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn ontouchcancel(this: &Document) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Document" , js_name = ontouchcancel ) ]
    ///Setter for the `ontouchcancel` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/ontouchcancel)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn set_ontouchcancel(this: &Document, value: Option<&::js_sys::Function>);

    #[wasm_bindgen(catch, constructor, js_class = "Document")]
    ///The `new Document(..)` constructor, creating a new instance of `Document`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/Document)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn new() -> Result<Document, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Document" , js_name = adoptNode ) ]
    ///The `adoptNode()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/adoptNode)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn adopt_node(this: &Document, node: &Node) -> Result<Node, JsValue>;

    #[cfg(feature = "CaretPosition")]
    # [ wasm_bindgen ( method , structural , js_class = "Document" , js_name = caretPositionFromPoint ) ]
    ///The `caretPositionFromPoint()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/caretPositionFromPoint)
    ///
    ///*This API requires the following crate features to be activated: `CaretPosition`, `Document`*
    pub fn caret_position_from_point(this: &Document, x: f32, y: f32) -> Option<CaretPosition>;

    #[cfg(feature = "Attr")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "Document" , js_name = createAttribute ) ]
    ///The `createAttribute()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/createAttribute)
    ///
    ///*This API requires the following crate features to be activated: `Attr`, `Document`*
    pub fn create_attribute(this: &Document, name: &str) -> Result<Attr, JsValue>;

    #[cfg(feature = "Attr")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "Document" , js_name = createAttributeNS ) ]
    ///The `createAttributeNS()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/createAttributeNS)
    ///
    ///*This API requires the following crate features to be activated: `Attr`, `Document`*
    pub fn create_attribute_ns(
        this: &Document,
        namespace: Option<&str>,
        name: &str,
    ) -> Result<Attr, JsValue>;

    #[cfg(feature = "CdataSection")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "Document" , js_name = createCDATASection ) ]
    ///The `createCDATASection()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/createCDATASection)
    ///
    ///*This API requires the following crate features to be activated: `CdataSection`, `Document`*
    pub fn create_cdata_section(this: &Document, data: &str) -> Result<CdataSection, JsValue>;

    #[cfg(feature = "Comment")]
    # [ wasm_bindgen ( method , structural , js_class = "Document" , js_name = createComment ) ]
    ///The `createComment()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/createComment)
    ///
    ///*This API requires the following crate features to be activated: `Comment`, `Document`*
    pub fn create_comment(this: &Document, data: &str) -> Comment;

    #[cfg(feature = "DocumentFragment")]
    # [ wasm_bindgen ( method , structural , js_class = "Document" , js_name = createDocumentFragment ) ]
    ///The `createDocumentFragment()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/createDocumentFragment)
    ///
    ///*This API requires the following crate features to be activated: `Document`, `DocumentFragment`*
    pub fn create_document_fragment(this: &Document) -> DocumentFragment;

    #[cfg(feature = "Element")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "Document" , js_name = createElement ) ]
    ///The `createElement()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/createElement)
    ///
    ///*This API requires the following crate features to be activated: `Document`, `Element`*
    pub fn create_element(this: &Document, local_name: &str) -> Result<Element, JsValue>;

    #[cfg(all(feature = "Element", feature = "ElementCreationOptions",))]
    # [ wasm_bindgen ( catch , method , structural , js_class = "Document" , js_name = createElement ) ]
    ///The `createElement()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/createElement)
    ///
    ///*This API requires the following crate features to be activated: `Document`, `Element`, `ElementCreationOptions`*
    pub fn create_element_with_element_creation_options(
        this: &Document,
        local_name: &str,
        options: &ElementCreationOptions,
    ) -> Result<Element, JsValue>;

    #[cfg(feature = "Element")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "Document" , js_name = createElement ) ]
    ///The `createElement()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/createElement)
    ///
    ///*This API requires the following crate features to be activated: `Document`, `Element`*
    pub fn create_element_with_str(
        this: &Document,
        local_name: &str,
        options: &str,
    ) -> Result<Element, JsValue>;

    #[cfg(feature = "Element")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "Document" , js_name = createElementNS ) ]
    ///The `createElementNS()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/createElementNS)
    ///
    ///*This API requires the following crate features to be activated: `Document`, `Element`*
    pub fn create_element_ns(
        this: &Document,
        namespace: Option<&str>,
        qualified_name: &str,
    ) -> Result<Element, JsValue>;

    #[cfg(all(feature = "Element", feature = "ElementCreationOptions",))]
    # [ wasm_bindgen ( catch , method , structural , js_class = "Document" , js_name = createElementNS ) ]
    ///The `createElementNS()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/createElementNS)
    ///
    ///*This API requires the following crate features to be activated: `Document`, `Element`, `ElementCreationOptions`*
    pub fn create_element_ns_with_element_creation_options(
        this: &Document,
        namespace: Option<&str>,
        qualified_name: &str,
        options: &ElementCreationOptions,
    ) -> Result<Element, JsValue>;

    #[cfg(feature = "Element")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "Document" , js_name = createElementNS ) ]
    ///The `createElementNS()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/createElementNS)
    ///
    ///*This API requires the following crate features to be activated: `Document`, `Element`*
    pub fn create_element_ns_with_str(
        this: &Document,
        namespace: Option<&str>,
        qualified_name: &str,
        options: &str,
    ) -> Result<Element, JsValue>;

    #[cfg(feature = "Event")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "Document" , js_name = createEvent ) ]
    ///The `createEvent()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/createEvent)
    ///
    ///*This API requires the following crate features to be activated: `Document`, `Event`*
    pub fn create_event(this: &Document, interface: &str) -> Result<Event, JsValue>;

    #[cfg(feature = "NodeIterator")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "Document" , js_name = createNodeIterator ) ]
    ///The `createNodeIterator()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/createNodeIterator)
    ///
    ///*This API requires the following crate features to be activated: `Document`, `NodeIterator`*
    pub fn create_node_iterator(this: &Document, root: &Node) -> Result<NodeIterator, JsValue>;

    #[cfg(feature = "NodeIterator")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "Document" , js_name = createNodeIterator ) ]
    ///The `createNodeIterator()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/createNodeIterator)
    ///
    ///*This API requires the following crate features to be activated: `Document`, `NodeIterator`*
    pub fn create_node_iterator_with_what_to_show(
        this: &Document,
        root: &Node,
        what_to_show: u32,
    ) -> Result<NodeIterator, JsValue>;

    #[cfg(all(feature = "NodeFilter", feature = "NodeIterator",))]
    # [ wasm_bindgen ( catch , method , structural , js_class = "Document" , js_name = createNodeIterator ) ]
    ///The `createNodeIterator()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/createNodeIterator)
    ///
    ///*This API requires the following crate features to be activated: `Document`, `NodeFilter`, `NodeIterator`*
    pub fn create_node_iterator_with_what_to_show_and_filter(
        this: &Document,
        root: &Node,
        what_to_show: u32,
        filter: Option<&NodeFilter>,
    ) -> Result<NodeIterator, JsValue>;

    #[cfg(feature = "ProcessingInstruction")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "Document" , js_name = createProcessingInstruction ) ]
    ///The `createProcessingInstruction()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/createProcessingInstruction)
    ///
    ///*This API requires the following crate features to be activated: `Document`, `ProcessingInstruction`*
    pub fn create_processing_instruction(
        this: &Document,
        target: &str,
        data: &str,
    ) -> Result<ProcessingInstruction, JsValue>;

    #[cfg(feature = "Range")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "Document" , js_name = createRange ) ]
    ///The `createRange()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/createRange)
    ///
    ///*This API requires the following crate features to be activated: `Document`, `Range`*
    pub fn create_range(this: &Document) -> Result<Range, JsValue>;

    #[cfg(feature = "Text")]
    # [ wasm_bindgen ( method , structural , js_class = "Document" , js_name = createTextNode ) ]
    ///The `createTextNode()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/createTextNode)
    ///
    ///*This API requires the following crate features to be activated: `Document`, `Text`*
    pub fn create_text_node(this: &Document, data: &str) -> Text;

    #[cfg(feature = "TreeWalker")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "Document" , js_name = createTreeWalker ) ]
    ///The `createTreeWalker()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/createTreeWalker)
    ///
    ///*This API requires the following crate features to be activated: `Document`, `TreeWalker`*
    pub fn create_tree_walker(this: &Document, root: &Node) -> Result<TreeWalker, JsValue>;

    #[cfg(feature = "TreeWalker")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "Document" , js_name = createTreeWalker ) ]
    ///The `createTreeWalker()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/createTreeWalker)
    ///
    ///*This API requires the following crate features to be activated: `Document`, `TreeWalker`*
    pub fn create_tree_walker_with_what_to_show(
        this: &Document,
        root: &Node,
        what_to_show: u32,
    ) -> Result<TreeWalker, JsValue>;

    #[cfg(all(feature = "NodeFilter", feature = "TreeWalker",))]
    # [ wasm_bindgen ( catch , method , structural , js_class = "Document" , js_name = createTreeWalker ) ]
    ///The `createTreeWalker()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/createTreeWalker)
    ///
    ///*This API requires the following crate features to be activated: `Document`, `NodeFilter`, `TreeWalker`*
    pub fn create_tree_walker_with_what_to_show_and_filter(
        this: &Document,
        root: &Node,
        what_to_show: u32,
        filter: Option<&NodeFilter>,
    ) -> Result<TreeWalker, JsValue>;

    # [ wasm_bindgen ( method , structural , js_class = "Document" , js_name = enableStyleSheetsForSet ) ]
    ///The `enableStyleSheetsForSet()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/enableStyleSheetsForSet)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn enable_style_sheets_for_set(this: &Document, name: Option<&str>);

    # [ wasm_bindgen ( method , structural , js_class = "Document" , js_name = exitFullscreen ) ]
    ///The `exitFullscreen()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/exitFullscreen)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn exit_fullscreen(this: &Document);

    # [ wasm_bindgen ( method , structural , js_class = "Document" , js_name = exitPointerLock ) ]
    ///The `exitPointerLock()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/exitPointerLock)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn exit_pointer_lock(this: &Document);

    # [ wasm_bindgen ( method , structural , js_class = "Document" , js_name = getAnimations ) ]
    ///The `getAnimations()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/getAnimations)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn get_animations(this: &Document) -> ::js_sys::Array;

    #[cfg(feature = "Element")]
    # [ wasm_bindgen ( method , structural , js_class = "Document" , js_name = getElementById ) ]
    ///The `getElementById()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/getElementById)
    ///
    ///*This API requires the following crate features to be activated: `Document`, `Element`*
    pub fn get_element_by_id(this: &Document, element_id: &str) -> Option<Element>;

    #[cfg(feature = "HtmlCollection")]
    # [ wasm_bindgen ( method , structural , js_class = "Document" , js_name = getElementsByClassName ) ]
    ///The `getElementsByClassName()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/getElementsByClassName)
    ///
    ///*This API requires the following crate features to be activated: `Document`, `HtmlCollection`*
    pub fn get_elements_by_class_name(this: &Document, class_names: &str) -> HtmlCollection;

    #[cfg(feature = "NodeList")]
    # [ wasm_bindgen ( method , structural , js_class = "Document" , js_name = getElementsByName ) ]
    ///The `getElementsByName()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/getElementsByName)
    ///
    ///*This API requires the following crate features to be activated: `Document`, `NodeList`*
    pub fn get_elements_by_name(this: &Document, element_name: &str) -> NodeList;

    #[cfg(feature = "HtmlCollection")]
    # [ wasm_bindgen ( method , structural , js_class = "Document" , js_name = getElementsByTagName ) ]
    ///The `getElementsByTagName()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/getElementsByTagName)
    ///
    ///*This API requires the following crate features to be activated: `Document`, `HtmlCollection`*
    pub fn get_elements_by_tag_name(this: &Document, local_name: &str) -> HtmlCollection;

    #[cfg(feature = "HtmlCollection")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "Document" , js_name = getElementsByTagNameNS ) ]
    ///The `getElementsByTagNameNS()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/getElementsByTagNameNS)
    ///
    ///*This API requires the following crate features to be activated: `Document`, `HtmlCollection`*
    pub fn get_elements_by_tag_name_ns(
        this: &Document,
        namespace: Option<&str>,
        local_name: &str,
    ) -> Result<HtmlCollection, JsValue>;

    #[cfg(feature = "Selection")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "Document" , js_name = getSelection ) ]
    ///The `getSelection()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/getSelection)
    ///
    ///*This API requires the following crate features to be activated: `Document`, `Selection`*
    pub fn get_selection(this: &Document) -> Result<Option<Selection>, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Document" , js_name = hasFocus ) ]
    ///The `hasFocus()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/hasFocus)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn has_focus(this: &Document) -> Result<bool, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Document" , js_name = importNode ) ]
    ///The `importNode()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/importNode)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn import_node(this: &Document, node: &Node) -> Result<Node, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Document" , js_name = importNode ) ]
    ///The `importNode()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/importNode)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn import_node_with_deep(this: &Document, node: &Node, deep: bool)
        -> Result<Node, JsValue>;

    #[cfg(feature = "Element")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "Document" , js_name = querySelector ) ]
    ///The `querySelector()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/querySelector)
    ///
    ///*This API requires the following crate features to be activated: `Document`, `Element`*
    pub fn query_selector(this: &Document, selectors: &str) -> Result<Option<Element>, JsValue>;

    #[cfg(feature = "NodeList")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "Document" , js_name = querySelectorAll ) ]
    ///The `querySelectorAll()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/querySelectorAll)
    ///
    ///*This API requires the following crate features to be activated: `Document`, `NodeList`*
    pub fn query_selector_all(this: &Document, selectors: &str) -> Result<NodeList, JsValue>;

    # [ wasm_bindgen ( method , structural , js_class = "Document" , js_name = releaseCapture ) ]
    ///The `releaseCapture()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/releaseCapture)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn release_capture(this: &Document);

    #[cfg(feature = "Element")]
    # [ wasm_bindgen ( method , structural , js_class = "Document" , js_name = elementFromPoint ) ]
    ///The `elementFromPoint()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/elementFromPoint)
    ///
    ///*This API requires the following crate features to be activated: `Document`, `Element`*
    pub fn element_from_point(this: &Document, x: f32, y: f32) -> Option<Element>;

    # [ wasm_bindgen ( method , structural , js_class = "Document" , js_name = elementsFromPoint ) ]
    ///The `elementsFromPoint()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/elementsFromPoint)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn elements_from_point(this: &Document, x: f32, y: f32) -> ::js_sys::Array;

    #[cfg(all(feature = "DomPoint", feature = "DomPointInit", feature = "Text",))]
    # [ wasm_bindgen ( catch , method , structural , js_class = "Document" , js_name = convertPointFromNode ) ]
    ///The `convertPointFromNode()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/convertPointFromNode)
    ///
    ///*This API requires the following crate features to be activated: `Document`, `DomPoint`, `DomPointInit`, `Text`*
    pub fn convert_point_from_node_with_text(
        this: &Document,
        point: &DomPointInit,
        from: &Text,
    ) -> Result<DomPoint, JsValue>;

    #[cfg(all(feature = "DomPoint", feature = "DomPointInit", feature = "Element",))]
    # [ wasm_bindgen ( catch , method , structural , js_class = "Document" , js_name = convertPointFromNode ) ]
    ///The `convertPointFromNode()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/convertPointFromNode)
    ///
    ///*This API requires the following crate features to be activated: `Document`, `DomPoint`, `DomPointInit`, `Element`*
    pub fn convert_point_from_node_with_element(
        this: &Document,
        point: &DomPointInit,
        from: &Element,
    ) -> Result<DomPoint, JsValue>;

    #[cfg(all(feature = "DomPoint", feature = "DomPointInit",))]
    # [ wasm_bindgen ( catch , method , structural , js_class = "Document" , js_name = convertPointFromNode ) ]
    ///The `convertPointFromNode()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/convertPointFromNode)
    ///
    ///*This API requires the following crate features to be activated: `Document`, `DomPoint`, `DomPointInit`*
    pub fn convert_point_from_node_with_document(
        this: &Document,
        point: &DomPointInit,
        from: &Document,
    ) -> Result<DomPoint, JsValue>;

    #[cfg(all(
        feature = "ConvertCoordinateOptions",
        feature = "DomPoint",
        feature = "DomPointInit",
        feature = "Text",
    ))]
    # [ wasm_bindgen ( catch , method , structural , js_class = "Document" , js_name = convertPointFromNode ) ]
    ///The `convertPointFromNode()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/convertPointFromNode)
    ///
    ///*This API requires the following crate features to be activated: `ConvertCoordinateOptions`, `Document`, `DomPoint`, `DomPointInit`, `Text`*
    pub fn convert_point_from_node_with_text_and_options(
        this: &Document,
        point: &DomPointInit,
        from: &Text,
        options: &ConvertCoordinateOptions,
    ) -> Result<DomPoint, JsValue>;

    #[cfg(all(
        feature = "ConvertCoordinateOptions",
        feature = "DomPoint",
        feature = "DomPointInit",
        feature = "Element",
    ))]
    # [ wasm_bindgen ( catch , method , structural , js_class = "Document" , js_name = convertPointFromNode ) ]
    ///The `convertPointFromNode()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/convertPointFromNode)
    ///
    ///*This API requires the following crate features to be activated: `ConvertCoordinateOptions`, `Document`, `DomPoint`, `DomPointInit`, `Element`*
    pub fn convert_point_from_node_with_element_and_options(
        this: &Document,
        point: &DomPointInit,
        from: &Element,
        options: &ConvertCoordinateOptions,
    ) -> Result<DomPoint, JsValue>;

    #[cfg(all(
        feature = "ConvertCoordinateOptions",
        feature = "DomPoint",
        feature = "DomPointInit",
    ))]
    # [ wasm_bindgen ( catch , method , structural , js_class = "Document" , js_name = convertPointFromNode ) ]
    ///The `convertPointFromNode()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/convertPointFromNode)
    ///
    ///*This API requires the following crate features to be activated: `ConvertCoordinateOptions`, `Document`, `DomPoint`, `DomPointInit`*
    pub fn convert_point_from_node_with_document_and_options(
        this: &Document,
        point: &DomPointInit,
        from: &Document,
        options: &ConvertCoordinateOptions,
    ) -> Result<DomPoint, JsValue>;

    #[cfg(all(feature = "DomQuad", feature = "Text",))]
    # [ wasm_bindgen ( catch , method , structural , js_class = "Document" , js_name = convertQuadFromNode ) ]
    ///The `convertQuadFromNode()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/convertQuadFromNode)
    ///
    ///*This API requires the following crate features to be activated: `Document`, `DomQuad`, `Text`*
    pub fn convert_quad_from_node_with_text(
        this: &Document,
        quad: &DomQuad,
        from: &Text,
    ) -> Result<DomQuad, JsValue>;

    #[cfg(all(feature = "DomQuad", feature = "Element",))]
    # [ wasm_bindgen ( catch , method , structural , js_class = "Document" , js_name = convertQuadFromNode ) ]
    ///The `convertQuadFromNode()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/convertQuadFromNode)
    ///
    ///*This API requires the following crate features to be activated: `Document`, `DomQuad`, `Element`*
    pub fn convert_quad_from_node_with_element(
        this: &Document,
        quad: &DomQuad,
        from: &Element,
    ) -> Result<DomQuad, JsValue>;

    #[cfg(feature = "DomQuad")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "Document" , js_name = convertQuadFromNode ) ]
    ///The `convertQuadFromNode()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/convertQuadFromNode)
    ///
    ///*This API requires the following crate features to be activated: `Document`, `DomQuad`*
    pub fn convert_quad_from_node_with_document(
        this: &Document,
        quad: &DomQuad,
        from: &Document,
    ) -> Result<DomQuad, JsValue>;

    #[cfg(all(
        feature = "ConvertCoordinateOptions",
        feature = "DomQuad",
        feature = "Text",
    ))]
    # [ wasm_bindgen ( catch , method , structural , js_class = "Document" , js_name = convertQuadFromNode ) ]
    ///The `convertQuadFromNode()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/convertQuadFromNode)
    ///
    ///*This API requires the following crate features to be activated: `ConvertCoordinateOptions`, `Document`, `DomQuad`, `Text`*
    pub fn convert_quad_from_node_with_text_and_options(
        this: &Document,
        quad: &DomQuad,
        from: &Text,
        options: &ConvertCoordinateOptions,
    ) -> Result<DomQuad, JsValue>;

    #[cfg(all(
        feature = "ConvertCoordinateOptions",
        feature = "DomQuad",
        feature = "Element",
    ))]
    # [ wasm_bindgen ( catch , method , structural , js_class = "Document" , js_name = convertQuadFromNode ) ]
    ///The `convertQuadFromNode()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/convertQuadFromNode)
    ///
    ///*This API requires the following crate features to be activated: `ConvertCoordinateOptions`, `Document`, `DomQuad`, `Element`*
    pub fn convert_quad_from_node_with_element_and_options(
        this: &Document,
        quad: &DomQuad,
        from: &Element,
        options: &ConvertCoordinateOptions,
    ) -> Result<DomQuad, JsValue>;

    #[cfg(all(feature = "ConvertCoordinateOptions", feature = "DomQuad",))]
    # [ wasm_bindgen ( catch , method , structural , js_class = "Document" , js_name = convertQuadFromNode ) ]
    ///The `convertQuadFromNode()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/convertQuadFromNode)
    ///
    ///*This API requires the following crate features to be activated: `ConvertCoordinateOptions`, `Document`, `DomQuad`*
    pub fn convert_quad_from_node_with_document_and_options(
        this: &Document,
        quad: &DomQuad,
        from: &Document,
        options: &ConvertCoordinateOptions,
    ) -> Result<DomQuad, JsValue>;

    #[cfg(all(feature = "DomQuad", feature = "DomRectReadOnly", feature = "Text",))]
    # [ wasm_bindgen ( catch , method , structural , js_class = "Document" , js_name = convertRectFromNode ) ]
    ///The `convertRectFromNode()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/convertRectFromNode)
    ///
    ///*This API requires the following crate features to be activated: `Document`, `DomQuad`, `DomRectReadOnly`, `Text`*
    pub fn convert_rect_from_node_with_text(
        this: &Document,
        rect: &DomRectReadOnly,
        from: &Text,
    ) -> Result<DomQuad, JsValue>;

    #[cfg(all(feature = "DomQuad", feature = "DomRectReadOnly", feature = "Element",))]
    # [ wasm_bindgen ( catch , method , structural , js_class = "Document" , js_name = convertRectFromNode ) ]
    ///The `convertRectFromNode()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/convertRectFromNode)
    ///
    ///*This API requires the following crate features to be activated: `Document`, `DomQuad`, `DomRectReadOnly`, `Element`*
    pub fn convert_rect_from_node_with_element(
        this: &Document,
        rect: &DomRectReadOnly,
        from: &Element,
    ) -> Result<DomQuad, JsValue>;

    #[cfg(all(feature = "DomQuad", feature = "DomRectReadOnly",))]
    # [ wasm_bindgen ( catch , method , structural , js_class = "Document" , js_name = convertRectFromNode ) ]
    ///The `convertRectFromNode()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/convertRectFromNode)
    ///
    ///*This API requires the following crate features to be activated: `Document`, `DomQuad`, `DomRectReadOnly`*
    pub fn convert_rect_from_node_with_document(
        this: &Document,
        rect: &DomRectReadOnly,
        from: &Document,
    ) -> Result<DomQuad, JsValue>;

    #[cfg(all(
        feature = "ConvertCoordinateOptions",
        feature = "DomQuad",
        feature = "DomRectReadOnly",
        feature = "Text",
    ))]
    # [ wasm_bindgen ( catch , method , structural , js_class = "Document" , js_name = convertRectFromNode ) ]
    ///The `convertRectFromNode()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/convertRectFromNode)
    ///
    ///*This API requires the following crate features to be activated: `ConvertCoordinateOptions`, `Document`, `DomQuad`, `DomRectReadOnly`, `Text`*
    pub fn convert_rect_from_node_with_text_and_options(
        this: &Document,
        rect: &DomRectReadOnly,
        from: &Text,
        options: &ConvertCoordinateOptions,
    ) -> Result<DomQuad, JsValue>;

    #[cfg(all(
        feature = "ConvertCoordinateOptions",
        feature = "DomQuad",
        feature = "DomRectReadOnly",
        feature = "Element",
    ))]
    # [ wasm_bindgen ( catch , method , structural , js_class = "Document" , js_name = convertRectFromNode ) ]
    ///The `convertRectFromNode()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/convertRectFromNode)
    ///
    ///*This API requires the following crate features to be activated: `ConvertCoordinateOptions`, `Document`, `DomQuad`, `DomRectReadOnly`, `Element`*
    pub fn convert_rect_from_node_with_element_and_options(
        this: &Document,
        rect: &DomRectReadOnly,
        from: &Element,
        options: &ConvertCoordinateOptions,
    ) -> Result<DomQuad, JsValue>;

    #[cfg(all(
        feature = "ConvertCoordinateOptions",
        feature = "DomQuad",
        feature = "DomRectReadOnly",
    ))]
    # [ wasm_bindgen ( catch , method , structural , js_class = "Document" , js_name = convertRectFromNode ) ]
    ///The `convertRectFromNode()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/convertRectFromNode)
    ///
    ///*This API requires the following crate features to be activated: `ConvertCoordinateOptions`, `Document`, `DomQuad`, `DomRectReadOnly`*
    pub fn convert_rect_from_node_with_document_and_options(
        this: &Document,
        rect: &DomRectReadOnly,
        from: &Document,
        options: &ConvertCoordinateOptions,
    ) -> Result<DomQuad, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Document" , js_name = getBoxQuads ) ]
    ///The `getBoxQuads()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/getBoxQuads)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn get_box_quads(this: &Document) -> Result<::js_sys::Array, JsValue>;

    #[cfg(feature = "BoxQuadOptions")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "Document" , js_name = getBoxQuads ) ]
    ///The `getBoxQuads()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/getBoxQuads)
    ///
    ///*This API requires the following crate features to be activated: `BoxQuadOptions`, `Document`*
    pub fn get_box_quads_with_options(
        this: &Document,
        options: &BoxQuadOptions,
    ) -> Result<::js_sys::Array, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , variadic , js_class = "Document" , js_name = append ) ]
    ///The `append()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/append)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn append_with_node(this: &Document, nodes: &::js_sys::Array) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Document" , js_name = append ) ]
    ///The `append()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/append)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn append_with_node_0(this: &Document) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Document" , js_name = append ) ]
    ///The `append()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/append)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn append_with_node_1(this: &Document, nodes_1: &Node) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Document" , js_name = append ) ]
    ///The `append()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/append)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn append_with_node_2(
        this: &Document,
        nodes_1: &Node,
        nodes_2: &Node,
    ) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Document" , js_name = append ) ]
    ///The `append()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/append)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn append_with_node_3(
        this: &Document,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
    ) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Document" , js_name = append ) ]
    ///The `append()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/append)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn append_with_node_4(
        this: &Document,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
        nodes_4: &Node,
    ) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Document" , js_name = append ) ]
    ///The `append()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/append)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn append_with_node_5(
        this: &Document,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
        nodes_4: &Node,
        nodes_5: &Node,
    ) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Document" , js_name = append ) ]
    ///The `append()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/append)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn append_with_node_6(
        this: &Document,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
        nodes_4: &Node,
        nodes_5: &Node,
        nodes_6: &Node,
    ) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Document" , js_name = append ) ]
    ///The `append()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/append)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn append_with_node_7(
        this: &Document,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
        nodes_4: &Node,
        nodes_5: &Node,
        nodes_6: &Node,
        nodes_7: &Node,
    ) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , variadic , js_class = "Document" , js_name = append ) ]
    ///The `append()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/append)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn append_with_str(this: &Document, nodes: &::js_sys::Array) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Document" , js_name = append ) ]
    ///The `append()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/append)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn append_with_str_0(this: &Document) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Document" , js_name = append ) ]
    ///The `append()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/append)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn append_with_str_1(this: &Document, nodes_1: &str) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Document" , js_name = append ) ]
    ///The `append()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/append)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn append_with_str_2(this: &Document, nodes_1: &str, nodes_2: &str) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Document" , js_name = append ) ]
    ///The `append()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/append)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn append_with_str_3(
        this: &Document,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
    ) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Document" , js_name = append ) ]
    ///The `append()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/append)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn append_with_str_4(
        this: &Document,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
        nodes_4: &str,
    ) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Document" , js_name = append ) ]
    ///The `append()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/append)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn append_with_str_5(
        this: &Document,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
        nodes_4: &str,
        nodes_5: &str,
    ) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Document" , js_name = append ) ]
    ///The `append()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/append)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn append_with_str_6(
        this: &Document,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
        nodes_4: &str,
        nodes_5: &str,
        nodes_6: &str,
    ) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Document" , js_name = append ) ]
    ///The `append()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/append)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn append_with_str_7(
        this: &Document,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
        nodes_4: &str,
        nodes_5: &str,
        nodes_6: &str,
        nodes_7: &str,
    ) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , variadic , js_class = "Document" , js_name = prepend ) ]
    ///The `prepend()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/prepend)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn prepend_with_node(this: &Document, nodes: &::js_sys::Array) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Document" , js_name = prepend ) ]
    ///The `prepend()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/prepend)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn prepend_with_node_0(this: &Document) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Document" , js_name = prepend ) ]
    ///The `prepend()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/prepend)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn prepend_with_node_1(this: &Document, nodes_1: &Node) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Document" , js_name = prepend ) ]
    ///The `prepend()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/prepend)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn prepend_with_node_2(
        this: &Document,
        nodes_1: &Node,
        nodes_2: &Node,
    ) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Document" , js_name = prepend ) ]
    ///The `prepend()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/prepend)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn prepend_with_node_3(
        this: &Document,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
    ) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Document" , js_name = prepend ) ]
    ///The `prepend()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/prepend)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn prepend_with_node_4(
        this: &Document,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
        nodes_4: &Node,
    ) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Document" , js_name = prepend ) ]
    ///The `prepend()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/prepend)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn prepend_with_node_5(
        this: &Document,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
        nodes_4: &Node,
        nodes_5: &Node,
    ) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Document" , js_name = prepend ) ]
    ///The `prepend()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/prepend)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn prepend_with_node_6(
        this: &Document,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
        nodes_4: &Node,
        nodes_5: &Node,
        nodes_6: &Node,
    ) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Document" , js_name = prepend ) ]
    ///The `prepend()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/prepend)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn prepend_with_node_7(
        this: &Document,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
        nodes_4: &Node,
        nodes_5: &Node,
        nodes_6: &Node,
        nodes_7: &Node,
    ) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , variadic , js_class = "Document" , js_name = prepend ) ]
    ///The `prepend()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/prepend)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn prepend_with_str(this: &Document, nodes: &::js_sys::Array) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Document" , js_name = prepend ) ]
    ///The `prepend()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/prepend)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn prepend_with_str_0(this: &Document) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Document" , js_name = prepend ) ]
    ///The `prepend()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/prepend)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn prepend_with_str_1(this: &Document, nodes_1: &str) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Document" , js_name = prepend ) ]
    ///The `prepend()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/prepend)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn prepend_with_str_2(this: &Document, nodes_1: &str, nodes_2: &str)
        -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Document" , js_name = prepend ) ]
    ///The `prepend()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/prepend)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn prepend_with_str_3(
        this: &Document,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
    ) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Document" , js_name = prepend ) ]
    ///The `prepend()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/prepend)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn prepend_with_str_4(
        this: &Document,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
        nodes_4: &str,
    ) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Document" , js_name = prepend ) ]
    ///The `prepend()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/prepend)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn prepend_with_str_5(
        this: &Document,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
        nodes_4: &str,
        nodes_5: &str,
    ) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Document" , js_name = prepend ) ]
    ///The `prepend()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/prepend)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn prepend_with_str_6(
        this: &Document,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
        nodes_4: &str,
        nodes_5: &str,
        nodes_6: &str,
    ) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Document" , js_name = prepend ) ]
    ///The `prepend()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/prepend)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn prepend_with_str_7(
        this: &Document,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
        nodes_4: &str,
        nodes_5: &str,
        nodes_6: &str,
        nodes_7: &str,
    ) -> Result<(), JsValue>;

    #[cfg(feature = "XPathExpression")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "Document" , js_name = createExpression ) ]
    ///The `createExpression()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/createExpression)
    ///
    ///*This API requires the following crate features to be activated: `Document`, `XPathExpression`*
    pub fn create_expression(this: &Document, expression: &str)
        -> Result<XPathExpression, JsValue>;

    #[cfg(feature = "XPathExpression")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "Document" , js_name = createExpression ) ]
    ///The `createExpression()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/createExpression)
    ///
    ///*This API requires the following crate features to be activated: `Document`, `XPathExpression`*
    pub fn create_expression_with_opt_callback(
        this: &Document,
        expression: &str,
        resolver: Option<&::js_sys::Function>,
    ) -> Result<XPathExpression, JsValue>;

    #[cfg(all(feature = "XPathExpression", feature = "XPathNsResolver",))]
    # [ wasm_bindgen ( catch , method , structural , js_class = "Document" , js_name = createExpression ) ]
    ///The `createExpression()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/createExpression)
    ///
    ///*This API requires the following crate features to be activated: `Document`, `XPathExpression`, `XPathNsResolver`*
    pub fn create_expression_with_opt_x_path_ns_resolver(
        this: &Document,
        expression: &str,
        resolver: Option<&XPathNsResolver>,
    ) -> Result<XPathExpression, JsValue>;

    # [ wasm_bindgen ( method , structural , js_class = "Document" , js_name = createNSResolver ) ]
    ///The `createNSResolver()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/createNSResolver)
    ///
    ///*This API requires the following crate features to be activated: `Document`*
    pub fn create_ns_resolver(this: &Document, node_resolver: &Node) -> Node;

    #[cfg(feature = "XPathResult")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "Document" , js_name = evaluate ) ]
    ///The `evaluate()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/evaluate)
    ///
    ///*This API requires the following crate features to be activated: `Document`, `XPathResult`*
    pub fn evaluate(
        this: &Document,
        expression: &str,
        context_node: &Node,
    ) -> Result<XPathResult, JsValue>;

    #[cfg(feature = "XPathResult")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "Document" , js_name = evaluate ) ]
    ///The `evaluate()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/evaluate)
    ///
    ///*This API requires the following crate features to be activated: `Document`, `XPathResult`*
    pub fn evaluate_with_opt_callback(
        this: &Document,
        expression: &str,
        context_node: &Node,
        resolver: Option<&::js_sys::Function>,
    ) -> Result<XPathResult, JsValue>;

    #[cfg(all(feature = "XPathNsResolver", feature = "XPathResult",))]
    # [ wasm_bindgen ( catch , method , structural , js_class = "Document" , js_name = evaluate ) ]
    ///The `evaluate()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/evaluate)
    ///
    ///*This API requires the following crate features to be activated: `Document`, `XPathNsResolver`, `XPathResult`*
    pub fn evaluate_with_opt_x_path_ns_resolver(
        this: &Document,
        expression: &str,
        context_node: &Node,
        resolver: Option<&XPathNsResolver>,
    ) -> Result<XPathResult, JsValue>;

    #[cfg(feature = "XPathResult")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "Document" , js_name = evaluate ) ]
    ///The `evaluate()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/evaluate)
    ///
    ///*This API requires the following crate features to be activated: `Document`, `XPathResult`*
    pub fn evaluate_with_opt_callback_and_type(
        this: &Document,
        expression: &str,
        context_node: &Node,
        resolver: Option<&::js_sys::Function>,
        type_: u16,
    ) -> Result<XPathResult, JsValue>;

    #[cfg(all(feature = "XPathNsResolver", feature = "XPathResult",))]
    # [ wasm_bindgen ( catch , method , structural , js_class = "Document" , js_name = evaluate ) ]
    ///The `evaluate()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/evaluate)
    ///
    ///*This API requires the following crate features to be activated: `Document`, `XPathNsResolver`, `XPathResult`*
    pub fn evaluate_with_opt_x_path_ns_resolver_and_type(
        this: &Document,
        expression: &str,
        context_node: &Node,
        resolver: Option<&XPathNsResolver>,
        type_: u16,
    ) -> Result<XPathResult, JsValue>;

    #[cfg(feature = "XPathResult")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "Document" , js_name = evaluate ) ]
    ///The `evaluate()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/evaluate)
    ///
    ///*This API requires the following crate features to be activated: `Document`, `XPathResult`*
    pub fn evaluate_with_opt_callback_and_type_and_result(
        this: &Document,
        expression: &str,
        context_node: &Node,
        resolver: Option<&::js_sys::Function>,
        type_: u16,
        result: Option<&::js_sys::Object>,
    ) -> Result<XPathResult, JsValue>;

    #[cfg(all(feature = "XPathNsResolver", feature = "XPathResult",))]
    # [ wasm_bindgen ( catch , method , structural , js_class = "Document" , js_name = evaluate ) ]
    ///The `evaluate()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/evaluate)
    ///
    ///*This API requires the following crate features to be activated: `Document`, `XPathNsResolver`, `XPathResult`*
    pub fn evaluate_with_opt_x_path_ns_resolver_and_type_and_result(
        this: &Document,
        expression: &str,
        context_node: &Node,
        resolver: Option<&XPathNsResolver>,
        type_: u16,
        result: Option<&::js_sys::Object>,
    ) -> Result<XPathResult, JsValue>;

}
