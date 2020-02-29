use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = Element , typescript_type = "Element" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `Element` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element)
    ///
    ///*This API requires the following crate features to be activated: `Element`*
    pub type Element;

    # [ wasm_bindgen ( structural , method , getter , js_class = "Element" , js_name = namespaceURI ) ]
    ///Getter for the `namespaceURI` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/namespaceURI)
    ///
    ///*This API requires the following crate features to be activated: `Element`*
    pub fn namespace_uri(this: &Element) -> Option<String>;

    # [ wasm_bindgen ( structural , method , getter , js_class = "Element" , js_name = prefix ) ]
    ///Getter for the `prefix` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/prefix)
    ///
    ///*This API requires the following crate features to be activated: `Element`*
    pub fn prefix(this: &Element) -> Option<String>;

    # [ wasm_bindgen ( structural , method , getter , js_class = "Element" , js_name = localName ) ]
    ///Getter for the `localName` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/localName)
    ///
    ///*This API requires the following crate features to be activated: `Element`*
    pub fn local_name(this: &Element) -> String;

    # [ wasm_bindgen ( structural , method , getter , js_class = "Element" , js_name = tagName ) ]
    ///Getter for the `tagName` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/tagName)
    ///
    ///*This API requires the following crate features to be activated: `Element`*
    pub fn tag_name(this: &Element) -> String;

    # [ wasm_bindgen ( structural , method , getter , js_class = "Element" , js_name = id ) ]
    ///Getter for the `id` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/id)
    ///
    ///*This API requires the following crate features to be activated: `Element`*
    pub fn id(this: &Element) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Element" , js_name = id ) ]
    ///Setter for the `id` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/id)
    ///
    ///*This API requires the following crate features to be activated: `Element`*
    pub fn set_id(this: &Element, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "Element" , js_name = className ) ]
    ///Getter for the `className` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/className)
    ///
    ///*This API requires the following crate features to be activated: `Element`*
    pub fn class_name(this: &Element) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Element" , js_name = className ) ]
    ///Setter for the `className` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/className)
    ///
    ///*This API requires the following crate features to be activated: `Element`*
    pub fn set_class_name(this: &Element, value: &str);

    #[cfg(feature = "DomTokenList")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "Element" , js_name = classList ) ]
    ///Getter for the `classList` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/classList)
    ///
    ///*This API requires the following crate features to be activated: `DomTokenList`, `Element`*
    pub fn class_list(this: &Element) -> DomTokenList;

    #[cfg(feature = "NamedNodeMap")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "Element" , js_name = attributes ) ]
    ///Getter for the `attributes` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/attributes)
    ///
    ///*This API requires the following crate features to be activated: `Element`, `NamedNodeMap`*
    pub fn attributes(this: &Element) -> NamedNodeMap;

    # [ wasm_bindgen ( structural , method , getter , js_class = "Element" , js_name = scrollTop ) ]
    ///Getter for the `scrollTop` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/scrollTop)
    ///
    ///*This API requires the following crate features to be activated: `Element`*
    pub fn scroll_top(this: &Element) -> i32;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Element" , js_name = scrollTop ) ]
    ///Setter for the `scrollTop` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/scrollTop)
    ///
    ///*This API requires the following crate features to be activated: `Element`*
    pub fn set_scroll_top(this: &Element, value: i32);

    # [ wasm_bindgen ( structural , method , getter , js_class = "Element" , js_name = scrollLeft ) ]
    ///Getter for the `scrollLeft` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/scrollLeft)
    ///
    ///*This API requires the following crate features to be activated: `Element`*
    pub fn scroll_left(this: &Element) -> i32;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Element" , js_name = scrollLeft ) ]
    ///Setter for the `scrollLeft` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/scrollLeft)
    ///
    ///*This API requires the following crate features to be activated: `Element`*
    pub fn set_scroll_left(this: &Element, value: i32);

    # [ wasm_bindgen ( structural , method , getter , js_class = "Element" , js_name = scrollWidth ) ]
    ///Getter for the `scrollWidth` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/scrollWidth)
    ///
    ///*This API requires the following crate features to be activated: `Element`*
    pub fn scroll_width(this: &Element) -> i32;

    # [ wasm_bindgen ( structural , method , getter , js_class = "Element" , js_name = scrollHeight ) ]
    ///Getter for the `scrollHeight` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/scrollHeight)
    ///
    ///*This API requires the following crate features to be activated: `Element`*
    pub fn scroll_height(this: &Element) -> i32;

    # [ wasm_bindgen ( structural , method , getter , js_class = "Element" , js_name = clientTop ) ]
    ///Getter for the `clientTop` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/clientTop)
    ///
    ///*This API requires the following crate features to be activated: `Element`*
    pub fn client_top(this: &Element) -> i32;

    # [ wasm_bindgen ( structural , method , getter , js_class = "Element" , js_name = clientLeft ) ]
    ///Getter for the `clientLeft` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/clientLeft)
    ///
    ///*This API requires the following crate features to be activated: `Element`*
    pub fn client_left(this: &Element) -> i32;

    # [ wasm_bindgen ( structural , method , getter , js_class = "Element" , js_name = clientWidth ) ]
    ///Getter for the `clientWidth` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/clientWidth)
    ///
    ///*This API requires the following crate features to be activated: `Element`*
    pub fn client_width(this: &Element) -> i32;

    # [ wasm_bindgen ( structural , method , getter , js_class = "Element" , js_name = clientHeight ) ]
    ///Getter for the `clientHeight` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/clientHeight)
    ///
    ///*This API requires the following crate features to be activated: `Element`*
    pub fn client_height(this: &Element) -> i32;

    # [ wasm_bindgen ( structural , method , getter , js_class = "Element" , js_name = innerHTML ) ]
    ///Getter for the `innerHTML` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/innerHTML)
    ///
    ///*This API requires the following crate features to be activated: `Element`*
    pub fn inner_html(this: &Element) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Element" , js_name = innerHTML ) ]
    ///Setter for the `innerHTML` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/innerHTML)
    ///
    ///*This API requires the following crate features to be activated: `Element`*
    pub fn set_inner_html(this: &Element, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "Element" , js_name = outerHTML ) ]
    ///Getter for the `outerHTML` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/outerHTML)
    ///
    ///*This API requires the following crate features to be activated: `Element`*
    pub fn outer_html(this: &Element) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Element" , js_name = outerHTML ) ]
    ///Setter for the `outerHTML` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/outerHTML)
    ///
    ///*This API requires the following crate features to be activated: `Element`*
    pub fn set_outer_html(this: &Element, value: &str);

    #[cfg(feature = "ShadowRoot")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "Element" , js_name = shadowRoot ) ]
    ///Getter for the `shadowRoot` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/shadowRoot)
    ///
    ///*This API requires the following crate features to be activated: `Element`, `ShadowRoot`*
    pub fn shadow_root(this: &Element) -> Option<ShadowRoot>;

    #[cfg(feature = "HtmlSlotElement")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "Element" , js_name = assignedSlot ) ]
    ///Getter for the `assignedSlot` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/assignedSlot)
    ///
    ///*This API requires the following crate features to be activated: `Element`, `HtmlSlotElement`*
    pub fn assigned_slot(this: &Element) -> Option<HtmlSlotElement>;

    # [ wasm_bindgen ( structural , method , getter , js_class = "Element" , js_name = slot ) ]
    ///Getter for the `slot` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/slot)
    ///
    ///*This API requires the following crate features to be activated: `Element`*
    pub fn slot(this: &Element) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Element" , js_name = slot ) ]
    ///Setter for the `slot` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/slot)
    ///
    ///*This API requires the following crate features to be activated: `Element`*
    pub fn set_slot(this: &Element, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "Element" , js_name = previousElementSibling ) ]
    ///Getter for the `previousElementSibling` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/previousElementSibling)
    ///
    ///*This API requires the following crate features to be activated: `Element`*
    pub fn previous_element_sibling(this: &Element) -> Option<Element>;

    # [ wasm_bindgen ( structural , method , getter , js_class = "Element" , js_name = nextElementSibling ) ]
    ///Getter for the `nextElementSibling` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/nextElementSibling)
    ///
    ///*This API requires the following crate features to be activated: `Element`*
    pub fn next_element_sibling(this: &Element) -> Option<Element>;

    #[cfg(feature = "HtmlCollection")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "Element" , js_name = children ) ]
    ///Getter for the `children` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/children)
    ///
    ///*This API requires the following crate features to be activated: `Element`, `HtmlCollection`*
    pub fn children(this: &Element) -> HtmlCollection;

    # [ wasm_bindgen ( structural , method , getter , js_class = "Element" , js_name = firstElementChild ) ]
    ///Getter for the `firstElementChild` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/firstElementChild)
    ///
    ///*This API requires the following crate features to be activated: `Element`*
    pub fn first_element_child(this: &Element) -> Option<Element>;

    # [ wasm_bindgen ( structural , method , getter , js_class = "Element" , js_name = lastElementChild ) ]
    ///Getter for the `lastElementChild` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/lastElementChild)
    ///
    ///*This API requires the following crate features to be activated: `Element`*
    pub fn last_element_child(this: &Element) -> Option<Element>;

    # [ wasm_bindgen ( structural , method , getter , js_class = "Element" , js_name = childElementCount ) ]
    ///Getter for the `childElementCount` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/childElementCount)
    ///
    ///*This API requires the following crate features to be activated: `Element`*
    pub fn child_element_count(this: &Element) -> u32;

    #[cfg(all(feature = "ShadowRoot", feature = "ShadowRootInit",))]
    # [ wasm_bindgen ( catch , method , structural , js_class = "Element" , js_name = attachShadow ) ]
    ///The `attachShadow()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/attachShadow)
    ///
    ///*This API requires the following crate features to be activated: `Element`, `ShadowRoot`, `ShadowRootInit`*
    pub fn attach_shadow(
        this: &Element,
        shadow_root_init_dict: &ShadowRootInit,
    ) -> Result<ShadowRoot, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Element" , js_name = closest ) ]
    ///The `closest()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/closest)
    ///
    ///*This API requires the following crate features to be activated: `Element`*
    pub fn closest(this: &Element, selector: &str) -> Result<Option<Element>, JsValue>;

    # [ wasm_bindgen ( method , structural , js_class = "Element" , js_name = getAttribute ) ]
    ///The `getAttribute()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/getAttribute)
    ///
    ///*This API requires the following crate features to be activated: `Element`*
    pub fn get_attribute(this: &Element, name: &str) -> Option<String>;

    # [ wasm_bindgen ( method , structural , js_class = "Element" , js_name = getAttributeNS ) ]
    ///The `getAttributeNS()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/getAttributeNS)
    ///
    ///*This API requires the following crate features to be activated: `Element`*
    pub fn get_attribute_ns(
        this: &Element,
        namespace: Option<&str>,
        local_name: &str,
    ) -> Option<String>;

    # [ wasm_bindgen ( method , structural , js_class = "Element" , js_name = getAttributeNames ) ]
    ///The `getAttributeNames()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/getAttributeNames)
    ///
    ///*This API requires the following crate features to be activated: `Element`*
    pub fn get_attribute_names(this: &Element) -> ::js_sys::Array;

    #[cfg(feature = "Attr")]
    # [ wasm_bindgen ( method , structural , js_class = "Element" , js_name = getAttributeNode ) ]
    ///The `getAttributeNode()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/getAttributeNode)
    ///
    ///*This API requires the following crate features to be activated: `Attr`, `Element`*
    pub fn get_attribute_node(this: &Element, name: &str) -> Option<Attr>;

    #[cfg(feature = "Attr")]
    # [ wasm_bindgen ( method , structural , js_class = "Element" , js_name = getAttributeNodeNS ) ]
    ///The `getAttributeNodeNS()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/getAttributeNodeNS)
    ///
    ///*This API requires the following crate features to be activated: `Attr`, `Element`*
    pub fn get_attribute_node_ns(
        this: &Element,
        namespace_uri: Option<&str>,
        local_name: &str,
    ) -> Option<Attr>;

    #[cfg(feature = "DomRect")]
    # [ wasm_bindgen ( method , structural , js_class = "Element" , js_name = getBoundingClientRect ) ]
    ///The `getBoundingClientRect()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/getBoundingClientRect)
    ///
    ///*This API requires the following crate features to be activated: `DomRect`, `Element`*
    pub fn get_bounding_client_rect(this: &Element) -> DomRect;

    #[cfg(feature = "DomRectList")]
    # [ wasm_bindgen ( method , structural , js_class = "Element" , js_name = getClientRects ) ]
    ///The `getClientRects()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/getClientRects)
    ///
    ///*This API requires the following crate features to be activated: `DomRectList`, `Element`*
    pub fn get_client_rects(this: &Element) -> DomRectList;

    #[cfg(feature = "HtmlCollection")]
    # [ wasm_bindgen ( method , structural , js_class = "Element" , js_name = getElementsByClassName ) ]
    ///The `getElementsByClassName()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/getElementsByClassName)
    ///
    ///*This API requires the following crate features to be activated: `Element`, `HtmlCollection`*
    pub fn get_elements_by_class_name(this: &Element, class_names: &str) -> HtmlCollection;

    #[cfg(feature = "HtmlCollection")]
    # [ wasm_bindgen ( method , structural , js_class = "Element" , js_name = getElementsByTagName ) ]
    ///The `getElementsByTagName()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/getElementsByTagName)
    ///
    ///*This API requires the following crate features to be activated: `Element`, `HtmlCollection`*
    pub fn get_elements_by_tag_name(this: &Element, local_name: &str) -> HtmlCollection;

    #[cfg(feature = "HtmlCollection")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "Element" , js_name = getElementsByTagNameNS ) ]
    ///The `getElementsByTagNameNS()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/getElementsByTagNameNS)
    ///
    ///*This API requires the following crate features to be activated: `Element`, `HtmlCollection`*
    pub fn get_elements_by_tag_name_ns(
        this: &Element,
        namespace: Option<&str>,
        local_name: &str,
    ) -> Result<HtmlCollection, JsValue>;

    # [ wasm_bindgen ( method , structural , js_class = "Element" , js_name = hasAttribute ) ]
    ///The `hasAttribute()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/hasAttribute)
    ///
    ///*This API requires the following crate features to be activated: `Element`*
    pub fn has_attribute(this: &Element, name: &str) -> bool;

    # [ wasm_bindgen ( method , structural , js_class = "Element" , js_name = hasAttributeNS ) ]
    ///The `hasAttributeNS()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/hasAttributeNS)
    ///
    ///*This API requires the following crate features to be activated: `Element`*
    pub fn has_attribute_ns(this: &Element, namespace: Option<&str>, local_name: &str) -> bool;

    # [ wasm_bindgen ( method , structural , js_class = "Element" , js_name = hasAttributes ) ]
    ///The `hasAttributes()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/hasAttributes)
    ///
    ///*This API requires the following crate features to be activated: `Element`*
    pub fn has_attributes(this: &Element) -> bool;

    # [ wasm_bindgen ( method , structural , js_class = "Element" , js_name = hasPointerCapture ) ]
    ///The `hasPointerCapture()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/hasPointerCapture)
    ///
    ///*This API requires the following crate features to be activated: `Element`*
    pub fn has_pointer_capture(this: &Element, pointer_id: i32) -> bool;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Element" , js_name = insertAdjacentElement ) ]
    ///The `insertAdjacentElement()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/insertAdjacentElement)
    ///
    ///*This API requires the following crate features to be activated: `Element`*
    pub fn insert_adjacent_element(
        this: &Element,
        where_: &str,
        element: &Element,
    ) -> Result<Option<Element>, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Element" , js_name = insertAdjacentHTML ) ]
    ///The `insertAdjacentHTML()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/insertAdjacentHTML)
    ///
    ///*This API requires the following crate features to be activated: `Element`*
    pub fn insert_adjacent_html(this: &Element, position: &str, text: &str) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Element" , js_name = insertAdjacentText ) ]
    ///The `insertAdjacentText()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/insertAdjacentText)
    ///
    ///*This API requires the following crate features to be activated: `Element`*
    pub fn insert_adjacent_text(this: &Element, where_: &str, data: &str) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Element" , js_name = matches ) ]
    ///The `matches()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/matches)
    ///
    ///*This API requires the following crate features to be activated: `Element`*
    pub fn matches(this: &Element, selector: &str) -> Result<bool, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Element" , js_name = querySelector ) ]
    ///The `querySelector()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/querySelector)
    ///
    ///*This API requires the following crate features to be activated: `Element`*
    pub fn query_selector(this: &Element, selectors: &str) -> Result<Option<Element>, JsValue>;

    #[cfg(feature = "NodeList")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "Element" , js_name = querySelectorAll ) ]
    ///The `querySelectorAll()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/querySelectorAll)
    ///
    ///*This API requires the following crate features to be activated: `Element`, `NodeList`*
    pub fn query_selector_all(this: &Element, selectors: &str) -> Result<NodeList, JsValue>;

    # [ wasm_bindgen ( method , structural , js_class = "Element" , js_name = releaseCapture ) ]
    ///The `releaseCapture()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/releaseCapture)
    ///
    ///*This API requires the following crate features to be activated: `Element`*
    pub fn release_capture(this: &Element);

    # [ wasm_bindgen ( catch , method , structural , js_class = "Element" , js_name = releasePointerCapture ) ]
    ///The `releasePointerCapture()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/releasePointerCapture)
    ///
    ///*This API requires the following crate features to be activated: `Element`*
    pub fn release_pointer_capture(this: &Element, pointer_id: i32) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Element" , js_name = removeAttribute ) ]
    ///The `removeAttribute()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/removeAttribute)
    ///
    ///*This API requires the following crate features to be activated: `Element`*
    pub fn remove_attribute(this: &Element, name: &str) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Element" , js_name = removeAttributeNS ) ]
    ///The `removeAttributeNS()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/removeAttributeNS)
    ///
    ///*This API requires the following crate features to be activated: `Element`*
    pub fn remove_attribute_ns(
        this: &Element,
        namespace: Option<&str>,
        local_name: &str,
    ) -> Result<(), JsValue>;

    #[cfg(feature = "Attr")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "Element" , js_name = removeAttributeNode ) ]
    ///The `removeAttributeNode()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/removeAttributeNode)
    ///
    ///*This API requires the following crate features to be activated: `Attr`, `Element`*
    pub fn remove_attribute_node(this: &Element, old_attr: &Attr) -> Result<Option<Attr>, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Element" , js_name = requestFullscreen ) ]
    ///The `requestFullscreen()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/requestFullscreen)
    ///
    ///*This API requires the following crate features to be activated: `Element`*
    pub fn request_fullscreen(this: &Element) -> Result<(), JsValue>;

    # [ wasm_bindgen ( method , structural , js_class = "Element" , js_name = requestPointerLock ) ]
    ///The `requestPointerLock()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/requestPointerLock)
    ///
    ///*This API requires the following crate features to be activated: `Element`*
    pub fn request_pointer_lock(this: &Element);

    # [ wasm_bindgen ( method , structural , js_class = "Element" , js_name = scroll ) ]
    ///The `scroll()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/scroll)
    ///
    ///*This API requires the following crate features to be activated: `Element`*
    pub fn scroll_with_x_and_y(this: &Element, x: f64, y: f64);

    # [ wasm_bindgen ( method , structural , js_class = "Element" , js_name = scroll ) ]
    ///The `scroll()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/scroll)
    ///
    ///*This API requires the following crate features to be activated: `Element`*
    pub fn scroll(this: &Element);

    #[cfg(feature = "ScrollToOptions")]
    # [ wasm_bindgen ( method , structural , js_class = "Element" , js_name = scroll ) ]
    ///The `scroll()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/scroll)
    ///
    ///*This API requires the following crate features to be activated: `Element`, `ScrollToOptions`*
    pub fn scroll_with_scroll_to_options(this: &Element, options: &ScrollToOptions);

    # [ wasm_bindgen ( method , structural , js_class = "Element" , js_name = scrollBy ) ]
    ///The `scrollBy()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/scrollBy)
    ///
    ///*This API requires the following crate features to be activated: `Element`*
    pub fn scroll_by_with_x_and_y(this: &Element, x: f64, y: f64);

    # [ wasm_bindgen ( method , structural , js_class = "Element" , js_name = scrollBy ) ]
    ///The `scrollBy()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/scrollBy)
    ///
    ///*This API requires the following crate features to be activated: `Element`*
    pub fn scroll_by(this: &Element);

    #[cfg(feature = "ScrollToOptions")]
    # [ wasm_bindgen ( method , structural , js_class = "Element" , js_name = scrollBy ) ]
    ///The `scrollBy()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/scrollBy)
    ///
    ///*This API requires the following crate features to be activated: `Element`, `ScrollToOptions`*
    pub fn scroll_by_with_scroll_to_options(this: &Element, options: &ScrollToOptions);

    # [ wasm_bindgen ( method , structural , js_class = "Element" , js_name = scrollIntoView ) ]
    ///The `scrollIntoView()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/scrollIntoView)
    ///
    ///*This API requires the following crate features to be activated: `Element`*
    pub fn scroll_into_view(this: &Element);

    # [ wasm_bindgen ( method , structural , js_class = "Element" , js_name = scrollIntoView ) ]
    ///The `scrollIntoView()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/scrollIntoView)
    ///
    ///*This API requires the following crate features to be activated: `Element`*
    pub fn scroll_into_view_with_bool(this: &Element, arg: bool);

    #[cfg(feature = "ScrollIntoViewOptions")]
    # [ wasm_bindgen ( method , structural , js_class = "Element" , js_name = scrollIntoView ) ]
    ///The `scrollIntoView()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/scrollIntoView)
    ///
    ///*This API requires the following crate features to be activated: `Element`, `ScrollIntoViewOptions`*
    pub fn scroll_into_view_with_scroll_into_view_options(
        this: &Element,
        arg: &ScrollIntoViewOptions,
    );

    # [ wasm_bindgen ( method , structural , js_class = "Element" , js_name = scrollTo ) ]
    ///The `scrollTo()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/scrollTo)
    ///
    ///*This API requires the following crate features to be activated: `Element`*
    pub fn scroll_to_with_x_and_y(this: &Element, x: f64, y: f64);

    # [ wasm_bindgen ( method , structural , js_class = "Element" , js_name = scrollTo ) ]
    ///The `scrollTo()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/scrollTo)
    ///
    ///*This API requires the following crate features to be activated: `Element`*
    pub fn scroll_to(this: &Element);

    #[cfg(feature = "ScrollToOptions")]
    # [ wasm_bindgen ( method , structural , js_class = "Element" , js_name = scrollTo ) ]
    ///The `scrollTo()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/scrollTo)
    ///
    ///*This API requires the following crate features to be activated: `Element`, `ScrollToOptions`*
    pub fn scroll_to_with_scroll_to_options(this: &Element, options: &ScrollToOptions);

    # [ wasm_bindgen ( catch , method , structural , js_class = "Element" , js_name = setAttribute ) ]
    ///The `setAttribute()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/setAttribute)
    ///
    ///*This API requires the following crate features to be activated: `Element`*
    pub fn set_attribute(this: &Element, name: &str, value: &str) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Element" , js_name = setAttributeNS ) ]
    ///The `setAttributeNS()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/setAttributeNS)
    ///
    ///*This API requires the following crate features to be activated: `Element`*
    pub fn set_attribute_ns(
        this: &Element,
        namespace: Option<&str>,
        name: &str,
        value: &str,
    ) -> Result<(), JsValue>;

    #[cfg(feature = "Attr")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "Element" , js_name = setAttributeNode ) ]
    ///The `setAttributeNode()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/setAttributeNode)
    ///
    ///*This API requires the following crate features to be activated: `Attr`, `Element`*
    pub fn set_attribute_node(this: &Element, new_attr: &Attr) -> Result<Option<Attr>, JsValue>;

    #[cfg(feature = "Attr")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "Element" , js_name = setAttributeNodeNS ) ]
    ///The `setAttributeNodeNS()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/setAttributeNodeNS)
    ///
    ///*This API requires the following crate features to be activated: `Attr`, `Element`*
    pub fn set_attribute_node_ns(this: &Element, new_attr: &Attr) -> Result<Option<Attr>, JsValue>;

    # [ wasm_bindgen ( method , structural , js_class = "Element" , js_name = setCapture ) ]
    ///The `setCapture()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/setCapture)
    ///
    ///*This API requires the following crate features to be activated: `Element`*
    pub fn set_capture(this: &Element);

    # [ wasm_bindgen ( method , structural , js_class = "Element" , js_name = setCapture ) ]
    ///The `setCapture()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/setCapture)
    ///
    ///*This API requires the following crate features to be activated: `Element`*
    pub fn set_capture_with_retarget_to_element(this: &Element, retarget_to_element: bool);

    # [ wasm_bindgen ( catch , method , structural , js_class = "Element" , js_name = setPointerCapture ) ]
    ///The `setPointerCapture()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/setPointerCapture)
    ///
    ///*This API requires the following crate features to be activated: `Element`*
    pub fn set_pointer_capture(this: &Element, pointer_id: i32) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Element" , js_name = toggleAttribute ) ]
    ///The `toggleAttribute()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/toggleAttribute)
    ///
    ///*This API requires the following crate features to be activated: `Element`*
    pub fn toggle_attribute(this: &Element, name: &str) -> Result<bool, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Element" , js_name = toggleAttribute ) ]
    ///The `toggleAttribute()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/toggleAttribute)
    ///
    ///*This API requires the following crate features to be activated: `Element`*
    pub fn toggle_attribute_with_force(
        this: &Element,
        name: &str,
        force: bool,
    ) -> Result<bool, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Element" , js_name = webkitMatchesSelector ) ]
    ///The `webkitMatchesSelector()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/webkitMatchesSelector)
    ///
    ///*This API requires the following crate features to be activated: `Element`*
    pub fn webkit_matches_selector(this: &Element, selector: &str) -> Result<bool, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , variadic , js_class = "Element" , js_name = after ) ]
    ///The `after()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/after)
    ///
    ///*This API requires the following crate features to be activated: `Element`*
    pub fn after_with_node(this: &Element, nodes: &::js_sys::Array) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Element" , js_name = after ) ]
    ///The `after()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/after)
    ///
    ///*This API requires the following crate features to be activated: `Element`*
    pub fn after_with_node_0(this: &Element) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Element" , js_name = after ) ]
    ///The `after()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/after)
    ///
    ///*This API requires the following crate features to be activated: `Element`*
    pub fn after_with_node_1(this: &Element, nodes_1: &Node) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Element" , js_name = after ) ]
    ///The `after()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/after)
    ///
    ///*This API requires the following crate features to be activated: `Element`*
    pub fn after_with_node_2(this: &Element, nodes_1: &Node, nodes_2: &Node)
        -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Element" , js_name = after ) ]
    ///The `after()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/after)
    ///
    ///*This API requires the following crate features to be activated: `Element`*
    pub fn after_with_node_3(
        this: &Element,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
    ) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Element" , js_name = after ) ]
    ///The `after()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/after)
    ///
    ///*This API requires the following crate features to be activated: `Element`*
    pub fn after_with_node_4(
        this: &Element,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
        nodes_4: &Node,
    ) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Element" , js_name = after ) ]
    ///The `after()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/after)
    ///
    ///*This API requires the following crate features to be activated: `Element`*
    pub fn after_with_node_5(
        this: &Element,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
        nodes_4: &Node,
        nodes_5: &Node,
    ) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Element" , js_name = after ) ]
    ///The `after()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/after)
    ///
    ///*This API requires the following crate features to be activated: `Element`*
    pub fn after_with_node_6(
        this: &Element,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
        nodes_4: &Node,
        nodes_5: &Node,
        nodes_6: &Node,
    ) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Element" , js_name = after ) ]
    ///The `after()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/after)
    ///
    ///*This API requires the following crate features to be activated: `Element`*
    pub fn after_with_node_7(
        this: &Element,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
        nodes_4: &Node,
        nodes_5: &Node,
        nodes_6: &Node,
        nodes_7: &Node,
    ) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , variadic , js_class = "Element" , js_name = after ) ]
    ///The `after()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/after)
    ///
    ///*This API requires the following crate features to be activated: `Element`*
    pub fn after_with_str(this: &Element, nodes: &::js_sys::Array) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Element" , js_name = after ) ]
    ///The `after()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/after)
    ///
    ///*This API requires the following crate features to be activated: `Element`*
    pub fn after_with_str_0(this: &Element) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Element" , js_name = after ) ]
    ///The `after()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/after)
    ///
    ///*This API requires the following crate features to be activated: `Element`*
    pub fn after_with_str_1(this: &Element, nodes_1: &str) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Element" , js_name = after ) ]
    ///The `after()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/after)
    ///
    ///*This API requires the following crate features to be activated: `Element`*
    pub fn after_with_str_2(this: &Element, nodes_1: &str, nodes_2: &str) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Element" , js_name = after ) ]
    ///The `after()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/after)
    ///
    ///*This API requires the following crate features to be activated: `Element`*
    pub fn after_with_str_3(
        this: &Element,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
    ) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Element" , js_name = after ) ]
    ///The `after()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/after)
    ///
    ///*This API requires the following crate features to be activated: `Element`*
    pub fn after_with_str_4(
        this: &Element,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
        nodes_4: &str,
    ) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Element" , js_name = after ) ]
    ///The `after()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/after)
    ///
    ///*This API requires the following crate features to be activated: `Element`*
    pub fn after_with_str_5(
        this: &Element,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
        nodes_4: &str,
        nodes_5: &str,
    ) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Element" , js_name = after ) ]
    ///The `after()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/after)
    ///
    ///*This API requires the following crate features to be activated: `Element`*
    pub fn after_with_str_6(
        this: &Element,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
        nodes_4: &str,
        nodes_5: &str,
        nodes_6: &str,
    ) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Element" , js_name = after ) ]
    ///The `after()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/after)
    ///
    ///*This API requires the following crate features to be activated: `Element`*
    pub fn after_with_str_7(
        this: &Element,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
        nodes_4: &str,
        nodes_5: &str,
        nodes_6: &str,
        nodes_7: &str,
    ) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , variadic , js_class = "Element" , js_name = before ) ]
    ///The `before()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/before)
    ///
    ///*This API requires the following crate features to be activated: `Element`*
    pub fn before_with_node(this: &Element, nodes: &::js_sys::Array) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Element" , js_name = before ) ]
    ///The `before()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/before)
    ///
    ///*This API requires the following crate features to be activated: `Element`*
    pub fn before_with_node_0(this: &Element) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Element" , js_name = before ) ]
    ///The `before()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/before)
    ///
    ///*This API requires the following crate features to be activated: `Element`*
    pub fn before_with_node_1(this: &Element, nodes_1: &Node) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Element" , js_name = before ) ]
    ///The `before()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/before)
    ///
    ///*This API requires the following crate features to be activated: `Element`*
    pub fn before_with_node_2(
        this: &Element,
        nodes_1: &Node,
        nodes_2: &Node,
    ) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Element" , js_name = before ) ]
    ///The `before()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/before)
    ///
    ///*This API requires the following crate features to be activated: `Element`*
    pub fn before_with_node_3(
        this: &Element,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
    ) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Element" , js_name = before ) ]
    ///The `before()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/before)
    ///
    ///*This API requires the following crate features to be activated: `Element`*
    pub fn before_with_node_4(
        this: &Element,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
        nodes_4: &Node,
    ) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Element" , js_name = before ) ]
    ///The `before()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/before)
    ///
    ///*This API requires the following crate features to be activated: `Element`*
    pub fn before_with_node_5(
        this: &Element,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
        nodes_4: &Node,
        nodes_5: &Node,
    ) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Element" , js_name = before ) ]
    ///The `before()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/before)
    ///
    ///*This API requires the following crate features to be activated: `Element`*
    pub fn before_with_node_6(
        this: &Element,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
        nodes_4: &Node,
        nodes_5: &Node,
        nodes_6: &Node,
    ) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Element" , js_name = before ) ]
    ///The `before()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/before)
    ///
    ///*This API requires the following crate features to be activated: `Element`*
    pub fn before_with_node_7(
        this: &Element,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
        nodes_4: &Node,
        nodes_5: &Node,
        nodes_6: &Node,
        nodes_7: &Node,
    ) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , variadic , js_class = "Element" , js_name = before ) ]
    ///The `before()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/before)
    ///
    ///*This API requires the following crate features to be activated: `Element`*
    pub fn before_with_str(this: &Element, nodes: &::js_sys::Array) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Element" , js_name = before ) ]
    ///The `before()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/before)
    ///
    ///*This API requires the following crate features to be activated: `Element`*
    pub fn before_with_str_0(this: &Element) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Element" , js_name = before ) ]
    ///The `before()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/before)
    ///
    ///*This API requires the following crate features to be activated: `Element`*
    pub fn before_with_str_1(this: &Element, nodes_1: &str) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Element" , js_name = before ) ]
    ///The `before()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/before)
    ///
    ///*This API requires the following crate features to be activated: `Element`*
    pub fn before_with_str_2(this: &Element, nodes_1: &str, nodes_2: &str) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Element" , js_name = before ) ]
    ///The `before()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/before)
    ///
    ///*This API requires the following crate features to be activated: `Element`*
    pub fn before_with_str_3(
        this: &Element,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
    ) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Element" , js_name = before ) ]
    ///The `before()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/before)
    ///
    ///*This API requires the following crate features to be activated: `Element`*
    pub fn before_with_str_4(
        this: &Element,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
        nodes_4: &str,
    ) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Element" , js_name = before ) ]
    ///The `before()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/before)
    ///
    ///*This API requires the following crate features to be activated: `Element`*
    pub fn before_with_str_5(
        this: &Element,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
        nodes_4: &str,
        nodes_5: &str,
    ) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Element" , js_name = before ) ]
    ///The `before()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/before)
    ///
    ///*This API requires the following crate features to be activated: `Element`*
    pub fn before_with_str_6(
        this: &Element,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
        nodes_4: &str,
        nodes_5: &str,
        nodes_6: &str,
    ) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Element" , js_name = before ) ]
    ///The `before()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/before)
    ///
    ///*This API requires the following crate features to be activated: `Element`*
    pub fn before_with_str_7(
        this: &Element,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
        nodes_4: &str,
        nodes_5: &str,
        nodes_6: &str,
        nodes_7: &str,
    ) -> Result<(), JsValue>;

    # [ wasm_bindgen ( method , structural , js_class = "Element" , js_name = remove ) ]
    ///The `remove()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/remove)
    ///
    ///*This API requires the following crate features to be activated: `Element`*
    pub fn remove(this: &Element);

    # [ wasm_bindgen ( catch , method , structural , variadic , js_class = "Element" , js_name = replaceWith ) ]
    ///The `replaceWith()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/replaceWith)
    ///
    ///*This API requires the following crate features to be activated: `Element`*
    pub fn replace_with_with_node(this: &Element, nodes: &::js_sys::Array) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Element" , js_name = replaceWith ) ]
    ///The `replaceWith()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/replaceWith)
    ///
    ///*This API requires the following crate features to be activated: `Element`*
    pub fn replace_with_with_node_0(this: &Element) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Element" , js_name = replaceWith ) ]
    ///The `replaceWith()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/replaceWith)
    ///
    ///*This API requires the following crate features to be activated: `Element`*
    pub fn replace_with_with_node_1(this: &Element, nodes_1: &Node) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Element" , js_name = replaceWith ) ]
    ///The `replaceWith()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/replaceWith)
    ///
    ///*This API requires the following crate features to be activated: `Element`*
    pub fn replace_with_with_node_2(
        this: &Element,
        nodes_1: &Node,
        nodes_2: &Node,
    ) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Element" , js_name = replaceWith ) ]
    ///The `replaceWith()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/replaceWith)
    ///
    ///*This API requires the following crate features to be activated: `Element`*
    pub fn replace_with_with_node_3(
        this: &Element,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
    ) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Element" , js_name = replaceWith ) ]
    ///The `replaceWith()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/replaceWith)
    ///
    ///*This API requires the following crate features to be activated: `Element`*
    pub fn replace_with_with_node_4(
        this: &Element,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
        nodes_4: &Node,
    ) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Element" , js_name = replaceWith ) ]
    ///The `replaceWith()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/replaceWith)
    ///
    ///*This API requires the following crate features to be activated: `Element`*
    pub fn replace_with_with_node_5(
        this: &Element,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
        nodes_4: &Node,
        nodes_5: &Node,
    ) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Element" , js_name = replaceWith ) ]
    ///The `replaceWith()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/replaceWith)
    ///
    ///*This API requires the following crate features to be activated: `Element`*
    pub fn replace_with_with_node_6(
        this: &Element,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
        nodes_4: &Node,
        nodes_5: &Node,
        nodes_6: &Node,
    ) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Element" , js_name = replaceWith ) ]
    ///The `replaceWith()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/replaceWith)
    ///
    ///*This API requires the following crate features to be activated: `Element`*
    pub fn replace_with_with_node_7(
        this: &Element,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
        nodes_4: &Node,
        nodes_5: &Node,
        nodes_6: &Node,
        nodes_7: &Node,
    ) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , variadic , js_class = "Element" , js_name = replaceWith ) ]
    ///The `replaceWith()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/replaceWith)
    ///
    ///*This API requires the following crate features to be activated: `Element`*
    pub fn replace_with_with_str(this: &Element, nodes: &::js_sys::Array) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Element" , js_name = replaceWith ) ]
    ///The `replaceWith()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/replaceWith)
    ///
    ///*This API requires the following crate features to be activated: `Element`*
    pub fn replace_with_with_str_0(this: &Element) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Element" , js_name = replaceWith ) ]
    ///The `replaceWith()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/replaceWith)
    ///
    ///*This API requires the following crate features to be activated: `Element`*
    pub fn replace_with_with_str_1(this: &Element, nodes_1: &str) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Element" , js_name = replaceWith ) ]
    ///The `replaceWith()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/replaceWith)
    ///
    ///*This API requires the following crate features to be activated: `Element`*
    pub fn replace_with_with_str_2(
        this: &Element,
        nodes_1: &str,
        nodes_2: &str,
    ) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Element" , js_name = replaceWith ) ]
    ///The `replaceWith()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/replaceWith)
    ///
    ///*This API requires the following crate features to be activated: `Element`*
    pub fn replace_with_with_str_3(
        this: &Element,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
    ) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Element" , js_name = replaceWith ) ]
    ///The `replaceWith()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/replaceWith)
    ///
    ///*This API requires the following crate features to be activated: `Element`*
    pub fn replace_with_with_str_4(
        this: &Element,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
        nodes_4: &str,
    ) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Element" , js_name = replaceWith ) ]
    ///The `replaceWith()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/replaceWith)
    ///
    ///*This API requires the following crate features to be activated: `Element`*
    pub fn replace_with_with_str_5(
        this: &Element,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
        nodes_4: &str,
        nodes_5: &str,
    ) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Element" , js_name = replaceWith ) ]
    ///The `replaceWith()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/replaceWith)
    ///
    ///*This API requires the following crate features to be activated: `Element`*
    pub fn replace_with_with_str_6(
        this: &Element,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
        nodes_4: &str,
        nodes_5: &str,
        nodes_6: &str,
    ) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Element" , js_name = replaceWith ) ]
    ///The `replaceWith()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/replaceWith)
    ///
    ///*This API requires the following crate features to be activated: `Element`*
    pub fn replace_with_with_str_7(
        this: &Element,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
        nodes_4: &str,
        nodes_5: &str,
        nodes_6: &str,
        nodes_7: &str,
    ) -> Result<(), JsValue>;

    #[cfg(all(feature = "DomPoint", feature = "DomPointInit", feature = "Text",))]
    # [ wasm_bindgen ( catch , method , structural , js_class = "Element" , js_name = convertPointFromNode ) ]
    ///The `convertPointFromNode()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/convertPointFromNode)
    ///
    ///*This API requires the following crate features to be activated: `DomPoint`, `DomPointInit`, `Element`, `Text`*
    pub fn convert_point_from_node_with_text(
        this: &Element,
        point: &DomPointInit,
        from: &Text,
    ) -> Result<DomPoint, JsValue>;

    #[cfg(all(feature = "DomPoint", feature = "DomPointInit",))]
    # [ wasm_bindgen ( catch , method , structural , js_class = "Element" , js_name = convertPointFromNode ) ]
    ///The `convertPointFromNode()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/convertPointFromNode)
    ///
    ///*This API requires the following crate features to be activated: `DomPoint`, `DomPointInit`, `Element`*
    pub fn convert_point_from_node_with_element(
        this: &Element,
        point: &DomPointInit,
        from: &Element,
    ) -> Result<DomPoint, JsValue>;

    #[cfg(all(feature = "Document", feature = "DomPoint", feature = "DomPointInit",))]
    # [ wasm_bindgen ( catch , method , structural , js_class = "Element" , js_name = convertPointFromNode ) ]
    ///The `convertPointFromNode()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/convertPointFromNode)
    ///
    ///*This API requires the following crate features to be activated: `Document`, `DomPoint`, `DomPointInit`, `Element`*
    pub fn convert_point_from_node_with_document(
        this: &Element,
        point: &DomPointInit,
        from: &Document,
    ) -> Result<DomPoint, JsValue>;

    #[cfg(all(
        feature = "ConvertCoordinateOptions",
        feature = "DomPoint",
        feature = "DomPointInit",
        feature = "Text",
    ))]
    # [ wasm_bindgen ( catch , method , structural , js_class = "Element" , js_name = convertPointFromNode ) ]
    ///The `convertPointFromNode()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/convertPointFromNode)
    ///
    ///*This API requires the following crate features to be activated: `ConvertCoordinateOptions`, `DomPoint`, `DomPointInit`, `Element`, `Text`*
    pub fn convert_point_from_node_with_text_and_options(
        this: &Element,
        point: &DomPointInit,
        from: &Text,
        options: &ConvertCoordinateOptions,
    ) -> Result<DomPoint, JsValue>;

    #[cfg(all(
        feature = "ConvertCoordinateOptions",
        feature = "DomPoint",
        feature = "DomPointInit",
    ))]
    # [ wasm_bindgen ( catch , method , structural , js_class = "Element" , js_name = convertPointFromNode ) ]
    ///The `convertPointFromNode()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/convertPointFromNode)
    ///
    ///*This API requires the following crate features to be activated: `ConvertCoordinateOptions`, `DomPoint`, `DomPointInit`, `Element`*
    pub fn convert_point_from_node_with_element_and_options(
        this: &Element,
        point: &DomPointInit,
        from: &Element,
        options: &ConvertCoordinateOptions,
    ) -> Result<DomPoint, JsValue>;

    #[cfg(all(
        feature = "ConvertCoordinateOptions",
        feature = "Document",
        feature = "DomPoint",
        feature = "DomPointInit",
    ))]
    # [ wasm_bindgen ( catch , method , structural , js_class = "Element" , js_name = convertPointFromNode ) ]
    ///The `convertPointFromNode()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/convertPointFromNode)
    ///
    ///*This API requires the following crate features to be activated: `ConvertCoordinateOptions`, `Document`, `DomPoint`, `DomPointInit`, `Element`*
    pub fn convert_point_from_node_with_document_and_options(
        this: &Element,
        point: &DomPointInit,
        from: &Document,
        options: &ConvertCoordinateOptions,
    ) -> Result<DomPoint, JsValue>;

    #[cfg(all(feature = "DomQuad", feature = "Text",))]
    # [ wasm_bindgen ( catch , method , structural , js_class = "Element" , js_name = convertQuadFromNode ) ]
    ///The `convertQuadFromNode()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/convertQuadFromNode)
    ///
    ///*This API requires the following crate features to be activated: `DomQuad`, `Element`, `Text`*
    pub fn convert_quad_from_node_with_text(
        this: &Element,
        quad: &DomQuad,
        from: &Text,
    ) -> Result<DomQuad, JsValue>;

    #[cfg(feature = "DomQuad")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "Element" , js_name = convertQuadFromNode ) ]
    ///The `convertQuadFromNode()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/convertQuadFromNode)
    ///
    ///*This API requires the following crate features to be activated: `DomQuad`, `Element`*
    pub fn convert_quad_from_node_with_element(
        this: &Element,
        quad: &DomQuad,
        from: &Element,
    ) -> Result<DomQuad, JsValue>;

    #[cfg(all(feature = "Document", feature = "DomQuad",))]
    # [ wasm_bindgen ( catch , method , structural , js_class = "Element" , js_name = convertQuadFromNode ) ]
    ///The `convertQuadFromNode()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/convertQuadFromNode)
    ///
    ///*This API requires the following crate features to be activated: `Document`, `DomQuad`, `Element`*
    pub fn convert_quad_from_node_with_document(
        this: &Element,
        quad: &DomQuad,
        from: &Document,
    ) -> Result<DomQuad, JsValue>;

    #[cfg(all(
        feature = "ConvertCoordinateOptions",
        feature = "DomQuad",
        feature = "Text",
    ))]
    # [ wasm_bindgen ( catch , method , structural , js_class = "Element" , js_name = convertQuadFromNode ) ]
    ///The `convertQuadFromNode()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/convertQuadFromNode)
    ///
    ///*This API requires the following crate features to be activated: `ConvertCoordinateOptions`, `DomQuad`, `Element`, `Text`*
    pub fn convert_quad_from_node_with_text_and_options(
        this: &Element,
        quad: &DomQuad,
        from: &Text,
        options: &ConvertCoordinateOptions,
    ) -> Result<DomQuad, JsValue>;

    #[cfg(all(feature = "ConvertCoordinateOptions", feature = "DomQuad",))]
    # [ wasm_bindgen ( catch , method , structural , js_class = "Element" , js_name = convertQuadFromNode ) ]
    ///The `convertQuadFromNode()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/convertQuadFromNode)
    ///
    ///*This API requires the following crate features to be activated: `ConvertCoordinateOptions`, `DomQuad`, `Element`*
    pub fn convert_quad_from_node_with_element_and_options(
        this: &Element,
        quad: &DomQuad,
        from: &Element,
        options: &ConvertCoordinateOptions,
    ) -> Result<DomQuad, JsValue>;

    #[cfg(all(
        feature = "ConvertCoordinateOptions",
        feature = "Document",
        feature = "DomQuad",
    ))]
    # [ wasm_bindgen ( catch , method , structural , js_class = "Element" , js_name = convertQuadFromNode ) ]
    ///The `convertQuadFromNode()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/convertQuadFromNode)
    ///
    ///*This API requires the following crate features to be activated: `ConvertCoordinateOptions`, `Document`, `DomQuad`, `Element`*
    pub fn convert_quad_from_node_with_document_and_options(
        this: &Element,
        quad: &DomQuad,
        from: &Document,
        options: &ConvertCoordinateOptions,
    ) -> Result<DomQuad, JsValue>;

    #[cfg(all(feature = "DomQuad", feature = "DomRectReadOnly", feature = "Text",))]
    # [ wasm_bindgen ( catch , method , structural , js_class = "Element" , js_name = convertRectFromNode ) ]
    ///The `convertRectFromNode()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/convertRectFromNode)
    ///
    ///*This API requires the following crate features to be activated: `DomQuad`, `DomRectReadOnly`, `Element`, `Text`*
    pub fn convert_rect_from_node_with_text(
        this: &Element,
        rect: &DomRectReadOnly,
        from: &Text,
    ) -> Result<DomQuad, JsValue>;

    #[cfg(all(feature = "DomQuad", feature = "DomRectReadOnly",))]
    # [ wasm_bindgen ( catch , method , structural , js_class = "Element" , js_name = convertRectFromNode ) ]
    ///The `convertRectFromNode()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/convertRectFromNode)
    ///
    ///*This API requires the following crate features to be activated: `DomQuad`, `DomRectReadOnly`, `Element`*
    pub fn convert_rect_from_node_with_element(
        this: &Element,
        rect: &DomRectReadOnly,
        from: &Element,
    ) -> Result<DomQuad, JsValue>;

    #[cfg(all(feature = "Document", feature = "DomQuad", feature = "DomRectReadOnly",))]
    # [ wasm_bindgen ( catch , method , structural , js_class = "Element" , js_name = convertRectFromNode ) ]
    ///The `convertRectFromNode()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/convertRectFromNode)
    ///
    ///*This API requires the following crate features to be activated: `Document`, `DomQuad`, `DomRectReadOnly`, `Element`*
    pub fn convert_rect_from_node_with_document(
        this: &Element,
        rect: &DomRectReadOnly,
        from: &Document,
    ) -> Result<DomQuad, JsValue>;

    #[cfg(all(
        feature = "ConvertCoordinateOptions",
        feature = "DomQuad",
        feature = "DomRectReadOnly",
        feature = "Text",
    ))]
    # [ wasm_bindgen ( catch , method , structural , js_class = "Element" , js_name = convertRectFromNode ) ]
    ///The `convertRectFromNode()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/convertRectFromNode)
    ///
    ///*This API requires the following crate features to be activated: `ConvertCoordinateOptions`, `DomQuad`, `DomRectReadOnly`, `Element`, `Text`*
    pub fn convert_rect_from_node_with_text_and_options(
        this: &Element,
        rect: &DomRectReadOnly,
        from: &Text,
        options: &ConvertCoordinateOptions,
    ) -> Result<DomQuad, JsValue>;

    #[cfg(all(
        feature = "ConvertCoordinateOptions",
        feature = "DomQuad",
        feature = "DomRectReadOnly",
    ))]
    # [ wasm_bindgen ( catch , method , structural , js_class = "Element" , js_name = convertRectFromNode ) ]
    ///The `convertRectFromNode()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/convertRectFromNode)
    ///
    ///*This API requires the following crate features to be activated: `ConvertCoordinateOptions`, `DomQuad`, `DomRectReadOnly`, `Element`*
    pub fn convert_rect_from_node_with_element_and_options(
        this: &Element,
        rect: &DomRectReadOnly,
        from: &Element,
        options: &ConvertCoordinateOptions,
    ) -> Result<DomQuad, JsValue>;

    #[cfg(all(
        feature = "ConvertCoordinateOptions",
        feature = "Document",
        feature = "DomQuad",
        feature = "DomRectReadOnly",
    ))]
    # [ wasm_bindgen ( catch , method , structural , js_class = "Element" , js_name = convertRectFromNode ) ]
    ///The `convertRectFromNode()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/convertRectFromNode)
    ///
    ///*This API requires the following crate features to be activated: `ConvertCoordinateOptions`, `Document`, `DomQuad`, `DomRectReadOnly`, `Element`*
    pub fn convert_rect_from_node_with_document_and_options(
        this: &Element,
        rect: &DomRectReadOnly,
        from: &Document,
        options: &ConvertCoordinateOptions,
    ) -> Result<DomQuad, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Element" , js_name = getBoxQuads ) ]
    ///The `getBoxQuads()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/getBoxQuads)
    ///
    ///*This API requires the following crate features to be activated: `Element`*
    pub fn get_box_quads(this: &Element) -> Result<::js_sys::Array, JsValue>;

    #[cfg(feature = "BoxQuadOptions")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "Element" , js_name = getBoxQuads ) ]
    ///The `getBoxQuads()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/getBoxQuads)
    ///
    ///*This API requires the following crate features to be activated: `BoxQuadOptions`, `Element`*
    pub fn get_box_quads_with_options(
        this: &Element,
        options: &BoxQuadOptions,
    ) -> Result<::js_sys::Array, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , variadic , js_class = "Element" , js_name = append ) ]
    ///The `append()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/append)
    ///
    ///*This API requires the following crate features to be activated: `Element`*
    pub fn append_with_node(this: &Element, nodes: &::js_sys::Array) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Element" , js_name = append ) ]
    ///The `append()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/append)
    ///
    ///*This API requires the following crate features to be activated: `Element`*
    pub fn append_with_node_0(this: &Element) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Element" , js_name = append ) ]
    ///The `append()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/append)
    ///
    ///*This API requires the following crate features to be activated: `Element`*
    pub fn append_with_node_1(this: &Element, nodes_1: &Node) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Element" , js_name = append ) ]
    ///The `append()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/append)
    ///
    ///*This API requires the following crate features to be activated: `Element`*
    pub fn append_with_node_2(
        this: &Element,
        nodes_1: &Node,
        nodes_2: &Node,
    ) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Element" , js_name = append ) ]
    ///The `append()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/append)
    ///
    ///*This API requires the following crate features to be activated: `Element`*
    pub fn append_with_node_3(
        this: &Element,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
    ) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Element" , js_name = append ) ]
    ///The `append()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/append)
    ///
    ///*This API requires the following crate features to be activated: `Element`*
    pub fn append_with_node_4(
        this: &Element,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
        nodes_4: &Node,
    ) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Element" , js_name = append ) ]
    ///The `append()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/append)
    ///
    ///*This API requires the following crate features to be activated: `Element`*
    pub fn append_with_node_5(
        this: &Element,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
        nodes_4: &Node,
        nodes_5: &Node,
    ) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Element" , js_name = append ) ]
    ///The `append()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/append)
    ///
    ///*This API requires the following crate features to be activated: `Element`*
    pub fn append_with_node_6(
        this: &Element,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
        nodes_4: &Node,
        nodes_5: &Node,
        nodes_6: &Node,
    ) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Element" , js_name = append ) ]
    ///The `append()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/append)
    ///
    ///*This API requires the following crate features to be activated: `Element`*
    pub fn append_with_node_7(
        this: &Element,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
        nodes_4: &Node,
        nodes_5: &Node,
        nodes_6: &Node,
        nodes_7: &Node,
    ) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , variadic , js_class = "Element" , js_name = append ) ]
    ///The `append()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/append)
    ///
    ///*This API requires the following crate features to be activated: `Element`*
    pub fn append_with_str(this: &Element, nodes: &::js_sys::Array) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Element" , js_name = append ) ]
    ///The `append()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/append)
    ///
    ///*This API requires the following crate features to be activated: `Element`*
    pub fn append_with_str_0(this: &Element) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Element" , js_name = append ) ]
    ///The `append()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/append)
    ///
    ///*This API requires the following crate features to be activated: `Element`*
    pub fn append_with_str_1(this: &Element, nodes_1: &str) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Element" , js_name = append ) ]
    ///The `append()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/append)
    ///
    ///*This API requires the following crate features to be activated: `Element`*
    pub fn append_with_str_2(this: &Element, nodes_1: &str, nodes_2: &str) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Element" , js_name = append ) ]
    ///The `append()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/append)
    ///
    ///*This API requires the following crate features to be activated: `Element`*
    pub fn append_with_str_3(
        this: &Element,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
    ) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Element" , js_name = append ) ]
    ///The `append()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/append)
    ///
    ///*This API requires the following crate features to be activated: `Element`*
    pub fn append_with_str_4(
        this: &Element,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
        nodes_4: &str,
    ) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Element" , js_name = append ) ]
    ///The `append()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/append)
    ///
    ///*This API requires the following crate features to be activated: `Element`*
    pub fn append_with_str_5(
        this: &Element,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
        nodes_4: &str,
        nodes_5: &str,
    ) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Element" , js_name = append ) ]
    ///The `append()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/append)
    ///
    ///*This API requires the following crate features to be activated: `Element`*
    pub fn append_with_str_6(
        this: &Element,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
        nodes_4: &str,
        nodes_5: &str,
        nodes_6: &str,
    ) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Element" , js_name = append ) ]
    ///The `append()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/append)
    ///
    ///*This API requires the following crate features to be activated: `Element`*
    pub fn append_with_str_7(
        this: &Element,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
        nodes_4: &str,
        nodes_5: &str,
        nodes_6: &str,
        nodes_7: &str,
    ) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , variadic , js_class = "Element" , js_name = prepend ) ]
    ///The `prepend()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/prepend)
    ///
    ///*This API requires the following crate features to be activated: `Element`*
    pub fn prepend_with_node(this: &Element, nodes: &::js_sys::Array) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Element" , js_name = prepend ) ]
    ///The `prepend()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/prepend)
    ///
    ///*This API requires the following crate features to be activated: `Element`*
    pub fn prepend_with_node_0(this: &Element) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Element" , js_name = prepend ) ]
    ///The `prepend()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/prepend)
    ///
    ///*This API requires the following crate features to be activated: `Element`*
    pub fn prepend_with_node_1(this: &Element, nodes_1: &Node) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Element" , js_name = prepend ) ]
    ///The `prepend()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/prepend)
    ///
    ///*This API requires the following crate features to be activated: `Element`*
    pub fn prepend_with_node_2(
        this: &Element,
        nodes_1: &Node,
        nodes_2: &Node,
    ) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Element" , js_name = prepend ) ]
    ///The `prepend()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/prepend)
    ///
    ///*This API requires the following crate features to be activated: `Element`*
    pub fn prepend_with_node_3(
        this: &Element,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
    ) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Element" , js_name = prepend ) ]
    ///The `prepend()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/prepend)
    ///
    ///*This API requires the following crate features to be activated: `Element`*
    pub fn prepend_with_node_4(
        this: &Element,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
        nodes_4: &Node,
    ) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Element" , js_name = prepend ) ]
    ///The `prepend()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/prepend)
    ///
    ///*This API requires the following crate features to be activated: `Element`*
    pub fn prepend_with_node_5(
        this: &Element,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
        nodes_4: &Node,
        nodes_5: &Node,
    ) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Element" , js_name = prepend ) ]
    ///The `prepend()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/prepend)
    ///
    ///*This API requires the following crate features to be activated: `Element`*
    pub fn prepend_with_node_6(
        this: &Element,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
        nodes_4: &Node,
        nodes_5: &Node,
        nodes_6: &Node,
    ) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Element" , js_name = prepend ) ]
    ///The `prepend()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/prepend)
    ///
    ///*This API requires the following crate features to be activated: `Element`*
    pub fn prepend_with_node_7(
        this: &Element,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
        nodes_4: &Node,
        nodes_5: &Node,
        nodes_6: &Node,
        nodes_7: &Node,
    ) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , variadic , js_class = "Element" , js_name = prepend ) ]
    ///The `prepend()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/prepend)
    ///
    ///*This API requires the following crate features to be activated: `Element`*
    pub fn prepend_with_str(this: &Element, nodes: &::js_sys::Array) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Element" , js_name = prepend ) ]
    ///The `prepend()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/prepend)
    ///
    ///*This API requires the following crate features to be activated: `Element`*
    pub fn prepend_with_str_0(this: &Element) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Element" , js_name = prepend ) ]
    ///The `prepend()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/prepend)
    ///
    ///*This API requires the following crate features to be activated: `Element`*
    pub fn prepend_with_str_1(this: &Element, nodes_1: &str) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Element" , js_name = prepend ) ]
    ///The `prepend()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/prepend)
    ///
    ///*This API requires the following crate features to be activated: `Element`*
    pub fn prepend_with_str_2(this: &Element, nodes_1: &str, nodes_2: &str) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Element" , js_name = prepend ) ]
    ///The `prepend()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/prepend)
    ///
    ///*This API requires the following crate features to be activated: `Element`*
    pub fn prepend_with_str_3(
        this: &Element,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
    ) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Element" , js_name = prepend ) ]
    ///The `prepend()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/prepend)
    ///
    ///*This API requires the following crate features to be activated: `Element`*
    pub fn prepend_with_str_4(
        this: &Element,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
        nodes_4: &str,
    ) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Element" , js_name = prepend ) ]
    ///The `prepend()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/prepend)
    ///
    ///*This API requires the following crate features to be activated: `Element`*
    pub fn prepend_with_str_5(
        this: &Element,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
        nodes_4: &str,
        nodes_5: &str,
    ) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Element" , js_name = prepend ) ]
    ///The `prepend()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/prepend)
    ///
    ///*This API requires the following crate features to be activated: `Element`*
    pub fn prepend_with_str_6(
        this: &Element,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
        nodes_4: &str,
        nodes_5: &str,
        nodes_6: &str,
    ) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Element" , js_name = prepend ) ]
    ///The `prepend()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/prepend)
    ///
    ///*This API requires the following crate features to be activated: `Element`*
    pub fn prepend_with_str_7(
        this: &Element,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
        nodes_4: &str,
        nodes_5: &str,
        nodes_6: &str,
        nodes_7: &str,
    ) -> Result<(), JsValue>;

}
