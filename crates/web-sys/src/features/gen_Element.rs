use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = Element , typescript_name = Element ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `Element` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element)\n\n*This API requires the following crate features to be activated: `Element`*"]
    pub type Element;
    # [ wasm_bindgen ( structural , method , getter , js_name = namespaceURI ) ]
    #[doc = "Getter for the `namespaceURI` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/namespaceURI)\n\n*This API requires the following crate features to be activated: `Element`*"]
    pub fn namespace_uri(this: &Element) -> Option<String>;
    # [ wasm_bindgen ( structural , method , getter , js_name = prefix ) ]
    #[doc = "Getter for the `prefix` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/prefix)\n\n*This API requires the following crate features to be activated: `Element`*"]
    pub fn prefix(this: &Element) -> Option<String>;
    # [ wasm_bindgen ( structural , method , getter , js_name = localName ) ]
    #[doc = "Getter for the `localName` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/localName)\n\n*This API requires the following crate features to be activated: `Element`*"]
    pub fn local_name(this: &Element) -> String;
    # [ wasm_bindgen ( structural , method , getter , js_name = tagName ) ]
    #[doc = "Getter for the `tagName` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/tagName)\n\n*This API requires the following crate features to be activated: `Element`*"]
    pub fn tag_name(this: &Element) -> String;
    # [ wasm_bindgen ( structural , method , getter , js_name = id ) ]
    #[doc = "Getter for the `id` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/id)\n\n*This API requires the following crate features to be activated: `Element`*"]
    pub fn id(this: &Element) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = id ) ]
    #[doc = "Setter for the `id` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/id)\n\n*This API requires the following crate features to be activated: `Element`*"]
    pub fn set_id(this: &Element, value: String);
    # [ wasm_bindgen ( structural , method , getter , js_name = className ) ]
    #[doc = "Getter for the `className` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/className)\n\n*This API requires the following crate features to be activated: `Element`*"]
    pub fn class_name(this: &Element) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = className ) ]
    #[doc = "Setter for the `className` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/className)\n\n*This API requires the following crate features to be activated: `Element`*"]
    pub fn set_class_name(this: &Element, value: String);
    # [ wasm_bindgen ( structural , method , getter , js_name = classList ) ]
    #[cfg(feature = "DomTokenList")]
    #[doc = "Getter for the `classList` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/classList)\n\n*This API requires the following crate features to be activated: `DomTokenList`, `Element`*"]
    pub fn class_list(this: &Element) -> DomTokenList;
    # [ wasm_bindgen ( structural , method , getter , js_name = attributes ) ]
    #[cfg(feature = "NamedNodeMap")]
    #[doc = "Getter for the `attributes` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/attributes)\n\n*This API requires the following crate features to be activated: `Element`, `NamedNodeMap`*"]
    pub fn attributes(this: &Element) -> NamedNodeMap;
    # [ wasm_bindgen ( structural , method , getter , js_name = scrollTop ) ]
    #[doc = "Getter for the `scrollTop` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/scrollTop)\n\n*This API requires the following crate features to be activated: `Element`*"]
    pub fn scroll_top(this: &Element) -> i32;
    # [ wasm_bindgen ( structural , method , setter , js_name = scrollTop ) ]
    #[doc = "Setter for the `scrollTop` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/scrollTop)\n\n*This API requires the following crate features to be activated: `Element`*"]
    pub fn set_scroll_top(this: &Element, value: i32);
    # [ wasm_bindgen ( structural , method , getter , js_name = scrollLeft ) ]
    #[doc = "Getter for the `scrollLeft` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/scrollLeft)\n\n*This API requires the following crate features to be activated: `Element`*"]
    pub fn scroll_left(this: &Element) -> i32;
    # [ wasm_bindgen ( structural , method , setter , js_name = scrollLeft ) ]
    #[doc = "Setter for the `scrollLeft` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/scrollLeft)\n\n*This API requires the following crate features to be activated: `Element`*"]
    pub fn set_scroll_left(this: &Element, value: i32);
    # [ wasm_bindgen ( structural , method , getter , js_name = scrollWidth ) ]
    #[doc = "Getter for the `scrollWidth` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/scrollWidth)\n\n*This API requires the following crate features to be activated: `Element`*"]
    pub fn scroll_width(this: &Element) -> i32;
    # [ wasm_bindgen ( structural , method , getter , js_name = scrollHeight ) ]
    #[doc = "Getter for the `scrollHeight` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/scrollHeight)\n\n*This API requires the following crate features to be activated: `Element`*"]
    pub fn scroll_height(this: &Element) -> i32;
    # [ wasm_bindgen ( structural , method , getter , js_name = clientTop ) ]
    #[doc = "Getter for the `clientTop` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/clientTop)\n\n*This API requires the following crate features to be activated: `Element`*"]
    pub fn client_top(this: &Element) -> i32;
    # [ wasm_bindgen ( structural , method , getter , js_name = clientLeft ) ]
    #[doc = "Getter for the `clientLeft` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/clientLeft)\n\n*This API requires the following crate features to be activated: `Element`*"]
    pub fn client_left(this: &Element) -> i32;
    # [ wasm_bindgen ( structural , method , getter , js_name = clientWidth ) ]
    #[doc = "Getter for the `clientWidth` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/clientWidth)\n\n*This API requires the following crate features to be activated: `Element`*"]
    pub fn client_width(this: &Element) -> i32;
    # [ wasm_bindgen ( structural , method , getter , js_name = clientHeight ) ]
    #[doc = "Getter for the `clientHeight` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/clientHeight)\n\n*This API requires the following crate features to be activated: `Element`*"]
    pub fn client_height(this: &Element) -> i32;
    # [ wasm_bindgen ( structural , method , getter , js_name = innerHTML ) ]
    #[doc = "Getter for the `innerHTML` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/innerHTML)\n\n*This API requires the following crate features to be activated: `Element`*"]
    pub fn inner_html(this: &Element) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = innerHTML ) ]
    #[doc = "Setter for the `innerHTML` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/innerHTML)\n\n*This API requires the following crate features to be activated: `Element`*"]
    pub fn set_inner_html(this: &Element, value: String);
    # [ wasm_bindgen ( structural , method , getter , js_name = outerHTML ) ]
    #[doc = "Getter for the `outerHTML` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/outerHTML)\n\n*This API requires the following crate features to be activated: `Element`*"]
    pub fn outer_html(this: &Element) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = outerHTML ) ]
    #[doc = "Setter for the `outerHTML` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/outerHTML)\n\n*This API requires the following crate features to be activated: `Element`*"]
    pub fn set_outer_html(this: &Element, value: String);
    # [ wasm_bindgen ( structural , method , getter , js_name = shadowRoot ) ]
    #[cfg(feature = "ShadowRoot")]
    #[doc = "Getter for the `shadowRoot` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/shadowRoot)\n\n*This API requires the following crate features to be activated: `Element`, `ShadowRoot`*"]
    pub fn shadow_root(this: &Element) -> Option<ShadowRoot>;
    # [ wasm_bindgen ( structural , method , getter , js_name = assignedSlot ) ]
    #[cfg(feature = "HtmlSlotElement")]
    #[doc = "Getter for the `assignedSlot` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/assignedSlot)\n\n*This API requires the following crate features to be activated: `Element`, `HtmlSlotElement`*"]
    pub fn assigned_slot(this: &Element) -> Option<HtmlSlotElement>;
    # [ wasm_bindgen ( structural , method , getter , js_name = slot ) ]
    #[doc = "Getter for the `slot` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/slot)\n\n*This API requires the following crate features to be activated: `Element`*"]
    pub fn slot(this: &Element) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = slot ) ]
    #[doc = "Setter for the `slot` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/slot)\n\n*This API requires the following crate features to be activated: `Element`*"]
    pub fn set_slot(this: &Element, value: String);
    # [ wasm_bindgen ( structural , method , getter , js_name = previousElementSibling ) ]
    #[doc = "Getter for the `previousElementSibling` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/previousElementSibling)\n\n*This API requires the following crate features to be activated: `Element`*"]
    pub fn previous_element_sibling(this: &Element) -> Option<Element>;
    # [ wasm_bindgen ( structural , method , getter , js_name = nextElementSibling ) ]
    #[doc = "Getter for the `nextElementSibling` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/nextElementSibling)\n\n*This API requires the following crate features to be activated: `Element`*"]
    pub fn next_element_sibling(this: &Element) -> Option<Element>;
    # [ wasm_bindgen ( structural , method , getter , js_name = children ) ]
    #[cfg(feature = "HtmlCollection")]
    #[doc = "Getter for the `children` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/children)\n\n*This API requires the following crate features to be activated: `Element`, `HtmlCollection`*"]
    pub fn children(this: &Element) -> HtmlCollection;
    # [ wasm_bindgen ( structural , method , getter , js_name = firstElementChild ) ]
    #[doc = "Getter for the `firstElementChild` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/firstElementChild)\n\n*This API requires the following crate features to be activated: `Element`*"]
    pub fn first_element_child(this: &Element) -> Option<Element>;
    # [ wasm_bindgen ( structural , method , getter , js_name = lastElementChild ) ]
    #[doc = "Getter for the `lastElementChild` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/lastElementChild)\n\n*This API requires the following crate features to be activated: `Element`*"]
    pub fn last_element_child(this: &Element) -> Option<Element>;
    # [ wasm_bindgen ( structural , method , getter , js_name = childElementCount ) ]
    #[doc = "Getter for the `childElementCount` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/childElementCount)\n\n*This API requires the following crate features to be activated: `Element`*"]
    pub fn child_element_count(this: &Element) -> u32;
    #[cfg(all(feature = "ShadowRoot", feature = "ShadowRootInit",))]
    # [ wasm_bindgen ( catch , method , structural , js_name = attachShadow ) ]
    #[doc = "The `attachShadow()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/attachShadow)\n\n*This API requires the following crate features to be activated: `Element`, `ShadowRoot`, `ShadowRootInit`*"]
    pub fn attach_shadow(
        this: &Element,
        shadow_root_init_dict: &ShadowRootInit,
    ) -> Result<ShadowRoot, JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = closest ) ]
    #[doc = "The `closest()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/closest)\n\n*This API requires the following crate features to be activated: `Element`*"]
    pub fn closest(this: &Element, selector: &str) -> Result<Option<Element>, JsValue>;
    # [ wasm_bindgen ( method , structural , js_name = getAttribute ) ]
    #[doc = "The `getAttribute()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/getAttribute)\n\n*This API requires the following crate features to be activated: `Element`*"]
    pub fn get_attribute(this: &Element, name: &str) -> Option<String>;
    # [ wasm_bindgen ( method , structural , js_name = getAttributeNS ) ]
    #[doc = "The `getAttributeNS()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/getAttributeNS)\n\n*This API requires the following crate features to be activated: `Element`*"]
    pub fn get_attribute_ns(
        this: &Element,
        namespace: Option<&str>,
        local_name: &str,
    ) -> Option<String>;
    # [ wasm_bindgen ( method , structural , js_name = getAttributeNames ) ]
    #[doc = "The `getAttributeNames()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/getAttributeNames)\n\n*This API requires the following crate features to be activated: `Element`*"]
    pub fn get_attribute_names(this: &Element) -> ::js_sys::Array;
    #[cfg(feature = "Attr")]
    # [ wasm_bindgen ( method , structural , js_name = getAttributeNode ) ]
    #[doc = "The `getAttributeNode()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/getAttributeNode)\n\n*This API requires the following crate features to be activated: `Attr`, `Element`*"]
    pub fn get_attribute_node(this: &Element, name: &str) -> Option<Attr>;
    #[cfg(feature = "Attr")]
    # [ wasm_bindgen ( method , structural , js_name = getAttributeNodeNS ) ]
    #[doc = "The `getAttributeNodeNS()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/getAttributeNodeNS)\n\n*This API requires the following crate features to be activated: `Attr`, `Element`*"]
    pub fn get_attribute_node_ns(
        this: &Element,
        namespace_uri: Option<&str>,
        local_name: &str,
    ) -> Option<Attr>;
    #[cfg(feature = "DomRect")]
    # [ wasm_bindgen ( method , structural , js_name = getBoundingClientRect ) ]
    #[doc = "The `getBoundingClientRect()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/getBoundingClientRect)\n\n*This API requires the following crate features to be activated: `DomRect`, `Element`*"]
    pub fn get_bounding_client_rect(this: &Element) -> DomRect;
    #[cfg(feature = "DomRectList")]
    # [ wasm_bindgen ( method , structural , js_name = getClientRects ) ]
    #[doc = "The `getClientRects()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/getClientRects)\n\n*This API requires the following crate features to be activated: `DomRectList`, `Element`*"]
    pub fn get_client_rects(this: &Element) -> DomRectList;
    #[cfg(feature = "HtmlCollection")]
    # [ wasm_bindgen ( method , structural , js_name = getElementsByClassName ) ]
    #[doc = "The `getElementsByClassName()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/getElementsByClassName)\n\n*This API requires the following crate features to be activated: `Element`, `HtmlCollection`*"]
    pub fn get_elements_by_class_name(this: &Element, class_names: &str) -> HtmlCollection;
    #[cfg(feature = "HtmlCollection")]
    # [ wasm_bindgen ( method , structural , js_name = getElementsByTagName ) ]
    #[doc = "The `getElementsByTagName()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/getElementsByTagName)\n\n*This API requires the following crate features to be activated: `Element`, `HtmlCollection`*"]
    pub fn get_elements_by_tag_name(this: &Element, local_name: &str) -> HtmlCollection;
    #[cfg(feature = "HtmlCollection")]
    # [ wasm_bindgen ( catch , method , structural , js_name = getElementsByTagNameNS ) ]
    #[doc = "The `getElementsByTagNameNS()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/getElementsByTagNameNS)\n\n*This API requires the following crate features to be activated: `Element`, `HtmlCollection`*"]
    pub fn get_elements_by_tag_name_ns(
        this: &Element,
        namespace: Option<&str>,
        local_name: &str,
    ) -> Result<HtmlCollection, JsValue>;
    # [ wasm_bindgen ( method , structural , js_name = hasAttribute ) ]
    #[doc = "The `hasAttribute()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/hasAttribute)\n\n*This API requires the following crate features to be activated: `Element`*"]
    pub fn has_attribute(this: &Element, name: &str) -> bool;
    # [ wasm_bindgen ( method , structural , js_name = hasAttributeNS ) ]
    #[doc = "The `hasAttributeNS()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/hasAttributeNS)\n\n*This API requires the following crate features to be activated: `Element`*"]
    pub fn has_attribute_ns(this: &Element, namespace: Option<&str>, local_name: &str) -> bool;
    # [ wasm_bindgen ( method , structural , js_name = hasAttributes ) ]
    #[doc = "The `hasAttributes()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/hasAttributes)\n\n*This API requires the following crate features to be activated: `Element`*"]
    pub fn has_attributes(this: &Element) -> bool;
    # [ wasm_bindgen ( method , structural , js_name = hasPointerCapture ) ]
    #[doc = "The `hasPointerCapture()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/hasPointerCapture)\n\n*This API requires the following crate features to be activated: `Element`*"]
    pub fn has_pointer_capture(this: &Element, pointer_id: i32) -> bool;
    # [ wasm_bindgen ( catch , method , structural , js_name = insertAdjacentElement ) ]
    #[doc = "The `insertAdjacentElement()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/insertAdjacentElement)\n\n*This API requires the following crate features to be activated: `Element`*"]
    pub fn insert_adjacent_element(
        this: &Element,
        where_: &str,
        element: &Element,
    ) -> Result<Option<Element>, JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = insertAdjacentHTML ) ]
    #[doc = "The `insertAdjacentHTML()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/insertAdjacentHTML)\n\n*This API requires the following crate features to be activated: `Element`*"]
    pub fn insert_adjacent_html(this: &Element, position: &str, text: &str) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = insertAdjacentText ) ]
    #[doc = "The `insertAdjacentText()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/insertAdjacentText)\n\n*This API requires the following crate features to be activated: `Element`*"]
    pub fn insert_adjacent_text(this: &Element, where_: &str, data: &str) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = matches ) ]
    #[doc = "The `matches()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/matches)\n\n*This API requires the following crate features to be activated: `Element`*"]
    pub fn matches(this: &Element, selector: &str) -> Result<bool, JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = querySelector ) ]
    #[doc = "The `querySelector()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/querySelector)\n\n*This API requires the following crate features to be activated: `Element`*"]
    pub fn query_selector(this: &Element, selectors: &str) -> Result<Option<Element>, JsValue>;
    #[cfg(feature = "NodeList")]
    # [ wasm_bindgen ( catch , method , structural , js_name = querySelectorAll ) ]
    #[doc = "The `querySelectorAll()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/querySelectorAll)\n\n*This API requires the following crate features to be activated: `Element`, `NodeList`*"]
    pub fn query_selector_all(this: &Element, selectors: &str) -> Result<NodeList, JsValue>;
    # [ wasm_bindgen ( method , structural , js_name = releaseCapture ) ]
    #[doc = "The `releaseCapture()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/releaseCapture)\n\n*This API requires the following crate features to be activated: `Element`*"]
    pub fn release_capture(this: &Element);
    # [ wasm_bindgen ( catch , method , structural , js_name = releasePointerCapture ) ]
    #[doc = "The `releasePointerCapture()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/releasePointerCapture)\n\n*This API requires the following crate features to be activated: `Element`*"]
    pub fn release_pointer_capture(this: &Element, pointer_id: i32) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = removeAttribute ) ]
    #[doc = "The `removeAttribute()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/removeAttribute)\n\n*This API requires the following crate features to be activated: `Element`*"]
    pub fn remove_attribute(this: &Element, name: &str) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = removeAttributeNS ) ]
    #[doc = "The `removeAttributeNS()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/removeAttributeNS)\n\n*This API requires the following crate features to be activated: `Element`*"]
    pub fn remove_attribute_ns(
        this: &Element,
        namespace: Option<&str>,
        local_name: &str,
    ) -> Result<(), JsValue>;
    #[cfg(feature = "Attr")]
    # [ wasm_bindgen ( catch , method , structural , js_name = removeAttributeNode ) ]
    #[doc = "The `removeAttributeNode()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/removeAttributeNode)\n\n*This API requires the following crate features to be activated: `Attr`, `Element`*"]
    pub fn remove_attribute_node(this: &Element, old_attr: &Attr) -> Result<Option<Attr>, JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = requestFullscreen ) ]
    #[doc = "The `requestFullscreen()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/requestFullscreen)\n\n*This API requires the following crate features to be activated: `Element`*"]
    pub fn request_fullscreen(this: &Element) -> Result<(), JsValue>;
    # [ wasm_bindgen ( method , structural , js_name = requestPointerLock ) ]
    #[doc = "The `requestPointerLock()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/requestPointerLock)\n\n*This API requires the following crate features to be activated: `Element`*"]
    pub fn request_pointer_lock(this: &Element);
    # [ wasm_bindgen ( method , structural , js_name = scroll ) ]
    #[doc = "The `scroll()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/scroll)\n\n*This API requires the following crate features to be activated: `Element`*"]
    pub fn scroll_with_x_and_y(this: &Element, x: f64, y: f64);
    # [ wasm_bindgen ( method , structural , js_name = scroll ) ]
    #[doc = "The `scroll()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/scroll)\n\n*This API requires the following crate features to be activated: `Element`*"]
    pub fn scroll(this: &Element);
    #[cfg(feature = "ScrollToOptions")]
    # [ wasm_bindgen ( method , structural , js_name = scroll ) ]
    #[doc = "The `scroll()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/scroll)\n\n*This API requires the following crate features to be activated: `Element`, `ScrollToOptions`*"]
    pub fn scroll_with_scroll_to_options(this: &Element, options: &ScrollToOptions);
    # [ wasm_bindgen ( method , structural , js_name = scrollBy ) ]
    #[doc = "The `scrollBy()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/scrollBy)\n\n*This API requires the following crate features to be activated: `Element`*"]
    pub fn scroll_by_with_x_and_y(this: &Element, x: f64, y: f64);
    # [ wasm_bindgen ( method , structural , js_name = scrollBy ) ]
    #[doc = "The `scrollBy()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/scrollBy)\n\n*This API requires the following crate features to be activated: `Element`*"]
    pub fn scroll_by(this: &Element);
    #[cfg(feature = "ScrollToOptions")]
    # [ wasm_bindgen ( method , structural , js_name = scrollBy ) ]
    #[doc = "The `scrollBy()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/scrollBy)\n\n*This API requires the following crate features to be activated: `Element`, `ScrollToOptions`*"]
    pub fn scroll_by_with_scroll_to_options(this: &Element, options: &ScrollToOptions);
    # [ wasm_bindgen ( method , structural , js_name = scrollIntoView ) ]
    #[doc = "The `scrollIntoView()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/scrollIntoView)\n\n*This API requires the following crate features to be activated: `Element`*"]
    pub fn scroll_into_view(this: &Element);
    # [ wasm_bindgen ( method , structural , js_name = scrollIntoView ) ]
    #[doc = "The `scrollIntoView()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/scrollIntoView)\n\n*This API requires the following crate features to be activated: `Element`*"]
    pub fn scroll_into_view_with_bool(this: &Element, arg: bool);
    #[cfg(feature = "ScrollIntoViewOptions")]
    # [ wasm_bindgen ( method , structural , js_name = scrollIntoView ) ]
    #[doc = "The `scrollIntoView()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/scrollIntoView)\n\n*This API requires the following crate features to be activated: `Element`, `ScrollIntoViewOptions`*"]
    pub fn scroll_into_view_with_scroll_into_view_options(
        this: &Element,
        arg: &ScrollIntoViewOptions,
    );
    # [ wasm_bindgen ( method , structural , js_name = scrollTo ) ]
    #[doc = "The `scrollTo()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/scrollTo)\n\n*This API requires the following crate features to be activated: `Element`*"]
    pub fn scroll_to_with_x_and_y(this: &Element, x: f64, y: f64);
    # [ wasm_bindgen ( method , structural , js_name = scrollTo ) ]
    #[doc = "The `scrollTo()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/scrollTo)\n\n*This API requires the following crate features to be activated: `Element`*"]
    pub fn scroll_to(this: &Element);
    #[cfg(feature = "ScrollToOptions")]
    # [ wasm_bindgen ( method , structural , js_name = scrollTo ) ]
    #[doc = "The `scrollTo()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/scrollTo)\n\n*This API requires the following crate features to be activated: `Element`, `ScrollToOptions`*"]
    pub fn scroll_to_with_scroll_to_options(this: &Element, options: &ScrollToOptions);
    # [ wasm_bindgen ( catch , method , structural , js_name = setAttribute ) ]
    #[doc = "The `setAttribute()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/setAttribute)\n\n*This API requires the following crate features to be activated: `Element`*"]
    pub fn set_attribute(this: &Element, name: &str, value: &str) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = setAttributeNS ) ]
    #[doc = "The `setAttributeNS()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/setAttributeNS)\n\n*This API requires the following crate features to be activated: `Element`*"]
    pub fn set_attribute_ns(
        this: &Element,
        namespace: Option<&str>,
        name: &str,
        value: &str,
    ) -> Result<(), JsValue>;
    #[cfg(feature = "Attr")]
    # [ wasm_bindgen ( catch , method , structural , js_name = setAttributeNode ) ]
    #[doc = "The `setAttributeNode()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/setAttributeNode)\n\n*This API requires the following crate features to be activated: `Attr`, `Element`*"]
    pub fn set_attribute_node(this: &Element, new_attr: &Attr) -> Result<Option<Attr>, JsValue>;
    #[cfg(feature = "Attr")]
    # [ wasm_bindgen ( catch , method , structural , js_name = setAttributeNodeNS ) ]
    #[doc = "The `setAttributeNodeNS()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/setAttributeNodeNS)\n\n*This API requires the following crate features to be activated: `Attr`, `Element`*"]
    pub fn set_attribute_node_ns(this: &Element, new_attr: &Attr) -> Result<Option<Attr>, JsValue>;
    # [ wasm_bindgen ( method , structural , js_name = setCapture ) ]
    #[doc = "The `setCapture()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/setCapture)\n\n*This API requires the following crate features to be activated: `Element`*"]
    pub fn set_capture(this: &Element);
    # [ wasm_bindgen ( method , structural , js_name = setCapture ) ]
    #[doc = "The `setCapture()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/setCapture)\n\n*This API requires the following crate features to be activated: `Element`*"]
    pub fn set_capture_with_retarget_to_element(this: &Element, retarget_to_element: bool);
    # [ wasm_bindgen ( catch , method , structural , js_name = setPointerCapture ) ]
    #[doc = "The `setPointerCapture()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/setPointerCapture)\n\n*This API requires the following crate features to be activated: `Element`*"]
    pub fn set_pointer_capture(this: &Element, pointer_id: i32) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = toggleAttribute ) ]
    #[doc = "The `toggleAttribute()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/toggleAttribute)\n\n*This API requires the following crate features to be activated: `Element`*"]
    pub fn toggle_attribute(this: &Element, name: &str) -> Result<bool, JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = toggleAttribute ) ]
    #[doc = "The `toggleAttribute()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/toggleAttribute)\n\n*This API requires the following crate features to be activated: `Element`*"]
    pub fn toggle_attribute_with_force(
        this: &Element,
        name: &str,
        force: bool,
    ) -> Result<bool, JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = webkitMatchesSelector ) ]
    #[doc = "The `webkitMatchesSelector()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/webkitMatchesSelector)\n\n*This API requires the following crate features to be activated: `Element`*"]
    pub fn webkit_matches_selector(this: &Element, selector: &str) -> Result<bool, JsValue>;
    #[cfg(feature = "Node")]
    # [ wasm_bindgen ( catch , method , structural , variadic , js_name = after ) ]
    #[doc = "The `after()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/after)\n\n*This API requires the following crate features to be activated: `Element`, `Node`*"]
    pub fn after_with_node(this: &Element, nodes: &Node) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = after ) ]
    #[doc = "The `after()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/after)\n\n*This API requires the following crate features to be activated: `Element`*"]
    pub fn after_with_node_0(this: &Element) -> Result<(), JsValue>;
    #[cfg(feature = "Node")]
    # [ wasm_bindgen ( catch , method , structural , js_name = after ) ]
    #[doc = "The `after()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/after)\n\n*This API requires the following crate features to be activated: `Element`, `Node`*"]
    pub fn after_with_node_1(this: &Element, nodes_1: &Node) -> Result<(), JsValue>;
    #[cfg(feature = "Node")]
    # [ wasm_bindgen ( catch , method , structural , js_name = after ) ]
    #[doc = "The `after()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/after)\n\n*This API requires the following crate features to be activated: `Element`, `Node`*"]
    pub fn after_with_node_2(this: &Element, nodes_1: &Node, nodes_2: &Node)
        -> Result<(), JsValue>;
    #[cfg(feature = "Node")]
    # [ wasm_bindgen ( catch , method , structural , js_name = after ) ]
    #[doc = "The `after()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/after)\n\n*This API requires the following crate features to be activated: `Element`, `Node`*"]
    pub fn after_with_node_3(
        this: &Element,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
    ) -> Result<(), JsValue>;
    #[cfg(feature = "Node")]
    # [ wasm_bindgen ( catch , method , structural , js_name = after ) ]
    #[doc = "The `after()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/after)\n\n*This API requires the following crate features to be activated: `Element`, `Node`*"]
    pub fn after_with_node_4(
        this: &Element,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
        nodes_4: &Node,
    ) -> Result<(), JsValue>;
    #[cfg(feature = "Node")]
    # [ wasm_bindgen ( catch , method , structural , js_name = after ) ]
    #[doc = "The `after()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/after)\n\n*This API requires the following crate features to be activated: `Element`, `Node`*"]
    pub fn after_with_node_5(
        this: &Element,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
        nodes_4: &Node,
        nodes_5: &Node,
    ) -> Result<(), JsValue>;
    #[cfg(feature = "Node")]
    # [ wasm_bindgen ( catch , method , structural , js_name = after ) ]
    #[doc = "The `after()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/after)\n\n*This API requires the following crate features to be activated: `Element`, `Node`*"]
    pub fn after_with_node_6(
        this: &Element,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
        nodes_4: &Node,
        nodes_5: &Node,
        nodes_6: &Node,
    ) -> Result<(), JsValue>;
    #[cfg(feature = "Node")]
    # [ wasm_bindgen ( catch , method , structural , js_name = after ) ]
    #[doc = "The `after()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/after)\n\n*This API requires the following crate features to be activated: `Element`, `Node`*"]
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
    # [ wasm_bindgen ( catch , method , structural , variadic , js_name = after ) ]
    #[doc = "The `after()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/after)\n\n*This API requires the following crate features to be activated: `Element`*"]
    pub fn after_with_str(this: &Element, nodes: &str) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = after ) ]
    #[doc = "The `after()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/after)\n\n*This API requires the following crate features to be activated: `Element`*"]
    pub fn after_with_str_0(this: &Element) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = after ) ]
    #[doc = "The `after()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/after)\n\n*This API requires the following crate features to be activated: `Element`*"]
    pub fn after_with_str_1(this: &Element, nodes_1: &str) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = after ) ]
    #[doc = "The `after()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/after)\n\n*This API requires the following crate features to be activated: `Element`*"]
    pub fn after_with_str_2(this: &Element, nodes_1: &str, nodes_2: &str) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = after ) ]
    #[doc = "The `after()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/after)\n\n*This API requires the following crate features to be activated: `Element`*"]
    pub fn after_with_str_3(
        this: &Element,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
    ) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = after ) ]
    #[doc = "The `after()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/after)\n\n*This API requires the following crate features to be activated: `Element`*"]
    pub fn after_with_str_4(
        this: &Element,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
        nodes_4: &str,
    ) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = after ) ]
    #[doc = "The `after()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/after)\n\n*This API requires the following crate features to be activated: `Element`*"]
    pub fn after_with_str_5(
        this: &Element,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
        nodes_4: &str,
        nodes_5: &str,
    ) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = after ) ]
    #[doc = "The `after()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/after)\n\n*This API requires the following crate features to be activated: `Element`*"]
    pub fn after_with_str_6(
        this: &Element,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
        nodes_4: &str,
        nodes_5: &str,
        nodes_6: &str,
    ) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = after ) ]
    #[doc = "The `after()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/after)\n\n*This API requires the following crate features to be activated: `Element`*"]
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
    #[cfg(feature = "Node")]
    # [ wasm_bindgen ( catch , method , structural , variadic , js_name = before ) ]
    #[doc = "The `before()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/before)\n\n*This API requires the following crate features to be activated: `Element`, `Node`*"]
    pub fn before_with_node(this: &Element, nodes: &Node) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = before ) ]
    #[doc = "The `before()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/before)\n\n*This API requires the following crate features to be activated: `Element`*"]
    pub fn before_with_node_0(this: &Element) -> Result<(), JsValue>;
    #[cfg(feature = "Node")]
    # [ wasm_bindgen ( catch , method , structural , js_name = before ) ]
    #[doc = "The `before()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/before)\n\n*This API requires the following crate features to be activated: `Element`, `Node`*"]
    pub fn before_with_node_1(this: &Element, nodes_1: &Node) -> Result<(), JsValue>;
    #[cfg(feature = "Node")]
    # [ wasm_bindgen ( catch , method , structural , js_name = before ) ]
    #[doc = "The `before()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/before)\n\n*This API requires the following crate features to be activated: `Element`, `Node`*"]
    pub fn before_with_node_2(
        this: &Element,
        nodes_1: &Node,
        nodes_2: &Node,
    ) -> Result<(), JsValue>;
    #[cfg(feature = "Node")]
    # [ wasm_bindgen ( catch , method , structural , js_name = before ) ]
    #[doc = "The `before()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/before)\n\n*This API requires the following crate features to be activated: `Element`, `Node`*"]
    pub fn before_with_node_3(
        this: &Element,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
    ) -> Result<(), JsValue>;
    #[cfg(feature = "Node")]
    # [ wasm_bindgen ( catch , method , structural , js_name = before ) ]
    #[doc = "The `before()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/before)\n\n*This API requires the following crate features to be activated: `Element`, `Node`*"]
    pub fn before_with_node_4(
        this: &Element,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
        nodes_4: &Node,
    ) -> Result<(), JsValue>;
    #[cfg(feature = "Node")]
    # [ wasm_bindgen ( catch , method , structural , js_name = before ) ]
    #[doc = "The `before()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/before)\n\n*This API requires the following crate features to be activated: `Element`, `Node`*"]
    pub fn before_with_node_5(
        this: &Element,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
        nodes_4: &Node,
        nodes_5: &Node,
    ) -> Result<(), JsValue>;
    #[cfg(feature = "Node")]
    # [ wasm_bindgen ( catch , method , structural , js_name = before ) ]
    #[doc = "The `before()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/before)\n\n*This API requires the following crate features to be activated: `Element`, `Node`*"]
    pub fn before_with_node_6(
        this: &Element,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
        nodes_4: &Node,
        nodes_5: &Node,
        nodes_6: &Node,
    ) -> Result<(), JsValue>;
    #[cfg(feature = "Node")]
    # [ wasm_bindgen ( catch , method , structural , js_name = before ) ]
    #[doc = "The `before()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/before)\n\n*This API requires the following crate features to be activated: `Element`, `Node`*"]
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
    # [ wasm_bindgen ( catch , method , structural , variadic , js_name = before ) ]
    #[doc = "The `before()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/before)\n\n*This API requires the following crate features to be activated: `Element`*"]
    pub fn before_with_str(this: &Element, nodes: &str) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = before ) ]
    #[doc = "The `before()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/before)\n\n*This API requires the following crate features to be activated: `Element`*"]
    pub fn before_with_str_0(this: &Element) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = before ) ]
    #[doc = "The `before()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/before)\n\n*This API requires the following crate features to be activated: `Element`*"]
    pub fn before_with_str_1(this: &Element, nodes_1: &str) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = before ) ]
    #[doc = "The `before()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/before)\n\n*This API requires the following crate features to be activated: `Element`*"]
    pub fn before_with_str_2(this: &Element, nodes_1: &str, nodes_2: &str) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = before ) ]
    #[doc = "The `before()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/before)\n\n*This API requires the following crate features to be activated: `Element`*"]
    pub fn before_with_str_3(
        this: &Element,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
    ) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = before ) ]
    #[doc = "The `before()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/before)\n\n*This API requires the following crate features to be activated: `Element`*"]
    pub fn before_with_str_4(
        this: &Element,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
        nodes_4: &str,
    ) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = before ) ]
    #[doc = "The `before()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/before)\n\n*This API requires the following crate features to be activated: `Element`*"]
    pub fn before_with_str_5(
        this: &Element,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
        nodes_4: &str,
        nodes_5: &str,
    ) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = before ) ]
    #[doc = "The `before()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/before)\n\n*This API requires the following crate features to be activated: `Element`*"]
    pub fn before_with_str_6(
        this: &Element,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
        nodes_4: &str,
        nodes_5: &str,
        nodes_6: &str,
    ) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = before ) ]
    #[doc = "The `before()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/before)\n\n*This API requires the following crate features to be activated: `Element`*"]
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
    # [ wasm_bindgen ( method , structural , js_name = remove ) ]
    #[doc = "The `remove()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/remove)\n\n*This API requires the following crate features to be activated: `Element`*"]
    pub fn remove(this: &Element);
    #[cfg(feature = "Node")]
    # [ wasm_bindgen ( catch , method , structural , variadic , js_name = replaceWith ) ]
    #[doc = "The `replaceWith()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/replaceWith)\n\n*This API requires the following crate features to be activated: `Element`, `Node`*"]
    pub fn replace_with_with_node(this: &Element, nodes: &Node) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = replaceWith ) ]
    #[doc = "The `replaceWith()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/replaceWith)\n\n*This API requires the following crate features to be activated: `Element`*"]
    pub fn replace_with_with_node_0(this: &Element) -> Result<(), JsValue>;
    #[cfg(feature = "Node")]
    # [ wasm_bindgen ( catch , method , structural , js_name = replaceWith ) ]
    #[doc = "The `replaceWith()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/replaceWith)\n\n*This API requires the following crate features to be activated: `Element`, `Node`*"]
    pub fn replace_with_with_node_1(this: &Element, nodes_1: &Node) -> Result<(), JsValue>;
    #[cfg(feature = "Node")]
    # [ wasm_bindgen ( catch , method , structural , js_name = replaceWith ) ]
    #[doc = "The `replaceWith()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/replaceWith)\n\n*This API requires the following crate features to be activated: `Element`, `Node`*"]
    pub fn replace_with_with_node_2(
        this: &Element,
        nodes_1: &Node,
        nodes_2: &Node,
    ) -> Result<(), JsValue>;
    #[cfg(feature = "Node")]
    # [ wasm_bindgen ( catch , method , structural , js_name = replaceWith ) ]
    #[doc = "The `replaceWith()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/replaceWith)\n\n*This API requires the following crate features to be activated: `Element`, `Node`*"]
    pub fn replace_with_with_node_3(
        this: &Element,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
    ) -> Result<(), JsValue>;
    #[cfg(feature = "Node")]
    # [ wasm_bindgen ( catch , method , structural , js_name = replaceWith ) ]
    #[doc = "The `replaceWith()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/replaceWith)\n\n*This API requires the following crate features to be activated: `Element`, `Node`*"]
    pub fn replace_with_with_node_4(
        this: &Element,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
        nodes_4: &Node,
    ) -> Result<(), JsValue>;
    #[cfg(feature = "Node")]
    # [ wasm_bindgen ( catch , method , structural , js_name = replaceWith ) ]
    #[doc = "The `replaceWith()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/replaceWith)\n\n*This API requires the following crate features to be activated: `Element`, `Node`*"]
    pub fn replace_with_with_node_5(
        this: &Element,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
        nodes_4: &Node,
        nodes_5: &Node,
    ) -> Result<(), JsValue>;
    #[cfg(feature = "Node")]
    # [ wasm_bindgen ( catch , method , structural , js_name = replaceWith ) ]
    #[doc = "The `replaceWith()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/replaceWith)\n\n*This API requires the following crate features to be activated: `Element`, `Node`*"]
    pub fn replace_with_with_node_6(
        this: &Element,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
        nodes_4: &Node,
        nodes_5: &Node,
        nodes_6: &Node,
    ) -> Result<(), JsValue>;
    #[cfg(feature = "Node")]
    # [ wasm_bindgen ( catch , method , structural , js_name = replaceWith ) ]
    #[doc = "The `replaceWith()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/replaceWith)\n\n*This API requires the following crate features to be activated: `Element`, `Node`*"]
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
    # [ wasm_bindgen ( catch , method , structural , variadic , js_name = replaceWith ) ]
    #[doc = "The `replaceWith()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/replaceWith)\n\n*This API requires the following crate features to be activated: `Element`*"]
    pub fn replace_with_with_str(this: &Element, nodes: &str) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = replaceWith ) ]
    #[doc = "The `replaceWith()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/replaceWith)\n\n*This API requires the following crate features to be activated: `Element`*"]
    pub fn replace_with_with_str_0(this: &Element) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = replaceWith ) ]
    #[doc = "The `replaceWith()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/replaceWith)\n\n*This API requires the following crate features to be activated: `Element`*"]
    pub fn replace_with_with_str_1(this: &Element, nodes_1: &str) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = replaceWith ) ]
    #[doc = "The `replaceWith()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/replaceWith)\n\n*This API requires the following crate features to be activated: `Element`*"]
    pub fn replace_with_with_str_2(
        this: &Element,
        nodes_1: &str,
        nodes_2: &str,
    ) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = replaceWith ) ]
    #[doc = "The `replaceWith()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/replaceWith)\n\n*This API requires the following crate features to be activated: `Element`*"]
    pub fn replace_with_with_str_3(
        this: &Element,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
    ) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = replaceWith ) ]
    #[doc = "The `replaceWith()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/replaceWith)\n\n*This API requires the following crate features to be activated: `Element`*"]
    pub fn replace_with_with_str_4(
        this: &Element,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
        nodes_4: &str,
    ) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = replaceWith ) ]
    #[doc = "The `replaceWith()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/replaceWith)\n\n*This API requires the following crate features to be activated: `Element`*"]
    pub fn replace_with_with_str_5(
        this: &Element,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
        nodes_4: &str,
        nodes_5: &str,
    ) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = replaceWith ) ]
    #[doc = "The `replaceWith()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/replaceWith)\n\n*This API requires the following crate features to be activated: `Element`*"]
    pub fn replace_with_with_str_6(
        this: &Element,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
        nodes_4: &str,
        nodes_5: &str,
        nodes_6: &str,
    ) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = replaceWith ) ]
    #[doc = "The `replaceWith()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/replaceWith)\n\n*This API requires the following crate features to be activated: `Element`*"]
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
    # [ wasm_bindgen ( catch , method , structural , js_name = convertPointFromNode ) ]
    #[doc = "The `convertPointFromNode()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/convertPointFromNode)\n\n*This API requires the following crate features to be activated: `DomPoint`, `DomPointInit`, `Element`, `Text`*"]
    pub fn convert_point_from_node_with_text(
        this: &Element,
        point: &DomPointInit,
        from: &Text,
    ) -> Result<DomPoint, JsValue>;
    #[cfg(all(feature = "DomPoint", feature = "DomPointInit",))]
    # [ wasm_bindgen ( catch , method , structural , js_name = convertPointFromNode ) ]
    #[doc = "The `convertPointFromNode()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/convertPointFromNode)\n\n*This API requires the following crate features to be activated: `DomPoint`, `DomPointInit`, `Element`*"]
    pub fn convert_point_from_node_with_element(
        this: &Element,
        point: &DomPointInit,
        from: &Element,
    ) -> Result<DomPoint, JsValue>;
    #[cfg(all(feature = "Document", feature = "DomPoint", feature = "DomPointInit",))]
    # [ wasm_bindgen ( catch , method , structural , js_name = convertPointFromNode ) ]
    #[doc = "The `convertPointFromNode()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/convertPointFromNode)\n\n*This API requires the following crate features to be activated: `Document`, `DomPoint`, `DomPointInit`, `Element`*"]
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
    # [ wasm_bindgen ( catch , method , structural , js_name = convertPointFromNode ) ]
    #[doc = "The `convertPointFromNode()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/convertPointFromNode)\n\n*This API requires the following crate features to be activated: `ConvertCoordinateOptions`, `DomPoint`, `DomPointInit`, `Element`, `Text`*"]
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
    # [ wasm_bindgen ( catch , method , structural , js_name = convertPointFromNode ) ]
    #[doc = "The `convertPointFromNode()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/convertPointFromNode)\n\n*This API requires the following crate features to be activated: `ConvertCoordinateOptions`, `DomPoint`, `DomPointInit`, `Element`*"]
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
    # [ wasm_bindgen ( catch , method , structural , js_name = convertPointFromNode ) ]
    #[doc = "The `convertPointFromNode()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/convertPointFromNode)\n\n*This API requires the following crate features to be activated: `ConvertCoordinateOptions`, `Document`, `DomPoint`, `DomPointInit`, `Element`*"]
    pub fn convert_point_from_node_with_document_and_options(
        this: &Element,
        point: &DomPointInit,
        from: &Document,
        options: &ConvertCoordinateOptions,
    ) -> Result<DomPoint, JsValue>;
    #[cfg(all(feature = "DomQuad", feature = "Text",))]
    # [ wasm_bindgen ( catch , method , structural , js_name = convertQuadFromNode ) ]
    #[doc = "The `convertQuadFromNode()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/convertQuadFromNode)\n\n*This API requires the following crate features to be activated: `DomQuad`, `Element`, `Text`*"]
    pub fn convert_quad_from_node_with_text(
        this: &Element,
        quad: &DomQuad,
        from: &Text,
    ) -> Result<DomQuad, JsValue>;
    #[cfg(feature = "DomQuad")]
    # [ wasm_bindgen ( catch , method , structural , js_name = convertQuadFromNode ) ]
    #[doc = "The `convertQuadFromNode()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/convertQuadFromNode)\n\n*This API requires the following crate features to be activated: `DomQuad`, `Element`*"]
    pub fn convert_quad_from_node_with_element(
        this: &Element,
        quad: &DomQuad,
        from: &Element,
    ) -> Result<DomQuad, JsValue>;
    #[cfg(all(feature = "Document", feature = "DomQuad",))]
    # [ wasm_bindgen ( catch , method , structural , js_name = convertQuadFromNode ) ]
    #[doc = "The `convertQuadFromNode()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/convertQuadFromNode)\n\n*This API requires the following crate features to be activated: `Document`, `DomQuad`, `Element`*"]
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
    # [ wasm_bindgen ( catch , method , structural , js_name = convertQuadFromNode ) ]
    #[doc = "The `convertQuadFromNode()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/convertQuadFromNode)\n\n*This API requires the following crate features to be activated: `ConvertCoordinateOptions`, `DomQuad`, `Element`, `Text`*"]
    pub fn convert_quad_from_node_with_text_and_options(
        this: &Element,
        quad: &DomQuad,
        from: &Text,
        options: &ConvertCoordinateOptions,
    ) -> Result<DomQuad, JsValue>;
    #[cfg(all(feature = "ConvertCoordinateOptions", feature = "DomQuad",))]
    # [ wasm_bindgen ( catch , method , structural , js_name = convertQuadFromNode ) ]
    #[doc = "The `convertQuadFromNode()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/convertQuadFromNode)\n\n*This API requires the following crate features to be activated: `ConvertCoordinateOptions`, `DomQuad`, `Element`*"]
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
    # [ wasm_bindgen ( catch , method , structural , js_name = convertQuadFromNode ) ]
    #[doc = "The `convertQuadFromNode()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/convertQuadFromNode)\n\n*This API requires the following crate features to be activated: `ConvertCoordinateOptions`, `Document`, `DomQuad`, `Element`*"]
    pub fn convert_quad_from_node_with_document_and_options(
        this: &Element,
        quad: &DomQuad,
        from: &Document,
        options: &ConvertCoordinateOptions,
    ) -> Result<DomQuad, JsValue>;
    #[cfg(all(feature = "DomQuad", feature = "DomRectReadOnly", feature = "Text",))]
    # [ wasm_bindgen ( catch , method , structural , js_name = convertRectFromNode ) ]
    #[doc = "The `convertRectFromNode()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/convertRectFromNode)\n\n*This API requires the following crate features to be activated: `DomQuad`, `DomRectReadOnly`, `Element`, `Text`*"]
    pub fn convert_rect_from_node_with_text(
        this: &Element,
        rect: &DomRectReadOnly,
        from: &Text,
    ) -> Result<DomQuad, JsValue>;
    #[cfg(all(feature = "DomQuad", feature = "DomRectReadOnly",))]
    # [ wasm_bindgen ( catch , method , structural , js_name = convertRectFromNode ) ]
    #[doc = "The `convertRectFromNode()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/convertRectFromNode)\n\n*This API requires the following crate features to be activated: `DomQuad`, `DomRectReadOnly`, `Element`*"]
    pub fn convert_rect_from_node_with_element(
        this: &Element,
        rect: &DomRectReadOnly,
        from: &Element,
    ) -> Result<DomQuad, JsValue>;
    #[cfg(all(feature = "Document", feature = "DomQuad", feature = "DomRectReadOnly",))]
    # [ wasm_bindgen ( catch , method , structural , js_name = convertRectFromNode ) ]
    #[doc = "The `convertRectFromNode()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/convertRectFromNode)\n\n*This API requires the following crate features to be activated: `Document`, `DomQuad`, `DomRectReadOnly`, `Element`*"]
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
    # [ wasm_bindgen ( catch , method , structural , js_name = convertRectFromNode ) ]
    #[doc = "The `convertRectFromNode()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/convertRectFromNode)\n\n*This API requires the following crate features to be activated: `ConvertCoordinateOptions`, `DomQuad`, `DomRectReadOnly`, `Element`, `Text`*"]
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
    # [ wasm_bindgen ( catch , method , structural , js_name = convertRectFromNode ) ]
    #[doc = "The `convertRectFromNode()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/convertRectFromNode)\n\n*This API requires the following crate features to be activated: `ConvertCoordinateOptions`, `DomQuad`, `DomRectReadOnly`, `Element`*"]
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
    # [ wasm_bindgen ( catch , method , structural , js_name = convertRectFromNode ) ]
    #[doc = "The `convertRectFromNode()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/convertRectFromNode)\n\n*This API requires the following crate features to be activated: `ConvertCoordinateOptions`, `Document`, `DomQuad`, `DomRectReadOnly`, `Element`*"]
    pub fn convert_rect_from_node_with_document_and_options(
        this: &Element,
        rect: &DomRectReadOnly,
        from: &Document,
        options: &ConvertCoordinateOptions,
    ) -> Result<DomQuad, JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = getBoxQuads ) ]
    #[doc = "The `getBoxQuads()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/getBoxQuads)\n\n*This API requires the following crate features to be activated: `Element`*"]
    pub fn get_box_quads(this: &Element) -> Result<::js_sys::Array, JsValue>;
    #[cfg(feature = "BoxQuadOptions")]
    # [ wasm_bindgen ( catch , method , structural , js_name = getBoxQuads ) ]
    #[doc = "The `getBoxQuads()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/getBoxQuads)\n\n*This API requires the following crate features to be activated: `BoxQuadOptions`, `Element`*"]
    pub fn get_box_quads_with_options(
        this: &Element,
        options: &BoxQuadOptions,
    ) -> Result<::js_sys::Array, JsValue>;
    #[cfg(feature = "Node")]
    # [ wasm_bindgen ( catch , method , structural , variadic , js_name = append ) ]
    #[doc = "The `append()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/append)\n\n*This API requires the following crate features to be activated: `Element`, `Node`*"]
    pub fn append_with_node(this: &Element, nodes: &Node) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = append ) ]
    #[doc = "The `append()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/append)\n\n*This API requires the following crate features to be activated: `Element`*"]
    pub fn append_with_node_0(this: &Element) -> Result<(), JsValue>;
    #[cfg(feature = "Node")]
    # [ wasm_bindgen ( catch , method , structural , js_name = append ) ]
    #[doc = "The `append()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/append)\n\n*This API requires the following crate features to be activated: `Element`, `Node`*"]
    pub fn append_with_node_1(this: &Element, nodes_1: &Node) -> Result<(), JsValue>;
    #[cfg(feature = "Node")]
    # [ wasm_bindgen ( catch , method , structural , js_name = append ) ]
    #[doc = "The `append()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/append)\n\n*This API requires the following crate features to be activated: `Element`, `Node`*"]
    pub fn append_with_node_2(
        this: &Element,
        nodes_1: &Node,
        nodes_2: &Node,
    ) -> Result<(), JsValue>;
    #[cfg(feature = "Node")]
    # [ wasm_bindgen ( catch , method , structural , js_name = append ) ]
    #[doc = "The `append()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/append)\n\n*This API requires the following crate features to be activated: `Element`, `Node`*"]
    pub fn append_with_node_3(
        this: &Element,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
    ) -> Result<(), JsValue>;
    #[cfg(feature = "Node")]
    # [ wasm_bindgen ( catch , method , structural , js_name = append ) ]
    #[doc = "The `append()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/append)\n\n*This API requires the following crate features to be activated: `Element`, `Node`*"]
    pub fn append_with_node_4(
        this: &Element,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
        nodes_4: &Node,
    ) -> Result<(), JsValue>;
    #[cfg(feature = "Node")]
    # [ wasm_bindgen ( catch , method , structural , js_name = append ) ]
    #[doc = "The `append()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/append)\n\n*This API requires the following crate features to be activated: `Element`, `Node`*"]
    pub fn append_with_node_5(
        this: &Element,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
        nodes_4: &Node,
        nodes_5: &Node,
    ) -> Result<(), JsValue>;
    #[cfg(feature = "Node")]
    # [ wasm_bindgen ( catch , method , structural , js_name = append ) ]
    #[doc = "The `append()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/append)\n\n*This API requires the following crate features to be activated: `Element`, `Node`*"]
    pub fn append_with_node_6(
        this: &Element,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
        nodes_4: &Node,
        nodes_5: &Node,
        nodes_6: &Node,
    ) -> Result<(), JsValue>;
    #[cfg(feature = "Node")]
    # [ wasm_bindgen ( catch , method , structural , js_name = append ) ]
    #[doc = "The `append()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/append)\n\n*This API requires the following crate features to be activated: `Element`, `Node`*"]
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
    # [ wasm_bindgen ( catch , method , structural , variadic , js_name = append ) ]
    #[doc = "The `append()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/append)\n\n*This API requires the following crate features to be activated: `Element`*"]
    pub fn append_with_str(this: &Element, nodes: &str) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = append ) ]
    #[doc = "The `append()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/append)\n\n*This API requires the following crate features to be activated: `Element`*"]
    pub fn append_with_str_0(this: &Element) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = append ) ]
    #[doc = "The `append()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/append)\n\n*This API requires the following crate features to be activated: `Element`*"]
    pub fn append_with_str_1(this: &Element, nodes_1: &str) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = append ) ]
    #[doc = "The `append()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/append)\n\n*This API requires the following crate features to be activated: `Element`*"]
    pub fn append_with_str_2(this: &Element, nodes_1: &str, nodes_2: &str) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = append ) ]
    #[doc = "The `append()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/append)\n\n*This API requires the following crate features to be activated: `Element`*"]
    pub fn append_with_str_3(
        this: &Element,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
    ) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = append ) ]
    #[doc = "The `append()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/append)\n\n*This API requires the following crate features to be activated: `Element`*"]
    pub fn append_with_str_4(
        this: &Element,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
        nodes_4: &str,
    ) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = append ) ]
    #[doc = "The `append()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/append)\n\n*This API requires the following crate features to be activated: `Element`*"]
    pub fn append_with_str_5(
        this: &Element,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
        nodes_4: &str,
        nodes_5: &str,
    ) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = append ) ]
    #[doc = "The `append()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/append)\n\n*This API requires the following crate features to be activated: `Element`*"]
    pub fn append_with_str_6(
        this: &Element,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
        nodes_4: &str,
        nodes_5: &str,
        nodes_6: &str,
    ) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = append ) ]
    #[doc = "The `append()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/append)\n\n*This API requires the following crate features to be activated: `Element`*"]
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
    #[cfg(feature = "Node")]
    # [ wasm_bindgen ( catch , method , structural , variadic , js_name = prepend ) ]
    #[doc = "The `prepend()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/prepend)\n\n*This API requires the following crate features to be activated: `Element`, `Node`*"]
    pub fn prepend_with_node(this: &Element, nodes: &Node) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = prepend ) ]
    #[doc = "The `prepend()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/prepend)\n\n*This API requires the following crate features to be activated: `Element`*"]
    pub fn prepend_with_node_0(this: &Element) -> Result<(), JsValue>;
    #[cfg(feature = "Node")]
    # [ wasm_bindgen ( catch , method , structural , js_name = prepend ) ]
    #[doc = "The `prepend()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/prepend)\n\n*This API requires the following crate features to be activated: `Element`, `Node`*"]
    pub fn prepend_with_node_1(this: &Element, nodes_1: &Node) -> Result<(), JsValue>;
    #[cfg(feature = "Node")]
    # [ wasm_bindgen ( catch , method , structural , js_name = prepend ) ]
    #[doc = "The `prepend()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/prepend)\n\n*This API requires the following crate features to be activated: `Element`, `Node`*"]
    pub fn prepend_with_node_2(
        this: &Element,
        nodes_1: &Node,
        nodes_2: &Node,
    ) -> Result<(), JsValue>;
    #[cfg(feature = "Node")]
    # [ wasm_bindgen ( catch , method , structural , js_name = prepend ) ]
    #[doc = "The `prepend()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/prepend)\n\n*This API requires the following crate features to be activated: `Element`, `Node`*"]
    pub fn prepend_with_node_3(
        this: &Element,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
    ) -> Result<(), JsValue>;
    #[cfg(feature = "Node")]
    # [ wasm_bindgen ( catch , method , structural , js_name = prepend ) ]
    #[doc = "The `prepend()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/prepend)\n\n*This API requires the following crate features to be activated: `Element`, `Node`*"]
    pub fn prepend_with_node_4(
        this: &Element,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
        nodes_4: &Node,
    ) -> Result<(), JsValue>;
    #[cfg(feature = "Node")]
    # [ wasm_bindgen ( catch , method , structural , js_name = prepend ) ]
    #[doc = "The `prepend()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/prepend)\n\n*This API requires the following crate features to be activated: `Element`, `Node`*"]
    pub fn prepend_with_node_5(
        this: &Element,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
        nodes_4: &Node,
        nodes_5: &Node,
    ) -> Result<(), JsValue>;
    #[cfg(feature = "Node")]
    # [ wasm_bindgen ( catch , method , structural , js_name = prepend ) ]
    #[doc = "The `prepend()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/prepend)\n\n*This API requires the following crate features to be activated: `Element`, `Node`*"]
    pub fn prepend_with_node_6(
        this: &Element,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
        nodes_4: &Node,
        nodes_5: &Node,
        nodes_6: &Node,
    ) -> Result<(), JsValue>;
    #[cfg(feature = "Node")]
    # [ wasm_bindgen ( catch , method , structural , js_name = prepend ) ]
    #[doc = "The `prepend()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/prepend)\n\n*This API requires the following crate features to be activated: `Element`, `Node`*"]
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
    # [ wasm_bindgen ( catch , method , structural , variadic , js_name = prepend ) ]
    #[doc = "The `prepend()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/prepend)\n\n*This API requires the following crate features to be activated: `Element`*"]
    pub fn prepend_with_str(this: &Element, nodes: &str) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = prepend ) ]
    #[doc = "The `prepend()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/prepend)\n\n*This API requires the following crate features to be activated: `Element`*"]
    pub fn prepend_with_str_0(this: &Element) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = prepend ) ]
    #[doc = "The `prepend()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/prepend)\n\n*This API requires the following crate features to be activated: `Element`*"]
    pub fn prepend_with_str_1(this: &Element, nodes_1: &str) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = prepend ) ]
    #[doc = "The `prepend()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/prepend)\n\n*This API requires the following crate features to be activated: `Element`*"]
    pub fn prepend_with_str_2(this: &Element, nodes_1: &str, nodes_2: &str) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = prepend ) ]
    #[doc = "The `prepend()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/prepend)\n\n*This API requires the following crate features to be activated: `Element`*"]
    pub fn prepend_with_str_3(
        this: &Element,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
    ) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = prepend ) ]
    #[doc = "The `prepend()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/prepend)\n\n*This API requires the following crate features to be activated: `Element`*"]
    pub fn prepend_with_str_4(
        this: &Element,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
        nodes_4: &str,
    ) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = prepend ) ]
    #[doc = "The `prepend()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/prepend)\n\n*This API requires the following crate features to be activated: `Element`*"]
    pub fn prepend_with_str_5(
        this: &Element,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
        nodes_4: &str,
        nodes_5: &str,
    ) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = prepend ) ]
    #[doc = "The `prepend()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/prepend)\n\n*This API requires the following crate features to be activated: `Element`*"]
    pub fn prepend_with_str_6(
        this: &Element,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
        nodes_4: &str,
        nodes_5: &str,
        nodes_6: &str,
    ) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = prepend ) ]
    #[doc = "The `prepend()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/prepend)\n\n*This API requires the following crate features to be activated: `Element`*"]
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
