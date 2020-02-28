use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = DOMTokenList , typescript_name = DOMTokenList ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `DomTokenList` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMTokenList)\n\n*This API requires the following crate features to be activated: `DomTokenList`*"]
    pub type DomTokenList;
    # [ wasm_bindgen ( structural , method , getter , js_name = length ) ]
    #[doc = "Getter for the `length` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMTokenList/length)\n\n*This API requires the following crate features to be activated: `DomTokenList`*"]
    pub fn length(this: &DomTokenList) -> u32;
    # [ wasm_bindgen ( structural , method , getter , js_name = value ) ]
    #[doc = "Getter for the `value` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMTokenList/value)\n\n*This API requires the following crate features to be activated: `DomTokenList`*"]
    pub fn value(this: &DomTokenList) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = value ) ]
    #[doc = "Setter for the `value` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMTokenList/value)\n\n*This API requires the following crate features to be activated: `DomTokenList`*"]
    pub fn set_value(this: &DomTokenList, value: String);
    # [ wasm_bindgen ( catch , method , structural , variadic , js_name = add ) ]
    #[doc = "The `add()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMTokenList/add)\n\n*This API requires the following crate features to be activated: `DomTokenList`*"]
    pub fn add(this: &DomTokenList, tokens: &str) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = add ) ]
    #[doc = "The `add()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMTokenList/add)\n\n*This API requires the following crate features to be activated: `DomTokenList`*"]
    pub fn add_0(this: &DomTokenList) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = add ) ]
    #[doc = "The `add()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMTokenList/add)\n\n*This API requires the following crate features to be activated: `DomTokenList`*"]
    pub fn add_1(this: &DomTokenList, tokens_1: &str) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = add ) ]
    #[doc = "The `add()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMTokenList/add)\n\n*This API requires the following crate features to be activated: `DomTokenList`*"]
    pub fn add_2(this: &DomTokenList, tokens_1: &str, tokens_2: &str) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = add ) ]
    #[doc = "The `add()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMTokenList/add)\n\n*This API requires the following crate features to be activated: `DomTokenList`*"]
    pub fn add_3(
        this: &DomTokenList,
        tokens_1: &str,
        tokens_2: &str,
        tokens_3: &str,
    ) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = add ) ]
    #[doc = "The `add()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMTokenList/add)\n\n*This API requires the following crate features to be activated: `DomTokenList`*"]
    pub fn add_4(
        this: &DomTokenList,
        tokens_1: &str,
        tokens_2: &str,
        tokens_3: &str,
        tokens_4: &str,
    ) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = add ) ]
    #[doc = "The `add()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMTokenList/add)\n\n*This API requires the following crate features to be activated: `DomTokenList`*"]
    pub fn add_5(
        this: &DomTokenList,
        tokens_1: &str,
        tokens_2: &str,
        tokens_3: &str,
        tokens_4: &str,
        tokens_5: &str,
    ) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = add ) ]
    #[doc = "The `add()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMTokenList/add)\n\n*This API requires the following crate features to be activated: `DomTokenList`*"]
    pub fn add_6(
        this: &DomTokenList,
        tokens_1: &str,
        tokens_2: &str,
        tokens_3: &str,
        tokens_4: &str,
        tokens_5: &str,
        tokens_6: &str,
    ) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = add ) ]
    #[doc = "The `add()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMTokenList/add)\n\n*This API requires the following crate features to be activated: `DomTokenList`*"]
    pub fn add_7(
        this: &DomTokenList,
        tokens_1: &str,
        tokens_2: &str,
        tokens_3: &str,
        tokens_4: &str,
        tokens_5: &str,
        tokens_6: &str,
        tokens_7: &str,
    ) -> Result<(), JsValue>;
    # [ wasm_bindgen ( method , structural , js_name = contains ) ]
    #[doc = "The `contains()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMTokenList/contains)\n\n*This API requires the following crate features to be activated: `DomTokenList`*"]
    pub fn contains(this: &DomTokenList, token: &str) -> bool;
    # [ wasm_bindgen ( method , structural , js_name = item ) ]
    #[doc = "The `item()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMTokenList/item)\n\n*This API requires the following crate features to be activated: `DomTokenList`*"]
    pub fn item(this: &DomTokenList, index: u32) -> Option<String>;
    # [ wasm_bindgen ( catch , method , structural , variadic , js_name = remove ) ]
    #[doc = "The `remove()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMTokenList/remove)\n\n*This API requires the following crate features to be activated: `DomTokenList`*"]
    pub fn remove(this: &DomTokenList, tokens: &str) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = remove ) ]
    #[doc = "The `remove()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMTokenList/remove)\n\n*This API requires the following crate features to be activated: `DomTokenList`*"]
    pub fn remove_0(this: &DomTokenList) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = remove ) ]
    #[doc = "The `remove()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMTokenList/remove)\n\n*This API requires the following crate features to be activated: `DomTokenList`*"]
    pub fn remove_1(this: &DomTokenList, tokens_1: &str) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = remove ) ]
    #[doc = "The `remove()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMTokenList/remove)\n\n*This API requires the following crate features to be activated: `DomTokenList`*"]
    pub fn remove_2(this: &DomTokenList, tokens_1: &str, tokens_2: &str) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = remove ) ]
    #[doc = "The `remove()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMTokenList/remove)\n\n*This API requires the following crate features to be activated: `DomTokenList`*"]
    pub fn remove_3(
        this: &DomTokenList,
        tokens_1: &str,
        tokens_2: &str,
        tokens_3: &str,
    ) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = remove ) ]
    #[doc = "The `remove()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMTokenList/remove)\n\n*This API requires the following crate features to be activated: `DomTokenList`*"]
    pub fn remove_4(
        this: &DomTokenList,
        tokens_1: &str,
        tokens_2: &str,
        tokens_3: &str,
        tokens_4: &str,
    ) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = remove ) ]
    #[doc = "The `remove()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMTokenList/remove)\n\n*This API requires the following crate features to be activated: `DomTokenList`*"]
    pub fn remove_5(
        this: &DomTokenList,
        tokens_1: &str,
        tokens_2: &str,
        tokens_3: &str,
        tokens_4: &str,
        tokens_5: &str,
    ) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = remove ) ]
    #[doc = "The `remove()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMTokenList/remove)\n\n*This API requires the following crate features to be activated: `DomTokenList`*"]
    pub fn remove_6(
        this: &DomTokenList,
        tokens_1: &str,
        tokens_2: &str,
        tokens_3: &str,
        tokens_4: &str,
        tokens_5: &str,
        tokens_6: &str,
    ) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = remove ) ]
    #[doc = "The `remove()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMTokenList/remove)\n\n*This API requires the following crate features to be activated: `DomTokenList`*"]
    pub fn remove_7(
        this: &DomTokenList,
        tokens_1: &str,
        tokens_2: &str,
        tokens_3: &str,
        tokens_4: &str,
        tokens_5: &str,
        tokens_6: &str,
        tokens_7: &str,
    ) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = replace ) ]
    #[doc = "The `replace()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMTokenList/replace)\n\n*This API requires the following crate features to be activated: `DomTokenList`*"]
    pub fn replace(this: &DomTokenList, token: &str, new_token: &str) -> Result<bool, JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = supports ) ]
    #[doc = "The `supports()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMTokenList/supports)\n\n*This API requires the following crate features to be activated: `DomTokenList`*"]
    pub fn supports(this: &DomTokenList, token: &str) -> Result<bool, JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = toggle ) ]
    #[doc = "The `toggle()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMTokenList/toggle)\n\n*This API requires the following crate features to be activated: `DomTokenList`*"]
    pub fn toggle(this: &DomTokenList, token: &str) -> Result<bool, JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = toggle ) ]
    #[doc = "The `toggle()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMTokenList/toggle)\n\n*This API requires the following crate features to be activated: `DomTokenList`*"]
    pub fn toggle_with_force(
        this: &DomTokenList,
        token: &str,
        force: bool,
    ) -> Result<bool, JsValue>;
    #[wasm_bindgen(method, structural, indexing_getter)]
    #[doc = "Indexing getter.\n\n\n\n*This API requires the following crate features to be activated: `DomTokenList`*"]
    pub fn get(this: &DomTokenList, index: u32) -> Option<String>;
}
