use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = Event , extends = :: js_sys :: Object , js_name = TimeEvent , typescript_name = TimeEvent ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `TimeEvent` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TimeEvent)\n\n*This API requires the following crate features to be activated: `TimeEvent`*"]
    pub type TimeEvent;
    # [ wasm_bindgen ( structural , method , getter , js_name = detail ) ]
    #[doc = "Getter for the `detail` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TimeEvent/detail)\n\n*This API requires the following crate features to be activated: `TimeEvent`*"]
    pub fn detail(this: &TimeEvent) -> i32;
    # [ wasm_bindgen ( structural , method , getter , js_name = view ) ]
    #[cfg(feature = "Window")]
    #[doc = "Getter for the `view` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TimeEvent/view)\n\n*This API requires the following crate features to be activated: `TimeEvent`, `Window`*"]
    pub fn view(this: &TimeEvent) -> Option<Window>;
    # [ wasm_bindgen ( method , structural , js_name = initTimeEvent ) ]
    #[doc = "The `initTimeEvent()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TimeEvent/initTimeEvent)\n\n*This API requires the following crate features to be activated: `TimeEvent`*"]
    pub fn init_time_event(this: &TimeEvent, a_type: &str);
    #[cfg(feature = "Window")]
    # [ wasm_bindgen ( method , structural , js_name = initTimeEvent ) ]
    #[doc = "The `initTimeEvent()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TimeEvent/initTimeEvent)\n\n*This API requires the following crate features to be activated: `TimeEvent`, `Window`*"]
    pub fn init_time_event_with_a_view(this: &TimeEvent, a_type: &str, a_view: Option<&Window>);
    #[cfg(feature = "Window")]
    # [ wasm_bindgen ( method , structural , js_name = initTimeEvent ) ]
    #[doc = "The `initTimeEvent()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TimeEvent/initTimeEvent)\n\n*This API requires the following crate features to be activated: `TimeEvent`, `Window`*"]
    pub fn init_time_event_with_a_view_and_a_detail(
        this: &TimeEvent,
        a_type: &str,
        a_view: Option<&Window>,
        a_detail: i32,
    );
}
