use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = Event , extends = :: js_sys :: Object , js_name = StorageEvent , typescript_name = StorageEvent ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `StorageEvent` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/StorageEvent)\n\n*This API requires the following crate features to be activated: `StorageEvent`*"]
    pub type StorageEvent;
    # [ wasm_bindgen ( structural , method , getter , js_name = key ) ]
    #[doc = "Getter for the `key` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/StorageEvent/key)\n\n*This API requires the following crate features to be activated: `StorageEvent`*"]
    pub fn key(this: &StorageEvent) -> Option<String>;
    # [ wasm_bindgen ( structural , method , getter , js_name = oldValue ) ]
    #[doc = "Getter for the `oldValue` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/StorageEvent/oldValue)\n\n*This API requires the following crate features to be activated: `StorageEvent`*"]
    pub fn old_value(this: &StorageEvent) -> Option<String>;
    # [ wasm_bindgen ( structural , method , getter , js_name = newValue ) ]
    #[doc = "Getter for the `newValue` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/StorageEvent/newValue)\n\n*This API requires the following crate features to be activated: `StorageEvent`*"]
    pub fn new_value(this: &StorageEvent) -> Option<String>;
    # [ wasm_bindgen ( structural , method , getter , js_name = url ) ]
    #[doc = "Getter for the `url` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/StorageEvent/url)\n\n*This API requires the following crate features to be activated: `StorageEvent`*"]
    pub fn url(this: &StorageEvent) -> Option<String>;
    # [ wasm_bindgen ( structural , method , getter , js_name = storageArea ) ]
    #[cfg(feature = "Storage")]
    #[doc = "Getter for the `storageArea` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/StorageEvent/storageArea)\n\n*This API requires the following crate features to be activated: `Storage`, `StorageEvent`*"]
    pub fn storage_area(this: &StorageEvent) -> Option<Storage>;
    #[wasm_bindgen(catch, constructor)]
    #[doc = "The `new StorageEvent(..)` constructor, creating a new instance of `StorageEvent`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/StorageEvent/StorageEvent)\n\n*This API requires the following crate features to be activated: `StorageEvent`*"]
    pub fn new(this: &StorageEvent, type_: &str) -> Result<StorageEvent, JsValue>;
    #[cfg(feature = "StorageEventInit")]
    #[wasm_bindgen(catch, constructor)]
    #[doc = "The `new StorageEvent(..)` constructor, creating a new instance of `StorageEvent`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/StorageEvent/StorageEvent)\n\n*This API requires the following crate features to be activated: `StorageEvent`, `StorageEventInit`*"]
    pub fn new_with_event_init_dict(
        this: &StorageEvent,
        type_: &str,
        event_init_dict: &StorageEventInit,
    ) -> Result<StorageEvent, JsValue>;
    # [ wasm_bindgen ( method , structural , js_name = initStorageEvent ) ]
    #[doc = "The `initStorageEvent()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/StorageEvent/initStorageEvent)\n\n*This API requires the following crate features to be activated: `StorageEvent`*"]
    pub fn init_storage_event(this: &StorageEvent, type_: &str);
    # [ wasm_bindgen ( method , structural , js_name = initStorageEvent ) ]
    #[doc = "The `initStorageEvent()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/StorageEvent/initStorageEvent)\n\n*This API requires the following crate features to be activated: `StorageEvent`*"]
    pub fn init_storage_event_with_can_bubble(this: &StorageEvent, type_: &str, can_bubble: bool);
    # [ wasm_bindgen ( method , structural , js_name = initStorageEvent ) ]
    #[doc = "The `initStorageEvent()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/StorageEvent/initStorageEvent)\n\n*This API requires the following crate features to be activated: `StorageEvent`*"]
    pub fn init_storage_event_with_can_bubble_and_cancelable(
        this: &StorageEvent,
        type_: &str,
        can_bubble: bool,
        cancelable: bool,
    );
    # [ wasm_bindgen ( method , structural , js_name = initStorageEvent ) ]
    #[doc = "The `initStorageEvent()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/StorageEvent/initStorageEvent)\n\n*This API requires the following crate features to be activated: `StorageEvent`*"]
    pub fn init_storage_event_with_can_bubble_and_cancelable_and_key(
        this: &StorageEvent,
        type_: &str,
        can_bubble: bool,
        cancelable: bool,
        key: Option<&str>,
    );
    # [ wasm_bindgen ( method , structural , js_name = initStorageEvent ) ]
    #[doc = "The `initStorageEvent()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/StorageEvent/initStorageEvent)\n\n*This API requires the following crate features to be activated: `StorageEvent`*"]
    pub fn init_storage_event_with_can_bubble_and_cancelable_and_key_and_old_value(
        this: &StorageEvent,
        type_: &str,
        can_bubble: bool,
        cancelable: bool,
        key: Option<&str>,
        old_value: Option<&str>,
    );
    # [ wasm_bindgen ( method , structural , js_name = initStorageEvent ) ]
    #[doc = "The `initStorageEvent()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/StorageEvent/initStorageEvent)\n\n*This API requires the following crate features to be activated: `StorageEvent`*"]
    pub fn init_storage_event_with_can_bubble_and_cancelable_and_key_and_old_value_and_new_value(
        this: &StorageEvent,
        type_: &str,
        can_bubble: bool,
        cancelable: bool,
        key: Option<&str>,
        old_value: Option<&str>,
        new_value: Option<&str>,
    );
    # [ wasm_bindgen ( method , structural , js_name = initStorageEvent ) ]
    #[doc = "The `initStorageEvent()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/StorageEvent/initStorageEvent)\n\n*This API requires the following crate features to be activated: `StorageEvent`*"]
    pub fn init_storage_event_with_can_bubble_and_cancelable_and_key_and_old_value_and_new_value_and_url(
        this: &StorageEvent,
        type_: &str,
        can_bubble: bool,
        cancelable: bool,
        key: Option<&str>,
        old_value: Option<&str>,
        new_value: Option<&str>,
        url: Option<&str>,
    );
    #[cfg(feature = "Storage")]
    # [ wasm_bindgen ( method , structural , js_name = initStorageEvent ) ]
    #[doc = "The `initStorageEvent()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/StorageEvent/initStorageEvent)\n\n*This API requires the following crate features to be activated: `Storage`, `StorageEvent`*"]
    pub fn init_storage_event_with_can_bubble_and_cancelable_and_key_and_old_value_and_new_value_and_url_and_storage_area(
        this: &StorageEvent,
        type_: &str,
        can_bubble: bool,
        cancelable: bool,
        key: Option<&str>,
        old_value: Option<&str>,
        new_value: Option<&str>,
        url: Option<&str>,
        storage_area: Option<&Storage>,
    );
}
