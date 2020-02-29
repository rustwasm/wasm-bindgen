use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = Event , typescript_type = "Event" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `Event` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Event)
    ///
    ///*This API requires the following crate features to be activated: `Event`*
    pub type Event;

    # [ wasm_bindgen ( structural , method , getter , js_class = "Event" , js_name = type ) ]
    ///Getter for the `type` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Event/type)
    ///
    ///*This API requires the following crate features to be activated: `Event`*
    pub fn type_(this: &Event) -> String;

    #[cfg(feature = "EventTarget")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "Event" , js_name = target ) ]
    ///Getter for the `target` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Event/target)
    ///
    ///*This API requires the following crate features to be activated: `Event`, `EventTarget`*
    pub fn target(this: &Event) -> Option<EventTarget>;

    #[cfg(feature = "EventTarget")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "Event" , js_name = currentTarget ) ]
    ///Getter for the `currentTarget` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Event/currentTarget)
    ///
    ///*This API requires the following crate features to be activated: `Event`, `EventTarget`*
    pub fn current_target(this: &Event) -> Option<EventTarget>;

    # [ wasm_bindgen ( structural , method , getter , js_class = "Event" , js_name = eventPhase ) ]
    ///Getter for the `eventPhase` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Event/eventPhase)
    ///
    ///*This API requires the following crate features to be activated: `Event`*
    pub fn event_phase(this: &Event) -> u16;

    # [ wasm_bindgen ( structural , method , getter , js_class = "Event" , js_name = bubbles ) ]
    ///Getter for the `bubbles` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Event/bubbles)
    ///
    ///*This API requires the following crate features to be activated: `Event`*
    pub fn bubbles(this: &Event) -> bool;

    # [ wasm_bindgen ( structural , method , getter , js_class = "Event" , js_name = cancelable ) ]
    ///Getter for the `cancelable` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Event/cancelable)
    ///
    ///*This API requires the following crate features to be activated: `Event`*
    pub fn cancelable(this: &Event) -> bool;

    # [ wasm_bindgen ( structural , method , getter , js_class = "Event" , js_name = defaultPrevented ) ]
    ///Getter for the `defaultPrevented` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Event/defaultPrevented)
    ///
    ///*This API requires the following crate features to be activated: `Event`*
    pub fn default_prevented(this: &Event) -> bool;

    # [ wasm_bindgen ( structural , method , getter , js_class = "Event" , js_name = composed ) ]
    ///Getter for the `composed` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Event/composed)
    ///
    ///*This API requires the following crate features to be activated: `Event`*
    pub fn composed(this: &Event) -> bool;

    # [ wasm_bindgen ( structural , method , getter , js_class = "Event" , js_name = isTrusted ) ]
    ///Getter for the `isTrusted` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Event/isTrusted)
    ///
    ///*This API requires the following crate features to be activated: `Event`*
    pub fn is_trusted(this: &Event) -> bool;

    # [ wasm_bindgen ( structural , method , getter , js_class = "Event" , js_name = timeStamp ) ]
    ///Getter for the `timeStamp` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Event/timeStamp)
    ///
    ///*This API requires the following crate features to be activated: `Event`*
    pub fn time_stamp(this: &Event) -> f64;

    # [ wasm_bindgen ( structural , method , getter , js_class = "Event" , js_name = cancelBubble ) ]
    ///Getter for the `cancelBubble` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Event/cancelBubble)
    ///
    ///*This API requires the following crate features to be activated: `Event`*
    pub fn cancel_bubble(this: &Event) -> bool;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Event" , js_name = cancelBubble ) ]
    ///Setter for the `cancelBubble` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Event/cancelBubble)
    ///
    ///*This API requires the following crate features to be activated: `Event`*
    pub fn set_cancel_bubble(this: &Event, value: bool);

    #[wasm_bindgen(catch, constructor, js_class = "Event")]
    ///The `new Event(..)` constructor, creating a new instance of `Event`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Event/Event)
    ///
    ///*This API requires the following crate features to be activated: `Event`*
    pub fn new(type_: &str) -> Result<Event, JsValue>;

    #[cfg(feature = "EventInit")]
    #[wasm_bindgen(catch, constructor, js_class = "Event")]
    ///The `new Event(..)` constructor, creating a new instance of `Event`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Event/Event)
    ///
    ///*This API requires the following crate features to be activated: `Event`, `EventInit`*
    pub fn new_with_event_init_dict(
        type_: &str,
        event_init_dict: &EventInit,
    ) -> Result<Event, JsValue>;

    # [ wasm_bindgen ( method , structural , js_class = "Event" , js_name = composedPath ) ]
    ///The `composedPath()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Event/composedPath)
    ///
    ///*This API requires the following crate features to be activated: `Event`*
    pub fn composed_path(this: &Event) -> ::js_sys::Array;

    # [ wasm_bindgen ( method , structural , js_class = "Event" , js_name = initEvent ) ]
    ///The `initEvent()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Event/initEvent)
    ///
    ///*This API requires the following crate features to be activated: `Event`*
    pub fn init_event(this: &Event, type_: &str);

    # [ wasm_bindgen ( method , structural , js_class = "Event" , js_name = initEvent ) ]
    ///The `initEvent()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Event/initEvent)
    ///
    ///*This API requires the following crate features to be activated: `Event`*
    pub fn init_event_with_bubbles(this: &Event, type_: &str, bubbles: bool);

    # [ wasm_bindgen ( method , structural , js_class = "Event" , js_name = initEvent ) ]
    ///The `initEvent()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Event/initEvent)
    ///
    ///*This API requires the following crate features to be activated: `Event`*
    pub fn init_event_with_bubbles_and_cancelable(
        this: &Event,
        type_: &str,
        bubbles: bool,
        cancelable: bool,
    );

    # [ wasm_bindgen ( method , structural , js_class = "Event" , js_name = preventDefault ) ]
    ///The `preventDefault()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Event/preventDefault)
    ///
    ///*This API requires the following crate features to be activated: `Event`*
    pub fn prevent_default(this: &Event);

    # [ wasm_bindgen ( method , structural , js_class = "Event" , js_name = stopImmediatePropagation ) ]
    ///The `stopImmediatePropagation()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Event/stopImmediatePropagation)
    ///
    ///*This API requires the following crate features to be activated: `Event`*
    pub fn stop_immediate_propagation(this: &Event);

    # [ wasm_bindgen ( method , structural , js_class = "Event" , js_name = stopPropagation ) ]
    ///The `stopPropagation()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Event/stopPropagation)
    ///
    ///*This API requires the following crate features to be activated: `Event`*
    pub fn stop_propagation(this: &Event);

}

impl Event {
    ///The `Event.NONE` const.
    ///
    ///*This API requires the following crate features to be activated: `Event`*

    pub const NONE: u16 = 0i64 as u16;

    ///The `Event.CAPTURING_PHASE` const.
    ///
    ///*This API requires the following crate features to be activated: `Event`*

    pub const CAPTURING_PHASE: u16 = 1u64 as u16;

    ///The `Event.AT_TARGET` const.
    ///
    ///*This API requires the following crate features to be activated: `Event`*

    pub const AT_TARGET: u16 = 2u64 as u16;

    ///The `Event.BUBBLING_PHASE` const.
    ///
    ///*This API requires the following crate features to be activated: `Event`*

    pub const BUBBLING_PHASE: u16 = 3u64 as u16;
}
