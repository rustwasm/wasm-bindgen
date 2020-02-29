use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = EventTarget , extends = :: js_sys :: Object , js_name = Window , typescript_name = Window ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `Window` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub type Window;

    # [ wasm_bindgen ( structural , method , getter , js_class = "Window" , js_name = window ) ]
    ///Getter for the `window` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/window)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn window(this: &Window) -> Window;

    # [ wasm_bindgen ( structural , method , getter , js_class = "Window" , js_name = self ) ]
    ///Getter for the `self` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/self)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn self_(this: &Window) -> Window;

    #[cfg(feature = "Document")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "Window" , js_name = document ) ]
    ///Getter for the `document` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/document)
    ///
    ///*This API requires the following crate features to be activated: `Document`, `Window`*
    pub fn document(this: &Window) -> Option<Document>;

    # [ wasm_bindgen ( structural , catch , method , getter , js_class = "Window" , js_name = name ) ]
    ///Getter for the `name` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/name)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn name(this: &Window) -> Result<String, JsValue>;

    # [ wasm_bindgen ( structural , catch , method , setter , js_class = "Window" , js_name = name ) ]
    ///Setter for the `name` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/name)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn set_name(this: &Window, value: &str) -> Result<(), JsValue>;

    #[cfg(feature = "Location")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "Window" , js_name = location ) ]
    ///Getter for the `location` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/location)
    ///
    ///*This API requires the following crate features to be activated: `Location`, `Window`*
    pub fn location(this: &Window) -> Location;

    #[cfg(feature = "History")]
    # [ wasm_bindgen ( structural , catch , method , getter , js_class = "Window" , js_name = history ) ]
    ///Getter for the `history` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/history)
    ///
    ///*This API requires the following crate features to be activated: `History`, `Window`*
    pub fn history(this: &Window) -> Result<History, JsValue>;

    #[cfg(feature = "CustomElementRegistry")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "Window" , js_name = customElements ) ]
    ///Getter for the `customElements` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/customElements)
    ///
    ///*This API requires the following crate features to be activated: `CustomElementRegistry`, `Window`*
    pub fn custom_elements(this: &Window) -> CustomElementRegistry;

    #[cfg(feature = "BarProp")]
    # [ wasm_bindgen ( structural , catch , method , getter , js_class = "Window" , js_name = locationbar ) ]
    ///Getter for the `locationbar` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/locationbar)
    ///
    ///*This API requires the following crate features to be activated: `BarProp`, `Window`*
    pub fn locationbar(this: &Window) -> Result<BarProp, JsValue>;

    #[cfg(feature = "BarProp")]
    # [ wasm_bindgen ( structural , catch , method , getter , js_class = "Window" , js_name = menubar ) ]
    ///Getter for the `menubar` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/menubar)
    ///
    ///*This API requires the following crate features to be activated: `BarProp`, `Window`*
    pub fn menubar(this: &Window) -> Result<BarProp, JsValue>;

    #[cfg(feature = "BarProp")]
    # [ wasm_bindgen ( structural , catch , method , getter , js_class = "Window" , js_name = personalbar ) ]
    ///Getter for the `personalbar` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/personalbar)
    ///
    ///*This API requires the following crate features to be activated: `BarProp`, `Window`*
    pub fn personalbar(this: &Window) -> Result<BarProp, JsValue>;

    #[cfg(feature = "BarProp")]
    # [ wasm_bindgen ( structural , catch , method , getter , js_class = "Window" , js_name = scrollbars ) ]
    ///Getter for the `scrollbars` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/scrollbars)
    ///
    ///*This API requires the following crate features to be activated: `BarProp`, `Window`*
    pub fn scrollbars(this: &Window) -> Result<BarProp, JsValue>;

    #[cfg(feature = "BarProp")]
    # [ wasm_bindgen ( structural , catch , method , getter , js_class = "Window" , js_name = statusbar ) ]
    ///Getter for the `statusbar` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/statusbar)
    ///
    ///*This API requires the following crate features to be activated: `BarProp`, `Window`*
    pub fn statusbar(this: &Window) -> Result<BarProp, JsValue>;

    #[cfg(feature = "BarProp")]
    # [ wasm_bindgen ( structural , catch , method , getter , js_class = "Window" , js_name = toolbar ) ]
    ///Getter for the `toolbar` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/toolbar)
    ///
    ///*This API requires the following crate features to be activated: `BarProp`, `Window`*
    pub fn toolbar(this: &Window) -> Result<BarProp, JsValue>;

    # [ wasm_bindgen ( structural , catch , method , getter , js_class = "Window" , js_name = status ) ]
    ///Getter for the `status` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/status)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn status(this: &Window) -> Result<String, JsValue>;

    # [ wasm_bindgen ( structural , catch , method , setter , js_class = "Window" , js_name = status ) ]
    ///Setter for the `status` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/status)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn set_status(this: &Window, value: &str) -> Result<(), JsValue>;

    # [ wasm_bindgen ( structural , catch , method , getter , js_class = "Window" , js_name = closed ) ]
    ///Getter for the `closed` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/closed)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn closed(this: &Window) -> Result<bool, JsValue>;

    # [ wasm_bindgen ( structural , method , getter , js_class = "Window" , js_name = event ) ]
    ///Getter for the `event` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/event)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn event(this: &Window) -> ::wasm_bindgen::JsValue;

    # [ wasm_bindgen ( structural , catch , method , getter , js_class = "Window" , js_name = frames ) ]
    ///Getter for the `frames` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/frames)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn frames(this: &Window) -> Result<Window, JsValue>;

    # [ wasm_bindgen ( structural , method , getter , js_class = "Window" , js_name = length ) ]
    ///Getter for the `length` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/length)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn length(this: &Window) -> u32;

    # [ wasm_bindgen ( structural , catch , method , getter , js_class = "Window" , js_name = top ) ]
    ///Getter for the `top` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/top)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn top(this: &Window) -> Result<Option<Window>, JsValue>;

    # [ wasm_bindgen ( structural , catch , method , getter , js_class = "Window" , js_name = opener ) ]
    ///Getter for the `opener` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/opener)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn opener(this: &Window) -> Result<::wasm_bindgen::JsValue, JsValue>;

    # [ wasm_bindgen ( structural , catch , method , setter , js_class = "Window" , js_name = opener ) ]
    ///Setter for the `opener` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/opener)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn set_opener(this: &Window, value: &::wasm_bindgen::JsValue) -> Result<(), JsValue>;

    # [ wasm_bindgen ( structural , catch , method , getter , js_class = "Window" , js_name = parent ) ]
    ///Getter for the `parent` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/parent)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn parent(this: &Window) -> Result<Option<Window>, JsValue>;

    #[cfg(feature = "Element")]
    # [ wasm_bindgen ( structural , catch , method , getter , js_class = "Window" , js_name = frameElement ) ]
    ///Getter for the `frameElement` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/frameElement)
    ///
    ///*This API requires the following crate features to be activated: `Element`, `Window`*
    pub fn frame_element(this: &Window) -> Result<Option<Element>, JsValue>;

    #[cfg(feature = "Navigator")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "Window" , js_name = navigator ) ]
    ///Getter for the `navigator` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/navigator)
    ///
    ///*This API requires the following crate features to be activated: `Navigator`, `Window`*
    pub fn navigator(this: &Window) -> Navigator;

    #[cfg(feature = "External")]
    # [ wasm_bindgen ( structural , catch , method , getter , js_class = "Window" , js_name = external ) ]
    ///Getter for the `external` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/external)
    ///
    ///*This API requires the following crate features to be activated: `External`, `Window`*
    pub fn external(this: &Window) -> Result<External, JsValue>;

    # [ wasm_bindgen ( structural , method , getter , js_class = "Window" , js_name = onappinstalled ) ]
    ///Getter for the `onappinstalled` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onappinstalled)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn onappinstalled(this: &Window) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Window" , js_name = onappinstalled ) ]
    ///Setter for the `onappinstalled` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onappinstalled)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn set_onappinstalled(this: &Window, value: Option<&::js_sys::Function>);

    #[cfg(feature = "Screen")]
    # [ wasm_bindgen ( structural , catch , method , getter , js_class = "Window" , js_name = screen ) ]
    ///Getter for the `screen` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/screen)
    ///
    ///*This API requires the following crate features to be activated: `Screen`, `Window`*
    pub fn screen(this: &Window) -> Result<Screen, JsValue>;

    # [ wasm_bindgen ( structural , catch , method , getter , js_class = "Window" , js_name = innerWidth ) ]
    ///Getter for the `innerWidth` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/innerWidth)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn inner_width(this: &Window) -> Result<::wasm_bindgen::JsValue, JsValue>;

    # [ wasm_bindgen ( structural , catch , method , setter , js_class = "Window" , js_name = innerWidth ) ]
    ///Setter for the `innerWidth` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/innerWidth)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn set_inner_width(this: &Window, value: &::wasm_bindgen::JsValue) -> Result<(), JsValue>;

    # [ wasm_bindgen ( structural , catch , method , getter , js_class = "Window" , js_name = innerHeight ) ]
    ///Getter for the `innerHeight` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/innerHeight)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn inner_height(this: &Window) -> Result<::wasm_bindgen::JsValue, JsValue>;

    # [ wasm_bindgen ( structural , catch , method , setter , js_class = "Window" , js_name = innerHeight ) ]
    ///Setter for the `innerHeight` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/innerHeight)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn set_inner_height(this: &Window, value: &::wasm_bindgen::JsValue) -> Result<(), JsValue>;

    # [ wasm_bindgen ( structural , catch , method , getter , js_class = "Window" , js_name = scrollX ) ]
    ///Getter for the `scrollX` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/scrollX)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn scroll_x(this: &Window) -> Result<f64, JsValue>;

    # [ wasm_bindgen ( structural , catch , method , getter , js_class = "Window" , js_name = pageXOffset ) ]
    ///Getter for the `pageXOffset` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/pageXOffset)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn page_x_offset(this: &Window) -> Result<f64, JsValue>;

    # [ wasm_bindgen ( structural , catch , method , getter , js_class = "Window" , js_name = scrollY ) ]
    ///Getter for the `scrollY` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/scrollY)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn scroll_y(this: &Window) -> Result<f64, JsValue>;

    # [ wasm_bindgen ( structural , catch , method , getter , js_class = "Window" , js_name = pageYOffset ) ]
    ///Getter for the `pageYOffset` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/pageYOffset)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn page_y_offset(this: &Window) -> Result<f64, JsValue>;

    # [ wasm_bindgen ( structural , catch , method , getter , js_class = "Window" , js_name = screenX ) ]
    ///Getter for the `screenX` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/screenX)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn screen_x(this: &Window) -> Result<::wasm_bindgen::JsValue, JsValue>;

    # [ wasm_bindgen ( structural , catch , method , setter , js_class = "Window" , js_name = screenX ) ]
    ///Setter for the `screenX` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/screenX)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn set_screen_x(this: &Window, value: &::wasm_bindgen::JsValue) -> Result<(), JsValue>;

    # [ wasm_bindgen ( structural , catch , method , getter , js_class = "Window" , js_name = screenY ) ]
    ///Getter for the `screenY` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/screenY)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn screen_y(this: &Window) -> Result<::wasm_bindgen::JsValue, JsValue>;

    # [ wasm_bindgen ( structural , catch , method , setter , js_class = "Window" , js_name = screenY ) ]
    ///Setter for the `screenY` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/screenY)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn set_screen_y(this: &Window, value: &::wasm_bindgen::JsValue) -> Result<(), JsValue>;

    # [ wasm_bindgen ( structural , catch , method , getter , js_class = "Window" , js_name = outerWidth ) ]
    ///Getter for the `outerWidth` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/outerWidth)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn outer_width(this: &Window) -> Result<::wasm_bindgen::JsValue, JsValue>;

    # [ wasm_bindgen ( structural , catch , method , setter , js_class = "Window" , js_name = outerWidth ) ]
    ///Setter for the `outerWidth` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/outerWidth)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn set_outer_width(this: &Window, value: &::wasm_bindgen::JsValue) -> Result<(), JsValue>;

    # [ wasm_bindgen ( structural , catch , method , getter , js_class = "Window" , js_name = outerHeight ) ]
    ///Getter for the `outerHeight` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/outerHeight)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn outer_height(this: &Window) -> Result<::wasm_bindgen::JsValue, JsValue>;

    # [ wasm_bindgen ( structural , catch , method , setter , js_class = "Window" , js_name = outerHeight ) ]
    ///Setter for the `outerHeight` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/outerHeight)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn set_outer_height(this: &Window, value: &::wasm_bindgen::JsValue) -> Result<(), JsValue>;

    # [ wasm_bindgen ( structural , method , getter , js_class = "Window" , js_name = devicePixelRatio ) ]
    ///Getter for the `devicePixelRatio` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/devicePixelRatio)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn device_pixel_ratio(this: &Window) -> f64;

    #[cfg(feature = "Performance")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "Window" , js_name = performance ) ]
    ///Getter for the `performance` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/performance)
    ///
    ///*This API requires the following crate features to be activated: `Performance`, `Window`*
    pub fn performance(this: &Window) -> Option<Performance>;

    # [ wasm_bindgen ( structural , method , getter , js_class = "Window" , js_name = orientation ) ]
    ///Getter for the `orientation` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/orientation)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn orientation(this: &Window) -> i16;

    # [ wasm_bindgen ( structural , method , getter , js_class = "Window" , js_name = onorientationchange ) ]
    ///Getter for the `onorientationchange` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onorientationchange)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn onorientationchange(this: &Window) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Window" , js_name = onorientationchange ) ]
    ///Setter for the `onorientationchange` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onorientationchange)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn set_onorientationchange(this: &Window, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "Window" , js_name = onvrdisplayconnect ) ]
    ///Getter for the `onvrdisplayconnect` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onvrdisplayconnect)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn onvrdisplayconnect(this: &Window) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Window" , js_name = onvrdisplayconnect ) ]
    ///Setter for the `onvrdisplayconnect` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onvrdisplayconnect)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn set_onvrdisplayconnect(this: &Window, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "Window" , js_name = onvrdisplaydisconnect ) ]
    ///Getter for the `onvrdisplaydisconnect` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onvrdisplaydisconnect)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn onvrdisplaydisconnect(this: &Window) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Window" , js_name = onvrdisplaydisconnect ) ]
    ///Setter for the `onvrdisplaydisconnect` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onvrdisplaydisconnect)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn set_onvrdisplaydisconnect(this: &Window, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "Window" , js_name = onvrdisplayactivate ) ]
    ///Getter for the `onvrdisplayactivate` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onvrdisplayactivate)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn onvrdisplayactivate(this: &Window) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Window" , js_name = onvrdisplayactivate ) ]
    ///Setter for the `onvrdisplayactivate` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onvrdisplayactivate)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn set_onvrdisplayactivate(this: &Window, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "Window" , js_name = onvrdisplaydeactivate ) ]
    ///Getter for the `onvrdisplaydeactivate` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onvrdisplaydeactivate)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn onvrdisplaydeactivate(this: &Window) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Window" , js_name = onvrdisplaydeactivate ) ]
    ///Setter for the `onvrdisplaydeactivate` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onvrdisplaydeactivate)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn set_onvrdisplaydeactivate(this: &Window, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "Window" , js_name = onvrdisplaypresentchange ) ]
    ///Getter for the `onvrdisplaypresentchange` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onvrdisplaypresentchange)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn onvrdisplaypresentchange(this: &Window) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Window" , js_name = onvrdisplaypresentchange ) ]
    ///Setter for the `onvrdisplaypresentchange` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onvrdisplaypresentchange)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn set_onvrdisplaypresentchange(this: &Window, value: Option<&::js_sys::Function>);

    #[cfg(feature = "Worklet")]
    # [ wasm_bindgen ( structural , catch , method , getter , js_class = "Window" , js_name = paintWorklet ) ]
    ///Getter for the `paintWorklet` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/paintWorklet)
    ///
    ///*This API requires the following crate features to be activated: `Window`, `Worklet`*
    pub fn paint_worklet(this: &Window) -> Result<Worklet, JsValue>;

    #[cfg(feature = "Crypto")]
    # [ wasm_bindgen ( structural , catch , method , getter , js_class = "Window" , js_name = crypto ) ]
    ///Getter for the `crypto` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/crypto)
    ///
    ///*This API requires the following crate features to be activated: `Crypto`, `Window`*
    pub fn crypto(this: &Window) -> Result<Crypto, JsValue>;

    # [ wasm_bindgen ( structural , method , getter , js_class = "Window" , js_name = onabort ) ]
    ///Getter for the `onabort` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onabort)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn onabort(this: &Window) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Window" , js_name = onabort ) ]
    ///Setter for the `onabort` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onabort)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn set_onabort(this: &Window, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "Window" , js_name = onblur ) ]
    ///Getter for the `onblur` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onblur)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn onblur(this: &Window) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Window" , js_name = onblur ) ]
    ///Setter for the `onblur` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onblur)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn set_onblur(this: &Window, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "Window" , js_name = onfocus ) ]
    ///Getter for the `onfocus` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onfocus)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn onfocus(this: &Window) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Window" , js_name = onfocus ) ]
    ///Setter for the `onfocus` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onfocus)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn set_onfocus(this: &Window, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "Window" , js_name = onauxclick ) ]
    ///Getter for the `onauxclick` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onauxclick)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn onauxclick(this: &Window) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Window" , js_name = onauxclick ) ]
    ///Setter for the `onauxclick` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onauxclick)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn set_onauxclick(this: &Window, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "Window" , js_name = oncanplay ) ]
    ///Getter for the `oncanplay` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/oncanplay)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn oncanplay(this: &Window) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Window" , js_name = oncanplay ) ]
    ///Setter for the `oncanplay` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/oncanplay)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn set_oncanplay(this: &Window, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "Window" , js_name = oncanplaythrough ) ]
    ///Getter for the `oncanplaythrough` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/oncanplaythrough)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn oncanplaythrough(this: &Window) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Window" , js_name = oncanplaythrough ) ]
    ///Setter for the `oncanplaythrough` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/oncanplaythrough)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn set_oncanplaythrough(this: &Window, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "Window" , js_name = onchange ) ]
    ///Getter for the `onchange` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onchange)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn onchange(this: &Window) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Window" , js_name = onchange ) ]
    ///Setter for the `onchange` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onchange)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn set_onchange(this: &Window, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "Window" , js_name = onclick ) ]
    ///Getter for the `onclick` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onclick)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn onclick(this: &Window) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Window" , js_name = onclick ) ]
    ///Setter for the `onclick` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onclick)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn set_onclick(this: &Window, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "Window" , js_name = onclose ) ]
    ///Getter for the `onclose` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onclose)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn onclose(this: &Window) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Window" , js_name = onclose ) ]
    ///Setter for the `onclose` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onclose)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn set_onclose(this: &Window, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "Window" , js_name = oncontextmenu ) ]
    ///Getter for the `oncontextmenu` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/oncontextmenu)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn oncontextmenu(this: &Window) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Window" , js_name = oncontextmenu ) ]
    ///Setter for the `oncontextmenu` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/oncontextmenu)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn set_oncontextmenu(this: &Window, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "Window" , js_name = ondblclick ) ]
    ///Getter for the `ondblclick` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/ondblclick)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn ondblclick(this: &Window) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Window" , js_name = ondblclick ) ]
    ///Setter for the `ondblclick` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/ondblclick)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn set_ondblclick(this: &Window, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "Window" , js_name = ondrag ) ]
    ///Getter for the `ondrag` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/ondrag)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn ondrag(this: &Window) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Window" , js_name = ondrag ) ]
    ///Setter for the `ondrag` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/ondrag)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn set_ondrag(this: &Window, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "Window" , js_name = ondragend ) ]
    ///Getter for the `ondragend` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/ondragend)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn ondragend(this: &Window) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Window" , js_name = ondragend ) ]
    ///Setter for the `ondragend` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/ondragend)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn set_ondragend(this: &Window, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "Window" , js_name = ondragenter ) ]
    ///Getter for the `ondragenter` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/ondragenter)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn ondragenter(this: &Window) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Window" , js_name = ondragenter ) ]
    ///Setter for the `ondragenter` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/ondragenter)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn set_ondragenter(this: &Window, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "Window" , js_name = ondragexit ) ]
    ///Getter for the `ondragexit` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/ondragexit)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn ondragexit(this: &Window) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Window" , js_name = ondragexit ) ]
    ///Setter for the `ondragexit` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/ondragexit)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn set_ondragexit(this: &Window, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "Window" , js_name = ondragleave ) ]
    ///Getter for the `ondragleave` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/ondragleave)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn ondragleave(this: &Window) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Window" , js_name = ondragleave ) ]
    ///Setter for the `ondragleave` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/ondragleave)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn set_ondragleave(this: &Window, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "Window" , js_name = ondragover ) ]
    ///Getter for the `ondragover` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/ondragover)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn ondragover(this: &Window) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Window" , js_name = ondragover ) ]
    ///Setter for the `ondragover` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/ondragover)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn set_ondragover(this: &Window, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "Window" , js_name = ondragstart ) ]
    ///Getter for the `ondragstart` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/ondragstart)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn ondragstart(this: &Window) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Window" , js_name = ondragstart ) ]
    ///Setter for the `ondragstart` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/ondragstart)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn set_ondragstart(this: &Window, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "Window" , js_name = ondrop ) ]
    ///Getter for the `ondrop` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/ondrop)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn ondrop(this: &Window) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Window" , js_name = ondrop ) ]
    ///Setter for the `ondrop` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/ondrop)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn set_ondrop(this: &Window, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "Window" , js_name = ondurationchange ) ]
    ///Getter for the `ondurationchange` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/ondurationchange)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn ondurationchange(this: &Window) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Window" , js_name = ondurationchange ) ]
    ///Setter for the `ondurationchange` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/ondurationchange)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn set_ondurationchange(this: &Window, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "Window" , js_name = onemptied ) ]
    ///Getter for the `onemptied` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onemptied)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn onemptied(this: &Window) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Window" , js_name = onemptied ) ]
    ///Setter for the `onemptied` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onemptied)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn set_onemptied(this: &Window, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "Window" , js_name = onended ) ]
    ///Getter for the `onended` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onended)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn onended(this: &Window) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Window" , js_name = onended ) ]
    ///Setter for the `onended` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onended)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn set_onended(this: &Window, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "Window" , js_name = oninput ) ]
    ///Getter for the `oninput` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/oninput)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn oninput(this: &Window) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Window" , js_name = oninput ) ]
    ///Setter for the `oninput` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/oninput)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn set_oninput(this: &Window, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "Window" , js_name = oninvalid ) ]
    ///Getter for the `oninvalid` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/oninvalid)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn oninvalid(this: &Window) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Window" , js_name = oninvalid ) ]
    ///Setter for the `oninvalid` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/oninvalid)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn set_oninvalid(this: &Window, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "Window" , js_name = onkeydown ) ]
    ///Getter for the `onkeydown` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onkeydown)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn onkeydown(this: &Window) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Window" , js_name = onkeydown ) ]
    ///Setter for the `onkeydown` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onkeydown)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn set_onkeydown(this: &Window, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "Window" , js_name = onkeypress ) ]
    ///Getter for the `onkeypress` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onkeypress)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn onkeypress(this: &Window) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Window" , js_name = onkeypress ) ]
    ///Setter for the `onkeypress` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onkeypress)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn set_onkeypress(this: &Window, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "Window" , js_name = onkeyup ) ]
    ///Getter for the `onkeyup` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onkeyup)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn onkeyup(this: &Window) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Window" , js_name = onkeyup ) ]
    ///Setter for the `onkeyup` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onkeyup)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn set_onkeyup(this: &Window, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "Window" , js_name = onload ) ]
    ///Getter for the `onload` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onload)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn onload(this: &Window) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Window" , js_name = onload ) ]
    ///Setter for the `onload` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onload)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn set_onload(this: &Window, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "Window" , js_name = onloadeddata ) ]
    ///Getter for the `onloadeddata` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onloadeddata)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn onloadeddata(this: &Window) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Window" , js_name = onloadeddata ) ]
    ///Setter for the `onloadeddata` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onloadeddata)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn set_onloadeddata(this: &Window, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "Window" , js_name = onloadedmetadata ) ]
    ///Getter for the `onloadedmetadata` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onloadedmetadata)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn onloadedmetadata(this: &Window) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Window" , js_name = onloadedmetadata ) ]
    ///Setter for the `onloadedmetadata` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onloadedmetadata)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn set_onloadedmetadata(this: &Window, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "Window" , js_name = onloadend ) ]
    ///Getter for the `onloadend` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onloadend)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn onloadend(this: &Window) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Window" , js_name = onloadend ) ]
    ///Setter for the `onloadend` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onloadend)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn set_onloadend(this: &Window, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "Window" , js_name = onloadstart ) ]
    ///Getter for the `onloadstart` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onloadstart)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn onloadstart(this: &Window) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Window" , js_name = onloadstart ) ]
    ///Setter for the `onloadstart` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onloadstart)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn set_onloadstart(this: &Window, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "Window" , js_name = onmousedown ) ]
    ///Getter for the `onmousedown` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onmousedown)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn onmousedown(this: &Window) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Window" , js_name = onmousedown ) ]
    ///Setter for the `onmousedown` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onmousedown)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn set_onmousedown(this: &Window, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "Window" , js_name = onmouseenter ) ]
    ///Getter for the `onmouseenter` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onmouseenter)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn onmouseenter(this: &Window) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Window" , js_name = onmouseenter ) ]
    ///Setter for the `onmouseenter` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onmouseenter)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn set_onmouseenter(this: &Window, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "Window" , js_name = onmouseleave ) ]
    ///Getter for the `onmouseleave` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onmouseleave)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn onmouseleave(this: &Window) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Window" , js_name = onmouseleave ) ]
    ///Setter for the `onmouseleave` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onmouseleave)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn set_onmouseleave(this: &Window, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "Window" , js_name = onmousemove ) ]
    ///Getter for the `onmousemove` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onmousemove)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn onmousemove(this: &Window) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Window" , js_name = onmousemove ) ]
    ///Setter for the `onmousemove` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onmousemove)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn set_onmousemove(this: &Window, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "Window" , js_name = onmouseout ) ]
    ///Getter for the `onmouseout` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onmouseout)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn onmouseout(this: &Window) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Window" , js_name = onmouseout ) ]
    ///Setter for the `onmouseout` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onmouseout)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn set_onmouseout(this: &Window, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "Window" , js_name = onmouseover ) ]
    ///Getter for the `onmouseover` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onmouseover)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn onmouseover(this: &Window) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Window" , js_name = onmouseover ) ]
    ///Setter for the `onmouseover` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onmouseover)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn set_onmouseover(this: &Window, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "Window" , js_name = onmouseup ) ]
    ///Getter for the `onmouseup` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onmouseup)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn onmouseup(this: &Window) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Window" , js_name = onmouseup ) ]
    ///Setter for the `onmouseup` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onmouseup)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn set_onmouseup(this: &Window, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "Window" , js_name = onwheel ) ]
    ///Getter for the `onwheel` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onwheel)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn onwheel(this: &Window) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Window" , js_name = onwheel ) ]
    ///Setter for the `onwheel` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onwheel)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn set_onwheel(this: &Window, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "Window" , js_name = onpause ) ]
    ///Getter for the `onpause` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onpause)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn onpause(this: &Window) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Window" , js_name = onpause ) ]
    ///Setter for the `onpause` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onpause)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn set_onpause(this: &Window, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "Window" , js_name = onplay ) ]
    ///Getter for the `onplay` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onplay)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn onplay(this: &Window) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Window" , js_name = onplay ) ]
    ///Setter for the `onplay` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onplay)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn set_onplay(this: &Window, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "Window" , js_name = onplaying ) ]
    ///Getter for the `onplaying` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onplaying)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn onplaying(this: &Window) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Window" , js_name = onplaying ) ]
    ///Setter for the `onplaying` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onplaying)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn set_onplaying(this: &Window, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "Window" , js_name = onprogress ) ]
    ///Getter for the `onprogress` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onprogress)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn onprogress(this: &Window) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Window" , js_name = onprogress ) ]
    ///Setter for the `onprogress` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onprogress)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn set_onprogress(this: &Window, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "Window" , js_name = onratechange ) ]
    ///Getter for the `onratechange` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onratechange)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn onratechange(this: &Window) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Window" , js_name = onratechange ) ]
    ///Setter for the `onratechange` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onratechange)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn set_onratechange(this: &Window, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "Window" , js_name = onreset ) ]
    ///Getter for the `onreset` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onreset)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn onreset(this: &Window) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Window" , js_name = onreset ) ]
    ///Setter for the `onreset` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onreset)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn set_onreset(this: &Window, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "Window" , js_name = onresize ) ]
    ///Getter for the `onresize` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onresize)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn onresize(this: &Window) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Window" , js_name = onresize ) ]
    ///Setter for the `onresize` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onresize)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn set_onresize(this: &Window, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "Window" , js_name = onscroll ) ]
    ///Getter for the `onscroll` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onscroll)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn onscroll(this: &Window) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Window" , js_name = onscroll ) ]
    ///Setter for the `onscroll` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onscroll)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn set_onscroll(this: &Window, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "Window" , js_name = onseeked ) ]
    ///Getter for the `onseeked` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onseeked)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn onseeked(this: &Window) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Window" , js_name = onseeked ) ]
    ///Setter for the `onseeked` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onseeked)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn set_onseeked(this: &Window, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "Window" , js_name = onseeking ) ]
    ///Getter for the `onseeking` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onseeking)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn onseeking(this: &Window) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Window" , js_name = onseeking ) ]
    ///Setter for the `onseeking` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onseeking)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn set_onseeking(this: &Window, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "Window" , js_name = onselect ) ]
    ///Getter for the `onselect` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onselect)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn onselect(this: &Window) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Window" , js_name = onselect ) ]
    ///Setter for the `onselect` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onselect)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn set_onselect(this: &Window, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "Window" , js_name = onshow ) ]
    ///Getter for the `onshow` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onshow)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn onshow(this: &Window) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Window" , js_name = onshow ) ]
    ///Setter for the `onshow` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onshow)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn set_onshow(this: &Window, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "Window" , js_name = onstalled ) ]
    ///Getter for the `onstalled` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onstalled)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn onstalled(this: &Window) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Window" , js_name = onstalled ) ]
    ///Setter for the `onstalled` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onstalled)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn set_onstalled(this: &Window, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "Window" , js_name = onsubmit ) ]
    ///Getter for the `onsubmit` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onsubmit)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn onsubmit(this: &Window) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Window" , js_name = onsubmit ) ]
    ///Setter for the `onsubmit` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onsubmit)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn set_onsubmit(this: &Window, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "Window" , js_name = onsuspend ) ]
    ///Getter for the `onsuspend` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onsuspend)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn onsuspend(this: &Window) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Window" , js_name = onsuspend ) ]
    ///Setter for the `onsuspend` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onsuspend)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn set_onsuspend(this: &Window, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "Window" , js_name = ontimeupdate ) ]
    ///Getter for the `ontimeupdate` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/ontimeupdate)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn ontimeupdate(this: &Window) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Window" , js_name = ontimeupdate ) ]
    ///Setter for the `ontimeupdate` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/ontimeupdate)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn set_ontimeupdate(this: &Window, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "Window" , js_name = onvolumechange ) ]
    ///Getter for the `onvolumechange` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onvolumechange)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn onvolumechange(this: &Window) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Window" , js_name = onvolumechange ) ]
    ///Setter for the `onvolumechange` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onvolumechange)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn set_onvolumechange(this: &Window, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "Window" , js_name = onwaiting ) ]
    ///Getter for the `onwaiting` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onwaiting)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn onwaiting(this: &Window) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Window" , js_name = onwaiting ) ]
    ///Setter for the `onwaiting` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onwaiting)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn set_onwaiting(this: &Window, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "Window" , js_name = onselectstart ) ]
    ///Getter for the `onselectstart` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onselectstart)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn onselectstart(this: &Window) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Window" , js_name = onselectstart ) ]
    ///Setter for the `onselectstart` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onselectstart)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn set_onselectstart(this: &Window, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "Window" , js_name = ontoggle ) ]
    ///Getter for the `ontoggle` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/ontoggle)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn ontoggle(this: &Window) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Window" , js_name = ontoggle ) ]
    ///Setter for the `ontoggle` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/ontoggle)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn set_ontoggle(this: &Window, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "Window" , js_name = onpointercancel ) ]
    ///Getter for the `onpointercancel` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onpointercancel)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn onpointercancel(this: &Window) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Window" , js_name = onpointercancel ) ]
    ///Setter for the `onpointercancel` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onpointercancel)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn set_onpointercancel(this: &Window, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "Window" , js_name = onpointerdown ) ]
    ///Getter for the `onpointerdown` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onpointerdown)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn onpointerdown(this: &Window) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Window" , js_name = onpointerdown ) ]
    ///Setter for the `onpointerdown` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onpointerdown)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn set_onpointerdown(this: &Window, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "Window" , js_name = onpointerup ) ]
    ///Getter for the `onpointerup` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onpointerup)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn onpointerup(this: &Window) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Window" , js_name = onpointerup ) ]
    ///Setter for the `onpointerup` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onpointerup)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn set_onpointerup(this: &Window, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "Window" , js_name = onpointermove ) ]
    ///Getter for the `onpointermove` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onpointermove)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn onpointermove(this: &Window) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Window" , js_name = onpointermove ) ]
    ///Setter for the `onpointermove` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onpointermove)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn set_onpointermove(this: &Window, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "Window" , js_name = onpointerout ) ]
    ///Getter for the `onpointerout` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onpointerout)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn onpointerout(this: &Window) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Window" , js_name = onpointerout ) ]
    ///Setter for the `onpointerout` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onpointerout)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn set_onpointerout(this: &Window, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "Window" , js_name = onpointerover ) ]
    ///Getter for the `onpointerover` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onpointerover)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn onpointerover(this: &Window) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Window" , js_name = onpointerover ) ]
    ///Setter for the `onpointerover` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onpointerover)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn set_onpointerover(this: &Window, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "Window" , js_name = onpointerenter ) ]
    ///Getter for the `onpointerenter` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onpointerenter)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn onpointerenter(this: &Window) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Window" , js_name = onpointerenter ) ]
    ///Setter for the `onpointerenter` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onpointerenter)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn set_onpointerenter(this: &Window, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "Window" , js_name = onpointerleave ) ]
    ///Getter for the `onpointerleave` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onpointerleave)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn onpointerleave(this: &Window) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Window" , js_name = onpointerleave ) ]
    ///Setter for the `onpointerleave` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onpointerleave)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn set_onpointerleave(this: &Window, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "Window" , js_name = ongotpointercapture ) ]
    ///Getter for the `ongotpointercapture` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/ongotpointercapture)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn ongotpointercapture(this: &Window) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Window" , js_name = ongotpointercapture ) ]
    ///Setter for the `ongotpointercapture` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/ongotpointercapture)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn set_ongotpointercapture(this: &Window, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "Window" , js_name = onlostpointercapture ) ]
    ///Getter for the `onlostpointercapture` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onlostpointercapture)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn onlostpointercapture(this: &Window) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Window" , js_name = onlostpointercapture ) ]
    ///Setter for the `onlostpointercapture` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onlostpointercapture)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn set_onlostpointercapture(this: &Window, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "Window" , js_name = onanimationcancel ) ]
    ///Getter for the `onanimationcancel` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onanimationcancel)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn onanimationcancel(this: &Window) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Window" , js_name = onanimationcancel ) ]
    ///Setter for the `onanimationcancel` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onanimationcancel)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn set_onanimationcancel(this: &Window, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "Window" , js_name = onanimationend ) ]
    ///Getter for the `onanimationend` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onanimationend)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn onanimationend(this: &Window) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Window" , js_name = onanimationend ) ]
    ///Setter for the `onanimationend` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onanimationend)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn set_onanimationend(this: &Window, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "Window" , js_name = onanimationiteration ) ]
    ///Getter for the `onanimationiteration` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onanimationiteration)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn onanimationiteration(this: &Window) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Window" , js_name = onanimationiteration ) ]
    ///Setter for the `onanimationiteration` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onanimationiteration)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn set_onanimationiteration(this: &Window, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "Window" , js_name = onanimationstart ) ]
    ///Getter for the `onanimationstart` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onanimationstart)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn onanimationstart(this: &Window) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Window" , js_name = onanimationstart ) ]
    ///Setter for the `onanimationstart` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onanimationstart)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn set_onanimationstart(this: &Window, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "Window" , js_name = ontransitioncancel ) ]
    ///Getter for the `ontransitioncancel` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/ontransitioncancel)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn ontransitioncancel(this: &Window) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Window" , js_name = ontransitioncancel ) ]
    ///Setter for the `ontransitioncancel` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/ontransitioncancel)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn set_ontransitioncancel(this: &Window, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "Window" , js_name = ontransitionend ) ]
    ///Getter for the `ontransitionend` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/ontransitionend)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn ontransitionend(this: &Window) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Window" , js_name = ontransitionend ) ]
    ///Setter for the `ontransitionend` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/ontransitionend)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn set_ontransitionend(this: &Window, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "Window" , js_name = ontransitionrun ) ]
    ///Getter for the `ontransitionrun` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/ontransitionrun)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn ontransitionrun(this: &Window) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Window" , js_name = ontransitionrun ) ]
    ///Setter for the `ontransitionrun` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/ontransitionrun)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn set_ontransitionrun(this: &Window, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "Window" , js_name = ontransitionstart ) ]
    ///Getter for the `ontransitionstart` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/ontransitionstart)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn ontransitionstart(this: &Window) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Window" , js_name = ontransitionstart ) ]
    ///Setter for the `ontransitionstart` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/ontransitionstart)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn set_ontransitionstart(this: &Window, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "Window" , js_name = onwebkitanimationend ) ]
    ///Getter for the `onwebkitanimationend` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onwebkitanimationend)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn onwebkitanimationend(this: &Window) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Window" , js_name = onwebkitanimationend ) ]
    ///Setter for the `onwebkitanimationend` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onwebkitanimationend)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn set_onwebkitanimationend(this: &Window, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "Window" , js_name = onwebkitanimationiteration ) ]
    ///Getter for the `onwebkitanimationiteration` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onwebkitanimationiteration)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn onwebkitanimationiteration(this: &Window) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Window" , js_name = onwebkitanimationiteration ) ]
    ///Setter for the `onwebkitanimationiteration` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onwebkitanimationiteration)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn set_onwebkitanimationiteration(this: &Window, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "Window" , js_name = onwebkitanimationstart ) ]
    ///Getter for the `onwebkitanimationstart` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onwebkitanimationstart)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn onwebkitanimationstart(this: &Window) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Window" , js_name = onwebkitanimationstart ) ]
    ///Setter for the `onwebkitanimationstart` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onwebkitanimationstart)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn set_onwebkitanimationstart(this: &Window, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "Window" , js_name = onwebkittransitionend ) ]
    ///Getter for the `onwebkittransitionend` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onwebkittransitionend)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn onwebkittransitionend(this: &Window) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Window" , js_name = onwebkittransitionend ) ]
    ///Setter for the `onwebkittransitionend` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onwebkittransitionend)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn set_onwebkittransitionend(this: &Window, value: Option<&::js_sys::Function>);

    #[cfg(feature = "U2f")]
    # [ wasm_bindgen ( structural , catch , method , getter , js_class = "Window" , js_name = u2f ) ]
    ///Getter for the `u2f` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/u2f)
    ///
    ///*This API requires the following crate features to be activated: `U2f`, `Window`*
    pub fn u2f(this: &Window) -> Result<U2f, JsValue>;

    # [ wasm_bindgen ( structural , method , getter , js_class = "Window" , js_name = onerror ) ]
    ///Getter for the `onerror` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onerror)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn onerror(this: &Window) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Window" , js_name = onerror ) ]
    ///Setter for the `onerror` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onerror)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn set_onerror(this: &Window, value: Option<&::js_sys::Function>);

    #[cfg(feature = "SpeechSynthesis")]
    # [ wasm_bindgen ( structural , catch , method , getter , js_class = "Window" , js_name = speechSynthesis ) ]
    ///Getter for the `speechSynthesis` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/speechSynthesis)
    ///
    ///*This API requires the following crate features to be activated: `SpeechSynthesis`, `Window`*
    pub fn speech_synthesis(this: &Window) -> Result<SpeechSynthesis, JsValue>;

    # [ wasm_bindgen ( structural , method , getter , js_class = "Window" , js_name = ontouchstart ) ]
    ///Getter for the `ontouchstart` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/ontouchstart)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn ontouchstart(this: &Window) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Window" , js_name = ontouchstart ) ]
    ///Setter for the `ontouchstart` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/ontouchstart)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn set_ontouchstart(this: &Window, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "Window" , js_name = ontouchend ) ]
    ///Getter for the `ontouchend` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/ontouchend)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn ontouchend(this: &Window) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Window" , js_name = ontouchend ) ]
    ///Setter for the `ontouchend` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/ontouchend)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn set_ontouchend(this: &Window, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "Window" , js_name = ontouchmove ) ]
    ///Getter for the `ontouchmove` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/ontouchmove)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn ontouchmove(this: &Window) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Window" , js_name = ontouchmove ) ]
    ///Setter for the `ontouchmove` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/ontouchmove)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn set_ontouchmove(this: &Window, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "Window" , js_name = ontouchcancel ) ]
    ///Getter for the `ontouchcancel` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/ontouchcancel)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn ontouchcancel(this: &Window) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Window" , js_name = ontouchcancel ) ]
    ///Setter for the `ontouchcancel` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/ontouchcancel)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn set_ontouchcancel(this: &Window, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "Window" , js_name = onafterprint ) ]
    ///Getter for the `onafterprint` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onafterprint)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn onafterprint(this: &Window) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Window" , js_name = onafterprint ) ]
    ///Setter for the `onafterprint` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onafterprint)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn set_onafterprint(this: &Window, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "Window" , js_name = onbeforeprint ) ]
    ///Getter for the `onbeforeprint` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onbeforeprint)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn onbeforeprint(this: &Window) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Window" , js_name = onbeforeprint ) ]
    ///Setter for the `onbeforeprint` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onbeforeprint)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn set_onbeforeprint(this: &Window, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "Window" , js_name = onbeforeunload ) ]
    ///Getter for the `onbeforeunload` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onbeforeunload)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn onbeforeunload(this: &Window) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Window" , js_name = onbeforeunload ) ]
    ///Setter for the `onbeforeunload` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onbeforeunload)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn set_onbeforeunload(this: &Window, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "Window" , js_name = onhashchange ) ]
    ///Getter for the `onhashchange` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onhashchange)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn onhashchange(this: &Window) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Window" , js_name = onhashchange ) ]
    ///Setter for the `onhashchange` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onhashchange)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn set_onhashchange(this: &Window, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "Window" , js_name = onlanguagechange ) ]
    ///Getter for the `onlanguagechange` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onlanguagechange)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn onlanguagechange(this: &Window) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Window" , js_name = onlanguagechange ) ]
    ///Setter for the `onlanguagechange` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onlanguagechange)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn set_onlanguagechange(this: &Window, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "Window" , js_name = onmessage ) ]
    ///Getter for the `onmessage` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onmessage)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn onmessage(this: &Window) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Window" , js_name = onmessage ) ]
    ///Setter for the `onmessage` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onmessage)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn set_onmessage(this: &Window, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "Window" , js_name = onmessageerror ) ]
    ///Getter for the `onmessageerror` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onmessageerror)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn onmessageerror(this: &Window) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Window" , js_name = onmessageerror ) ]
    ///Setter for the `onmessageerror` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onmessageerror)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn set_onmessageerror(this: &Window, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "Window" , js_name = onoffline ) ]
    ///Getter for the `onoffline` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onoffline)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn onoffline(this: &Window) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Window" , js_name = onoffline ) ]
    ///Setter for the `onoffline` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onoffline)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn set_onoffline(this: &Window, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "Window" , js_name = ononline ) ]
    ///Getter for the `ononline` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/ononline)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn ononline(this: &Window) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Window" , js_name = ononline ) ]
    ///Setter for the `ononline` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/ononline)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn set_ononline(this: &Window, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "Window" , js_name = onpagehide ) ]
    ///Getter for the `onpagehide` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onpagehide)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn onpagehide(this: &Window) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Window" , js_name = onpagehide ) ]
    ///Setter for the `onpagehide` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onpagehide)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn set_onpagehide(this: &Window, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "Window" , js_name = onpageshow ) ]
    ///Getter for the `onpageshow` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onpageshow)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn onpageshow(this: &Window) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Window" , js_name = onpageshow ) ]
    ///Setter for the `onpageshow` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onpageshow)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn set_onpageshow(this: &Window, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "Window" , js_name = onpopstate ) ]
    ///Getter for the `onpopstate` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onpopstate)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn onpopstate(this: &Window) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Window" , js_name = onpopstate ) ]
    ///Setter for the `onpopstate` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onpopstate)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn set_onpopstate(this: &Window, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "Window" , js_name = onstorage ) ]
    ///Getter for the `onstorage` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onstorage)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn onstorage(this: &Window) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Window" , js_name = onstorage ) ]
    ///Setter for the `onstorage` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onstorage)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn set_onstorage(this: &Window, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "Window" , js_name = onunload ) ]
    ///Getter for the `onunload` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onunload)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn onunload(this: &Window) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Window" , js_name = onunload ) ]
    ///Setter for the `onunload` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onunload)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn set_onunload(this: &Window, value: Option<&::js_sys::Function>);

    #[cfg(feature = "Storage")]
    # [ wasm_bindgen ( structural , catch , method , getter , js_class = "Window" , js_name = localStorage ) ]
    ///Getter for the `localStorage` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/localStorage)
    ///
    ///*This API requires the following crate features to be activated: `Storage`, `Window`*
    pub fn local_storage(this: &Window) -> Result<Option<Storage>, JsValue>;

    # [ wasm_bindgen ( structural , method , getter , js_class = "Window" , js_name = origin ) ]
    ///Getter for the `origin` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/origin)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn origin(this: &Window) -> String;

    # [ wasm_bindgen ( structural , method , getter , js_class = "Window" , js_name = isSecureContext ) ]
    ///Getter for the `isSecureContext` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/isSecureContext)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn is_secure_context(this: &Window) -> bool;

    #[cfg(feature = "IdbFactory")]
    # [ wasm_bindgen ( structural , catch , method , getter , js_class = "Window" , js_name = indexedDB ) ]
    ///Getter for the `indexedDB` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/indexedDB)
    ///
    ///*This API requires the following crate features to be activated: `IdbFactory`, `Window`*
    pub fn indexed_db(this: &Window) -> Result<Option<IdbFactory>, JsValue>;

    #[cfg(feature = "CacheStorage")]
    # [ wasm_bindgen ( structural , catch , method , getter , js_class = "Window" , js_name = caches ) ]
    ///Getter for the `caches` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/caches)
    ///
    ///*This API requires the following crate features to be activated: `CacheStorage`, `Window`*
    pub fn caches(this: &Window) -> Result<CacheStorage, JsValue>;

    #[cfg(feature = "Storage")]
    # [ wasm_bindgen ( structural , catch , method , getter , js_class = "Window" , js_name = sessionStorage ) ]
    ///Getter for the `sessionStorage` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/sessionStorage)
    ///
    ///*This API requires the following crate features to be activated: `Storage`, `Window`*
    pub fn session_storage(this: &Window) -> Result<Option<Storage>, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Window" , js_name = alert ) ]
    ///The `alert()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/alert)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn alert(this: &Window) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Window" , js_name = alert ) ]
    ///The `alert()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/alert)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn alert_with_message(this: &Window, message: &str) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Window" , js_name = blur ) ]
    ///The `blur()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/blur)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn blur(this: &Window) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Window" , js_name = cancelAnimationFrame ) ]
    ///The `cancelAnimationFrame()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/cancelAnimationFrame)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn cancel_animation_frame(this: &Window, handle: i32) -> Result<(), JsValue>;

    # [ wasm_bindgen ( method , structural , js_class = "Window" , js_name = cancelIdleCallback ) ]
    ///The `cancelIdleCallback()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/cancelIdleCallback)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn cancel_idle_callback(this: &Window, handle: u32);

    # [ wasm_bindgen ( method , structural , js_class = "Window" , js_name = captureEvents ) ]
    ///The `captureEvents()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/captureEvents)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn capture_events(this: &Window);

    # [ wasm_bindgen ( catch , method , structural , js_class = "Window" , js_name = close ) ]
    ///The `close()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/close)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn close(this: &Window) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Window" , js_name = confirm ) ]
    ///The `confirm()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/confirm)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn confirm(this: &Window) -> Result<bool, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Window" , js_name = confirm ) ]
    ///The `confirm()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/confirm)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn confirm_with_message(this: &Window, message: &str) -> Result<bool, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Window" , js_name = focus ) ]
    ///The `focus()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/focus)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn focus(this: &Window) -> Result<(), JsValue>;

    #[cfg(all(feature = "CssStyleDeclaration", feature = "Element",))]
    # [ wasm_bindgen ( catch , method , structural , js_class = "Window" , js_name = getComputedStyle ) ]
    ///The `getComputedStyle()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/getComputedStyle)
    ///
    ///*This API requires the following crate features to be activated: `CssStyleDeclaration`, `Element`, `Window`*
    pub fn get_computed_style(
        this: &Window,
        elt: &Element,
    ) -> Result<Option<CssStyleDeclaration>, JsValue>;

    #[cfg(all(feature = "CssStyleDeclaration", feature = "Element",))]
    # [ wasm_bindgen ( catch , method , structural , js_class = "Window" , js_name = getComputedStyle ) ]
    ///The `getComputedStyle()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/getComputedStyle)
    ///
    ///*This API requires the following crate features to be activated: `CssStyleDeclaration`, `Element`, `Window`*
    pub fn get_computed_style_with_pseudo_elt(
        this: &Window,
        elt: &Element,
        pseudo_elt: &str,
    ) -> Result<Option<CssStyleDeclaration>, JsValue>;

    #[cfg(feature = "Selection")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "Window" , js_name = getSelection ) ]
    ///The `getSelection()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/getSelection)
    ///
    ///*This API requires the following crate features to be activated: `Selection`, `Window`*
    pub fn get_selection(this: &Window) -> Result<Option<Selection>, JsValue>;

    #[cfg(feature = "MediaQueryList")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "Window" , js_name = matchMedia ) ]
    ///The `matchMedia()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/matchMedia)
    ///
    ///*This API requires the following crate features to be activated: `MediaQueryList`, `Window`*
    pub fn match_media(this: &Window, query: &str) -> Result<Option<MediaQueryList>, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Window" , js_name = moveBy ) ]
    ///The `moveBy()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/moveBy)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn move_by(this: &Window, x: i32, y: i32) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Window" , js_name = moveTo ) ]
    ///The `moveTo()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/moveTo)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn move_to(this: &Window, x: i32, y: i32) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Window" , js_name = open ) ]
    ///The `open()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/open)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn open(this: &Window) -> Result<Option<Window>, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Window" , js_name = open ) ]
    ///The `open()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/open)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn open_with_url(this: &Window, url: &str) -> Result<Option<Window>, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Window" , js_name = open ) ]
    ///The `open()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/open)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn open_with_url_and_target(
        this: &Window,
        url: &str,
        target: &str,
    ) -> Result<Option<Window>, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Window" , js_name = open ) ]
    ///The `open()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/open)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn open_with_url_and_target_and_features(
        this: &Window,
        url: &str,
        target: &str,
        features: &str,
    ) -> Result<Option<Window>, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Window" , js_name = postMessage ) ]
    ///The `postMessage()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/postMessage)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn post_message(
        this: &Window,
        message: &::wasm_bindgen::JsValue,
        target_origin: &str,
    ) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Window" , js_name = postMessage ) ]
    ///The `postMessage()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/postMessage)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn post_message_with_transfer(
        this: &Window,
        message: &::wasm_bindgen::JsValue,
        target_origin: &str,
        transfer: &::wasm_bindgen::JsValue,
    ) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Window" , js_name = print ) ]
    ///The `print()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/print)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn print(this: &Window) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Window" , js_name = prompt ) ]
    ///The `prompt()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/prompt)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn prompt(this: &Window) -> Result<Option<String>, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Window" , js_name = prompt ) ]
    ///The `prompt()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/prompt)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn prompt_with_message(this: &Window, message: &str) -> Result<Option<String>, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Window" , js_name = prompt ) ]
    ///The `prompt()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/prompt)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn prompt_with_message_and_default(
        this: &Window,
        message: &str,
        default: &str,
    ) -> Result<Option<String>, JsValue>;

    # [ wasm_bindgen ( method , structural , js_class = "Window" , js_name = releaseEvents ) ]
    ///The `releaseEvents()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/releaseEvents)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn release_events(this: &Window);

    # [ wasm_bindgen ( catch , method , structural , js_class = "Window" , js_name = requestAnimationFrame ) ]
    ///The `requestAnimationFrame()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/requestAnimationFrame)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn request_animation_frame(
        this: &Window,
        callback: &::js_sys::Function,
    ) -> Result<i32, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Window" , js_name = requestIdleCallback ) ]
    ///The `requestIdleCallback()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/requestIdleCallback)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn request_idle_callback(
        this: &Window,
        callback: &::js_sys::Function,
    ) -> Result<u32, JsValue>;

    #[cfg(feature = "IdleRequestOptions")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "Window" , js_name = requestIdleCallback ) ]
    ///The `requestIdleCallback()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/requestIdleCallback)
    ///
    ///*This API requires the following crate features to be activated: `IdleRequestOptions`, `Window`*
    pub fn request_idle_callback_with_options(
        this: &Window,
        callback: &::js_sys::Function,
        options: &IdleRequestOptions,
    ) -> Result<u32, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Window" , js_name = resizeBy ) ]
    ///The `resizeBy()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/resizeBy)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn resize_by(this: &Window, x: i32, y: i32) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Window" , js_name = resizeTo ) ]
    ///The `resizeTo()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/resizeTo)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn resize_to(this: &Window, x: i32, y: i32) -> Result<(), JsValue>;

    # [ wasm_bindgen ( method , structural , js_class = "Window" , js_name = scroll ) ]
    ///The `scroll()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/scroll)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn scroll_with_x_and_y(this: &Window, x: f64, y: f64);

    # [ wasm_bindgen ( method , structural , js_class = "Window" , js_name = scroll ) ]
    ///The `scroll()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/scroll)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn scroll(this: &Window);

    #[cfg(feature = "ScrollToOptions")]
    # [ wasm_bindgen ( method , structural , js_class = "Window" , js_name = scroll ) ]
    ///The `scroll()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/scroll)
    ///
    ///*This API requires the following crate features to be activated: `ScrollToOptions`, `Window`*
    pub fn scroll_with_scroll_to_options(this: &Window, options: &ScrollToOptions);

    # [ wasm_bindgen ( method , structural , js_class = "Window" , js_name = scrollBy ) ]
    ///The `scrollBy()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/scrollBy)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn scroll_by_with_x_and_y(this: &Window, x: f64, y: f64);

    # [ wasm_bindgen ( method , structural , js_class = "Window" , js_name = scrollBy ) ]
    ///The `scrollBy()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/scrollBy)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn scroll_by(this: &Window);

    #[cfg(feature = "ScrollToOptions")]
    # [ wasm_bindgen ( method , structural , js_class = "Window" , js_name = scrollBy ) ]
    ///The `scrollBy()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/scrollBy)
    ///
    ///*This API requires the following crate features to be activated: `ScrollToOptions`, `Window`*
    pub fn scroll_by_with_scroll_to_options(this: &Window, options: &ScrollToOptions);

    # [ wasm_bindgen ( method , structural , js_class = "Window" , js_name = scrollTo ) ]
    ///The `scrollTo()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/scrollTo)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn scroll_to_with_x_and_y(this: &Window, x: f64, y: f64);

    # [ wasm_bindgen ( method , structural , js_class = "Window" , js_name = scrollTo ) ]
    ///The `scrollTo()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/scrollTo)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn scroll_to(this: &Window);

    #[cfg(feature = "ScrollToOptions")]
    # [ wasm_bindgen ( method , structural , js_class = "Window" , js_name = scrollTo ) ]
    ///The `scrollTo()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/scrollTo)
    ///
    ///*This API requires the following crate features to be activated: `ScrollToOptions`, `Window`*
    pub fn scroll_to_with_scroll_to_options(this: &Window, options: &ScrollToOptions);

    # [ wasm_bindgen ( catch , method , structural , js_class = "Window" , js_name = stop ) ]
    ///The `stop()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/stop)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn stop(this: &Window) -> Result<(), JsValue>;

    #[wasm_bindgen(method, structural, js_class = "Window", indexing_getter)]
    ///Indexing getter.
    ///
    ///
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn get(this: &Window, name: &str) -> Option<::js_sys::Object>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Window" , js_name = atob ) ]
    ///The `atob()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/atob)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn atob(this: &Window, atob: &str) -> Result<String, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Window" , js_name = btoa ) ]
    ///The `btoa()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/btoa)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn btoa(this: &Window, btoa: &str) -> Result<String, JsValue>;

    # [ wasm_bindgen ( method , structural , js_class = "Window" , js_name = clearInterval ) ]
    ///The `clearInterval()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/clearInterval)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn clear_interval(this: &Window);

    # [ wasm_bindgen ( method , structural , js_class = "Window" , js_name = clearInterval ) ]
    ///The `clearInterval()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/clearInterval)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn clear_interval_with_handle(this: &Window, handle: i32);

    # [ wasm_bindgen ( method , structural , js_class = "Window" , js_name = clearTimeout ) ]
    ///The `clearTimeout()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/clearTimeout)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn clear_timeout(this: &Window);

    # [ wasm_bindgen ( method , structural , js_class = "Window" , js_name = clearTimeout ) ]
    ///The `clearTimeout()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/clearTimeout)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn clear_timeout_with_handle(this: &Window, handle: i32);

    #[cfg(feature = "HtmlImageElement")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "Window" , js_name = createImageBitmap ) ]
    ///The `createImageBitmap()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/createImageBitmap)
    ///
    ///*This API requires the following crate features to be activated: `HtmlImageElement`, `Window`*
    pub fn create_image_bitmap_with_html_image_element(
        this: &Window,
        a_image: &HtmlImageElement,
    ) -> Result<::js_sys::Promise, JsValue>;

    #[cfg(feature = "HtmlVideoElement")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "Window" , js_name = createImageBitmap ) ]
    ///The `createImageBitmap()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/createImageBitmap)
    ///
    ///*This API requires the following crate features to be activated: `HtmlVideoElement`, `Window`*
    pub fn create_image_bitmap_with_html_video_element(
        this: &Window,
        a_image: &HtmlVideoElement,
    ) -> Result<::js_sys::Promise, JsValue>;

    #[cfg(feature = "HtmlCanvasElement")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "Window" , js_name = createImageBitmap ) ]
    ///The `createImageBitmap()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/createImageBitmap)
    ///
    ///*This API requires the following crate features to be activated: `HtmlCanvasElement`, `Window`*
    pub fn create_image_bitmap_with_html_canvas_element(
        this: &Window,
        a_image: &HtmlCanvasElement,
    ) -> Result<::js_sys::Promise, JsValue>;

    #[cfg(feature = "Blob")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "Window" , js_name = createImageBitmap ) ]
    ///The `createImageBitmap()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/createImageBitmap)
    ///
    ///*This API requires the following crate features to be activated: `Blob`, `Window`*
    pub fn create_image_bitmap_with_blob(
        this: &Window,
        a_image: &Blob,
    ) -> Result<::js_sys::Promise, JsValue>;

    #[cfg(feature = "ImageData")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "Window" , js_name = createImageBitmap ) ]
    ///The `createImageBitmap()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/createImageBitmap)
    ///
    ///*This API requires the following crate features to be activated: `ImageData`, `Window`*
    pub fn create_image_bitmap_with_image_data(
        this: &Window,
        a_image: &ImageData,
    ) -> Result<::js_sys::Promise, JsValue>;

    #[cfg(feature = "CanvasRenderingContext2d")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "Window" , js_name = createImageBitmap ) ]
    ///The `createImageBitmap()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/createImageBitmap)
    ///
    ///*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `Window`*
    pub fn create_image_bitmap_with_canvas_rendering_context_2d(
        this: &Window,
        a_image: &CanvasRenderingContext2d,
    ) -> Result<::js_sys::Promise, JsValue>;

    #[cfg(feature = "ImageBitmap")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "Window" , js_name = createImageBitmap ) ]
    ///The `createImageBitmap()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/createImageBitmap)
    ///
    ///*This API requires the following crate features to be activated: `ImageBitmap`, `Window`*
    pub fn create_image_bitmap_with_image_bitmap(
        this: &Window,
        a_image: &ImageBitmap,
    ) -> Result<::js_sys::Promise, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Window" , js_name = createImageBitmap ) ]
    ///The `createImageBitmap()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/createImageBitmap)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn create_image_bitmap_with_buffer_source(
        this: &Window,
        a_image: &::js_sys::Object,
    ) -> Result<::js_sys::Promise, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Window" , js_name = createImageBitmap ) ]
    ///The `createImageBitmap()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/createImageBitmap)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn create_image_bitmap_with_u8_array(
        this: &Window,
        a_image: &mut [u8],
    ) -> Result<::js_sys::Promise, JsValue>;

    #[cfg(feature = "HtmlImageElement")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "Window" , js_name = createImageBitmap ) ]
    ///The `createImageBitmap()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/createImageBitmap)
    ///
    ///*This API requires the following crate features to be activated: `HtmlImageElement`, `Window`*
    pub fn create_image_bitmap_with_html_image_element_and_a_sx_and_a_sy_and_a_sw_and_a_sh(
        this: &Window,
        a_image: &HtmlImageElement,
        a_sx: i32,
        a_sy: i32,
        a_sw: i32,
        a_sh: i32,
    ) -> Result<::js_sys::Promise, JsValue>;

    #[cfg(feature = "HtmlVideoElement")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "Window" , js_name = createImageBitmap ) ]
    ///The `createImageBitmap()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/createImageBitmap)
    ///
    ///*This API requires the following crate features to be activated: `HtmlVideoElement`, `Window`*
    pub fn create_image_bitmap_with_html_video_element_and_a_sx_and_a_sy_and_a_sw_and_a_sh(
        this: &Window,
        a_image: &HtmlVideoElement,
        a_sx: i32,
        a_sy: i32,
        a_sw: i32,
        a_sh: i32,
    ) -> Result<::js_sys::Promise, JsValue>;

    #[cfg(feature = "HtmlCanvasElement")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "Window" , js_name = createImageBitmap ) ]
    ///The `createImageBitmap()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/createImageBitmap)
    ///
    ///*This API requires the following crate features to be activated: `HtmlCanvasElement`, `Window`*
    pub fn create_image_bitmap_with_html_canvas_element_and_a_sx_and_a_sy_and_a_sw_and_a_sh(
        this: &Window,
        a_image: &HtmlCanvasElement,
        a_sx: i32,
        a_sy: i32,
        a_sw: i32,
        a_sh: i32,
    ) -> Result<::js_sys::Promise, JsValue>;

    #[cfg(feature = "Blob")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "Window" , js_name = createImageBitmap ) ]
    ///The `createImageBitmap()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/createImageBitmap)
    ///
    ///*This API requires the following crate features to be activated: `Blob`, `Window`*
    pub fn create_image_bitmap_with_blob_and_a_sx_and_a_sy_and_a_sw_and_a_sh(
        this: &Window,
        a_image: &Blob,
        a_sx: i32,
        a_sy: i32,
        a_sw: i32,
        a_sh: i32,
    ) -> Result<::js_sys::Promise, JsValue>;

    #[cfg(feature = "ImageData")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "Window" , js_name = createImageBitmap ) ]
    ///The `createImageBitmap()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/createImageBitmap)
    ///
    ///*This API requires the following crate features to be activated: `ImageData`, `Window`*
    pub fn create_image_bitmap_with_image_data_and_a_sx_and_a_sy_and_a_sw_and_a_sh(
        this: &Window,
        a_image: &ImageData,
        a_sx: i32,
        a_sy: i32,
        a_sw: i32,
        a_sh: i32,
    ) -> Result<::js_sys::Promise, JsValue>;

    #[cfg(feature = "CanvasRenderingContext2d")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "Window" , js_name = createImageBitmap ) ]
    ///The `createImageBitmap()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/createImageBitmap)
    ///
    ///*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `Window`*
    pub fn create_image_bitmap_with_canvas_rendering_context_2d_and_a_sx_and_a_sy_and_a_sw_and_a_sh(
        this: &Window,
        a_image: &CanvasRenderingContext2d,
        a_sx: i32,
        a_sy: i32,
        a_sw: i32,
        a_sh: i32,
    ) -> Result<::js_sys::Promise, JsValue>;

    #[cfg(feature = "ImageBitmap")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "Window" , js_name = createImageBitmap ) ]
    ///The `createImageBitmap()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/createImageBitmap)
    ///
    ///*This API requires the following crate features to be activated: `ImageBitmap`, `Window`*
    pub fn create_image_bitmap_with_image_bitmap_and_a_sx_and_a_sy_and_a_sw_and_a_sh(
        this: &Window,
        a_image: &ImageBitmap,
        a_sx: i32,
        a_sy: i32,
        a_sw: i32,
        a_sh: i32,
    ) -> Result<::js_sys::Promise, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Window" , js_name = createImageBitmap ) ]
    ///The `createImageBitmap()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/createImageBitmap)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn create_image_bitmap_with_buffer_source_and_a_sx_and_a_sy_and_a_sw_and_a_sh(
        this: &Window,
        a_image: &::js_sys::Object,
        a_sx: i32,
        a_sy: i32,
        a_sw: i32,
        a_sh: i32,
    ) -> Result<::js_sys::Promise, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Window" , js_name = createImageBitmap ) ]
    ///The `createImageBitmap()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/createImageBitmap)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn create_image_bitmap_with_u8_array_and_a_sx_and_a_sy_and_a_sw_and_a_sh(
        this: &Window,
        a_image: &mut [u8],
        a_sx: i32,
        a_sy: i32,
        a_sw: i32,
        a_sh: i32,
    ) -> Result<::js_sys::Promise, JsValue>;

    #[cfg(feature = "Request")]
    # [ wasm_bindgen ( method , structural , js_class = "Window" , js_name = fetch ) ]
    ///The `fetch()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/fetch)
    ///
    ///*This API requires the following crate features to be activated: `Request`, `Window`*
    pub fn fetch_with_request(this: &Window, input: &Request) -> ::js_sys::Promise;

    # [ wasm_bindgen ( method , structural , js_class = "Window" , js_name = fetch ) ]
    ///The `fetch()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/fetch)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn fetch_with_str(this: &Window, input: &str) -> ::js_sys::Promise;

    #[cfg(all(feature = "Request", feature = "RequestInit",))]
    # [ wasm_bindgen ( method , structural , js_class = "Window" , js_name = fetch ) ]
    ///The `fetch()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/fetch)
    ///
    ///*This API requires the following crate features to be activated: `Request`, `RequestInit`, `Window`*
    pub fn fetch_with_request_and_init(
        this: &Window,
        input: &Request,
        init: &RequestInit,
    ) -> ::js_sys::Promise;

    #[cfg(feature = "RequestInit")]
    # [ wasm_bindgen ( method , structural , js_class = "Window" , js_name = fetch ) ]
    ///The `fetch()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/fetch)
    ///
    ///*This API requires the following crate features to be activated: `RequestInit`, `Window`*
    pub fn fetch_with_str_and_init(
        this: &Window,
        input: &str,
        init: &RequestInit,
    ) -> ::js_sys::Promise;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Window" , js_name = setInterval ) ]
    ///The `setInterval()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/setInterval)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn set_interval_with_callback(
        this: &Window,
        handler: &::js_sys::Function,
    ) -> Result<i32, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , variadic , js_class = "Window" , js_name = setInterval ) ]
    ///The `setInterval()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/setInterval)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn set_interval_with_callback_and_timeout_and_arguments(
        this: &Window,
        handler: &::js_sys::Function,
        timeout: i32,
        arguments: &::js_sys::Array,
    ) -> Result<i32, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Window" , js_name = setInterval ) ]
    ///The `setInterval()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/setInterval)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn set_interval_with_callback_and_timeout_and_arguments_0(
        this: &Window,
        handler: &::js_sys::Function,
        timeout: i32,
    ) -> Result<i32, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Window" , js_name = setInterval ) ]
    ///The `setInterval()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/setInterval)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn set_interval_with_callback_and_timeout_and_arguments_1(
        this: &Window,
        handler: &::js_sys::Function,
        timeout: i32,
        arguments_1: &::wasm_bindgen::JsValue,
    ) -> Result<i32, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Window" , js_name = setInterval ) ]
    ///The `setInterval()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/setInterval)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn set_interval_with_callback_and_timeout_and_arguments_2(
        this: &Window,
        handler: &::js_sys::Function,
        timeout: i32,
        arguments_1: &::wasm_bindgen::JsValue,
        arguments_2: &::wasm_bindgen::JsValue,
    ) -> Result<i32, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Window" , js_name = setInterval ) ]
    ///The `setInterval()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/setInterval)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn set_interval_with_callback_and_timeout_and_arguments_3(
        this: &Window,
        handler: &::js_sys::Function,
        timeout: i32,
        arguments_1: &::wasm_bindgen::JsValue,
        arguments_2: &::wasm_bindgen::JsValue,
        arguments_3: &::wasm_bindgen::JsValue,
    ) -> Result<i32, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Window" , js_name = setInterval ) ]
    ///The `setInterval()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/setInterval)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn set_interval_with_callback_and_timeout_and_arguments_4(
        this: &Window,
        handler: &::js_sys::Function,
        timeout: i32,
        arguments_1: &::wasm_bindgen::JsValue,
        arguments_2: &::wasm_bindgen::JsValue,
        arguments_3: &::wasm_bindgen::JsValue,
        arguments_4: &::wasm_bindgen::JsValue,
    ) -> Result<i32, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Window" , js_name = setInterval ) ]
    ///The `setInterval()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/setInterval)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn set_interval_with_callback_and_timeout_and_arguments_5(
        this: &Window,
        handler: &::js_sys::Function,
        timeout: i32,
        arguments_1: &::wasm_bindgen::JsValue,
        arguments_2: &::wasm_bindgen::JsValue,
        arguments_3: &::wasm_bindgen::JsValue,
        arguments_4: &::wasm_bindgen::JsValue,
        arguments_5: &::wasm_bindgen::JsValue,
    ) -> Result<i32, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Window" , js_name = setInterval ) ]
    ///The `setInterval()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/setInterval)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn set_interval_with_callback_and_timeout_and_arguments_6(
        this: &Window,
        handler: &::js_sys::Function,
        timeout: i32,
        arguments_1: &::wasm_bindgen::JsValue,
        arguments_2: &::wasm_bindgen::JsValue,
        arguments_3: &::wasm_bindgen::JsValue,
        arguments_4: &::wasm_bindgen::JsValue,
        arguments_5: &::wasm_bindgen::JsValue,
        arguments_6: &::wasm_bindgen::JsValue,
    ) -> Result<i32, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Window" , js_name = setInterval ) ]
    ///The `setInterval()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/setInterval)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn set_interval_with_callback_and_timeout_and_arguments_7(
        this: &Window,
        handler: &::js_sys::Function,
        timeout: i32,
        arguments_1: &::wasm_bindgen::JsValue,
        arguments_2: &::wasm_bindgen::JsValue,
        arguments_3: &::wasm_bindgen::JsValue,
        arguments_4: &::wasm_bindgen::JsValue,
        arguments_5: &::wasm_bindgen::JsValue,
        arguments_6: &::wasm_bindgen::JsValue,
        arguments_7: &::wasm_bindgen::JsValue,
    ) -> Result<i32, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Window" , js_name = setInterval ) ]
    ///The `setInterval()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/setInterval)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn set_interval_with_str(this: &Window, handler: &str) -> Result<i32, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , variadic , js_class = "Window" , js_name = setInterval ) ]
    ///The `setInterval()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/setInterval)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn set_interval_with_str_and_timeout_and_unused(
        this: &Window,
        handler: &str,
        timeout: i32,
        unused: &::js_sys::Array,
    ) -> Result<i32, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Window" , js_name = setInterval ) ]
    ///The `setInterval()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/setInterval)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn set_interval_with_str_and_timeout_and_unused_0(
        this: &Window,
        handler: &str,
        timeout: i32,
    ) -> Result<i32, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Window" , js_name = setInterval ) ]
    ///The `setInterval()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/setInterval)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn set_interval_with_str_and_timeout_and_unused_1(
        this: &Window,
        handler: &str,
        timeout: i32,
        unused_1: &::wasm_bindgen::JsValue,
    ) -> Result<i32, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Window" , js_name = setInterval ) ]
    ///The `setInterval()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/setInterval)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn set_interval_with_str_and_timeout_and_unused_2(
        this: &Window,
        handler: &str,
        timeout: i32,
        unused_1: &::wasm_bindgen::JsValue,
        unused_2: &::wasm_bindgen::JsValue,
    ) -> Result<i32, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Window" , js_name = setInterval ) ]
    ///The `setInterval()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/setInterval)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn set_interval_with_str_and_timeout_and_unused_3(
        this: &Window,
        handler: &str,
        timeout: i32,
        unused_1: &::wasm_bindgen::JsValue,
        unused_2: &::wasm_bindgen::JsValue,
        unused_3: &::wasm_bindgen::JsValue,
    ) -> Result<i32, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Window" , js_name = setInterval ) ]
    ///The `setInterval()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/setInterval)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn set_interval_with_str_and_timeout_and_unused_4(
        this: &Window,
        handler: &str,
        timeout: i32,
        unused_1: &::wasm_bindgen::JsValue,
        unused_2: &::wasm_bindgen::JsValue,
        unused_3: &::wasm_bindgen::JsValue,
        unused_4: &::wasm_bindgen::JsValue,
    ) -> Result<i32, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Window" , js_name = setInterval ) ]
    ///The `setInterval()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/setInterval)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn set_interval_with_str_and_timeout_and_unused_5(
        this: &Window,
        handler: &str,
        timeout: i32,
        unused_1: &::wasm_bindgen::JsValue,
        unused_2: &::wasm_bindgen::JsValue,
        unused_3: &::wasm_bindgen::JsValue,
        unused_4: &::wasm_bindgen::JsValue,
        unused_5: &::wasm_bindgen::JsValue,
    ) -> Result<i32, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Window" , js_name = setInterval ) ]
    ///The `setInterval()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/setInterval)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn set_interval_with_str_and_timeout_and_unused_6(
        this: &Window,
        handler: &str,
        timeout: i32,
        unused_1: &::wasm_bindgen::JsValue,
        unused_2: &::wasm_bindgen::JsValue,
        unused_3: &::wasm_bindgen::JsValue,
        unused_4: &::wasm_bindgen::JsValue,
        unused_5: &::wasm_bindgen::JsValue,
        unused_6: &::wasm_bindgen::JsValue,
    ) -> Result<i32, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Window" , js_name = setInterval ) ]
    ///The `setInterval()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/setInterval)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn set_interval_with_str_and_timeout_and_unused_7(
        this: &Window,
        handler: &str,
        timeout: i32,
        unused_1: &::wasm_bindgen::JsValue,
        unused_2: &::wasm_bindgen::JsValue,
        unused_3: &::wasm_bindgen::JsValue,
        unused_4: &::wasm_bindgen::JsValue,
        unused_5: &::wasm_bindgen::JsValue,
        unused_6: &::wasm_bindgen::JsValue,
        unused_7: &::wasm_bindgen::JsValue,
    ) -> Result<i32, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Window" , js_name = setTimeout ) ]
    ///The `setTimeout()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/setTimeout)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn set_timeout_with_callback(
        this: &Window,
        handler: &::js_sys::Function,
    ) -> Result<i32, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , variadic , js_class = "Window" , js_name = setTimeout ) ]
    ///The `setTimeout()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/setTimeout)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn set_timeout_with_callback_and_timeout_and_arguments(
        this: &Window,
        handler: &::js_sys::Function,
        timeout: i32,
        arguments: &::js_sys::Array,
    ) -> Result<i32, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Window" , js_name = setTimeout ) ]
    ///The `setTimeout()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/setTimeout)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn set_timeout_with_callback_and_timeout_and_arguments_0(
        this: &Window,
        handler: &::js_sys::Function,
        timeout: i32,
    ) -> Result<i32, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Window" , js_name = setTimeout ) ]
    ///The `setTimeout()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/setTimeout)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn set_timeout_with_callback_and_timeout_and_arguments_1(
        this: &Window,
        handler: &::js_sys::Function,
        timeout: i32,
        arguments_1: &::wasm_bindgen::JsValue,
    ) -> Result<i32, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Window" , js_name = setTimeout ) ]
    ///The `setTimeout()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/setTimeout)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn set_timeout_with_callback_and_timeout_and_arguments_2(
        this: &Window,
        handler: &::js_sys::Function,
        timeout: i32,
        arguments_1: &::wasm_bindgen::JsValue,
        arguments_2: &::wasm_bindgen::JsValue,
    ) -> Result<i32, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Window" , js_name = setTimeout ) ]
    ///The `setTimeout()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/setTimeout)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn set_timeout_with_callback_and_timeout_and_arguments_3(
        this: &Window,
        handler: &::js_sys::Function,
        timeout: i32,
        arguments_1: &::wasm_bindgen::JsValue,
        arguments_2: &::wasm_bindgen::JsValue,
        arguments_3: &::wasm_bindgen::JsValue,
    ) -> Result<i32, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Window" , js_name = setTimeout ) ]
    ///The `setTimeout()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/setTimeout)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn set_timeout_with_callback_and_timeout_and_arguments_4(
        this: &Window,
        handler: &::js_sys::Function,
        timeout: i32,
        arguments_1: &::wasm_bindgen::JsValue,
        arguments_2: &::wasm_bindgen::JsValue,
        arguments_3: &::wasm_bindgen::JsValue,
        arguments_4: &::wasm_bindgen::JsValue,
    ) -> Result<i32, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Window" , js_name = setTimeout ) ]
    ///The `setTimeout()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/setTimeout)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn set_timeout_with_callback_and_timeout_and_arguments_5(
        this: &Window,
        handler: &::js_sys::Function,
        timeout: i32,
        arguments_1: &::wasm_bindgen::JsValue,
        arguments_2: &::wasm_bindgen::JsValue,
        arguments_3: &::wasm_bindgen::JsValue,
        arguments_4: &::wasm_bindgen::JsValue,
        arguments_5: &::wasm_bindgen::JsValue,
    ) -> Result<i32, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Window" , js_name = setTimeout ) ]
    ///The `setTimeout()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/setTimeout)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn set_timeout_with_callback_and_timeout_and_arguments_6(
        this: &Window,
        handler: &::js_sys::Function,
        timeout: i32,
        arguments_1: &::wasm_bindgen::JsValue,
        arguments_2: &::wasm_bindgen::JsValue,
        arguments_3: &::wasm_bindgen::JsValue,
        arguments_4: &::wasm_bindgen::JsValue,
        arguments_5: &::wasm_bindgen::JsValue,
        arguments_6: &::wasm_bindgen::JsValue,
    ) -> Result<i32, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Window" , js_name = setTimeout ) ]
    ///The `setTimeout()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/setTimeout)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn set_timeout_with_callback_and_timeout_and_arguments_7(
        this: &Window,
        handler: &::js_sys::Function,
        timeout: i32,
        arguments_1: &::wasm_bindgen::JsValue,
        arguments_2: &::wasm_bindgen::JsValue,
        arguments_3: &::wasm_bindgen::JsValue,
        arguments_4: &::wasm_bindgen::JsValue,
        arguments_5: &::wasm_bindgen::JsValue,
        arguments_6: &::wasm_bindgen::JsValue,
        arguments_7: &::wasm_bindgen::JsValue,
    ) -> Result<i32, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Window" , js_name = setTimeout ) ]
    ///The `setTimeout()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/setTimeout)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn set_timeout_with_str(this: &Window, handler: &str) -> Result<i32, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , variadic , js_class = "Window" , js_name = setTimeout ) ]
    ///The `setTimeout()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/setTimeout)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn set_timeout_with_str_and_timeout_and_unused(
        this: &Window,
        handler: &str,
        timeout: i32,
        unused: &::js_sys::Array,
    ) -> Result<i32, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Window" , js_name = setTimeout ) ]
    ///The `setTimeout()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/setTimeout)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn set_timeout_with_str_and_timeout_and_unused_0(
        this: &Window,
        handler: &str,
        timeout: i32,
    ) -> Result<i32, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Window" , js_name = setTimeout ) ]
    ///The `setTimeout()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/setTimeout)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn set_timeout_with_str_and_timeout_and_unused_1(
        this: &Window,
        handler: &str,
        timeout: i32,
        unused_1: &::wasm_bindgen::JsValue,
    ) -> Result<i32, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Window" , js_name = setTimeout ) ]
    ///The `setTimeout()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/setTimeout)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn set_timeout_with_str_and_timeout_and_unused_2(
        this: &Window,
        handler: &str,
        timeout: i32,
        unused_1: &::wasm_bindgen::JsValue,
        unused_2: &::wasm_bindgen::JsValue,
    ) -> Result<i32, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Window" , js_name = setTimeout ) ]
    ///The `setTimeout()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/setTimeout)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn set_timeout_with_str_and_timeout_and_unused_3(
        this: &Window,
        handler: &str,
        timeout: i32,
        unused_1: &::wasm_bindgen::JsValue,
        unused_2: &::wasm_bindgen::JsValue,
        unused_3: &::wasm_bindgen::JsValue,
    ) -> Result<i32, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Window" , js_name = setTimeout ) ]
    ///The `setTimeout()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/setTimeout)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn set_timeout_with_str_and_timeout_and_unused_4(
        this: &Window,
        handler: &str,
        timeout: i32,
        unused_1: &::wasm_bindgen::JsValue,
        unused_2: &::wasm_bindgen::JsValue,
        unused_3: &::wasm_bindgen::JsValue,
        unused_4: &::wasm_bindgen::JsValue,
    ) -> Result<i32, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Window" , js_name = setTimeout ) ]
    ///The `setTimeout()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/setTimeout)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn set_timeout_with_str_and_timeout_and_unused_5(
        this: &Window,
        handler: &str,
        timeout: i32,
        unused_1: &::wasm_bindgen::JsValue,
        unused_2: &::wasm_bindgen::JsValue,
        unused_3: &::wasm_bindgen::JsValue,
        unused_4: &::wasm_bindgen::JsValue,
        unused_5: &::wasm_bindgen::JsValue,
    ) -> Result<i32, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Window" , js_name = setTimeout ) ]
    ///The `setTimeout()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/setTimeout)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn set_timeout_with_str_and_timeout_and_unused_6(
        this: &Window,
        handler: &str,
        timeout: i32,
        unused_1: &::wasm_bindgen::JsValue,
        unused_2: &::wasm_bindgen::JsValue,
        unused_3: &::wasm_bindgen::JsValue,
        unused_4: &::wasm_bindgen::JsValue,
        unused_5: &::wasm_bindgen::JsValue,
        unused_6: &::wasm_bindgen::JsValue,
    ) -> Result<i32, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Window" , js_name = setTimeout ) ]
    ///The `setTimeout()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/setTimeout)
    ///
    ///*This API requires the following crate features to be activated: `Window`*
    pub fn set_timeout_with_str_and_timeout_and_unused_7(
        this: &Window,
        handler: &str,
        timeout: i32,
        unused_1: &::wasm_bindgen::JsValue,
        unused_2: &::wasm_bindgen::JsValue,
        unused_3: &::wasm_bindgen::JsValue,
        unused_4: &::wasm_bindgen::JsValue,
        unused_5: &::wasm_bindgen::JsValue,
        unused_6: &::wasm_bindgen::JsValue,
        unused_7: &::wasm_bindgen::JsValue,
    ) -> Result<i32, JsValue>;

}
