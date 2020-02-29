use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = Event , extends = :: js_sys :: Object , js_name = TimeEvent , typescript_type = "TimeEvent" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `TimeEvent` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TimeEvent)
    ///
    ///*This API requires the following crate features to be activated: `TimeEvent`*
    pub type TimeEvent;

    # [ wasm_bindgen ( structural , method , getter , js_class = "TimeEvent" , js_name = detail ) ]
    ///Getter for the `detail` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TimeEvent/detail)
    ///
    ///*This API requires the following crate features to be activated: `TimeEvent`*
    pub fn detail(this: &TimeEvent) -> i32;

    #[cfg(feature = "Window")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "TimeEvent" , js_name = view ) ]
    ///Getter for the `view` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TimeEvent/view)
    ///
    ///*This API requires the following crate features to be activated: `TimeEvent`, `Window`*
    pub fn view(this: &TimeEvent) -> Option<Window>;

    # [ wasm_bindgen ( method , structural , js_class = "TimeEvent" , js_name = initTimeEvent ) ]
    ///The `initTimeEvent()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TimeEvent/initTimeEvent)
    ///
    ///*This API requires the following crate features to be activated: `TimeEvent`*
    pub fn init_time_event(this: &TimeEvent, a_type: &str);

    #[cfg(feature = "Window")]
    # [ wasm_bindgen ( method , structural , js_class = "TimeEvent" , js_name = initTimeEvent ) ]
    ///The `initTimeEvent()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TimeEvent/initTimeEvent)
    ///
    ///*This API requires the following crate features to be activated: `TimeEvent`, `Window`*
    pub fn init_time_event_with_a_view(this: &TimeEvent, a_type: &str, a_view: Option<&Window>);

    #[cfg(feature = "Window")]
    # [ wasm_bindgen ( method , structural , js_class = "TimeEvent" , js_name = initTimeEvent ) ]
    ///The `initTimeEvent()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TimeEvent/initTimeEvent)
    ///
    ///*This API requires the following crate features to be activated: `TimeEvent`, `Window`*
    pub fn init_time_event_with_a_view_and_a_detail(
        this: &TimeEvent,
        a_type: &str,
        a_view: Option<&Window>,
        a_detail: i32,
    );

}
