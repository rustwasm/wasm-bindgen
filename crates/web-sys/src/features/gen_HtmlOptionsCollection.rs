use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = HtmlCollection , extends = :: js_sys :: Object , js_name = HTMLOptionsCollection , typescript_type = "HTMLOptionsCollection" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `HtmlOptionsCollection` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOptionsCollection)
    ///
    ///*This API requires the following crate features to be activated: `HtmlOptionsCollection`*
    pub type HtmlOptionsCollection;

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLOptionsCollection" , js_name = length ) ]
    ///Getter for the `length` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOptionsCollection/length)
    ///
    ///*This API requires the following crate features to be activated: `HtmlOptionsCollection`*
    pub fn length(this: &HtmlOptionsCollection) -> u32;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLOptionsCollection" , js_name = length ) ]
    ///Setter for the `length` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOptionsCollection/length)
    ///
    ///*This API requires the following crate features to be activated: `HtmlOptionsCollection`*
    pub fn set_length(this: &HtmlOptionsCollection, value: u32);

    # [ wasm_bindgen ( structural , catch , method , getter , js_class = "HTMLOptionsCollection" , js_name = selectedIndex ) ]
    ///Getter for the `selectedIndex` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOptionsCollection/selectedIndex)
    ///
    ///*This API requires the following crate features to be activated: `HtmlOptionsCollection`*
    pub fn selected_index(this: &HtmlOptionsCollection) -> Result<i32, JsValue>;

    # [ wasm_bindgen ( structural , catch , method , setter , js_class = "HTMLOptionsCollection" , js_name = selectedIndex ) ]
    ///Setter for the `selectedIndex` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOptionsCollection/selectedIndex)
    ///
    ///*This API requires the following crate features to be activated: `HtmlOptionsCollection`*
    pub fn set_selected_index(this: &HtmlOptionsCollection, value: i32) -> Result<(), JsValue>;

    #[cfg(feature = "HtmlOptionElement")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "HTMLOptionsCollection" , js_name = add ) ]
    ///The `add()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOptionsCollection/add)
    ///
    ///*This API requires the following crate features to be activated: `HtmlOptionElement`, `HtmlOptionsCollection`*
    pub fn add_with_html_option_element(
        this: &HtmlOptionsCollection,
        element: &HtmlOptionElement,
    ) -> Result<(), JsValue>;

    #[cfg(feature = "HtmlOptGroupElement")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "HTMLOptionsCollection" , js_name = add ) ]
    ///The `add()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOptionsCollection/add)
    ///
    ///*This API requires the following crate features to be activated: `HtmlOptGroupElement`, `HtmlOptionsCollection`*
    pub fn add_with_html_opt_group_element(
        this: &HtmlOptionsCollection,
        element: &HtmlOptGroupElement,
    ) -> Result<(), JsValue>;

    #[cfg(all(feature = "HtmlElement", feature = "HtmlOptionElement",))]
    # [ wasm_bindgen ( catch , method , structural , js_class = "HTMLOptionsCollection" , js_name = add ) ]
    ///The `add()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOptionsCollection/add)
    ///
    ///*This API requires the following crate features to be activated: `HtmlElement`, `HtmlOptionElement`, `HtmlOptionsCollection`*
    pub fn add_with_html_option_element_and_opt_html_element(
        this: &HtmlOptionsCollection,
        element: &HtmlOptionElement,
        before: Option<&HtmlElement>,
    ) -> Result<(), JsValue>;

    #[cfg(all(feature = "HtmlElement", feature = "HtmlOptGroupElement",))]
    # [ wasm_bindgen ( catch , method , structural , js_class = "HTMLOptionsCollection" , js_name = add ) ]
    ///The `add()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOptionsCollection/add)
    ///
    ///*This API requires the following crate features to be activated: `HtmlElement`, `HtmlOptGroupElement`, `HtmlOptionsCollection`*
    pub fn add_with_html_opt_group_element_and_opt_html_element(
        this: &HtmlOptionsCollection,
        element: &HtmlOptGroupElement,
        before: Option<&HtmlElement>,
    ) -> Result<(), JsValue>;

    #[cfg(feature = "HtmlOptionElement")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "HTMLOptionsCollection" , js_name = add ) ]
    ///The `add()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOptionsCollection/add)
    ///
    ///*This API requires the following crate features to be activated: `HtmlOptionElement`, `HtmlOptionsCollection`*
    pub fn add_with_html_option_element_and_opt_i32(
        this: &HtmlOptionsCollection,
        element: &HtmlOptionElement,
        before: Option<i32>,
    ) -> Result<(), JsValue>;

    #[cfg(feature = "HtmlOptGroupElement")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "HTMLOptionsCollection" , js_name = add ) ]
    ///The `add()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOptionsCollection/add)
    ///
    ///*This API requires the following crate features to be activated: `HtmlOptGroupElement`, `HtmlOptionsCollection`*
    pub fn add_with_html_opt_group_element_and_opt_i32(
        this: &HtmlOptionsCollection,
        element: &HtmlOptGroupElement,
        before: Option<i32>,
    ) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "HTMLOptionsCollection" , js_name = remove ) ]
    ///The `remove()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOptionsCollection/remove)
    ///
    ///*This API requires the following crate features to be activated: `HtmlOptionsCollection`*
    pub fn remove(this: &HtmlOptionsCollection, index: i32) -> Result<(), JsValue>;

    #[cfg(feature = "HtmlOptionElement")]
    #[wasm_bindgen(
        catch,
        method,
        structural,
        js_class = "HTMLOptionsCollection",
        indexing_setter
    )]
    ///Indexing setter.
    ///
    ///
    ///
    ///*This API requires the following crate features to be activated: `HtmlOptionElement`, `HtmlOptionsCollection`*
    pub fn set(
        this: &HtmlOptionsCollection,
        index: u32,
        option: Option<&HtmlOptionElement>,
    ) -> Result<(), JsValue>;

}
