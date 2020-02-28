use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = HTMLElement , typescript_name = HTMLElement ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `HtmlElement` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    pub type HtmlElement;
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLElement" , js_name = title ) ]
    #[doc = "Getter for the `title` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/title)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    pub fn title(this: &HtmlElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLElement" , js_name = title ) ]
    #[doc = "Setter for the `title` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/title)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    pub fn set_title(this: &HtmlElement, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLElement" , js_name = lang ) ]
    #[doc = "Getter for the `lang` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/lang)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    pub fn lang(this: &HtmlElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLElement" , js_name = lang ) ]
    #[doc = "Setter for the `lang` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/lang)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    pub fn set_lang(this: &HtmlElement, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLElement" , js_name = dir ) ]
    #[doc = "Getter for the `dir` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/dir)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    pub fn dir(this: &HtmlElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLElement" , js_name = dir ) ]
    #[doc = "Setter for the `dir` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/dir)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    pub fn set_dir(this: &HtmlElement, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLElement" , js_name = dataset ) ]
    #[cfg(feature = "DomStringMap")]
    #[doc = "Getter for the `dataset` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/dataset)\n\n*This API requires the following crate features to be activated: `DomStringMap`, `HtmlElement`*"]
    pub fn dataset(this: &HtmlElement) -> DomStringMap;
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLElement" , js_name = innerText ) ]
    #[doc = "Getter for the `innerText` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/innerText)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    pub fn inner_text(this: &HtmlElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLElement" , js_name = innerText ) ]
    #[doc = "Setter for the `innerText` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/innerText)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    pub fn set_inner_text(this: &HtmlElement, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLElement" , js_name = hidden ) ]
    #[doc = "Getter for the `hidden` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/hidden)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    pub fn hidden(this: &HtmlElement) -> bool;
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLElement" , js_name = hidden ) ]
    #[doc = "Setter for the `hidden` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/hidden)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    pub fn set_hidden(this: &HtmlElement, value: bool);
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLElement" , js_name = tabIndex ) ]
    #[doc = "Getter for the `tabIndex` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/tabIndex)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    pub fn tab_index(this: &HtmlElement) -> i32;
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLElement" , js_name = tabIndex ) ]
    #[doc = "Setter for the `tabIndex` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/tabIndex)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    pub fn set_tab_index(this: &HtmlElement, value: i32);
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLElement" , js_name = accessKey ) ]
    #[doc = "Getter for the `accessKey` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/accessKey)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    pub fn access_key(this: &HtmlElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLElement" , js_name = accessKey ) ]
    #[doc = "Setter for the `accessKey` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/accessKey)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    pub fn set_access_key(this: &HtmlElement, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLElement" , js_name = accessKeyLabel ) ]
    #[doc = "Getter for the `accessKeyLabel` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/accessKeyLabel)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    pub fn access_key_label(this: &HtmlElement) -> String;
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLElement" , js_name = draggable ) ]
    #[doc = "Getter for the `draggable` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/draggable)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    pub fn draggable(this: &HtmlElement) -> bool;
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLElement" , js_name = draggable ) ]
    #[doc = "Setter for the `draggable` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/draggable)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    pub fn set_draggable(this: &HtmlElement, value: bool);
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLElement" , js_name = contentEditable ) ]
    #[doc = "Getter for the `contentEditable` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/contentEditable)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    pub fn content_editable(this: &HtmlElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLElement" , js_name = contentEditable ) ]
    #[doc = "Setter for the `contentEditable` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/contentEditable)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    pub fn set_content_editable(this: &HtmlElement, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLElement" , js_name = isContentEditable ) ]
    #[doc = "Getter for the `isContentEditable` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/isContentEditable)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    pub fn is_content_editable(this: &HtmlElement) -> bool;
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLElement" , js_name = spellcheck ) ]
    #[doc = "Getter for the `spellcheck` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/spellcheck)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    pub fn spellcheck(this: &HtmlElement) -> bool;
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLElement" , js_name = spellcheck ) ]
    #[doc = "Setter for the `spellcheck` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/spellcheck)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    pub fn set_spellcheck(this: &HtmlElement, value: bool);
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLElement" , js_name = style ) ]
    #[cfg(feature = "CssStyleDeclaration")]
    #[doc = "Getter for the `style` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/style)\n\n*This API requires the following crate features to be activated: `CssStyleDeclaration`, `HtmlElement`*"]
    pub fn style(this: &HtmlElement) -> CssStyleDeclaration;
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLElement" , js_name = offsetParent ) ]
    #[cfg(feature = "Element")]
    #[doc = "Getter for the `offsetParent` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/offsetParent)\n\n*This API requires the following crate features to be activated: `Element`, `HtmlElement`*"]
    pub fn offset_parent(this: &HtmlElement) -> Option<Element>;
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLElement" , js_name = offsetTop ) ]
    #[doc = "Getter for the `offsetTop` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/offsetTop)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    pub fn offset_top(this: &HtmlElement) -> i32;
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLElement" , js_name = offsetLeft ) ]
    #[doc = "Getter for the `offsetLeft` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/offsetLeft)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    pub fn offset_left(this: &HtmlElement) -> i32;
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLElement" , js_name = offsetWidth ) ]
    #[doc = "Getter for the `offsetWidth` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/offsetWidth)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    pub fn offset_width(this: &HtmlElement) -> i32;
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLElement" , js_name = offsetHeight ) ]
    #[doc = "Getter for the `offsetHeight` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/offsetHeight)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    pub fn offset_height(this: &HtmlElement) -> i32;
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLElement" , js_name = oncopy ) ]
    #[doc = "Getter for the `oncopy` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/oncopy)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    pub fn oncopy(this: &HtmlElement) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLElement" , js_name = oncopy ) ]
    #[doc = "Setter for the `oncopy` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/oncopy)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    pub fn set_oncopy(this: &HtmlElement, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLElement" , js_name = oncut ) ]
    #[doc = "Getter for the `oncut` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/oncut)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    pub fn oncut(this: &HtmlElement) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLElement" , js_name = oncut ) ]
    #[doc = "Setter for the `oncut` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/oncut)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    pub fn set_oncut(this: &HtmlElement, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLElement" , js_name = onpaste ) ]
    #[doc = "Getter for the `onpaste` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onpaste)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    pub fn onpaste(this: &HtmlElement) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLElement" , js_name = onpaste ) ]
    #[doc = "Setter for the `onpaste` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onpaste)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    pub fn set_onpaste(this: &HtmlElement, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLElement" , js_name = onabort ) ]
    #[doc = "Getter for the `onabort` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onabort)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    pub fn onabort(this: &HtmlElement) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLElement" , js_name = onabort ) ]
    #[doc = "Setter for the `onabort` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onabort)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    pub fn set_onabort(this: &HtmlElement, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLElement" , js_name = onblur ) ]
    #[doc = "Getter for the `onblur` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onblur)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    pub fn onblur(this: &HtmlElement) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLElement" , js_name = onblur ) ]
    #[doc = "Setter for the `onblur` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onblur)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    pub fn set_onblur(this: &HtmlElement, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLElement" , js_name = onfocus ) ]
    #[doc = "Getter for the `onfocus` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onfocus)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    pub fn onfocus(this: &HtmlElement) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLElement" , js_name = onfocus ) ]
    #[doc = "Setter for the `onfocus` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onfocus)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    pub fn set_onfocus(this: &HtmlElement, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLElement" , js_name = onauxclick ) ]
    #[doc = "Getter for the `onauxclick` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onauxclick)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    pub fn onauxclick(this: &HtmlElement) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLElement" , js_name = onauxclick ) ]
    #[doc = "Setter for the `onauxclick` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onauxclick)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    pub fn set_onauxclick(this: &HtmlElement, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLElement" , js_name = oncanplay ) ]
    #[doc = "Getter for the `oncanplay` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/oncanplay)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    pub fn oncanplay(this: &HtmlElement) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLElement" , js_name = oncanplay ) ]
    #[doc = "Setter for the `oncanplay` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/oncanplay)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    pub fn set_oncanplay(this: &HtmlElement, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLElement" , js_name = oncanplaythrough ) ]
    #[doc = "Getter for the `oncanplaythrough` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/oncanplaythrough)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    pub fn oncanplaythrough(this: &HtmlElement) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLElement" , js_name = oncanplaythrough ) ]
    #[doc = "Setter for the `oncanplaythrough` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/oncanplaythrough)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    pub fn set_oncanplaythrough(this: &HtmlElement, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLElement" , js_name = onchange ) ]
    #[doc = "Getter for the `onchange` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onchange)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    pub fn onchange(this: &HtmlElement) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLElement" , js_name = onchange ) ]
    #[doc = "Setter for the `onchange` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onchange)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    pub fn set_onchange(this: &HtmlElement, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLElement" , js_name = onclick ) ]
    #[doc = "Getter for the `onclick` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onclick)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    pub fn onclick(this: &HtmlElement) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLElement" , js_name = onclick ) ]
    #[doc = "Setter for the `onclick` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onclick)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    pub fn set_onclick(this: &HtmlElement, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLElement" , js_name = onclose ) ]
    #[doc = "Getter for the `onclose` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onclose)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    pub fn onclose(this: &HtmlElement) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLElement" , js_name = onclose ) ]
    #[doc = "Setter for the `onclose` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onclose)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    pub fn set_onclose(this: &HtmlElement, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLElement" , js_name = oncontextmenu ) ]
    #[doc = "Getter for the `oncontextmenu` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/oncontextmenu)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    pub fn oncontextmenu(this: &HtmlElement) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLElement" , js_name = oncontextmenu ) ]
    #[doc = "Setter for the `oncontextmenu` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/oncontextmenu)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    pub fn set_oncontextmenu(this: &HtmlElement, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLElement" , js_name = ondblclick ) ]
    #[doc = "Getter for the `ondblclick` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/ondblclick)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    pub fn ondblclick(this: &HtmlElement) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLElement" , js_name = ondblclick ) ]
    #[doc = "Setter for the `ondblclick` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/ondblclick)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    pub fn set_ondblclick(this: &HtmlElement, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLElement" , js_name = ondrag ) ]
    #[doc = "Getter for the `ondrag` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/ondrag)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    pub fn ondrag(this: &HtmlElement) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLElement" , js_name = ondrag ) ]
    #[doc = "Setter for the `ondrag` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/ondrag)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    pub fn set_ondrag(this: &HtmlElement, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLElement" , js_name = ondragend ) ]
    #[doc = "Getter for the `ondragend` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/ondragend)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    pub fn ondragend(this: &HtmlElement) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLElement" , js_name = ondragend ) ]
    #[doc = "Setter for the `ondragend` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/ondragend)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    pub fn set_ondragend(this: &HtmlElement, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLElement" , js_name = ondragenter ) ]
    #[doc = "Getter for the `ondragenter` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/ondragenter)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    pub fn ondragenter(this: &HtmlElement) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLElement" , js_name = ondragenter ) ]
    #[doc = "Setter for the `ondragenter` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/ondragenter)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    pub fn set_ondragenter(this: &HtmlElement, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLElement" , js_name = ondragexit ) ]
    #[doc = "Getter for the `ondragexit` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/ondragexit)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    pub fn ondragexit(this: &HtmlElement) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLElement" , js_name = ondragexit ) ]
    #[doc = "Setter for the `ondragexit` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/ondragexit)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    pub fn set_ondragexit(this: &HtmlElement, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLElement" , js_name = ondragleave ) ]
    #[doc = "Getter for the `ondragleave` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/ondragleave)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    pub fn ondragleave(this: &HtmlElement) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLElement" , js_name = ondragleave ) ]
    #[doc = "Setter for the `ondragleave` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/ondragleave)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    pub fn set_ondragleave(this: &HtmlElement, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLElement" , js_name = ondragover ) ]
    #[doc = "Getter for the `ondragover` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/ondragover)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    pub fn ondragover(this: &HtmlElement) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLElement" , js_name = ondragover ) ]
    #[doc = "Setter for the `ondragover` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/ondragover)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    pub fn set_ondragover(this: &HtmlElement, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLElement" , js_name = ondragstart ) ]
    #[doc = "Getter for the `ondragstart` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/ondragstart)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    pub fn ondragstart(this: &HtmlElement) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLElement" , js_name = ondragstart ) ]
    #[doc = "Setter for the `ondragstart` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/ondragstart)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    pub fn set_ondragstart(this: &HtmlElement, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLElement" , js_name = ondrop ) ]
    #[doc = "Getter for the `ondrop` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/ondrop)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    pub fn ondrop(this: &HtmlElement) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLElement" , js_name = ondrop ) ]
    #[doc = "Setter for the `ondrop` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/ondrop)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    pub fn set_ondrop(this: &HtmlElement, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLElement" , js_name = ondurationchange ) ]
    #[doc = "Getter for the `ondurationchange` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/ondurationchange)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    pub fn ondurationchange(this: &HtmlElement) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLElement" , js_name = ondurationchange ) ]
    #[doc = "Setter for the `ondurationchange` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/ondurationchange)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    pub fn set_ondurationchange(this: &HtmlElement, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLElement" , js_name = onemptied ) ]
    #[doc = "Getter for the `onemptied` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onemptied)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    pub fn onemptied(this: &HtmlElement) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLElement" , js_name = onemptied ) ]
    #[doc = "Setter for the `onemptied` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onemptied)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    pub fn set_onemptied(this: &HtmlElement, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLElement" , js_name = onended ) ]
    #[doc = "Getter for the `onended` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onended)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    pub fn onended(this: &HtmlElement) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLElement" , js_name = onended ) ]
    #[doc = "Setter for the `onended` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onended)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    pub fn set_onended(this: &HtmlElement, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLElement" , js_name = oninput ) ]
    #[doc = "Getter for the `oninput` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/oninput)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    pub fn oninput(this: &HtmlElement) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLElement" , js_name = oninput ) ]
    #[doc = "Setter for the `oninput` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/oninput)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    pub fn set_oninput(this: &HtmlElement, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLElement" , js_name = oninvalid ) ]
    #[doc = "Getter for the `oninvalid` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/oninvalid)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    pub fn oninvalid(this: &HtmlElement) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLElement" , js_name = oninvalid ) ]
    #[doc = "Setter for the `oninvalid` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/oninvalid)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    pub fn set_oninvalid(this: &HtmlElement, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLElement" , js_name = onkeydown ) ]
    #[doc = "Getter for the `onkeydown` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onkeydown)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    pub fn onkeydown(this: &HtmlElement) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLElement" , js_name = onkeydown ) ]
    #[doc = "Setter for the `onkeydown` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onkeydown)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    pub fn set_onkeydown(this: &HtmlElement, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLElement" , js_name = onkeypress ) ]
    #[doc = "Getter for the `onkeypress` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onkeypress)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    pub fn onkeypress(this: &HtmlElement) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLElement" , js_name = onkeypress ) ]
    #[doc = "Setter for the `onkeypress` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onkeypress)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    pub fn set_onkeypress(this: &HtmlElement, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLElement" , js_name = onkeyup ) ]
    #[doc = "Getter for the `onkeyup` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onkeyup)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    pub fn onkeyup(this: &HtmlElement) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLElement" , js_name = onkeyup ) ]
    #[doc = "Setter for the `onkeyup` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onkeyup)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    pub fn set_onkeyup(this: &HtmlElement, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLElement" , js_name = onload ) ]
    #[doc = "Getter for the `onload` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onload)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    pub fn onload(this: &HtmlElement) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLElement" , js_name = onload ) ]
    #[doc = "Setter for the `onload` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onload)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    pub fn set_onload(this: &HtmlElement, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLElement" , js_name = onloadeddata ) ]
    #[doc = "Getter for the `onloadeddata` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onloadeddata)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    pub fn onloadeddata(this: &HtmlElement) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLElement" , js_name = onloadeddata ) ]
    #[doc = "Setter for the `onloadeddata` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onloadeddata)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    pub fn set_onloadeddata(this: &HtmlElement, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLElement" , js_name = onloadedmetadata ) ]
    #[doc = "Getter for the `onloadedmetadata` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onloadedmetadata)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    pub fn onloadedmetadata(this: &HtmlElement) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLElement" , js_name = onloadedmetadata ) ]
    #[doc = "Setter for the `onloadedmetadata` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onloadedmetadata)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    pub fn set_onloadedmetadata(this: &HtmlElement, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLElement" , js_name = onloadend ) ]
    #[doc = "Getter for the `onloadend` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onloadend)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    pub fn onloadend(this: &HtmlElement) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLElement" , js_name = onloadend ) ]
    #[doc = "Setter for the `onloadend` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onloadend)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    pub fn set_onloadend(this: &HtmlElement, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLElement" , js_name = onloadstart ) ]
    #[doc = "Getter for the `onloadstart` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onloadstart)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    pub fn onloadstart(this: &HtmlElement) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLElement" , js_name = onloadstart ) ]
    #[doc = "Setter for the `onloadstart` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onloadstart)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    pub fn set_onloadstart(this: &HtmlElement, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLElement" , js_name = onmousedown ) ]
    #[doc = "Getter for the `onmousedown` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onmousedown)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    pub fn onmousedown(this: &HtmlElement) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLElement" , js_name = onmousedown ) ]
    #[doc = "Setter for the `onmousedown` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onmousedown)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    pub fn set_onmousedown(this: &HtmlElement, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLElement" , js_name = onmouseenter ) ]
    #[doc = "Getter for the `onmouseenter` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onmouseenter)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    pub fn onmouseenter(this: &HtmlElement) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLElement" , js_name = onmouseenter ) ]
    #[doc = "Setter for the `onmouseenter` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onmouseenter)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    pub fn set_onmouseenter(this: &HtmlElement, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLElement" , js_name = onmouseleave ) ]
    #[doc = "Getter for the `onmouseleave` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onmouseleave)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    pub fn onmouseleave(this: &HtmlElement) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLElement" , js_name = onmouseleave ) ]
    #[doc = "Setter for the `onmouseleave` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onmouseleave)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    pub fn set_onmouseleave(this: &HtmlElement, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLElement" , js_name = onmousemove ) ]
    #[doc = "Getter for the `onmousemove` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onmousemove)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    pub fn onmousemove(this: &HtmlElement) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLElement" , js_name = onmousemove ) ]
    #[doc = "Setter for the `onmousemove` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onmousemove)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    pub fn set_onmousemove(this: &HtmlElement, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLElement" , js_name = onmouseout ) ]
    #[doc = "Getter for the `onmouseout` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onmouseout)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    pub fn onmouseout(this: &HtmlElement) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLElement" , js_name = onmouseout ) ]
    #[doc = "Setter for the `onmouseout` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onmouseout)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    pub fn set_onmouseout(this: &HtmlElement, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLElement" , js_name = onmouseover ) ]
    #[doc = "Getter for the `onmouseover` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onmouseover)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    pub fn onmouseover(this: &HtmlElement) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLElement" , js_name = onmouseover ) ]
    #[doc = "Setter for the `onmouseover` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onmouseover)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    pub fn set_onmouseover(this: &HtmlElement, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLElement" , js_name = onmouseup ) ]
    #[doc = "Getter for the `onmouseup` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onmouseup)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    pub fn onmouseup(this: &HtmlElement) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLElement" , js_name = onmouseup ) ]
    #[doc = "Setter for the `onmouseup` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onmouseup)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    pub fn set_onmouseup(this: &HtmlElement, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLElement" , js_name = onwheel ) ]
    #[doc = "Getter for the `onwheel` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onwheel)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    pub fn onwheel(this: &HtmlElement) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLElement" , js_name = onwheel ) ]
    #[doc = "Setter for the `onwheel` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onwheel)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    pub fn set_onwheel(this: &HtmlElement, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLElement" , js_name = onpause ) ]
    #[doc = "Getter for the `onpause` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onpause)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    pub fn onpause(this: &HtmlElement) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLElement" , js_name = onpause ) ]
    #[doc = "Setter for the `onpause` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onpause)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    pub fn set_onpause(this: &HtmlElement, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLElement" , js_name = onplay ) ]
    #[doc = "Getter for the `onplay` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onplay)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    pub fn onplay(this: &HtmlElement) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLElement" , js_name = onplay ) ]
    #[doc = "Setter for the `onplay` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onplay)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    pub fn set_onplay(this: &HtmlElement, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLElement" , js_name = onplaying ) ]
    #[doc = "Getter for the `onplaying` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onplaying)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    pub fn onplaying(this: &HtmlElement) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLElement" , js_name = onplaying ) ]
    #[doc = "Setter for the `onplaying` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onplaying)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    pub fn set_onplaying(this: &HtmlElement, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLElement" , js_name = onprogress ) ]
    #[doc = "Getter for the `onprogress` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onprogress)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    pub fn onprogress(this: &HtmlElement) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLElement" , js_name = onprogress ) ]
    #[doc = "Setter for the `onprogress` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onprogress)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    pub fn set_onprogress(this: &HtmlElement, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLElement" , js_name = onratechange ) ]
    #[doc = "Getter for the `onratechange` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onratechange)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    pub fn onratechange(this: &HtmlElement) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLElement" , js_name = onratechange ) ]
    #[doc = "Setter for the `onratechange` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onratechange)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    pub fn set_onratechange(this: &HtmlElement, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLElement" , js_name = onreset ) ]
    #[doc = "Getter for the `onreset` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onreset)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    pub fn onreset(this: &HtmlElement) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLElement" , js_name = onreset ) ]
    #[doc = "Setter for the `onreset` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onreset)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    pub fn set_onreset(this: &HtmlElement, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLElement" , js_name = onresize ) ]
    #[doc = "Getter for the `onresize` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onresize)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    pub fn onresize(this: &HtmlElement) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLElement" , js_name = onresize ) ]
    #[doc = "Setter for the `onresize` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onresize)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    pub fn set_onresize(this: &HtmlElement, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLElement" , js_name = onscroll ) ]
    #[doc = "Getter for the `onscroll` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onscroll)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    pub fn onscroll(this: &HtmlElement) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLElement" , js_name = onscroll ) ]
    #[doc = "Setter for the `onscroll` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onscroll)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    pub fn set_onscroll(this: &HtmlElement, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLElement" , js_name = onseeked ) ]
    #[doc = "Getter for the `onseeked` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onseeked)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    pub fn onseeked(this: &HtmlElement) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLElement" , js_name = onseeked ) ]
    #[doc = "Setter for the `onseeked` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onseeked)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    pub fn set_onseeked(this: &HtmlElement, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLElement" , js_name = onseeking ) ]
    #[doc = "Getter for the `onseeking` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onseeking)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    pub fn onseeking(this: &HtmlElement) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLElement" , js_name = onseeking ) ]
    #[doc = "Setter for the `onseeking` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onseeking)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    pub fn set_onseeking(this: &HtmlElement, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLElement" , js_name = onselect ) ]
    #[doc = "Getter for the `onselect` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onselect)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    pub fn onselect(this: &HtmlElement) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLElement" , js_name = onselect ) ]
    #[doc = "Setter for the `onselect` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onselect)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    pub fn set_onselect(this: &HtmlElement, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLElement" , js_name = onshow ) ]
    #[doc = "Getter for the `onshow` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onshow)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    pub fn onshow(this: &HtmlElement) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLElement" , js_name = onshow ) ]
    #[doc = "Setter for the `onshow` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onshow)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    pub fn set_onshow(this: &HtmlElement, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLElement" , js_name = onstalled ) ]
    #[doc = "Getter for the `onstalled` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onstalled)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    pub fn onstalled(this: &HtmlElement) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLElement" , js_name = onstalled ) ]
    #[doc = "Setter for the `onstalled` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onstalled)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    pub fn set_onstalled(this: &HtmlElement, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLElement" , js_name = onsubmit ) ]
    #[doc = "Getter for the `onsubmit` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onsubmit)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    pub fn onsubmit(this: &HtmlElement) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLElement" , js_name = onsubmit ) ]
    #[doc = "Setter for the `onsubmit` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onsubmit)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    pub fn set_onsubmit(this: &HtmlElement, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLElement" , js_name = onsuspend ) ]
    #[doc = "Getter for the `onsuspend` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onsuspend)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    pub fn onsuspend(this: &HtmlElement) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLElement" , js_name = onsuspend ) ]
    #[doc = "Setter for the `onsuspend` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onsuspend)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    pub fn set_onsuspend(this: &HtmlElement, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLElement" , js_name = ontimeupdate ) ]
    #[doc = "Getter for the `ontimeupdate` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/ontimeupdate)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    pub fn ontimeupdate(this: &HtmlElement) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLElement" , js_name = ontimeupdate ) ]
    #[doc = "Setter for the `ontimeupdate` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/ontimeupdate)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    pub fn set_ontimeupdate(this: &HtmlElement, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLElement" , js_name = onvolumechange ) ]
    #[doc = "Getter for the `onvolumechange` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onvolumechange)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    pub fn onvolumechange(this: &HtmlElement) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLElement" , js_name = onvolumechange ) ]
    #[doc = "Setter for the `onvolumechange` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onvolumechange)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    pub fn set_onvolumechange(this: &HtmlElement, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLElement" , js_name = onwaiting ) ]
    #[doc = "Getter for the `onwaiting` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onwaiting)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    pub fn onwaiting(this: &HtmlElement) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLElement" , js_name = onwaiting ) ]
    #[doc = "Setter for the `onwaiting` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onwaiting)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    pub fn set_onwaiting(this: &HtmlElement, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLElement" , js_name = onselectstart ) ]
    #[doc = "Getter for the `onselectstart` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onselectstart)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    pub fn onselectstart(this: &HtmlElement) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLElement" , js_name = onselectstart ) ]
    #[doc = "Setter for the `onselectstart` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onselectstart)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    pub fn set_onselectstart(this: &HtmlElement, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLElement" , js_name = ontoggle ) ]
    #[doc = "Getter for the `ontoggle` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/ontoggle)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    pub fn ontoggle(this: &HtmlElement) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLElement" , js_name = ontoggle ) ]
    #[doc = "Setter for the `ontoggle` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/ontoggle)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    pub fn set_ontoggle(this: &HtmlElement, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLElement" , js_name = onpointercancel ) ]
    #[doc = "Getter for the `onpointercancel` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onpointercancel)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    pub fn onpointercancel(this: &HtmlElement) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLElement" , js_name = onpointercancel ) ]
    #[doc = "Setter for the `onpointercancel` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onpointercancel)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    pub fn set_onpointercancel(this: &HtmlElement, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLElement" , js_name = onpointerdown ) ]
    #[doc = "Getter for the `onpointerdown` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onpointerdown)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    pub fn onpointerdown(this: &HtmlElement) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLElement" , js_name = onpointerdown ) ]
    #[doc = "Setter for the `onpointerdown` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onpointerdown)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    pub fn set_onpointerdown(this: &HtmlElement, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLElement" , js_name = onpointerup ) ]
    #[doc = "Getter for the `onpointerup` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onpointerup)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    pub fn onpointerup(this: &HtmlElement) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLElement" , js_name = onpointerup ) ]
    #[doc = "Setter for the `onpointerup` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onpointerup)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    pub fn set_onpointerup(this: &HtmlElement, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLElement" , js_name = onpointermove ) ]
    #[doc = "Getter for the `onpointermove` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onpointermove)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    pub fn onpointermove(this: &HtmlElement) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLElement" , js_name = onpointermove ) ]
    #[doc = "Setter for the `onpointermove` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onpointermove)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    pub fn set_onpointermove(this: &HtmlElement, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLElement" , js_name = onpointerout ) ]
    #[doc = "Getter for the `onpointerout` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onpointerout)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    pub fn onpointerout(this: &HtmlElement) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLElement" , js_name = onpointerout ) ]
    #[doc = "Setter for the `onpointerout` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onpointerout)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    pub fn set_onpointerout(this: &HtmlElement, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLElement" , js_name = onpointerover ) ]
    #[doc = "Getter for the `onpointerover` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onpointerover)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    pub fn onpointerover(this: &HtmlElement) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLElement" , js_name = onpointerover ) ]
    #[doc = "Setter for the `onpointerover` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onpointerover)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    pub fn set_onpointerover(this: &HtmlElement, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLElement" , js_name = onpointerenter ) ]
    #[doc = "Getter for the `onpointerenter` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onpointerenter)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    pub fn onpointerenter(this: &HtmlElement) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLElement" , js_name = onpointerenter ) ]
    #[doc = "Setter for the `onpointerenter` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onpointerenter)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    pub fn set_onpointerenter(this: &HtmlElement, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLElement" , js_name = onpointerleave ) ]
    #[doc = "Getter for the `onpointerleave` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onpointerleave)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    pub fn onpointerleave(this: &HtmlElement) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLElement" , js_name = onpointerleave ) ]
    #[doc = "Setter for the `onpointerleave` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onpointerleave)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    pub fn set_onpointerleave(this: &HtmlElement, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLElement" , js_name = ongotpointercapture ) ]
    #[doc = "Getter for the `ongotpointercapture` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/ongotpointercapture)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    pub fn ongotpointercapture(this: &HtmlElement) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLElement" , js_name = ongotpointercapture ) ]
    #[doc = "Setter for the `ongotpointercapture` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/ongotpointercapture)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    pub fn set_ongotpointercapture(this: &HtmlElement, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLElement" , js_name = onlostpointercapture ) ]
    #[doc = "Getter for the `onlostpointercapture` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onlostpointercapture)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    pub fn onlostpointercapture(this: &HtmlElement) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLElement" , js_name = onlostpointercapture ) ]
    #[doc = "Setter for the `onlostpointercapture` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onlostpointercapture)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    pub fn set_onlostpointercapture(this: &HtmlElement, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLElement" , js_name = onanimationcancel ) ]
    #[doc = "Getter for the `onanimationcancel` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onanimationcancel)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    pub fn onanimationcancel(this: &HtmlElement) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLElement" , js_name = onanimationcancel ) ]
    #[doc = "Setter for the `onanimationcancel` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onanimationcancel)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    pub fn set_onanimationcancel(this: &HtmlElement, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLElement" , js_name = onanimationend ) ]
    #[doc = "Getter for the `onanimationend` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onanimationend)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    pub fn onanimationend(this: &HtmlElement) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLElement" , js_name = onanimationend ) ]
    #[doc = "Setter for the `onanimationend` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onanimationend)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    pub fn set_onanimationend(this: &HtmlElement, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLElement" , js_name = onanimationiteration ) ]
    #[doc = "Getter for the `onanimationiteration` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onanimationiteration)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    pub fn onanimationiteration(this: &HtmlElement) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLElement" , js_name = onanimationiteration ) ]
    #[doc = "Setter for the `onanimationiteration` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onanimationiteration)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    pub fn set_onanimationiteration(this: &HtmlElement, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLElement" , js_name = onanimationstart ) ]
    #[doc = "Getter for the `onanimationstart` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onanimationstart)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    pub fn onanimationstart(this: &HtmlElement) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLElement" , js_name = onanimationstart ) ]
    #[doc = "Setter for the `onanimationstart` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onanimationstart)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    pub fn set_onanimationstart(this: &HtmlElement, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLElement" , js_name = ontransitioncancel ) ]
    #[doc = "Getter for the `ontransitioncancel` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/ontransitioncancel)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    pub fn ontransitioncancel(this: &HtmlElement) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLElement" , js_name = ontransitioncancel ) ]
    #[doc = "Setter for the `ontransitioncancel` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/ontransitioncancel)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    pub fn set_ontransitioncancel(this: &HtmlElement, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLElement" , js_name = ontransitionend ) ]
    #[doc = "Getter for the `ontransitionend` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/ontransitionend)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    pub fn ontransitionend(this: &HtmlElement) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLElement" , js_name = ontransitionend ) ]
    #[doc = "Setter for the `ontransitionend` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/ontransitionend)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    pub fn set_ontransitionend(this: &HtmlElement, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLElement" , js_name = ontransitionrun ) ]
    #[doc = "Getter for the `ontransitionrun` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/ontransitionrun)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    pub fn ontransitionrun(this: &HtmlElement) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLElement" , js_name = ontransitionrun ) ]
    #[doc = "Setter for the `ontransitionrun` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/ontransitionrun)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    pub fn set_ontransitionrun(this: &HtmlElement, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLElement" , js_name = ontransitionstart ) ]
    #[doc = "Getter for the `ontransitionstart` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/ontransitionstart)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    pub fn ontransitionstart(this: &HtmlElement) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLElement" , js_name = ontransitionstart ) ]
    #[doc = "Setter for the `ontransitionstart` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/ontransitionstart)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    pub fn set_ontransitionstart(this: &HtmlElement, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLElement" , js_name = onwebkitanimationend ) ]
    #[doc = "Getter for the `onwebkitanimationend` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onwebkitanimationend)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    pub fn onwebkitanimationend(this: &HtmlElement) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLElement" , js_name = onwebkitanimationend ) ]
    #[doc = "Setter for the `onwebkitanimationend` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onwebkitanimationend)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    pub fn set_onwebkitanimationend(this: &HtmlElement, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLElement" , js_name = onwebkitanimationiteration ) ]
    #[doc = "Getter for the `onwebkitanimationiteration` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onwebkitanimationiteration)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    pub fn onwebkitanimationiteration(this: &HtmlElement) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLElement" , js_name = onwebkitanimationiteration ) ]
    #[doc = "Setter for the `onwebkitanimationiteration` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onwebkitanimationiteration)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    pub fn set_onwebkitanimationiteration(this: &HtmlElement, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLElement" , js_name = onwebkitanimationstart ) ]
    #[doc = "Getter for the `onwebkitanimationstart` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onwebkitanimationstart)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    pub fn onwebkitanimationstart(this: &HtmlElement) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLElement" , js_name = onwebkitanimationstart ) ]
    #[doc = "Setter for the `onwebkitanimationstart` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onwebkitanimationstart)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    pub fn set_onwebkitanimationstart(this: &HtmlElement, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLElement" , js_name = onwebkittransitionend ) ]
    #[doc = "Getter for the `onwebkittransitionend` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onwebkittransitionend)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    pub fn onwebkittransitionend(this: &HtmlElement) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLElement" , js_name = onwebkittransitionend ) ]
    #[doc = "Setter for the `onwebkittransitionend` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onwebkittransitionend)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    pub fn set_onwebkittransitionend(this: &HtmlElement, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLElement" , js_name = onerror ) ]
    #[doc = "Getter for the `onerror` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onerror)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    pub fn onerror(this: &HtmlElement) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLElement" , js_name = onerror ) ]
    #[doc = "Setter for the `onerror` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onerror)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    pub fn set_onerror(this: &HtmlElement, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLElement" , js_name = ontouchstart ) ]
    #[doc = "Getter for the `ontouchstart` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/ontouchstart)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    pub fn ontouchstart(this: &HtmlElement) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLElement" , js_name = ontouchstart ) ]
    #[doc = "Setter for the `ontouchstart` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/ontouchstart)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    pub fn set_ontouchstart(this: &HtmlElement, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLElement" , js_name = ontouchend ) ]
    #[doc = "Getter for the `ontouchend` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/ontouchend)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    pub fn ontouchend(this: &HtmlElement) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLElement" , js_name = ontouchend ) ]
    #[doc = "Setter for the `ontouchend` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/ontouchend)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    pub fn set_ontouchend(this: &HtmlElement, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLElement" , js_name = ontouchmove ) ]
    #[doc = "Getter for the `ontouchmove` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/ontouchmove)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    pub fn ontouchmove(this: &HtmlElement) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLElement" , js_name = ontouchmove ) ]
    #[doc = "Setter for the `ontouchmove` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/ontouchmove)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    pub fn set_ontouchmove(this: &HtmlElement, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLElement" , js_name = ontouchcancel ) ]
    #[doc = "Getter for the `ontouchcancel` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/ontouchcancel)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    pub fn ontouchcancel(this: &HtmlElement) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLElement" , js_name = ontouchcancel ) ]
    #[doc = "Setter for the `ontouchcancel` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/ontouchcancel)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    pub fn set_ontouchcancel(this: &HtmlElement, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( catch , method , structural , js_class = "HTMLElement" , js_name = blur ) ]
    #[doc = "The `blur()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/blur)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    pub fn blur(this: &HtmlElement) -> Result<(), JsValue>;
    # [ wasm_bindgen ( method , structural , js_class = "HTMLElement" , js_name = click ) ]
    #[doc = "The `click()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/click)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    pub fn click(this: &HtmlElement);
    # [ wasm_bindgen ( catch , method , structural , js_class = "HTMLElement" , js_name = focus ) ]
    #[doc = "The `focus()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/focus)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    pub fn focus(this: &HtmlElement) -> Result<(), JsValue>;
}
