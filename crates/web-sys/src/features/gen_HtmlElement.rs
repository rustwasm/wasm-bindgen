use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = HTMLElement , typescript_type = "HTMLElement" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `HtmlElement` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement)
    ///
    ///*This API requires the following crate features to be activated: `HtmlElement`*
    pub type HtmlElement;

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLElement" , js_name = title ) ]
    ///Getter for the `title` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/title)
    ///
    ///*This API requires the following crate features to be activated: `HtmlElement`*
    pub fn title(this: &HtmlElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLElement" , js_name = title ) ]
    ///Setter for the `title` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/title)
    ///
    ///*This API requires the following crate features to be activated: `HtmlElement`*
    pub fn set_title(this: &HtmlElement, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLElement" , js_name = lang ) ]
    ///Getter for the `lang` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/lang)
    ///
    ///*This API requires the following crate features to be activated: `HtmlElement`*
    pub fn lang(this: &HtmlElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLElement" , js_name = lang ) ]
    ///Setter for the `lang` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/lang)
    ///
    ///*This API requires the following crate features to be activated: `HtmlElement`*
    pub fn set_lang(this: &HtmlElement, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLElement" , js_name = dir ) ]
    ///Getter for the `dir` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/dir)
    ///
    ///*This API requires the following crate features to be activated: `HtmlElement`*
    pub fn dir(this: &HtmlElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLElement" , js_name = dir ) ]
    ///Setter for the `dir` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/dir)
    ///
    ///*This API requires the following crate features to be activated: `HtmlElement`*
    pub fn set_dir(this: &HtmlElement, value: &str);

    #[cfg(feature = "DomStringMap")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLElement" , js_name = dataset ) ]
    ///Getter for the `dataset` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/dataset)
    ///
    ///*This API requires the following crate features to be activated: `DomStringMap`, `HtmlElement`*
    pub fn dataset(this: &HtmlElement) -> DomStringMap;

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLElement" , js_name = innerText ) ]
    ///Getter for the `innerText` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/innerText)
    ///
    ///*This API requires the following crate features to be activated: `HtmlElement`*
    pub fn inner_text(this: &HtmlElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLElement" , js_name = innerText ) ]
    ///Setter for the `innerText` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/innerText)
    ///
    ///*This API requires the following crate features to be activated: `HtmlElement`*
    pub fn set_inner_text(this: &HtmlElement, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLElement" , js_name = hidden ) ]
    ///Getter for the `hidden` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/hidden)
    ///
    ///*This API requires the following crate features to be activated: `HtmlElement`*
    pub fn hidden(this: &HtmlElement) -> bool;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLElement" , js_name = hidden ) ]
    ///Setter for the `hidden` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/hidden)
    ///
    ///*This API requires the following crate features to be activated: `HtmlElement`*
    pub fn set_hidden(this: &HtmlElement, value: bool);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLElement" , js_name = tabIndex ) ]
    ///Getter for the `tabIndex` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/tabIndex)
    ///
    ///*This API requires the following crate features to be activated: `HtmlElement`*
    pub fn tab_index(this: &HtmlElement) -> i32;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLElement" , js_name = tabIndex ) ]
    ///Setter for the `tabIndex` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/tabIndex)
    ///
    ///*This API requires the following crate features to be activated: `HtmlElement`*
    pub fn set_tab_index(this: &HtmlElement, value: i32);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLElement" , js_name = accessKey ) ]
    ///Getter for the `accessKey` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/accessKey)
    ///
    ///*This API requires the following crate features to be activated: `HtmlElement`*
    pub fn access_key(this: &HtmlElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLElement" , js_name = accessKey ) ]
    ///Setter for the `accessKey` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/accessKey)
    ///
    ///*This API requires the following crate features to be activated: `HtmlElement`*
    pub fn set_access_key(this: &HtmlElement, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLElement" , js_name = accessKeyLabel ) ]
    ///Getter for the `accessKeyLabel` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/accessKeyLabel)
    ///
    ///*This API requires the following crate features to be activated: `HtmlElement`*
    pub fn access_key_label(this: &HtmlElement) -> String;

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLElement" , js_name = draggable ) ]
    ///Getter for the `draggable` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/draggable)
    ///
    ///*This API requires the following crate features to be activated: `HtmlElement`*
    pub fn draggable(this: &HtmlElement) -> bool;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLElement" , js_name = draggable ) ]
    ///Setter for the `draggable` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/draggable)
    ///
    ///*This API requires the following crate features to be activated: `HtmlElement`*
    pub fn set_draggable(this: &HtmlElement, value: bool);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLElement" , js_name = contentEditable ) ]
    ///Getter for the `contentEditable` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/contentEditable)
    ///
    ///*This API requires the following crate features to be activated: `HtmlElement`*
    pub fn content_editable(this: &HtmlElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLElement" , js_name = contentEditable ) ]
    ///Setter for the `contentEditable` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/contentEditable)
    ///
    ///*This API requires the following crate features to be activated: `HtmlElement`*
    pub fn set_content_editable(this: &HtmlElement, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLElement" , js_name = isContentEditable ) ]
    ///Getter for the `isContentEditable` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/isContentEditable)
    ///
    ///*This API requires the following crate features to be activated: `HtmlElement`*
    pub fn is_content_editable(this: &HtmlElement) -> bool;

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLElement" , js_name = spellcheck ) ]
    ///Getter for the `spellcheck` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/spellcheck)
    ///
    ///*This API requires the following crate features to be activated: `HtmlElement`*
    pub fn spellcheck(this: &HtmlElement) -> bool;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLElement" , js_name = spellcheck ) ]
    ///Setter for the `spellcheck` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/spellcheck)
    ///
    ///*This API requires the following crate features to be activated: `HtmlElement`*
    pub fn set_spellcheck(this: &HtmlElement, value: bool);

    #[cfg(feature = "CssStyleDeclaration")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLElement" , js_name = style ) ]
    ///Getter for the `style` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/style)
    ///
    ///*This API requires the following crate features to be activated: `CssStyleDeclaration`, `HtmlElement`*
    pub fn style(this: &HtmlElement) -> CssStyleDeclaration;

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLElement" , js_name = offsetParent ) ]
    ///Getter for the `offsetParent` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/offsetParent)
    ///
    ///*This API requires the following crate features to be activated: `HtmlElement`*
    pub fn offset_parent(this: &HtmlElement) -> Option<Element>;

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLElement" , js_name = offsetTop ) ]
    ///Getter for the `offsetTop` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/offsetTop)
    ///
    ///*This API requires the following crate features to be activated: `HtmlElement`*
    pub fn offset_top(this: &HtmlElement) -> i32;

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLElement" , js_name = offsetLeft ) ]
    ///Getter for the `offsetLeft` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/offsetLeft)
    ///
    ///*This API requires the following crate features to be activated: `HtmlElement`*
    pub fn offset_left(this: &HtmlElement) -> i32;

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLElement" , js_name = offsetWidth ) ]
    ///Getter for the `offsetWidth` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/offsetWidth)
    ///
    ///*This API requires the following crate features to be activated: `HtmlElement`*
    pub fn offset_width(this: &HtmlElement) -> i32;

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLElement" , js_name = offsetHeight ) ]
    ///Getter for the `offsetHeight` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/offsetHeight)
    ///
    ///*This API requires the following crate features to be activated: `HtmlElement`*
    pub fn offset_height(this: &HtmlElement) -> i32;

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLElement" , js_name = oncopy ) ]
    ///Getter for the `oncopy` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/oncopy)
    ///
    ///*This API requires the following crate features to be activated: `HtmlElement`*
    pub fn oncopy(this: &HtmlElement) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLElement" , js_name = oncopy ) ]
    ///Setter for the `oncopy` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/oncopy)
    ///
    ///*This API requires the following crate features to be activated: `HtmlElement`*
    pub fn set_oncopy(this: &HtmlElement, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLElement" , js_name = oncut ) ]
    ///Getter for the `oncut` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/oncut)
    ///
    ///*This API requires the following crate features to be activated: `HtmlElement`*
    pub fn oncut(this: &HtmlElement) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLElement" , js_name = oncut ) ]
    ///Setter for the `oncut` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/oncut)
    ///
    ///*This API requires the following crate features to be activated: `HtmlElement`*
    pub fn set_oncut(this: &HtmlElement, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLElement" , js_name = onpaste ) ]
    ///Getter for the `onpaste` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onpaste)
    ///
    ///*This API requires the following crate features to be activated: `HtmlElement`*
    pub fn onpaste(this: &HtmlElement) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLElement" , js_name = onpaste ) ]
    ///Setter for the `onpaste` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onpaste)
    ///
    ///*This API requires the following crate features to be activated: `HtmlElement`*
    pub fn set_onpaste(this: &HtmlElement, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLElement" , js_name = onabort ) ]
    ///Getter for the `onabort` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onabort)
    ///
    ///*This API requires the following crate features to be activated: `HtmlElement`*
    pub fn onabort(this: &HtmlElement) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLElement" , js_name = onabort ) ]
    ///Setter for the `onabort` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onabort)
    ///
    ///*This API requires the following crate features to be activated: `HtmlElement`*
    pub fn set_onabort(this: &HtmlElement, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLElement" , js_name = onblur ) ]
    ///Getter for the `onblur` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onblur)
    ///
    ///*This API requires the following crate features to be activated: `HtmlElement`*
    pub fn onblur(this: &HtmlElement) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLElement" , js_name = onblur ) ]
    ///Setter for the `onblur` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onblur)
    ///
    ///*This API requires the following crate features to be activated: `HtmlElement`*
    pub fn set_onblur(this: &HtmlElement, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLElement" , js_name = onfocus ) ]
    ///Getter for the `onfocus` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onfocus)
    ///
    ///*This API requires the following crate features to be activated: `HtmlElement`*
    pub fn onfocus(this: &HtmlElement) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLElement" , js_name = onfocus ) ]
    ///Setter for the `onfocus` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onfocus)
    ///
    ///*This API requires the following crate features to be activated: `HtmlElement`*
    pub fn set_onfocus(this: &HtmlElement, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLElement" , js_name = onauxclick ) ]
    ///Getter for the `onauxclick` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onauxclick)
    ///
    ///*This API requires the following crate features to be activated: `HtmlElement`*
    pub fn onauxclick(this: &HtmlElement) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLElement" , js_name = onauxclick ) ]
    ///Setter for the `onauxclick` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onauxclick)
    ///
    ///*This API requires the following crate features to be activated: `HtmlElement`*
    pub fn set_onauxclick(this: &HtmlElement, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLElement" , js_name = oncanplay ) ]
    ///Getter for the `oncanplay` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/oncanplay)
    ///
    ///*This API requires the following crate features to be activated: `HtmlElement`*
    pub fn oncanplay(this: &HtmlElement) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLElement" , js_name = oncanplay ) ]
    ///Setter for the `oncanplay` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/oncanplay)
    ///
    ///*This API requires the following crate features to be activated: `HtmlElement`*
    pub fn set_oncanplay(this: &HtmlElement, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLElement" , js_name = oncanplaythrough ) ]
    ///Getter for the `oncanplaythrough` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/oncanplaythrough)
    ///
    ///*This API requires the following crate features to be activated: `HtmlElement`*
    pub fn oncanplaythrough(this: &HtmlElement) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLElement" , js_name = oncanplaythrough ) ]
    ///Setter for the `oncanplaythrough` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/oncanplaythrough)
    ///
    ///*This API requires the following crate features to be activated: `HtmlElement`*
    pub fn set_oncanplaythrough(this: &HtmlElement, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLElement" , js_name = onchange ) ]
    ///Getter for the `onchange` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onchange)
    ///
    ///*This API requires the following crate features to be activated: `HtmlElement`*
    pub fn onchange(this: &HtmlElement) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLElement" , js_name = onchange ) ]
    ///Setter for the `onchange` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onchange)
    ///
    ///*This API requires the following crate features to be activated: `HtmlElement`*
    pub fn set_onchange(this: &HtmlElement, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLElement" , js_name = onclick ) ]
    ///Getter for the `onclick` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onclick)
    ///
    ///*This API requires the following crate features to be activated: `HtmlElement`*
    pub fn onclick(this: &HtmlElement) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLElement" , js_name = onclick ) ]
    ///Setter for the `onclick` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onclick)
    ///
    ///*This API requires the following crate features to be activated: `HtmlElement`*
    pub fn set_onclick(this: &HtmlElement, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLElement" , js_name = onclose ) ]
    ///Getter for the `onclose` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onclose)
    ///
    ///*This API requires the following crate features to be activated: `HtmlElement`*
    pub fn onclose(this: &HtmlElement) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLElement" , js_name = onclose ) ]
    ///Setter for the `onclose` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onclose)
    ///
    ///*This API requires the following crate features to be activated: `HtmlElement`*
    pub fn set_onclose(this: &HtmlElement, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLElement" , js_name = oncontextmenu ) ]
    ///Getter for the `oncontextmenu` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/oncontextmenu)
    ///
    ///*This API requires the following crate features to be activated: `HtmlElement`*
    pub fn oncontextmenu(this: &HtmlElement) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLElement" , js_name = oncontextmenu ) ]
    ///Setter for the `oncontextmenu` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/oncontextmenu)
    ///
    ///*This API requires the following crate features to be activated: `HtmlElement`*
    pub fn set_oncontextmenu(this: &HtmlElement, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLElement" , js_name = ondblclick ) ]
    ///Getter for the `ondblclick` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/ondblclick)
    ///
    ///*This API requires the following crate features to be activated: `HtmlElement`*
    pub fn ondblclick(this: &HtmlElement) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLElement" , js_name = ondblclick ) ]
    ///Setter for the `ondblclick` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/ondblclick)
    ///
    ///*This API requires the following crate features to be activated: `HtmlElement`*
    pub fn set_ondblclick(this: &HtmlElement, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLElement" , js_name = ondrag ) ]
    ///Getter for the `ondrag` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/ondrag)
    ///
    ///*This API requires the following crate features to be activated: `HtmlElement`*
    pub fn ondrag(this: &HtmlElement) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLElement" , js_name = ondrag ) ]
    ///Setter for the `ondrag` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/ondrag)
    ///
    ///*This API requires the following crate features to be activated: `HtmlElement`*
    pub fn set_ondrag(this: &HtmlElement, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLElement" , js_name = ondragend ) ]
    ///Getter for the `ondragend` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/ondragend)
    ///
    ///*This API requires the following crate features to be activated: `HtmlElement`*
    pub fn ondragend(this: &HtmlElement) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLElement" , js_name = ondragend ) ]
    ///Setter for the `ondragend` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/ondragend)
    ///
    ///*This API requires the following crate features to be activated: `HtmlElement`*
    pub fn set_ondragend(this: &HtmlElement, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLElement" , js_name = ondragenter ) ]
    ///Getter for the `ondragenter` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/ondragenter)
    ///
    ///*This API requires the following crate features to be activated: `HtmlElement`*
    pub fn ondragenter(this: &HtmlElement) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLElement" , js_name = ondragenter ) ]
    ///Setter for the `ondragenter` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/ondragenter)
    ///
    ///*This API requires the following crate features to be activated: `HtmlElement`*
    pub fn set_ondragenter(this: &HtmlElement, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLElement" , js_name = ondragexit ) ]
    ///Getter for the `ondragexit` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/ondragexit)
    ///
    ///*This API requires the following crate features to be activated: `HtmlElement`*
    pub fn ondragexit(this: &HtmlElement) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLElement" , js_name = ondragexit ) ]
    ///Setter for the `ondragexit` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/ondragexit)
    ///
    ///*This API requires the following crate features to be activated: `HtmlElement`*
    pub fn set_ondragexit(this: &HtmlElement, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLElement" , js_name = ondragleave ) ]
    ///Getter for the `ondragleave` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/ondragleave)
    ///
    ///*This API requires the following crate features to be activated: `HtmlElement`*
    pub fn ondragleave(this: &HtmlElement) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLElement" , js_name = ondragleave ) ]
    ///Setter for the `ondragleave` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/ondragleave)
    ///
    ///*This API requires the following crate features to be activated: `HtmlElement`*
    pub fn set_ondragleave(this: &HtmlElement, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLElement" , js_name = ondragover ) ]
    ///Getter for the `ondragover` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/ondragover)
    ///
    ///*This API requires the following crate features to be activated: `HtmlElement`*
    pub fn ondragover(this: &HtmlElement) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLElement" , js_name = ondragover ) ]
    ///Setter for the `ondragover` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/ondragover)
    ///
    ///*This API requires the following crate features to be activated: `HtmlElement`*
    pub fn set_ondragover(this: &HtmlElement, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLElement" , js_name = ondragstart ) ]
    ///Getter for the `ondragstart` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/ondragstart)
    ///
    ///*This API requires the following crate features to be activated: `HtmlElement`*
    pub fn ondragstart(this: &HtmlElement) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLElement" , js_name = ondragstart ) ]
    ///Setter for the `ondragstart` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/ondragstart)
    ///
    ///*This API requires the following crate features to be activated: `HtmlElement`*
    pub fn set_ondragstart(this: &HtmlElement, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLElement" , js_name = ondrop ) ]
    ///Getter for the `ondrop` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/ondrop)
    ///
    ///*This API requires the following crate features to be activated: `HtmlElement`*
    pub fn ondrop(this: &HtmlElement) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLElement" , js_name = ondrop ) ]
    ///Setter for the `ondrop` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/ondrop)
    ///
    ///*This API requires the following crate features to be activated: `HtmlElement`*
    pub fn set_ondrop(this: &HtmlElement, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLElement" , js_name = ondurationchange ) ]
    ///Getter for the `ondurationchange` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/ondurationchange)
    ///
    ///*This API requires the following crate features to be activated: `HtmlElement`*
    pub fn ondurationchange(this: &HtmlElement) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLElement" , js_name = ondurationchange ) ]
    ///Setter for the `ondurationchange` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/ondurationchange)
    ///
    ///*This API requires the following crate features to be activated: `HtmlElement`*
    pub fn set_ondurationchange(this: &HtmlElement, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLElement" , js_name = onemptied ) ]
    ///Getter for the `onemptied` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onemptied)
    ///
    ///*This API requires the following crate features to be activated: `HtmlElement`*
    pub fn onemptied(this: &HtmlElement) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLElement" , js_name = onemptied ) ]
    ///Setter for the `onemptied` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onemptied)
    ///
    ///*This API requires the following crate features to be activated: `HtmlElement`*
    pub fn set_onemptied(this: &HtmlElement, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLElement" , js_name = onended ) ]
    ///Getter for the `onended` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onended)
    ///
    ///*This API requires the following crate features to be activated: `HtmlElement`*
    pub fn onended(this: &HtmlElement) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLElement" , js_name = onended ) ]
    ///Setter for the `onended` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onended)
    ///
    ///*This API requires the following crate features to be activated: `HtmlElement`*
    pub fn set_onended(this: &HtmlElement, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLElement" , js_name = oninput ) ]
    ///Getter for the `oninput` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/oninput)
    ///
    ///*This API requires the following crate features to be activated: `HtmlElement`*
    pub fn oninput(this: &HtmlElement) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLElement" , js_name = oninput ) ]
    ///Setter for the `oninput` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/oninput)
    ///
    ///*This API requires the following crate features to be activated: `HtmlElement`*
    pub fn set_oninput(this: &HtmlElement, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLElement" , js_name = oninvalid ) ]
    ///Getter for the `oninvalid` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/oninvalid)
    ///
    ///*This API requires the following crate features to be activated: `HtmlElement`*
    pub fn oninvalid(this: &HtmlElement) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLElement" , js_name = oninvalid ) ]
    ///Setter for the `oninvalid` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/oninvalid)
    ///
    ///*This API requires the following crate features to be activated: `HtmlElement`*
    pub fn set_oninvalid(this: &HtmlElement, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLElement" , js_name = onkeydown ) ]
    ///Getter for the `onkeydown` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onkeydown)
    ///
    ///*This API requires the following crate features to be activated: `HtmlElement`*
    pub fn onkeydown(this: &HtmlElement) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLElement" , js_name = onkeydown ) ]
    ///Setter for the `onkeydown` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onkeydown)
    ///
    ///*This API requires the following crate features to be activated: `HtmlElement`*
    pub fn set_onkeydown(this: &HtmlElement, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLElement" , js_name = onkeypress ) ]
    ///Getter for the `onkeypress` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onkeypress)
    ///
    ///*This API requires the following crate features to be activated: `HtmlElement`*
    pub fn onkeypress(this: &HtmlElement) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLElement" , js_name = onkeypress ) ]
    ///Setter for the `onkeypress` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onkeypress)
    ///
    ///*This API requires the following crate features to be activated: `HtmlElement`*
    pub fn set_onkeypress(this: &HtmlElement, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLElement" , js_name = onkeyup ) ]
    ///Getter for the `onkeyup` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onkeyup)
    ///
    ///*This API requires the following crate features to be activated: `HtmlElement`*
    pub fn onkeyup(this: &HtmlElement) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLElement" , js_name = onkeyup ) ]
    ///Setter for the `onkeyup` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onkeyup)
    ///
    ///*This API requires the following crate features to be activated: `HtmlElement`*
    pub fn set_onkeyup(this: &HtmlElement, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLElement" , js_name = onload ) ]
    ///Getter for the `onload` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onload)
    ///
    ///*This API requires the following crate features to be activated: `HtmlElement`*
    pub fn onload(this: &HtmlElement) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLElement" , js_name = onload ) ]
    ///Setter for the `onload` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onload)
    ///
    ///*This API requires the following crate features to be activated: `HtmlElement`*
    pub fn set_onload(this: &HtmlElement, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLElement" , js_name = onloadeddata ) ]
    ///Getter for the `onloadeddata` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onloadeddata)
    ///
    ///*This API requires the following crate features to be activated: `HtmlElement`*
    pub fn onloadeddata(this: &HtmlElement) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLElement" , js_name = onloadeddata ) ]
    ///Setter for the `onloadeddata` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onloadeddata)
    ///
    ///*This API requires the following crate features to be activated: `HtmlElement`*
    pub fn set_onloadeddata(this: &HtmlElement, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLElement" , js_name = onloadedmetadata ) ]
    ///Getter for the `onloadedmetadata` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onloadedmetadata)
    ///
    ///*This API requires the following crate features to be activated: `HtmlElement`*
    pub fn onloadedmetadata(this: &HtmlElement) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLElement" , js_name = onloadedmetadata ) ]
    ///Setter for the `onloadedmetadata` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onloadedmetadata)
    ///
    ///*This API requires the following crate features to be activated: `HtmlElement`*
    pub fn set_onloadedmetadata(this: &HtmlElement, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLElement" , js_name = onloadend ) ]
    ///Getter for the `onloadend` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onloadend)
    ///
    ///*This API requires the following crate features to be activated: `HtmlElement`*
    pub fn onloadend(this: &HtmlElement) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLElement" , js_name = onloadend ) ]
    ///Setter for the `onloadend` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onloadend)
    ///
    ///*This API requires the following crate features to be activated: `HtmlElement`*
    pub fn set_onloadend(this: &HtmlElement, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLElement" , js_name = onloadstart ) ]
    ///Getter for the `onloadstart` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onloadstart)
    ///
    ///*This API requires the following crate features to be activated: `HtmlElement`*
    pub fn onloadstart(this: &HtmlElement) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLElement" , js_name = onloadstart ) ]
    ///Setter for the `onloadstart` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onloadstart)
    ///
    ///*This API requires the following crate features to be activated: `HtmlElement`*
    pub fn set_onloadstart(this: &HtmlElement, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLElement" , js_name = onmousedown ) ]
    ///Getter for the `onmousedown` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onmousedown)
    ///
    ///*This API requires the following crate features to be activated: `HtmlElement`*
    pub fn onmousedown(this: &HtmlElement) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLElement" , js_name = onmousedown ) ]
    ///Setter for the `onmousedown` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onmousedown)
    ///
    ///*This API requires the following crate features to be activated: `HtmlElement`*
    pub fn set_onmousedown(this: &HtmlElement, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLElement" , js_name = onmouseenter ) ]
    ///Getter for the `onmouseenter` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onmouseenter)
    ///
    ///*This API requires the following crate features to be activated: `HtmlElement`*
    pub fn onmouseenter(this: &HtmlElement) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLElement" , js_name = onmouseenter ) ]
    ///Setter for the `onmouseenter` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onmouseenter)
    ///
    ///*This API requires the following crate features to be activated: `HtmlElement`*
    pub fn set_onmouseenter(this: &HtmlElement, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLElement" , js_name = onmouseleave ) ]
    ///Getter for the `onmouseleave` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onmouseleave)
    ///
    ///*This API requires the following crate features to be activated: `HtmlElement`*
    pub fn onmouseleave(this: &HtmlElement) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLElement" , js_name = onmouseleave ) ]
    ///Setter for the `onmouseleave` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onmouseleave)
    ///
    ///*This API requires the following crate features to be activated: `HtmlElement`*
    pub fn set_onmouseleave(this: &HtmlElement, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLElement" , js_name = onmousemove ) ]
    ///Getter for the `onmousemove` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onmousemove)
    ///
    ///*This API requires the following crate features to be activated: `HtmlElement`*
    pub fn onmousemove(this: &HtmlElement) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLElement" , js_name = onmousemove ) ]
    ///Setter for the `onmousemove` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onmousemove)
    ///
    ///*This API requires the following crate features to be activated: `HtmlElement`*
    pub fn set_onmousemove(this: &HtmlElement, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLElement" , js_name = onmouseout ) ]
    ///Getter for the `onmouseout` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onmouseout)
    ///
    ///*This API requires the following crate features to be activated: `HtmlElement`*
    pub fn onmouseout(this: &HtmlElement) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLElement" , js_name = onmouseout ) ]
    ///Setter for the `onmouseout` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onmouseout)
    ///
    ///*This API requires the following crate features to be activated: `HtmlElement`*
    pub fn set_onmouseout(this: &HtmlElement, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLElement" , js_name = onmouseover ) ]
    ///Getter for the `onmouseover` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onmouseover)
    ///
    ///*This API requires the following crate features to be activated: `HtmlElement`*
    pub fn onmouseover(this: &HtmlElement) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLElement" , js_name = onmouseover ) ]
    ///Setter for the `onmouseover` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onmouseover)
    ///
    ///*This API requires the following crate features to be activated: `HtmlElement`*
    pub fn set_onmouseover(this: &HtmlElement, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLElement" , js_name = onmouseup ) ]
    ///Getter for the `onmouseup` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onmouseup)
    ///
    ///*This API requires the following crate features to be activated: `HtmlElement`*
    pub fn onmouseup(this: &HtmlElement) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLElement" , js_name = onmouseup ) ]
    ///Setter for the `onmouseup` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onmouseup)
    ///
    ///*This API requires the following crate features to be activated: `HtmlElement`*
    pub fn set_onmouseup(this: &HtmlElement, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLElement" , js_name = onwheel ) ]
    ///Getter for the `onwheel` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onwheel)
    ///
    ///*This API requires the following crate features to be activated: `HtmlElement`*
    pub fn onwheel(this: &HtmlElement) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLElement" , js_name = onwheel ) ]
    ///Setter for the `onwheel` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onwheel)
    ///
    ///*This API requires the following crate features to be activated: `HtmlElement`*
    pub fn set_onwheel(this: &HtmlElement, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLElement" , js_name = onpause ) ]
    ///Getter for the `onpause` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onpause)
    ///
    ///*This API requires the following crate features to be activated: `HtmlElement`*
    pub fn onpause(this: &HtmlElement) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLElement" , js_name = onpause ) ]
    ///Setter for the `onpause` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onpause)
    ///
    ///*This API requires the following crate features to be activated: `HtmlElement`*
    pub fn set_onpause(this: &HtmlElement, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLElement" , js_name = onplay ) ]
    ///Getter for the `onplay` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onplay)
    ///
    ///*This API requires the following crate features to be activated: `HtmlElement`*
    pub fn onplay(this: &HtmlElement) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLElement" , js_name = onplay ) ]
    ///Setter for the `onplay` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onplay)
    ///
    ///*This API requires the following crate features to be activated: `HtmlElement`*
    pub fn set_onplay(this: &HtmlElement, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLElement" , js_name = onplaying ) ]
    ///Getter for the `onplaying` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onplaying)
    ///
    ///*This API requires the following crate features to be activated: `HtmlElement`*
    pub fn onplaying(this: &HtmlElement) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLElement" , js_name = onplaying ) ]
    ///Setter for the `onplaying` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onplaying)
    ///
    ///*This API requires the following crate features to be activated: `HtmlElement`*
    pub fn set_onplaying(this: &HtmlElement, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLElement" , js_name = onprogress ) ]
    ///Getter for the `onprogress` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onprogress)
    ///
    ///*This API requires the following crate features to be activated: `HtmlElement`*
    pub fn onprogress(this: &HtmlElement) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLElement" , js_name = onprogress ) ]
    ///Setter for the `onprogress` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onprogress)
    ///
    ///*This API requires the following crate features to be activated: `HtmlElement`*
    pub fn set_onprogress(this: &HtmlElement, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLElement" , js_name = onratechange ) ]
    ///Getter for the `onratechange` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onratechange)
    ///
    ///*This API requires the following crate features to be activated: `HtmlElement`*
    pub fn onratechange(this: &HtmlElement) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLElement" , js_name = onratechange ) ]
    ///Setter for the `onratechange` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onratechange)
    ///
    ///*This API requires the following crate features to be activated: `HtmlElement`*
    pub fn set_onratechange(this: &HtmlElement, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLElement" , js_name = onreset ) ]
    ///Getter for the `onreset` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onreset)
    ///
    ///*This API requires the following crate features to be activated: `HtmlElement`*
    pub fn onreset(this: &HtmlElement) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLElement" , js_name = onreset ) ]
    ///Setter for the `onreset` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onreset)
    ///
    ///*This API requires the following crate features to be activated: `HtmlElement`*
    pub fn set_onreset(this: &HtmlElement, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLElement" , js_name = onresize ) ]
    ///Getter for the `onresize` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onresize)
    ///
    ///*This API requires the following crate features to be activated: `HtmlElement`*
    pub fn onresize(this: &HtmlElement) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLElement" , js_name = onresize ) ]
    ///Setter for the `onresize` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onresize)
    ///
    ///*This API requires the following crate features to be activated: `HtmlElement`*
    pub fn set_onresize(this: &HtmlElement, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLElement" , js_name = onscroll ) ]
    ///Getter for the `onscroll` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onscroll)
    ///
    ///*This API requires the following crate features to be activated: `HtmlElement`*
    pub fn onscroll(this: &HtmlElement) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLElement" , js_name = onscroll ) ]
    ///Setter for the `onscroll` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onscroll)
    ///
    ///*This API requires the following crate features to be activated: `HtmlElement`*
    pub fn set_onscroll(this: &HtmlElement, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLElement" , js_name = onseeked ) ]
    ///Getter for the `onseeked` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onseeked)
    ///
    ///*This API requires the following crate features to be activated: `HtmlElement`*
    pub fn onseeked(this: &HtmlElement) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLElement" , js_name = onseeked ) ]
    ///Setter for the `onseeked` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onseeked)
    ///
    ///*This API requires the following crate features to be activated: `HtmlElement`*
    pub fn set_onseeked(this: &HtmlElement, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLElement" , js_name = onseeking ) ]
    ///Getter for the `onseeking` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onseeking)
    ///
    ///*This API requires the following crate features to be activated: `HtmlElement`*
    pub fn onseeking(this: &HtmlElement) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLElement" , js_name = onseeking ) ]
    ///Setter for the `onseeking` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onseeking)
    ///
    ///*This API requires the following crate features to be activated: `HtmlElement`*
    pub fn set_onseeking(this: &HtmlElement, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLElement" , js_name = onselect ) ]
    ///Getter for the `onselect` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onselect)
    ///
    ///*This API requires the following crate features to be activated: `HtmlElement`*
    pub fn onselect(this: &HtmlElement) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLElement" , js_name = onselect ) ]
    ///Setter for the `onselect` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onselect)
    ///
    ///*This API requires the following crate features to be activated: `HtmlElement`*
    pub fn set_onselect(this: &HtmlElement, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLElement" , js_name = onshow ) ]
    ///Getter for the `onshow` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onshow)
    ///
    ///*This API requires the following crate features to be activated: `HtmlElement`*
    pub fn onshow(this: &HtmlElement) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLElement" , js_name = onshow ) ]
    ///Setter for the `onshow` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onshow)
    ///
    ///*This API requires the following crate features to be activated: `HtmlElement`*
    pub fn set_onshow(this: &HtmlElement, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLElement" , js_name = onstalled ) ]
    ///Getter for the `onstalled` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onstalled)
    ///
    ///*This API requires the following crate features to be activated: `HtmlElement`*
    pub fn onstalled(this: &HtmlElement) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLElement" , js_name = onstalled ) ]
    ///Setter for the `onstalled` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onstalled)
    ///
    ///*This API requires the following crate features to be activated: `HtmlElement`*
    pub fn set_onstalled(this: &HtmlElement, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLElement" , js_name = onsubmit ) ]
    ///Getter for the `onsubmit` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onsubmit)
    ///
    ///*This API requires the following crate features to be activated: `HtmlElement`*
    pub fn onsubmit(this: &HtmlElement) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLElement" , js_name = onsubmit ) ]
    ///Setter for the `onsubmit` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onsubmit)
    ///
    ///*This API requires the following crate features to be activated: `HtmlElement`*
    pub fn set_onsubmit(this: &HtmlElement, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLElement" , js_name = onsuspend ) ]
    ///Getter for the `onsuspend` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onsuspend)
    ///
    ///*This API requires the following crate features to be activated: `HtmlElement`*
    pub fn onsuspend(this: &HtmlElement) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLElement" , js_name = onsuspend ) ]
    ///Setter for the `onsuspend` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onsuspend)
    ///
    ///*This API requires the following crate features to be activated: `HtmlElement`*
    pub fn set_onsuspend(this: &HtmlElement, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLElement" , js_name = ontimeupdate ) ]
    ///Getter for the `ontimeupdate` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/ontimeupdate)
    ///
    ///*This API requires the following crate features to be activated: `HtmlElement`*
    pub fn ontimeupdate(this: &HtmlElement) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLElement" , js_name = ontimeupdate ) ]
    ///Setter for the `ontimeupdate` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/ontimeupdate)
    ///
    ///*This API requires the following crate features to be activated: `HtmlElement`*
    pub fn set_ontimeupdate(this: &HtmlElement, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLElement" , js_name = onvolumechange ) ]
    ///Getter for the `onvolumechange` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onvolumechange)
    ///
    ///*This API requires the following crate features to be activated: `HtmlElement`*
    pub fn onvolumechange(this: &HtmlElement) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLElement" , js_name = onvolumechange ) ]
    ///Setter for the `onvolumechange` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onvolumechange)
    ///
    ///*This API requires the following crate features to be activated: `HtmlElement`*
    pub fn set_onvolumechange(this: &HtmlElement, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLElement" , js_name = onwaiting ) ]
    ///Getter for the `onwaiting` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onwaiting)
    ///
    ///*This API requires the following crate features to be activated: `HtmlElement`*
    pub fn onwaiting(this: &HtmlElement) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLElement" , js_name = onwaiting ) ]
    ///Setter for the `onwaiting` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onwaiting)
    ///
    ///*This API requires the following crate features to be activated: `HtmlElement`*
    pub fn set_onwaiting(this: &HtmlElement, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLElement" , js_name = onselectstart ) ]
    ///Getter for the `onselectstart` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onselectstart)
    ///
    ///*This API requires the following crate features to be activated: `HtmlElement`*
    pub fn onselectstart(this: &HtmlElement) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLElement" , js_name = onselectstart ) ]
    ///Setter for the `onselectstart` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onselectstart)
    ///
    ///*This API requires the following crate features to be activated: `HtmlElement`*
    pub fn set_onselectstart(this: &HtmlElement, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLElement" , js_name = ontoggle ) ]
    ///Getter for the `ontoggle` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/ontoggle)
    ///
    ///*This API requires the following crate features to be activated: `HtmlElement`*
    pub fn ontoggle(this: &HtmlElement) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLElement" , js_name = ontoggle ) ]
    ///Setter for the `ontoggle` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/ontoggle)
    ///
    ///*This API requires the following crate features to be activated: `HtmlElement`*
    pub fn set_ontoggle(this: &HtmlElement, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLElement" , js_name = onpointercancel ) ]
    ///Getter for the `onpointercancel` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onpointercancel)
    ///
    ///*This API requires the following crate features to be activated: `HtmlElement`*
    pub fn onpointercancel(this: &HtmlElement) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLElement" , js_name = onpointercancel ) ]
    ///Setter for the `onpointercancel` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onpointercancel)
    ///
    ///*This API requires the following crate features to be activated: `HtmlElement`*
    pub fn set_onpointercancel(this: &HtmlElement, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLElement" , js_name = onpointerdown ) ]
    ///Getter for the `onpointerdown` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onpointerdown)
    ///
    ///*This API requires the following crate features to be activated: `HtmlElement`*
    pub fn onpointerdown(this: &HtmlElement) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLElement" , js_name = onpointerdown ) ]
    ///Setter for the `onpointerdown` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onpointerdown)
    ///
    ///*This API requires the following crate features to be activated: `HtmlElement`*
    pub fn set_onpointerdown(this: &HtmlElement, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLElement" , js_name = onpointerup ) ]
    ///Getter for the `onpointerup` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onpointerup)
    ///
    ///*This API requires the following crate features to be activated: `HtmlElement`*
    pub fn onpointerup(this: &HtmlElement) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLElement" , js_name = onpointerup ) ]
    ///Setter for the `onpointerup` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onpointerup)
    ///
    ///*This API requires the following crate features to be activated: `HtmlElement`*
    pub fn set_onpointerup(this: &HtmlElement, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLElement" , js_name = onpointermove ) ]
    ///Getter for the `onpointermove` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onpointermove)
    ///
    ///*This API requires the following crate features to be activated: `HtmlElement`*
    pub fn onpointermove(this: &HtmlElement) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLElement" , js_name = onpointermove ) ]
    ///Setter for the `onpointermove` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onpointermove)
    ///
    ///*This API requires the following crate features to be activated: `HtmlElement`*
    pub fn set_onpointermove(this: &HtmlElement, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLElement" , js_name = onpointerout ) ]
    ///Getter for the `onpointerout` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onpointerout)
    ///
    ///*This API requires the following crate features to be activated: `HtmlElement`*
    pub fn onpointerout(this: &HtmlElement) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLElement" , js_name = onpointerout ) ]
    ///Setter for the `onpointerout` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onpointerout)
    ///
    ///*This API requires the following crate features to be activated: `HtmlElement`*
    pub fn set_onpointerout(this: &HtmlElement, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLElement" , js_name = onpointerover ) ]
    ///Getter for the `onpointerover` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onpointerover)
    ///
    ///*This API requires the following crate features to be activated: `HtmlElement`*
    pub fn onpointerover(this: &HtmlElement) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLElement" , js_name = onpointerover ) ]
    ///Setter for the `onpointerover` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onpointerover)
    ///
    ///*This API requires the following crate features to be activated: `HtmlElement`*
    pub fn set_onpointerover(this: &HtmlElement, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLElement" , js_name = onpointerenter ) ]
    ///Getter for the `onpointerenter` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onpointerenter)
    ///
    ///*This API requires the following crate features to be activated: `HtmlElement`*
    pub fn onpointerenter(this: &HtmlElement) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLElement" , js_name = onpointerenter ) ]
    ///Setter for the `onpointerenter` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onpointerenter)
    ///
    ///*This API requires the following crate features to be activated: `HtmlElement`*
    pub fn set_onpointerenter(this: &HtmlElement, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLElement" , js_name = onpointerleave ) ]
    ///Getter for the `onpointerleave` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onpointerleave)
    ///
    ///*This API requires the following crate features to be activated: `HtmlElement`*
    pub fn onpointerleave(this: &HtmlElement) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLElement" , js_name = onpointerleave ) ]
    ///Setter for the `onpointerleave` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onpointerleave)
    ///
    ///*This API requires the following crate features to be activated: `HtmlElement`*
    pub fn set_onpointerleave(this: &HtmlElement, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLElement" , js_name = ongotpointercapture ) ]
    ///Getter for the `ongotpointercapture` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/ongotpointercapture)
    ///
    ///*This API requires the following crate features to be activated: `HtmlElement`*
    pub fn ongotpointercapture(this: &HtmlElement) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLElement" , js_name = ongotpointercapture ) ]
    ///Setter for the `ongotpointercapture` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/ongotpointercapture)
    ///
    ///*This API requires the following crate features to be activated: `HtmlElement`*
    pub fn set_ongotpointercapture(this: &HtmlElement, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLElement" , js_name = onlostpointercapture ) ]
    ///Getter for the `onlostpointercapture` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onlostpointercapture)
    ///
    ///*This API requires the following crate features to be activated: `HtmlElement`*
    pub fn onlostpointercapture(this: &HtmlElement) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLElement" , js_name = onlostpointercapture ) ]
    ///Setter for the `onlostpointercapture` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onlostpointercapture)
    ///
    ///*This API requires the following crate features to be activated: `HtmlElement`*
    pub fn set_onlostpointercapture(this: &HtmlElement, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLElement" , js_name = onanimationcancel ) ]
    ///Getter for the `onanimationcancel` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onanimationcancel)
    ///
    ///*This API requires the following crate features to be activated: `HtmlElement`*
    pub fn onanimationcancel(this: &HtmlElement) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLElement" , js_name = onanimationcancel ) ]
    ///Setter for the `onanimationcancel` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onanimationcancel)
    ///
    ///*This API requires the following crate features to be activated: `HtmlElement`*
    pub fn set_onanimationcancel(this: &HtmlElement, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLElement" , js_name = onanimationend ) ]
    ///Getter for the `onanimationend` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onanimationend)
    ///
    ///*This API requires the following crate features to be activated: `HtmlElement`*
    pub fn onanimationend(this: &HtmlElement) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLElement" , js_name = onanimationend ) ]
    ///Setter for the `onanimationend` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onanimationend)
    ///
    ///*This API requires the following crate features to be activated: `HtmlElement`*
    pub fn set_onanimationend(this: &HtmlElement, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLElement" , js_name = onanimationiteration ) ]
    ///Getter for the `onanimationiteration` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onanimationiteration)
    ///
    ///*This API requires the following crate features to be activated: `HtmlElement`*
    pub fn onanimationiteration(this: &HtmlElement) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLElement" , js_name = onanimationiteration ) ]
    ///Setter for the `onanimationiteration` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onanimationiteration)
    ///
    ///*This API requires the following crate features to be activated: `HtmlElement`*
    pub fn set_onanimationiteration(this: &HtmlElement, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLElement" , js_name = onanimationstart ) ]
    ///Getter for the `onanimationstart` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onanimationstart)
    ///
    ///*This API requires the following crate features to be activated: `HtmlElement`*
    pub fn onanimationstart(this: &HtmlElement) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLElement" , js_name = onanimationstart ) ]
    ///Setter for the `onanimationstart` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onanimationstart)
    ///
    ///*This API requires the following crate features to be activated: `HtmlElement`*
    pub fn set_onanimationstart(this: &HtmlElement, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLElement" , js_name = ontransitioncancel ) ]
    ///Getter for the `ontransitioncancel` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/ontransitioncancel)
    ///
    ///*This API requires the following crate features to be activated: `HtmlElement`*
    pub fn ontransitioncancel(this: &HtmlElement) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLElement" , js_name = ontransitioncancel ) ]
    ///Setter for the `ontransitioncancel` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/ontransitioncancel)
    ///
    ///*This API requires the following crate features to be activated: `HtmlElement`*
    pub fn set_ontransitioncancel(this: &HtmlElement, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLElement" , js_name = ontransitionend ) ]
    ///Getter for the `ontransitionend` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/ontransitionend)
    ///
    ///*This API requires the following crate features to be activated: `HtmlElement`*
    pub fn ontransitionend(this: &HtmlElement) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLElement" , js_name = ontransitionend ) ]
    ///Setter for the `ontransitionend` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/ontransitionend)
    ///
    ///*This API requires the following crate features to be activated: `HtmlElement`*
    pub fn set_ontransitionend(this: &HtmlElement, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLElement" , js_name = ontransitionrun ) ]
    ///Getter for the `ontransitionrun` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/ontransitionrun)
    ///
    ///*This API requires the following crate features to be activated: `HtmlElement`*
    pub fn ontransitionrun(this: &HtmlElement) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLElement" , js_name = ontransitionrun ) ]
    ///Setter for the `ontransitionrun` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/ontransitionrun)
    ///
    ///*This API requires the following crate features to be activated: `HtmlElement`*
    pub fn set_ontransitionrun(this: &HtmlElement, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLElement" , js_name = ontransitionstart ) ]
    ///Getter for the `ontransitionstart` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/ontransitionstart)
    ///
    ///*This API requires the following crate features to be activated: `HtmlElement`*
    pub fn ontransitionstart(this: &HtmlElement) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLElement" , js_name = ontransitionstart ) ]
    ///Setter for the `ontransitionstart` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/ontransitionstart)
    ///
    ///*This API requires the following crate features to be activated: `HtmlElement`*
    pub fn set_ontransitionstart(this: &HtmlElement, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLElement" , js_name = onwebkitanimationend ) ]
    ///Getter for the `onwebkitanimationend` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onwebkitanimationend)
    ///
    ///*This API requires the following crate features to be activated: `HtmlElement`*
    pub fn onwebkitanimationend(this: &HtmlElement) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLElement" , js_name = onwebkitanimationend ) ]
    ///Setter for the `onwebkitanimationend` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onwebkitanimationend)
    ///
    ///*This API requires the following crate features to be activated: `HtmlElement`*
    pub fn set_onwebkitanimationend(this: &HtmlElement, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLElement" , js_name = onwebkitanimationiteration ) ]
    ///Getter for the `onwebkitanimationiteration` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onwebkitanimationiteration)
    ///
    ///*This API requires the following crate features to be activated: `HtmlElement`*
    pub fn onwebkitanimationiteration(this: &HtmlElement) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLElement" , js_name = onwebkitanimationiteration ) ]
    ///Setter for the `onwebkitanimationiteration` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onwebkitanimationiteration)
    ///
    ///*This API requires the following crate features to be activated: `HtmlElement`*
    pub fn set_onwebkitanimationiteration(this: &HtmlElement, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLElement" , js_name = onwebkitanimationstart ) ]
    ///Getter for the `onwebkitanimationstart` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onwebkitanimationstart)
    ///
    ///*This API requires the following crate features to be activated: `HtmlElement`*
    pub fn onwebkitanimationstart(this: &HtmlElement) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLElement" , js_name = onwebkitanimationstart ) ]
    ///Setter for the `onwebkitanimationstart` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onwebkitanimationstart)
    ///
    ///*This API requires the following crate features to be activated: `HtmlElement`*
    pub fn set_onwebkitanimationstart(this: &HtmlElement, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLElement" , js_name = onwebkittransitionend ) ]
    ///Getter for the `onwebkittransitionend` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onwebkittransitionend)
    ///
    ///*This API requires the following crate features to be activated: `HtmlElement`*
    pub fn onwebkittransitionend(this: &HtmlElement) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLElement" , js_name = onwebkittransitionend ) ]
    ///Setter for the `onwebkittransitionend` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onwebkittransitionend)
    ///
    ///*This API requires the following crate features to be activated: `HtmlElement`*
    pub fn set_onwebkittransitionend(this: &HtmlElement, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLElement" , js_name = onerror ) ]
    ///Getter for the `onerror` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onerror)
    ///
    ///*This API requires the following crate features to be activated: `HtmlElement`*
    pub fn onerror(this: &HtmlElement) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLElement" , js_name = onerror ) ]
    ///Setter for the `onerror` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onerror)
    ///
    ///*This API requires the following crate features to be activated: `HtmlElement`*
    pub fn set_onerror(this: &HtmlElement, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLElement" , js_name = ontouchstart ) ]
    ///Getter for the `ontouchstart` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/ontouchstart)
    ///
    ///*This API requires the following crate features to be activated: `HtmlElement`*
    pub fn ontouchstart(this: &HtmlElement) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLElement" , js_name = ontouchstart ) ]
    ///Setter for the `ontouchstart` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/ontouchstart)
    ///
    ///*This API requires the following crate features to be activated: `HtmlElement`*
    pub fn set_ontouchstart(this: &HtmlElement, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLElement" , js_name = ontouchend ) ]
    ///Getter for the `ontouchend` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/ontouchend)
    ///
    ///*This API requires the following crate features to be activated: `HtmlElement`*
    pub fn ontouchend(this: &HtmlElement) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLElement" , js_name = ontouchend ) ]
    ///Setter for the `ontouchend` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/ontouchend)
    ///
    ///*This API requires the following crate features to be activated: `HtmlElement`*
    pub fn set_ontouchend(this: &HtmlElement, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLElement" , js_name = ontouchmove ) ]
    ///Getter for the `ontouchmove` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/ontouchmove)
    ///
    ///*This API requires the following crate features to be activated: `HtmlElement`*
    pub fn ontouchmove(this: &HtmlElement) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLElement" , js_name = ontouchmove ) ]
    ///Setter for the `ontouchmove` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/ontouchmove)
    ///
    ///*This API requires the following crate features to be activated: `HtmlElement`*
    pub fn set_ontouchmove(this: &HtmlElement, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLElement" , js_name = ontouchcancel ) ]
    ///Getter for the `ontouchcancel` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/ontouchcancel)
    ///
    ///*This API requires the following crate features to be activated: `HtmlElement`*
    pub fn ontouchcancel(this: &HtmlElement) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLElement" , js_name = ontouchcancel ) ]
    ///Setter for the `ontouchcancel` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/ontouchcancel)
    ///
    ///*This API requires the following crate features to be activated: `HtmlElement`*
    pub fn set_ontouchcancel(this: &HtmlElement, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( catch , method , structural , js_class = "HTMLElement" , js_name = blur ) ]
    ///The `blur()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/blur)
    ///
    ///*This API requires the following crate features to be activated: `HtmlElement`*
    pub fn blur(this: &HtmlElement) -> Result<(), JsValue>;

    # [ wasm_bindgen ( method , structural , js_class = "HTMLElement" , js_name = click ) ]
    ///The `click()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/click)
    ///
    ///*This API requires the following crate features to be activated: `HtmlElement`*
    pub fn click(this: &HtmlElement);

    # [ wasm_bindgen ( catch , method , structural , js_class = "HTMLElement" , js_name = focus ) ]
    ///The `focus()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/focus)
    ///
    ///*This API requires the following crate features to be activated: `HtmlElement`*
    pub fn focus(this: &HtmlElement) -> Result<(), JsValue>;

}
