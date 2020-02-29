use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = Event , extends = :: js_sys :: Object , js_name = MutationEvent , typescript_name = MutationEvent ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `MutationEvent` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MutationEvent)
    ///
    ///*This API requires the following crate features to be activated: `MutationEvent`*
    pub type MutationEvent;

    #[cfg(feature = "Node")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "MutationEvent" , js_name = relatedNode ) ]
    ///Getter for the `relatedNode` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MutationEvent/relatedNode)
    ///
    ///*This API requires the following crate features to be activated: `MutationEvent`, `Node`*
    pub fn related_node(this: &MutationEvent) -> Option<Node>;

    # [ wasm_bindgen ( structural , method , getter , js_class = "MutationEvent" , js_name = prevValue ) ]
    ///Getter for the `prevValue` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MutationEvent/prevValue)
    ///
    ///*This API requires the following crate features to be activated: `MutationEvent`*
    pub fn prev_value(this: &MutationEvent) -> String;

    # [ wasm_bindgen ( structural , method , getter , js_class = "MutationEvent" , js_name = newValue ) ]
    ///Getter for the `newValue` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MutationEvent/newValue)
    ///
    ///*This API requires the following crate features to be activated: `MutationEvent`*
    pub fn new_value(this: &MutationEvent) -> String;

    # [ wasm_bindgen ( structural , method , getter , js_class = "MutationEvent" , js_name = attrName ) ]
    ///Getter for the `attrName` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MutationEvent/attrName)
    ///
    ///*This API requires the following crate features to be activated: `MutationEvent`*
    pub fn attr_name(this: &MutationEvent) -> String;

    # [ wasm_bindgen ( structural , method , getter , js_class = "MutationEvent" , js_name = attrChange ) ]
    ///Getter for the `attrChange` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MutationEvent/attrChange)
    ///
    ///*This API requires the following crate features to be activated: `MutationEvent`*
    pub fn attr_change(this: &MutationEvent) -> u16;

    # [ wasm_bindgen ( catch , method , structural , js_class = "MutationEvent" , js_name = initMutationEvent ) ]
    ///The `initMutationEvent()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MutationEvent/initMutationEvent)
    ///
    ///*This API requires the following crate features to be activated: `MutationEvent`*
    pub fn init_mutation_event(this: &MutationEvent, type_: &str) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "MutationEvent" , js_name = initMutationEvent ) ]
    ///The `initMutationEvent()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MutationEvent/initMutationEvent)
    ///
    ///*This API requires the following crate features to be activated: `MutationEvent`*
    pub fn init_mutation_event_with_can_bubble(
        this: &MutationEvent,
        type_: &str,
        can_bubble: bool,
    ) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "MutationEvent" , js_name = initMutationEvent ) ]
    ///The `initMutationEvent()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MutationEvent/initMutationEvent)
    ///
    ///*This API requires the following crate features to be activated: `MutationEvent`*
    pub fn init_mutation_event_with_can_bubble_and_cancelable(
        this: &MutationEvent,
        type_: &str,
        can_bubble: bool,
        cancelable: bool,
    ) -> Result<(), JsValue>;

    #[cfg(feature = "Node")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "MutationEvent" , js_name = initMutationEvent ) ]
    ///The `initMutationEvent()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MutationEvent/initMutationEvent)
    ///
    ///*This API requires the following crate features to be activated: `MutationEvent`, `Node`*
    pub fn init_mutation_event_with_can_bubble_and_cancelable_and_related_node(
        this: &MutationEvent,
        type_: &str,
        can_bubble: bool,
        cancelable: bool,
        related_node: Option<&Node>,
    ) -> Result<(), JsValue>;

    #[cfg(feature = "Node")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "MutationEvent" , js_name = initMutationEvent ) ]
    ///The `initMutationEvent()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MutationEvent/initMutationEvent)
    ///
    ///*This API requires the following crate features to be activated: `MutationEvent`, `Node`*
    pub fn init_mutation_event_with_can_bubble_and_cancelable_and_related_node_and_prev_value(
        this: &MutationEvent,
        type_: &str,
        can_bubble: bool,
        cancelable: bool,
        related_node: Option<&Node>,
        prev_value: &str,
    ) -> Result<(), JsValue>;

    #[cfg(feature = "Node")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "MutationEvent" , js_name = initMutationEvent ) ]
    ///The `initMutationEvent()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MutationEvent/initMutationEvent)
    ///
    ///*This API requires the following crate features to be activated: `MutationEvent`, `Node`*
    pub fn init_mutation_event_with_can_bubble_and_cancelable_and_related_node_and_prev_value_and_new_value(
        this: &MutationEvent,
        type_: &str,
        can_bubble: bool,
        cancelable: bool,
        related_node: Option<&Node>,
        prev_value: &str,
        new_value: &str,
    ) -> Result<(), JsValue>;

    #[cfg(feature = "Node")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "MutationEvent" , js_name = initMutationEvent ) ]
    ///The `initMutationEvent()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MutationEvent/initMutationEvent)
    ///
    ///*This API requires the following crate features to be activated: `MutationEvent`, `Node`*
    pub fn init_mutation_event_with_can_bubble_and_cancelable_and_related_node_and_prev_value_and_new_value_and_attr_name(
        this: &MutationEvent,
        type_: &str,
        can_bubble: bool,
        cancelable: bool,
        related_node: Option<&Node>,
        prev_value: &str,
        new_value: &str,
        attr_name: &str,
    ) -> Result<(), JsValue>;

    #[cfg(feature = "Node")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "MutationEvent" , js_name = initMutationEvent ) ]
    ///The `initMutationEvent()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MutationEvent/initMutationEvent)
    ///
    ///*This API requires the following crate features to be activated: `MutationEvent`, `Node`*
    pub fn init_mutation_event_with_can_bubble_and_cancelable_and_related_node_and_prev_value_and_new_value_and_attr_name_and_attr_change(
        this: &MutationEvent,
        type_: &str,
        can_bubble: bool,
        cancelable: bool,
        related_node: Option<&Node>,
        prev_value: &str,
        new_value: &str,
        attr_name: &str,
        attr_change: u16,
    ) -> Result<(), JsValue>;

}

impl MutationEvent {
    ///The `MutationEvent.MODIFICATION` const.
    ///
    ///*This API requires the following crate features to be activated: `MutationEvent`*

    pub const MODIFICATION: u16 = 1u64 as u16;

    ///The `MutationEvent.ADDITION` const.
    ///
    ///*This API requires the following crate features to be activated: `MutationEvent`*

    pub const ADDITION: u16 = 2u64 as u16;

    ///The `MutationEvent.REMOVAL` const.
    ///
    ///*This API requires the following crate features to be activated: `MutationEvent`*

    pub const REMOVAL: u16 = 3u64 as u16;
}
