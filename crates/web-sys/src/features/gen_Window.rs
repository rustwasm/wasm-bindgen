use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = EventTarget , extends = :: js_sys :: Object , js_name = Window , typescript_name = Window ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `Window` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub type Window;
    # [ wasm_bindgen ( structural , method , getter , js_name = window ) ]
    #[doc = "Getter for the `window` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/window)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn window(this: &Window) -> Window;
    # [ wasm_bindgen ( structural , method , getter , js_name = self ) ]
    #[doc = "Getter for the `self` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/self)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn self_(this: &Window) -> Window;
    # [ wasm_bindgen ( structural , method , getter , js_name = document ) ]
    #[cfg(feature = "Document")]
    #[doc = "Getter for the `document` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/document)\n\n*This API requires the following crate features to be activated: `Document`, `Window`*"]
    pub fn document(this: &Window) -> Option<Document>;
    # [ wasm_bindgen ( structural , catch , method , getter , js_name = name ) ]
    #[doc = "Getter for the `name` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/name)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn name(this: &Window) -> Result<String, JsValue>;
    # [ wasm_bindgen ( structural , catch , method , setter , js_name = name ) ]
    #[doc = "Setter for the `name` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/name)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_name(this: &Window, value: &str) -> Result<(), JsValue>;
    # [ wasm_bindgen ( structural , method , getter , js_name = location ) ]
    #[cfg(feature = "Location")]
    #[doc = "Getter for the `location` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/location)\n\n*This API requires the following crate features to be activated: `Location`, `Window`*"]
    pub fn location(this: &Window) -> Location;
    # [ wasm_bindgen ( structural , catch , method , getter , js_name = history ) ]
    #[cfg(feature = "History")]
    #[doc = "Getter for the `history` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/history)\n\n*This API requires the following crate features to be activated: `History`, `Window`*"]
    pub fn history(this: &Window) -> Result<History, JsValue>;
    # [ wasm_bindgen ( structural , method , getter , js_name = customElements ) ]
    #[cfg(feature = "CustomElementRegistry")]
    #[doc = "Getter for the `customElements` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/customElements)\n\n*This API requires the following crate features to be activated: `CustomElementRegistry`, `Window`*"]
    pub fn custom_elements(this: &Window) -> CustomElementRegistry;
    # [ wasm_bindgen ( structural , catch , method , getter , js_name = locationbar ) ]
    #[cfg(feature = "BarProp")]
    #[doc = "Getter for the `locationbar` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/locationbar)\n\n*This API requires the following crate features to be activated: `BarProp`, `Window`*"]
    pub fn locationbar(this: &Window) -> Result<BarProp, JsValue>;
    # [ wasm_bindgen ( structural , catch , method , getter , js_name = menubar ) ]
    #[cfg(feature = "BarProp")]
    #[doc = "Getter for the `menubar` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/menubar)\n\n*This API requires the following crate features to be activated: `BarProp`, `Window`*"]
    pub fn menubar(this: &Window) -> Result<BarProp, JsValue>;
    # [ wasm_bindgen ( structural , catch , method , getter , js_name = personalbar ) ]
    #[cfg(feature = "BarProp")]
    #[doc = "Getter for the `personalbar` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/personalbar)\n\n*This API requires the following crate features to be activated: `BarProp`, `Window`*"]
    pub fn personalbar(this: &Window) -> Result<BarProp, JsValue>;
    # [ wasm_bindgen ( structural , catch , method , getter , js_name = scrollbars ) ]
    #[cfg(feature = "BarProp")]
    #[doc = "Getter for the `scrollbars` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/scrollbars)\n\n*This API requires the following crate features to be activated: `BarProp`, `Window`*"]
    pub fn scrollbars(this: &Window) -> Result<BarProp, JsValue>;
    # [ wasm_bindgen ( structural , catch , method , getter , js_name = statusbar ) ]
    #[cfg(feature = "BarProp")]
    #[doc = "Getter for the `statusbar` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/statusbar)\n\n*This API requires the following crate features to be activated: `BarProp`, `Window`*"]
    pub fn statusbar(this: &Window) -> Result<BarProp, JsValue>;
    # [ wasm_bindgen ( structural , catch , method , getter , js_name = toolbar ) ]
    #[cfg(feature = "BarProp")]
    #[doc = "Getter for the `toolbar` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/toolbar)\n\n*This API requires the following crate features to be activated: `BarProp`, `Window`*"]
    pub fn toolbar(this: &Window) -> Result<BarProp, JsValue>;
    # [ wasm_bindgen ( structural , catch , method , getter , js_name = status ) ]
    #[doc = "Getter for the `status` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/status)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn status(this: &Window) -> Result<String, JsValue>;
    # [ wasm_bindgen ( structural , catch , method , setter , js_name = status ) ]
    #[doc = "Setter for the `status` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/status)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_status(this: &Window, value: &str) -> Result<(), JsValue>;
    # [ wasm_bindgen ( structural , catch , method , getter , js_name = closed ) ]
    #[doc = "Getter for the `closed` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/closed)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn closed(this: &Window) -> Result<bool, JsValue>;
    # [ wasm_bindgen ( structural , method , getter , js_name = event ) ]
    #[doc = "Getter for the `event` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/event)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn event(this: &Window) -> ::wasm_bindgen::JsValue;
    # [ wasm_bindgen ( structural , catch , method , getter , js_name = frames ) ]
    #[doc = "Getter for the `frames` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/frames)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn frames(this: &Window) -> Result<Window, JsValue>;
    # [ wasm_bindgen ( structural , method , getter , js_name = length ) ]
    #[doc = "Getter for the `length` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/length)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn length(this: &Window) -> u32;
    # [ wasm_bindgen ( structural , catch , method , getter , js_name = top ) ]
    #[doc = "Getter for the `top` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/top)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn top(this: &Window) -> Result<Option<Window>, JsValue>;
    # [ wasm_bindgen ( structural , catch , method , getter , js_name = opener ) ]
    #[doc = "Getter for the `opener` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/opener)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn opener(this: &Window) -> Result<::wasm_bindgen::JsValue, JsValue>;
    # [ wasm_bindgen ( structural , catch , method , setter , js_name = opener ) ]
    #[doc = "Setter for the `opener` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/opener)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_opener(this: &Window, value: &::wasm_bindgen::JsValue) -> Result<(), JsValue>;
    # [ wasm_bindgen ( structural , catch , method , getter , js_name = parent ) ]
    #[doc = "Getter for the `parent` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/parent)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn parent(this: &Window) -> Result<Option<Window>, JsValue>;
    # [ wasm_bindgen ( structural , catch , method , getter , js_name = frameElement ) ]
    #[cfg(feature = "Element")]
    #[doc = "Getter for the `frameElement` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/frameElement)\n\n*This API requires the following crate features to be activated: `Element`, `Window`*"]
    pub fn frame_element(this: &Window) -> Result<Option<Element>, JsValue>;
    # [ wasm_bindgen ( structural , method , getter , js_name = navigator ) ]
    #[cfg(feature = "Navigator")]
    #[doc = "Getter for the `navigator` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/navigator)\n\n*This API requires the following crate features to be activated: `Navigator`, `Window`*"]
    pub fn navigator(this: &Window) -> Navigator;
    # [ wasm_bindgen ( structural , catch , method , getter , js_name = external ) ]
    #[cfg(feature = "External")]
    #[doc = "Getter for the `external` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/external)\n\n*This API requires the following crate features to be activated: `External`, `Window`*"]
    pub fn external(this: &Window) -> Result<External, JsValue>;
    # [ wasm_bindgen ( structural , method , getter , js_name = onappinstalled ) ]
    #[doc = "Getter for the `onappinstalled` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onappinstalled)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn onappinstalled(this: &Window) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onappinstalled ) ]
    #[doc = "Setter for the `onappinstalled` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onappinstalled)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_onappinstalled(this: &Window, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , catch , method , getter , js_name = screen ) ]
    #[cfg(feature = "Screen")]
    #[doc = "Getter for the `screen` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/screen)\n\n*This API requires the following crate features to be activated: `Screen`, `Window`*"]
    pub fn screen(this: &Window) -> Result<Screen, JsValue>;
    # [ wasm_bindgen ( structural , catch , method , getter , js_name = innerWidth ) ]
    #[doc = "Getter for the `innerWidth` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/innerWidth)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn inner_width(this: &Window) -> Result<::wasm_bindgen::JsValue, JsValue>;
    # [ wasm_bindgen ( structural , catch , method , setter , js_name = innerWidth ) ]
    #[doc = "Setter for the `innerWidth` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/innerWidth)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_inner_width(this: &Window, value: &::wasm_bindgen::JsValue) -> Result<(), JsValue>;
    # [ wasm_bindgen ( structural , catch , method , getter , js_name = innerHeight ) ]
    #[doc = "Getter for the `innerHeight` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/innerHeight)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn inner_height(this: &Window) -> Result<::wasm_bindgen::JsValue, JsValue>;
    # [ wasm_bindgen ( structural , catch , method , setter , js_name = innerHeight ) ]
    #[doc = "Setter for the `innerHeight` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/innerHeight)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_inner_height(this: &Window, value: &::wasm_bindgen::JsValue) -> Result<(), JsValue>;
    # [ wasm_bindgen ( structural , catch , method , getter , js_name = scrollX ) ]
    #[doc = "Getter for the `scrollX` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/scrollX)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn scroll_x(this: &Window) -> Result<f64, JsValue>;
    # [ wasm_bindgen ( structural , catch , method , getter , js_name = pageXOffset ) ]
    #[doc = "Getter for the `pageXOffset` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/pageXOffset)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn page_x_offset(this: &Window) -> Result<f64, JsValue>;
    # [ wasm_bindgen ( structural , catch , method , getter , js_name = scrollY ) ]
    #[doc = "Getter for the `scrollY` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/scrollY)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn scroll_y(this: &Window) -> Result<f64, JsValue>;
    # [ wasm_bindgen ( structural , catch , method , getter , js_name = pageYOffset ) ]
    #[doc = "Getter for the `pageYOffset` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/pageYOffset)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn page_y_offset(this: &Window) -> Result<f64, JsValue>;
    # [ wasm_bindgen ( structural , catch , method , getter , js_name = screenX ) ]
    #[doc = "Getter for the `screenX` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/screenX)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn screen_x(this: &Window) -> Result<::wasm_bindgen::JsValue, JsValue>;
    # [ wasm_bindgen ( structural , catch , method , setter , js_name = screenX ) ]
    #[doc = "Setter for the `screenX` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/screenX)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_screen_x(this: &Window, value: &::wasm_bindgen::JsValue) -> Result<(), JsValue>;
    # [ wasm_bindgen ( structural , catch , method , getter , js_name = screenY ) ]
    #[doc = "Getter for the `screenY` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/screenY)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn screen_y(this: &Window) -> Result<::wasm_bindgen::JsValue, JsValue>;
    # [ wasm_bindgen ( structural , catch , method , setter , js_name = screenY ) ]
    #[doc = "Setter for the `screenY` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/screenY)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_screen_y(this: &Window, value: &::wasm_bindgen::JsValue) -> Result<(), JsValue>;
    # [ wasm_bindgen ( structural , catch , method , getter , js_name = outerWidth ) ]
    #[doc = "Getter for the `outerWidth` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/outerWidth)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn outer_width(this: &Window) -> Result<::wasm_bindgen::JsValue, JsValue>;
    # [ wasm_bindgen ( structural , catch , method , setter , js_name = outerWidth ) ]
    #[doc = "Setter for the `outerWidth` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/outerWidth)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_outer_width(this: &Window, value: &::wasm_bindgen::JsValue) -> Result<(), JsValue>;
    # [ wasm_bindgen ( structural , catch , method , getter , js_name = outerHeight ) ]
    #[doc = "Getter for the `outerHeight` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/outerHeight)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn outer_height(this: &Window) -> Result<::wasm_bindgen::JsValue, JsValue>;
    # [ wasm_bindgen ( structural , catch , method , setter , js_name = outerHeight ) ]
    #[doc = "Setter for the `outerHeight` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/outerHeight)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_outer_height(this: &Window, value: &::wasm_bindgen::JsValue) -> Result<(), JsValue>;
    # [ wasm_bindgen ( structural , method , getter , js_name = devicePixelRatio ) ]
    #[doc = "Getter for the `devicePixelRatio` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/devicePixelRatio)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn device_pixel_ratio(this: &Window) -> f64;
    # [ wasm_bindgen ( structural , method , getter , js_name = performance ) ]
    #[cfg(feature = "Performance")]
    #[doc = "Getter for the `performance` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/performance)\n\n*This API requires the following crate features to be activated: `Performance`, `Window`*"]
    pub fn performance(this: &Window) -> Option<Performance>;
    # [ wasm_bindgen ( structural , method , getter , js_name = orientation ) ]
    #[doc = "Getter for the `orientation` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/orientation)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn orientation(this: &Window) -> i16;
    # [ wasm_bindgen ( structural , method , getter , js_name = onorientationchange ) ]
    #[doc = "Getter for the `onorientationchange` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onorientationchange)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn onorientationchange(this: &Window) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onorientationchange ) ]
    #[doc = "Setter for the `onorientationchange` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onorientationchange)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_onorientationchange(this: &Window, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = onvrdisplayconnect ) ]
    #[doc = "Getter for the `onvrdisplayconnect` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onvrdisplayconnect)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn onvrdisplayconnect(this: &Window) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onvrdisplayconnect ) ]
    #[doc = "Setter for the `onvrdisplayconnect` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onvrdisplayconnect)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_onvrdisplayconnect(this: &Window, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = onvrdisplaydisconnect ) ]
    #[doc = "Getter for the `onvrdisplaydisconnect` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onvrdisplaydisconnect)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn onvrdisplaydisconnect(this: &Window) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onvrdisplaydisconnect ) ]
    #[doc = "Setter for the `onvrdisplaydisconnect` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onvrdisplaydisconnect)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_onvrdisplaydisconnect(this: &Window, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = onvrdisplayactivate ) ]
    #[doc = "Getter for the `onvrdisplayactivate` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onvrdisplayactivate)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn onvrdisplayactivate(this: &Window) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onvrdisplayactivate ) ]
    #[doc = "Setter for the `onvrdisplayactivate` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onvrdisplayactivate)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_onvrdisplayactivate(this: &Window, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = onvrdisplaydeactivate ) ]
    #[doc = "Getter for the `onvrdisplaydeactivate` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onvrdisplaydeactivate)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn onvrdisplaydeactivate(this: &Window) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onvrdisplaydeactivate ) ]
    #[doc = "Setter for the `onvrdisplaydeactivate` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onvrdisplaydeactivate)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_onvrdisplaydeactivate(this: &Window, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = onvrdisplaypresentchange ) ]
    #[doc = "Getter for the `onvrdisplaypresentchange` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onvrdisplaypresentchange)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn onvrdisplaypresentchange(this: &Window) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onvrdisplaypresentchange ) ]
    #[doc = "Setter for the `onvrdisplaypresentchange` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onvrdisplaypresentchange)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_onvrdisplaypresentchange(this: &Window, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , catch , method , getter , js_name = paintWorklet ) ]
    #[cfg(feature = "Worklet")]
    #[doc = "Getter for the `paintWorklet` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/paintWorklet)\n\n*This API requires the following crate features to be activated: `Window`, `Worklet`*"]
    pub fn paint_worklet(this: &Window) -> Result<Worklet, JsValue>;
    # [ wasm_bindgen ( structural , catch , method , getter , js_name = crypto ) ]
    #[cfg(feature = "Crypto")]
    #[doc = "Getter for the `crypto` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/crypto)\n\n*This API requires the following crate features to be activated: `Crypto`, `Window`*"]
    pub fn crypto(this: &Window) -> Result<Crypto, JsValue>;
    # [ wasm_bindgen ( structural , method , getter , js_name = onabort ) ]
    #[doc = "Getter for the `onabort` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onabort)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn onabort(this: &Window) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onabort ) ]
    #[doc = "Setter for the `onabort` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onabort)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_onabort(this: &Window, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = onblur ) ]
    #[doc = "Getter for the `onblur` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onblur)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn onblur(this: &Window) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onblur ) ]
    #[doc = "Setter for the `onblur` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onblur)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_onblur(this: &Window, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = onfocus ) ]
    #[doc = "Getter for the `onfocus` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onfocus)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn onfocus(this: &Window) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onfocus ) ]
    #[doc = "Setter for the `onfocus` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onfocus)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_onfocus(this: &Window, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = onauxclick ) ]
    #[doc = "Getter for the `onauxclick` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onauxclick)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn onauxclick(this: &Window) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onauxclick ) ]
    #[doc = "Setter for the `onauxclick` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onauxclick)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_onauxclick(this: &Window, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = oncanplay ) ]
    #[doc = "Getter for the `oncanplay` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/oncanplay)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn oncanplay(this: &Window) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = oncanplay ) ]
    #[doc = "Setter for the `oncanplay` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/oncanplay)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_oncanplay(this: &Window, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = oncanplaythrough ) ]
    #[doc = "Getter for the `oncanplaythrough` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/oncanplaythrough)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn oncanplaythrough(this: &Window) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = oncanplaythrough ) ]
    #[doc = "Setter for the `oncanplaythrough` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/oncanplaythrough)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_oncanplaythrough(this: &Window, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = onchange ) ]
    #[doc = "Getter for the `onchange` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onchange)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn onchange(this: &Window) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onchange ) ]
    #[doc = "Setter for the `onchange` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onchange)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_onchange(this: &Window, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = onclick ) ]
    #[doc = "Getter for the `onclick` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onclick)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn onclick(this: &Window) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onclick ) ]
    #[doc = "Setter for the `onclick` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onclick)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_onclick(this: &Window, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = onclose ) ]
    #[doc = "Getter for the `onclose` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onclose)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn onclose(this: &Window) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onclose ) ]
    #[doc = "Setter for the `onclose` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onclose)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_onclose(this: &Window, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = oncontextmenu ) ]
    #[doc = "Getter for the `oncontextmenu` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/oncontextmenu)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn oncontextmenu(this: &Window) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = oncontextmenu ) ]
    #[doc = "Setter for the `oncontextmenu` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/oncontextmenu)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_oncontextmenu(this: &Window, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = ondblclick ) ]
    #[doc = "Getter for the `ondblclick` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/ondblclick)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn ondblclick(this: &Window) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = ondblclick ) ]
    #[doc = "Setter for the `ondblclick` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/ondblclick)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_ondblclick(this: &Window, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = ondrag ) ]
    #[doc = "Getter for the `ondrag` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/ondrag)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn ondrag(this: &Window) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = ondrag ) ]
    #[doc = "Setter for the `ondrag` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/ondrag)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_ondrag(this: &Window, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = ondragend ) ]
    #[doc = "Getter for the `ondragend` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/ondragend)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn ondragend(this: &Window) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = ondragend ) ]
    #[doc = "Setter for the `ondragend` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/ondragend)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_ondragend(this: &Window, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = ondragenter ) ]
    #[doc = "Getter for the `ondragenter` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/ondragenter)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn ondragenter(this: &Window) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = ondragenter ) ]
    #[doc = "Setter for the `ondragenter` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/ondragenter)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_ondragenter(this: &Window, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = ondragexit ) ]
    #[doc = "Getter for the `ondragexit` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/ondragexit)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn ondragexit(this: &Window) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = ondragexit ) ]
    #[doc = "Setter for the `ondragexit` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/ondragexit)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_ondragexit(this: &Window, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = ondragleave ) ]
    #[doc = "Getter for the `ondragleave` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/ondragleave)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn ondragleave(this: &Window) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = ondragleave ) ]
    #[doc = "Setter for the `ondragleave` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/ondragleave)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_ondragleave(this: &Window, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = ondragover ) ]
    #[doc = "Getter for the `ondragover` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/ondragover)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn ondragover(this: &Window) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = ondragover ) ]
    #[doc = "Setter for the `ondragover` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/ondragover)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_ondragover(this: &Window, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = ondragstart ) ]
    #[doc = "Getter for the `ondragstart` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/ondragstart)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn ondragstart(this: &Window) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = ondragstart ) ]
    #[doc = "Setter for the `ondragstart` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/ondragstart)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_ondragstart(this: &Window, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = ondrop ) ]
    #[doc = "Getter for the `ondrop` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/ondrop)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn ondrop(this: &Window) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = ondrop ) ]
    #[doc = "Setter for the `ondrop` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/ondrop)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_ondrop(this: &Window, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = ondurationchange ) ]
    #[doc = "Getter for the `ondurationchange` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/ondurationchange)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn ondurationchange(this: &Window) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = ondurationchange ) ]
    #[doc = "Setter for the `ondurationchange` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/ondurationchange)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_ondurationchange(this: &Window, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = onemptied ) ]
    #[doc = "Getter for the `onemptied` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onemptied)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn onemptied(this: &Window) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onemptied ) ]
    #[doc = "Setter for the `onemptied` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onemptied)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_onemptied(this: &Window, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = onended ) ]
    #[doc = "Getter for the `onended` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onended)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn onended(this: &Window) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onended ) ]
    #[doc = "Setter for the `onended` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onended)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_onended(this: &Window, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = oninput ) ]
    #[doc = "Getter for the `oninput` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/oninput)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn oninput(this: &Window) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = oninput ) ]
    #[doc = "Setter for the `oninput` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/oninput)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_oninput(this: &Window, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = oninvalid ) ]
    #[doc = "Getter for the `oninvalid` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/oninvalid)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn oninvalid(this: &Window) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = oninvalid ) ]
    #[doc = "Setter for the `oninvalid` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/oninvalid)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_oninvalid(this: &Window, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = onkeydown ) ]
    #[doc = "Getter for the `onkeydown` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onkeydown)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn onkeydown(this: &Window) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onkeydown ) ]
    #[doc = "Setter for the `onkeydown` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onkeydown)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_onkeydown(this: &Window, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = onkeypress ) ]
    #[doc = "Getter for the `onkeypress` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onkeypress)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn onkeypress(this: &Window) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onkeypress ) ]
    #[doc = "Setter for the `onkeypress` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onkeypress)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_onkeypress(this: &Window, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = onkeyup ) ]
    #[doc = "Getter for the `onkeyup` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onkeyup)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn onkeyup(this: &Window) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onkeyup ) ]
    #[doc = "Setter for the `onkeyup` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onkeyup)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_onkeyup(this: &Window, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = onload ) ]
    #[doc = "Getter for the `onload` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onload)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn onload(this: &Window) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onload ) ]
    #[doc = "Setter for the `onload` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onload)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_onload(this: &Window, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = onloadeddata ) ]
    #[doc = "Getter for the `onloadeddata` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onloadeddata)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn onloadeddata(this: &Window) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onloadeddata ) ]
    #[doc = "Setter for the `onloadeddata` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onloadeddata)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_onloadeddata(this: &Window, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = onloadedmetadata ) ]
    #[doc = "Getter for the `onloadedmetadata` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onloadedmetadata)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn onloadedmetadata(this: &Window) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onloadedmetadata ) ]
    #[doc = "Setter for the `onloadedmetadata` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onloadedmetadata)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_onloadedmetadata(this: &Window, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = onloadend ) ]
    #[doc = "Getter for the `onloadend` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onloadend)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn onloadend(this: &Window) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onloadend ) ]
    #[doc = "Setter for the `onloadend` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onloadend)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_onloadend(this: &Window, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = onloadstart ) ]
    #[doc = "Getter for the `onloadstart` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onloadstart)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn onloadstart(this: &Window) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onloadstart ) ]
    #[doc = "Setter for the `onloadstart` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onloadstart)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_onloadstart(this: &Window, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = onmousedown ) ]
    #[doc = "Getter for the `onmousedown` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onmousedown)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn onmousedown(this: &Window) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onmousedown ) ]
    #[doc = "Setter for the `onmousedown` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onmousedown)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_onmousedown(this: &Window, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = onmouseenter ) ]
    #[doc = "Getter for the `onmouseenter` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onmouseenter)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn onmouseenter(this: &Window) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onmouseenter ) ]
    #[doc = "Setter for the `onmouseenter` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onmouseenter)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_onmouseenter(this: &Window, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = onmouseleave ) ]
    #[doc = "Getter for the `onmouseleave` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onmouseleave)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn onmouseleave(this: &Window) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onmouseleave ) ]
    #[doc = "Setter for the `onmouseleave` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onmouseleave)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_onmouseleave(this: &Window, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = onmousemove ) ]
    #[doc = "Getter for the `onmousemove` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onmousemove)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn onmousemove(this: &Window) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onmousemove ) ]
    #[doc = "Setter for the `onmousemove` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onmousemove)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_onmousemove(this: &Window, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = onmouseout ) ]
    #[doc = "Getter for the `onmouseout` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onmouseout)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn onmouseout(this: &Window) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onmouseout ) ]
    #[doc = "Setter for the `onmouseout` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onmouseout)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_onmouseout(this: &Window, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = onmouseover ) ]
    #[doc = "Getter for the `onmouseover` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onmouseover)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn onmouseover(this: &Window) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onmouseover ) ]
    #[doc = "Setter for the `onmouseover` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onmouseover)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_onmouseover(this: &Window, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = onmouseup ) ]
    #[doc = "Getter for the `onmouseup` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onmouseup)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn onmouseup(this: &Window) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onmouseup ) ]
    #[doc = "Setter for the `onmouseup` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onmouseup)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_onmouseup(this: &Window, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = onwheel ) ]
    #[doc = "Getter for the `onwheel` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onwheel)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn onwheel(this: &Window) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onwheel ) ]
    #[doc = "Setter for the `onwheel` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onwheel)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_onwheel(this: &Window, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = onpause ) ]
    #[doc = "Getter for the `onpause` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onpause)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn onpause(this: &Window) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onpause ) ]
    #[doc = "Setter for the `onpause` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onpause)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_onpause(this: &Window, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = onplay ) ]
    #[doc = "Getter for the `onplay` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onplay)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn onplay(this: &Window) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onplay ) ]
    #[doc = "Setter for the `onplay` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onplay)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_onplay(this: &Window, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = onplaying ) ]
    #[doc = "Getter for the `onplaying` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onplaying)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn onplaying(this: &Window) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onplaying ) ]
    #[doc = "Setter for the `onplaying` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onplaying)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_onplaying(this: &Window, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = onprogress ) ]
    #[doc = "Getter for the `onprogress` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onprogress)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn onprogress(this: &Window) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onprogress ) ]
    #[doc = "Setter for the `onprogress` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onprogress)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_onprogress(this: &Window, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = onratechange ) ]
    #[doc = "Getter for the `onratechange` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onratechange)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn onratechange(this: &Window) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onratechange ) ]
    #[doc = "Setter for the `onratechange` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onratechange)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_onratechange(this: &Window, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = onreset ) ]
    #[doc = "Getter for the `onreset` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onreset)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn onreset(this: &Window) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onreset ) ]
    #[doc = "Setter for the `onreset` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onreset)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_onreset(this: &Window, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = onresize ) ]
    #[doc = "Getter for the `onresize` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onresize)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn onresize(this: &Window) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onresize ) ]
    #[doc = "Setter for the `onresize` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onresize)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_onresize(this: &Window, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = onscroll ) ]
    #[doc = "Getter for the `onscroll` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onscroll)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn onscroll(this: &Window) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onscroll ) ]
    #[doc = "Setter for the `onscroll` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onscroll)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_onscroll(this: &Window, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = onseeked ) ]
    #[doc = "Getter for the `onseeked` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onseeked)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn onseeked(this: &Window) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onseeked ) ]
    #[doc = "Setter for the `onseeked` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onseeked)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_onseeked(this: &Window, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = onseeking ) ]
    #[doc = "Getter for the `onseeking` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onseeking)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn onseeking(this: &Window) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onseeking ) ]
    #[doc = "Setter for the `onseeking` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onseeking)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_onseeking(this: &Window, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = onselect ) ]
    #[doc = "Getter for the `onselect` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onselect)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn onselect(this: &Window) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onselect ) ]
    #[doc = "Setter for the `onselect` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onselect)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_onselect(this: &Window, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = onshow ) ]
    #[doc = "Getter for the `onshow` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onshow)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn onshow(this: &Window) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onshow ) ]
    #[doc = "Setter for the `onshow` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onshow)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_onshow(this: &Window, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = onstalled ) ]
    #[doc = "Getter for the `onstalled` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onstalled)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn onstalled(this: &Window) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onstalled ) ]
    #[doc = "Setter for the `onstalled` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onstalled)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_onstalled(this: &Window, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = onsubmit ) ]
    #[doc = "Getter for the `onsubmit` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onsubmit)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn onsubmit(this: &Window) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onsubmit ) ]
    #[doc = "Setter for the `onsubmit` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onsubmit)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_onsubmit(this: &Window, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = onsuspend ) ]
    #[doc = "Getter for the `onsuspend` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onsuspend)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn onsuspend(this: &Window) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onsuspend ) ]
    #[doc = "Setter for the `onsuspend` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onsuspend)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_onsuspend(this: &Window, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = ontimeupdate ) ]
    #[doc = "Getter for the `ontimeupdate` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/ontimeupdate)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn ontimeupdate(this: &Window) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = ontimeupdate ) ]
    #[doc = "Setter for the `ontimeupdate` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/ontimeupdate)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_ontimeupdate(this: &Window, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = onvolumechange ) ]
    #[doc = "Getter for the `onvolumechange` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onvolumechange)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn onvolumechange(this: &Window) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onvolumechange ) ]
    #[doc = "Setter for the `onvolumechange` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onvolumechange)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_onvolumechange(this: &Window, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = onwaiting ) ]
    #[doc = "Getter for the `onwaiting` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onwaiting)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn onwaiting(this: &Window) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onwaiting ) ]
    #[doc = "Setter for the `onwaiting` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onwaiting)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_onwaiting(this: &Window, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = onselectstart ) ]
    #[doc = "Getter for the `onselectstart` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onselectstart)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn onselectstart(this: &Window) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onselectstart ) ]
    #[doc = "Setter for the `onselectstart` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onselectstart)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_onselectstart(this: &Window, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = ontoggle ) ]
    #[doc = "Getter for the `ontoggle` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/ontoggle)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn ontoggle(this: &Window) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = ontoggle ) ]
    #[doc = "Setter for the `ontoggle` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/ontoggle)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_ontoggle(this: &Window, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = onpointercancel ) ]
    #[doc = "Getter for the `onpointercancel` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onpointercancel)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn onpointercancel(this: &Window) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onpointercancel ) ]
    #[doc = "Setter for the `onpointercancel` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onpointercancel)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_onpointercancel(this: &Window, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = onpointerdown ) ]
    #[doc = "Getter for the `onpointerdown` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onpointerdown)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn onpointerdown(this: &Window) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onpointerdown ) ]
    #[doc = "Setter for the `onpointerdown` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onpointerdown)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_onpointerdown(this: &Window, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = onpointerup ) ]
    #[doc = "Getter for the `onpointerup` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onpointerup)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn onpointerup(this: &Window) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onpointerup ) ]
    #[doc = "Setter for the `onpointerup` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onpointerup)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_onpointerup(this: &Window, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = onpointermove ) ]
    #[doc = "Getter for the `onpointermove` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onpointermove)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn onpointermove(this: &Window) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onpointermove ) ]
    #[doc = "Setter for the `onpointermove` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onpointermove)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_onpointermove(this: &Window, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = onpointerout ) ]
    #[doc = "Getter for the `onpointerout` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onpointerout)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn onpointerout(this: &Window) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onpointerout ) ]
    #[doc = "Setter for the `onpointerout` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onpointerout)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_onpointerout(this: &Window, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = onpointerover ) ]
    #[doc = "Getter for the `onpointerover` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onpointerover)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn onpointerover(this: &Window) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onpointerover ) ]
    #[doc = "Setter for the `onpointerover` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onpointerover)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_onpointerover(this: &Window, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = onpointerenter ) ]
    #[doc = "Getter for the `onpointerenter` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onpointerenter)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn onpointerenter(this: &Window) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onpointerenter ) ]
    #[doc = "Setter for the `onpointerenter` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onpointerenter)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_onpointerenter(this: &Window, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = onpointerleave ) ]
    #[doc = "Getter for the `onpointerleave` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onpointerleave)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn onpointerleave(this: &Window) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onpointerleave ) ]
    #[doc = "Setter for the `onpointerleave` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onpointerleave)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_onpointerleave(this: &Window, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = ongotpointercapture ) ]
    #[doc = "Getter for the `ongotpointercapture` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/ongotpointercapture)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn ongotpointercapture(this: &Window) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = ongotpointercapture ) ]
    #[doc = "Setter for the `ongotpointercapture` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/ongotpointercapture)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_ongotpointercapture(this: &Window, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = onlostpointercapture ) ]
    #[doc = "Getter for the `onlostpointercapture` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onlostpointercapture)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn onlostpointercapture(this: &Window) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onlostpointercapture ) ]
    #[doc = "Setter for the `onlostpointercapture` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onlostpointercapture)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_onlostpointercapture(this: &Window, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = onanimationcancel ) ]
    #[doc = "Getter for the `onanimationcancel` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onanimationcancel)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn onanimationcancel(this: &Window) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onanimationcancel ) ]
    #[doc = "Setter for the `onanimationcancel` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onanimationcancel)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_onanimationcancel(this: &Window, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = onanimationend ) ]
    #[doc = "Getter for the `onanimationend` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onanimationend)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn onanimationend(this: &Window) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onanimationend ) ]
    #[doc = "Setter for the `onanimationend` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onanimationend)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_onanimationend(this: &Window, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = onanimationiteration ) ]
    #[doc = "Getter for the `onanimationiteration` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onanimationiteration)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn onanimationiteration(this: &Window) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onanimationiteration ) ]
    #[doc = "Setter for the `onanimationiteration` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onanimationiteration)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_onanimationiteration(this: &Window, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = onanimationstart ) ]
    #[doc = "Getter for the `onanimationstart` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onanimationstart)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn onanimationstart(this: &Window) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onanimationstart ) ]
    #[doc = "Setter for the `onanimationstart` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onanimationstart)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_onanimationstart(this: &Window, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = ontransitioncancel ) ]
    #[doc = "Getter for the `ontransitioncancel` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/ontransitioncancel)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn ontransitioncancel(this: &Window) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = ontransitioncancel ) ]
    #[doc = "Setter for the `ontransitioncancel` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/ontransitioncancel)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_ontransitioncancel(this: &Window, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = ontransitionend ) ]
    #[doc = "Getter for the `ontransitionend` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/ontransitionend)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn ontransitionend(this: &Window) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = ontransitionend ) ]
    #[doc = "Setter for the `ontransitionend` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/ontransitionend)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_ontransitionend(this: &Window, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = ontransitionrun ) ]
    #[doc = "Getter for the `ontransitionrun` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/ontransitionrun)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn ontransitionrun(this: &Window) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = ontransitionrun ) ]
    #[doc = "Setter for the `ontransitionrun` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/ontransitionrun)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_ontransitionrun(this: &Window, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = ontransitionstart ) ]
    #[doc = "Getter for the `ontransitionstart` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/ontransitionstart)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn ontransitionstart(this: &Window) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = ontransitionstart ) ]
    #[doc = "Setter for the `ontransitionstart` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/ontransitionstart)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_ontransitionstart(this: &Window, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = onwebkitanimationend ) ]
    #[doc = "Getter for the `onwebkitanimationend` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onwebkitanimationend)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn onwebkitanimationend(this: &Window) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onwebkitanimationend ) ]
    #[doc = "Setter for the `onwebkitanimationend` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onwebkitanimationend)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_onwebkitanimationend(this: &Window, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = onwebkitanimationiteration ) ]
    #[doc = "Getter for the `onwebkitanimationiteration` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onwebkitanimationiteration)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn onwebkitanimationiteration(this: &Window) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onwebkitanimationiteration ) ]
    #[doc = "Setter for the `onwebkitanimationiteration` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onwebkitanimationiteration)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_onwebkitanimationiteration(this: &Window, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = onwebkitanimationstart ) ]
    #[doc = "Getter for the `onwebkitanimationstart` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onwebkitanimationstart)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn onwebkitanimationstart(this: &Window) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onwebkitanimationstart ) ]
    #[doc = "Setter for the `onwebkitanimationstart` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onwebkitanimationstart)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_onwebkitanimationstart(this: &Window, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = onwebkittransitionend ) ]
    #[doc = "Getter for the `onwebkittransitionend` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onwebkittransitionend)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn onwebkittransitionend(this: &Window) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onwebkittransitionend ) ]
    #[doc = "Setter for the `onwebkittransitionend` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onwebkittransitionend)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_onwebkittransitionend(this: &Window, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , catch , method , getter , js_name = u2f ) ]
    #[cfg(feature = "U2f")]
    #[doc = "Getter for the `u2f` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/u2f)\n\n*This API requires the following crate features to be activated: `U2f`, `Window`*"]
    pub fn u2f(this: &Window) -> Result<U2f, JsValue>;
    # [ wasm_bindgen ( structural , method , getter , js_name = onerror ) ]
    #[doc = "Getter for the `onerror` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onerror)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn onerror(this: &Window) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onerror ) ]
    #[doc = "Setter for the `onerror` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onerror)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_onerror(this: &Window, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , catch , method , getter , js_name = speechSynthesis ) ]
    #[cfg(feature = "SpeechSynthesis")]
    #[doc = "Getter for the `speechSynthesis` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/speechSynthesis)\n\n*This API requires the following crate features to be activated: `SpeechSynthesis`, `Window`*"]
    pub fn speech_synthesis(this: &Window) -> Result<SpeechSynthesis, JsValue>;
    # [ wasm_bindgen ( structural , method , getter , js_name = ontouchstart ) ]
    #[doc = "Getter for the `ontouchstart` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/ontouchstart)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn ontouchstart(this: &Window) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = ontouchstart ) ]
    #[doc = "Setter for the `ontouchstart` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/ontouchstart)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_ontouchstart(this: &Window, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = ontouchend ) ]
    #[doc = "Getter for the `ontouchend` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/ontouchend)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn ontouchend(this: &Window) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = ontouchend ) ]
    #[doc = "Setter for the `ontouchend` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/ontouchend)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_ontouchend(this: &Window, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = ontouchmove ) ]
    #[doc = "Getter for the `ontouchmove` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/ontouchmove)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn ontouchmove(this: &Window) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = ontouchmove ) ]
    #[doc = "Setter for the `ontouchmove` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/ontouchmove)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_ontouchmove(this: &Window, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = ontouchcancel ) ]
    #[doc = "Getter for the `ontouchcancel` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/ontouchcancel)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn ontouchcancel(this: &Window) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = ontouchcancel ) ]
    #[doc = "Setter for the `ontouchcancel` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/ontouchcancel)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_ontouchcancel(this: &Window, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = onafterprint ) ]
    #[doc = "Getter for the `onafterprint` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onafterprint)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn onafterprint(this: &Window) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onafterprint ) ]
    #[doc = "Setter for the `onafterprint` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onafterprint)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_onafterprint(this: &Window, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = onbeforeprint ) ]
    #[doc = "Getter for the `onbeforeprint` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onbeforeprint)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn onbeforeprint(this: &Window) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onbeforeprint ) ]
    #[doc = "Setter for the `onbeforeprint` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onbeforeprint)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_onbeforeprint(this: &Window, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = onbeforeunload ) ]
    #[doc = "Getter for the `onbeforeunload` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onbeforeunload)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn onbeforeunload(this: &Window) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onbeforeunload ) ]
    #[doc = "Setter for the `onbeforeunload` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onbeforeunload)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_onbeforeunload(this: &Window, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = onhashchange ) ]
    #[doc = "Getter for the `onhashchange` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onhashchange)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn onhashchange(this: &Window) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onhashchange ) ]
    #[doc = "Setter for the `onhashchange` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onhashchange)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_onhashchange(this: &Window, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = onlanguagechange ) ]
    #[doc = "Getter for the `onlanguagechange` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onlanguagechange)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn onlanguagechange(this: &Window) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onlanguagechange ) ]
    #[doc = "Setter for the `onlanguagechange` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onlanguagechange)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_onlanguagechange(this: &Window, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = onmessage ) ]
    #[doc = "Getter for the `onmessage` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onmessage)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn onmessage(this: &Window) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onmessage ) ]
    #[doc = "Setter for the `onmessage` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onmessage)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_onmessage(this: &Window, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = onmessageerror ) ]
    #[doc = "Getter for the `onmessageerror` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onmessageerror)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn onmessageerror(this: &Window) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onmessageerror ) ]
    #[doc = "Setter for the `onmessageerror` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onmessageerror)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_onmessageerror(this: &Window, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = onoffline ) ]
    #[doc = "Getter for the `onoffline` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onoffline)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn onoffline(this: &Window) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onoffline ) ]
    #[doc = "Setter for the `onoffline` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onoffline)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_onoffline(this: &Window, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = ononline ) ]
    #[doc = "Getter for the `ononline` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/ononline)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn ononline(this: &Window) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = ononline ) ]
    #[doc = "Setter for the `ononline` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/ononline)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_ononline(this: &Window, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = onpagehide ) ]
    #[doc = "Getter for the `onpagehide` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onpagehide)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn onpagehide(this: &Window) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onpagehide ) ]
    #[doc = "Setter for the `onpagehide` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onpagehide)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_onpagehide(this: &Window, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = onpageshow ) ]
    #[doc = "Getter for the `onpageshow` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onpageshow)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn onpageshow(this: &Window) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onpageshow ) ]
    #[doc = "Setter for the `onpageshow` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onpageshow)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_onpageshow(this: &Window, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = onpopstate ) ]
    #[doc = "Getter for the `onpopstate` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onpopstate)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn onpopstate(this: &Window) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onpopstate ) ]
    #[doc = "Setter for the `onpopstate` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onpopstate)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_onpopstate(this: &Window, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = onstorage ) ]
    #[doc = "Getter for the `onstorage` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onstorage)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn onstorage(this: &Window) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onstorage ) ]
    #[doc = "Setter for the `onstorage` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onstorage)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_onstorage(this: &Window, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = onunload ) ]
    #[doc = "Getter for the `onunload` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onunload)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn onunload(this: &Window) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onunload ) ]
    #[doc = "Setter for the `onunload` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onunload)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_onunload(this: &Window, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , catch , method , getter , js_name = localStorage ) ]
    #[cfg(feature = "Storage")]
    #[doc = "Getter for the `localStorage` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/localStorage)\n\n*This API requires the following crate features to be activated: `Storage`, `Window`*"]
    pub fn local_storage(this: &Window) -> Result<Option<Storage>, JsValue>;
    # [ wasm_bindgen ( structural , method , getter , js_name = origin ) ]
    #[doc = "Getter for the `origin` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/origin)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn origin(this: &Window) -> String;
    # [ wasm_bindgen ( structural , method , getter , js_name = isSecureContext ) ]
    #[doc = "Getter for the `isSecureContext` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/isSecureContext)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn is_secure_context(this: &Window) -> bool;
    # [ wasm_bindgen ( structural , catch , method , getter , js_name = indexedDB ) ]
    #[cfg(feature = "IdbFactory")]
    #[doc = "Getter for the `indexedDB` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/indexedDB)\n\n*This API requires the following crate features to be activated: `IdbFactory`, `Window`*"]
    pub fn indexed_db(this: &Window) -> Result<Option<IdbFactory>, JsValue>;
    # [ wasm_bindgen ( structural , catch , method , getter , js_name = caches ) ]
    #[cfg(feature = "CacheStorage")]
    #[doc = "Getter for the `caches` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/caches)\n\n*This API requires the following crate features to be activated: `CacheStorage`, `Window`*"]
    pub fn caches(this: &Window) -> Result<CacheStorage, JsValue>;
    # [ wasm_bindgen ( structural , catch , method , getter , js_name = sessionStorage ) ]
    #[cfg(feature = "Storage")]
    #[doc = "Getter for the `sessionStorage` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/sessionStorage)\n\n*This API requires the following crate features to be activated: `Storage`, `Window`*"]
    pub fn session_storage(this: &Window) -> Result<Option<Storage>, JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = alert ) ]
    #[doc = "The `alert()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/alert)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn alert(this: &Window) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = alert ) ]
    #[doc = "The `alert()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/alert)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn alert_with_message(this: &Window, message: &str) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = blur ) ]
    #[doc = "The `blur()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/blur)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn blur(this: &Window) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = cancelAnimationFrame ) ]
    #[doc = "The `cancelAnimationFrame()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/cancelAnimationFrame)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn cancel_animation_frame(this: &Window, handle: i32) -> Result<(), JsValue>;
    # [ wasm_bindgen ( method , structural , js_name = cancelIdleCallback ) ]
    #[doc = "The `cancelIdleCallback()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/cancelIdleCallback)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn cancel_idle_callback(this: &Window, handle: u32);
    # [ wasm_bindgen ( method , structural , js_name = captureEvents ) ]
    #[doc = "The `captureEvents()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/captureEvents)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn capture_events(this: &Window);
    # [ wasm_bindgen ( catch , method , structural , js_name = close ) ]
    #[doc = "The `close()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/close)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn close(this: &Window) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = confirm ) ]
    #[doc = "The `confirm()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/confirm)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn confirm(this: &Window) -> Result<bool, JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = confirm ) ]
    #[doc = "The `confirm()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/confirm)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn confirm_with_message(this: &Window, message: &str) -> Result<bool, JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = focus ) ]
    #[doc = "The `focus()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/focus)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn focus(this: &Window) -> Result<(), JsValue>;
    #[cfg(all(feature = "CssStyleDeclaration", feature = "Element",))]
    # [ wasm_bindgen ( catch , method , structural , js_name = getComputedStyle ) ]
    #[doc = "The `getComputedStyle()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/getComputedStyle)\n\n*This API requires the following crate features to be activated: `CssStyleDeclaration`, `Element`, `Window`*"]
    pub fn get_computed_style(
        this: &Window,
        elt: &Element,
    ) -> Result<Option<CssStyleDeclaration>, JsValue>;
    #[cfg(all(feature = "CssStyleDeclaration", feature = "Element",))]
    # [ wasm_bindgen ( catch , method , structural , js_name = getComputedStyle ) ]
    #[doc = "The `getComputedStyle()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/getComputedStyle)\n\n*This API requires the following crate features to be activated: `CssStyleDeclaration`, `Element`, `Window`*"]
    pub fn get_computed_style_with_pseudo_elt(
        this: &Window,
        elt: &Element,
        pseudo_elt: &str,
    ) -> Result<Option<CssStyleDeclaration>, JsValue>;
    #[cfg(feature = "Selection")]
    # [ wasm_bindgen ( catch , method , structural , js_name = getSelection ) ]
    #[doc = "The `getSelection()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/getSelection)\n\n*This API requires the following crate features to be activated: `Selection`, `Window`*"]
    pub fn get_selection(this: &Window) -> Result<Option<Selection>, JsValue>;
    #[cfg(feature = "MediaQueryList")]
    # [ wasm_bindgen ( catch , method , structural , js_name = matchMedia ) ]
    #[doc = "The `matchMedia()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/matchMedia)\n\n*This API requires the following crate features to be activated: `MediaQueryList`, `Window`*"]
    pub fn match_media(this: &Window, query: &str) -> Result<Option<MediaQueryList>, JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = moveBy ) ]
    #[doc = "The `moveBy()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/moveBy)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn move_by(this: &Window, x: i32, y: i32) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = moveTo ) ]
    #[doc = "The `moveTo()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/moveTo)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn move_to(this: &Window, x: i32, y: i32) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = open ) ]
    #[doc = "The `open()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/open)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn open(this: &Window) -> Result<Option<Window>, JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = open ) ]
    #[doc = "The `open()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/open)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn open_with_url(this: &Window, url: &str) -> Result<Option<Window>, JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = open ) ]
    #[doc = "The `open()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/open)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn open_with_url_and_target(
        this: &Window,
        url: &str,
        target: &str,
    ) -> Result<Option<Window>, JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = open ) ]
    #[doc = "The `open()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/open)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn open_with_url_and_target_and_features(
        this: &Window,
        url: &str,
        target: &str,
        features: &str,
    ) -> Result<Option<Window>, JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = postMessage ) ]
    #[doc = "The `postMessage()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/postMessage)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn post_message(
        this: &Window,
        message: &::wasm_bindgen::JsValue,
        target_origin: &str,
    ) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = postMessage ) ]
    #[doc = "The `postMessage()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/postMessage)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn post_message_with_transfer(
        this: &Window,
        message: &::wasm_bindgen::JsValue,
        target_origin: &str,
        transfer: &::wasm_bindgen::JsValue,
    ) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = print ) ]
    #[doc = "The `print()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/print)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn print(this: &Window) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = prompt ) ]
    #[doc = "The `prompt()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/prompt)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn prompt(this: &Window) -> Result<Option<String>, JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = prompt ) ]
    #[doc = "The `prompt()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/prompt)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn prompt_with_message(this: &Window, message: &str) -> Result<Option<String>, JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = prompt ) ]
    #[doc = "The `prompt()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/prompt)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn prompt_with_message_and_default(
        this: &Window,
        message: &str,
        default: &str,
    ) -> Result<Option<String>, JsValue>;
    # [ wasm_bindgen ( method , structural , js_name = releaseEvents ) ]
    #[doc = "The `releaseEvents()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/releaseEvents)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn release_events(this: &Window);
    # [ wasm_bindgen ( catch , method , structural , js_name = requestAnimationFrame ) ]
    #[doc = "The `requestAnimationFrame()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/requestAnimationFrame)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn request_animation_frame(
        this: &Window,
        callback: &::js_sys::Function,
    ) -> Result<i32, JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = requestIdleCallback ) ]
    #[doc = "The `requestIdleCallback()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/requestIdleCallback)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn request_idle_callback(
        this: &Window,
        callback: &::js_sys::Function,
    ) -> Result<u32, JsValue>;
    #[cfg(feature = "IdleRequestOptions")]
    # [ wasm_bindgen ( catch , method , structural , js_name = requestIdleCallback ) ]
    #[doc = "The `requestIdleCallback()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/requestIdleCallback)\n\n*This API requires the following crate features to be activated: `IdleRequestOptions`, `Window`*"]
    pub fn request_idle_callback_with_options(
        this: &Window,
        callback: &::js_sys::Function,
        options: &IdleRequestOptions,
    ) -> Result<u32, JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = resizeBy ) ]
    #[doc = "The `resizeBy()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/resizeBy)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn resize_by(this: &Window, x: i32, y: i32) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = resizeTo ) ]
    #[doc = "The `resizeTo()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/resizeTo)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn resize_to(this: &Window, x: i32, y: i32) -> Result<(), JsValue>;
    # [ wasm_bindgen ( method , structural , js_name = scroll ) ]
    #[doc = "The `scroll()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/scroll)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn scroll_with_x_and_y(this: &Window, x: f64, y: f64);
    # [ wasm_bindgen ( method , structural , js_name = scroll ) ]
    #[doc = "The `scroll()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/scroll)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn scroll(this: &Window);
    #[cfg(feature = "ScrollToOptions")]
    # [ wasm_bindgen ( method , structural , js_name = scroll ) ]
    #[doc = "The `scroll()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/scroll)\n\n*This API requires the following crate features to be activated: `ScrollToOptions`, `Window`*"]
    pub fn scroll_with_scroll_to_options(this: &Window, options: &ScrollToOptions);
    # [ wasm_bindgen ( method , structural , js_name = scrollBy ) ]
    #[doc = "The `scrollBy()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/scrollBy)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn scroll_by_with_x_and_y(this: &Window, x: f64, y: f64);
    # [ wasm_bindgen ( method , structural , js_name = scrollBy ) ]
    #[doc = "The `scrollBy()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/scrollBy)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn scroll_by(this: &Window);
    #[cfg(feature = "ScrollToOptions")]
    # [ wasm_bindgen ( method , structural , js_name = scrollBy ) ]
    #[doc = "The `scrollBy()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/scrollBy)\n\n*This API requires the following crate features to be activated: `ScrollToOptions`, `Window`*"]
    pub fn scroll_by_with_scroll_to_options(this: &Window, options: &ScrollToOptions);
    # [ wasm_bindgen ( method , structural , js_name = scrollTo ) ]
    #[doc = "The `scrollTo()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/scrollTo)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn scroll_to_with_x_and_y(this: &Window, x: f64, y: f64);
    # [ wasm_bindgen ( method , structural , js_name = scrollTo ) ]
    #[doc = "The `scrollTo()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/scrollTo)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn scroll_to(this: &Window);
    #[cfg(feature = "ScrollToOptions")]
    # [ wasm_bindgen ( method , structural , js_name = scrollTo ) ]
    #[doc = "The `scrollTo()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/scrollTo)\n\n*This API requires the following crate features to be activated: `ScrollToOptions`, `Window`*"]
    pub fn scroll_to_with_scroll_to_options(this: &Window, options: &ScrollToOptions);
    # [ wasm_bindgen ( catch , method , structural , js_name = stop ) ]
    #[doc = "The `stop()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/stop)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn stop(this: &Window) -> Result<(), JsValue>;
    #[wasm_bindgen(method, structural, indexing_getter)]
    #[doc = "Indexing getter.\n\n\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn get(this: &Window, name: &str) -> Option<::js_sys::Object>;
    # [ wasm_bindgen ( catch , method , structural , js_name = atob ) ]
    #[doc = "The `atob()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/atob)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn atob(this: &Window, atob: &str) -> Result<String, JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = btoa ) ]
    #[doc = "The `btoa()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/btoa)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn btoa(this: &Window, btoa: &str) -> Result<String, JsValue>;
    # [ wasm_bindgen ( method , structural , js_name = clearInterval ) ]
    #[doc = "The `clearInterval()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/clearInterval)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn clear_interval(this: &Window);
    # [ wasm_bindgen ( method , structural , js_name = clearInterval ) ]
    #[doc = "The `clearInterval()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/clearInterval)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn clear_interval_with_handle(this: &Window, handle: i32);
    # [ wasm_bindgen ( method , structural , js_name = clearTimeout ) ]
    #[doc = "The `clearTimeout()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/clearTimeout)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn clear_timeout(this: &Window);
    # [ wasm_bindgen ( method , structural , js_name = clearTimeout ) ]
    #[doc = "The `clearTimeout()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/clearTimeout)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn clear_timeout_with_handle(this: &Window, handle: i32);
    #[cfg(feature = "HtmlImageElement")]
    # [ wasm_bindgen ( catch , method , structural , js_name = createImageBitmap ) ]
    #[doc = "The `createImageBitmap()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/createImageBitmap)\n\n*This API requires the following crate features to be activated: `HtmlImageElement`, `Window`*"]
    pub fn create_image_bitmap_with_html_image_element(
        this: &Window,
        a_image: &HtmlImageElement,
    ) -> Result<::js_sys::Promise, JsValue>;
    #[cfg(feature = "HtmlVideoElement")]
    # [ wasm_bindgen ( catch , method , structural , js_name = createImageBitmap ) ]
    #[doc = "The `createImageBitmap()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/createImageBitmap)\n\n*This API requires the following crate features to be activated: `HtmlVideoElement`, `Window`*"]
    pub fn create_image_bitmap_with_html_video_element(
        this: &Window,
        a_image: &HtmlVideoElement,
    ) -> Result<::js_sys::Promise, JsValue>;
    #[cfg(feature = "HtmlCanvasElement")]
    # [ wasm_bindgen ( catch , method , structural , js_name = createImageBitmap ) ]
    #[doc = "The `createImageBitmap()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/createImageBitmap)\n\n*This API requires the following crate features to be activated: `HtmlCanvasElement`, `Window`*"]
    pub fn create_image_bitmap_with_html_canvas_element(
        this: &Window,
        a_image: &HtmlCanvasElement,
    ) -> Result<::js_sys::Promise, JsValue>;
    #[cfg(feature = "Blob")]
    # [ wasm_bindgen ( catch , method , structural , js_name = createImageBitmap ) ]
    #[doc = "The `createImageBitmap()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/createImageBitmap)\n\n*This API requires the following crate features to be activated: `Blob`, `Window`*"]
    pub fn create_image_bitmap_with_blob(
        this: &Window,
        a_image: &Blob,
    ) -> Result<::js_sys::Promise, JsValue>;
    #[cfg(feature = "ImageData")]
    # [ wasm_bindgen ( catch , method , structural , js_name = createImageBitmap ) ]
    #[doc = "The `createImageBitmap()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/createImageBitmap)\n\n*This API requires the following crate features to be activated: `ImageData`, `Window`*"]
    pub fn create_image_bitmap_with_image_data(
        this: &Window,
        a_image: &ImageData,
    ) -> Result<::js_sys::Promise, JsValue>;
    #[cfg(feature = "CanvasRenderingContext2d")]
    # [ wasm_bindgen ( catch , method , structural , js_name = createImageBitmap ) ]
    #[doc = "The `createImageBitmap()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/createImageBitmap)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `Window`*"]
    pub fn create_image_bitmap_with_canvas_rendering_context_2d(
        this: &Window,
        a_image: &CanvasRenderingContext2d,
    ) -> Result<::js_sys::Promise, JsValue>;
    #[cfg(feature = "ImageBitmap")]
    # [ wasm_bindgen ( catch , method , structural , js_name = createImageBitmap ) ]
    #[doc = "The `createImageBitmap()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/createImageBitmap)\n\n*This API requires the following crate features to be activated: `ImageBitmap`, `Window`*"]
    pub fn create_image_bitmap_with_image_bitmap(
        this: &Window,
        a_image: &ImageBitmap,
    ) -> Result<::js_sys::Promise, JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = createImageBitmap ) ]
    #[doc = "The `createImageBitmap()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/createImageBitmap)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn create_image_bitmap_with_buffer_source(
        this: &Window,
        a_image: &::js_sys::Object,
    ) -> Result<::js_sys::Promise, JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = createImageBitmap ) ]
    #[doc = "The `createImageBitmap()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/createImageBitmap)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn create_image_bitmap_with_u8_array(
        this: &Window,
        a_image: &mut [u8],
    ) -> Result<::js_sys::Promise, JsValue>;
    #[cfg(feature = "HtmlImageElement")]
    # [ wasm_bindgen ( catch , method , structural , js_name = createImageBitmap ) ]
    #[doc = "The `createImageBitmap()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/createImageBitmap)\n\n*This API requires the following crate features to be activated: `HtmlImageElement`, `Window`*"]
    pub fn create_image_bitmap_with_html_image_element_and_a_sx_and_a_sy_and_a_sw_and_a_sh(
        this: &Window,
        a_image: &HtmlImageElement,
        a_sx: i32,
        a_sy: i32,
        a_sw: i32,
        a_sh: i32,
    ) -> Result<::js_sys::Promise, JsValue>;
    #[cfg(feature = "HtmlVideoElement")]
    # [ wasm_bindgen ( catch , method , structural , js_name = createImageBitmap ) ]
    #[doc = "The `createImageBitmap()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/createImageBitmap)\n\n*This API requires the following crate features to be activated: `HtmlVideoElement`, `Window`*"]
    pub fn create_image_bitmap_with_html_video_element_and_a_sx_and_a_sy_and_a_sw_and_a_sh(
        this: &Window,
        a_image: &HtmlVideoElement,
        a_sx: i32,
        a_sy: i32,
        a_sw: i32,
        a_sh: i32,
    ) -> Result<::js_sys::Promise, JsValue>;
    #[cfg(feature = "HtmlCanvasElement")]
    # [ wasm_bindgen ( catch , method , structural , js_name = createImageBitmap ) ]
    #[doc = "The `createImageBitmap()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/createImageBitmap)\n\n*This API requires the following crate features to be activated: `HtmlCanvasElement`, `Window`*"]
    pub fn create_image_bitmap_with_html_canvas_element_and_a_sx_and_a_sy_and_a_sw_and_a_sh(
        this: &Window,
        a_image: &HtmlCanvasElement,
        a_sx: i32,
        a_sy: i32,
        a_sw: i32,
        a_sh: i32,
    ) -> Result<::js_sys::Promise, JsValue>;
    #[cfg(feature = "Blob")]
    # [ wasm_bindgen ( catch , method , structural , js_name = createImageBitmap ) ]
    #[doc = "The `createImageBitmap()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/createImageBitmap)\n\n*This API requires the following crate features to be activated: `Blob`, `Window`*"]
    pub fn create_image_bitmap_with_blob_and_a_sx_and_a_sy_and_a_sw_and_a_sh(
        this: &Window,
        a_image: &Blob,
        a_sx: i32,
        a_sy: i32,
        a_sw: i32,
        a_sh: i32,
    ) -> Result<::js_sys::Promise, JsValue>;
    #[cfg(feature = "ImageData")]
    # [ wasm_bindgen ( catch , method , structural , js_name = createImageBitmap ) ]
    #[doc = "The `createImageBitmap()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/createImageBitmap)\n\n*This API requires the following crate features to be activated: `ImageData`, `Window`*"]
    pub fn create_image_bitmap_with_image_data_and_a_sx_and_a_sy_and_a_sw_and_a_sh(
        this: &Window,
        a_image: &ImageData,
        a_sx: i32,
        a_sy: i32,
        a_sw: i32,
        a_sh: i32,
    ) -> Result<::js_sys::Promise, JsValue>;
    #[cfg(feature = "CanvasRenderingContext2d")]
    # [ wasm_bindgen ( catch , method , structural , js_name = createImageBitmap ) ]
    #[doc = "The `createImageBitmap()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/createImageBitmap)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `Window`*"]
    pub fn create_image_bitmap_with_canvas_rendering_context_2d_and_a_sx_and_a_sy_and_a_sw_and_a_sh(
        this: &Window,
        a_image: &CanvasRenderingContext2d,
        a_sx: i32,
        a_sy: i32,
        a_sw: i32,
        a_sh: i32,
    ) -> Result<::js_sys::Promise, JsValue>;
    #[cfg(feature = "ImageBitmap")]
    # [ wasm_bindgen ( catch , method , structural , js_name = createImageBitmap ) ]
    #[doc = "The `createImageBitmap()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/createImageBitmap)\n\n*This API requires the following crate features to be activated: `ImageBitmap`, `Window`*"]
    pub fn create_image_bitmap_with_image_bitmap_and_a_sx_and_a_sy_and_a_sw_and_a_sh(
        this: &Window,
        a_image: &ImageBitmap,
        a_sx: i32,
        a_sy: i32,
        a_sw: i32,
        a_sh: i32,
    ) -> Result<::js_sys::Promise, JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = createImageBitmap ) ]
    #[doc = "The `createImageBitmap()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/createImageBitmap)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn create_image_bitmap_with_buffer_source_and_a_sx_and_a_sy_and_a_sw_and_a_sh(
        this: &Window,
        a_image: &::js_sys::Object,
        a_sx: i32,
        a_sy: i32,
        a_sw: i32,
        a_sh: i32,
    ) -> Result<::js_sys::Promise, JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = createImageBitmap ) ]
    #[doc = "The `createImageBitmap()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/createImageBitmap)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn create_image_bitmap_with_u8_array_and_a_sx_and_a_sy_and_a_sw_and_a_sh(
        this: &Window,
        a_image: &mut [u8],
        a_sx: i32,
        a_sy: i32,
        a_sw: i32,
        a_sh: i32,
    ) -> Result<::js_sys::Promise, JsValue>;
    #[cfg(feature = "Request")]
    # [ wasm_bindgen ( method , structural , js_name = fetch ) ]
    #[doc = "The `fetch()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/fetch)\n\n*This API requires the following crate features to be activated: `Request`, `Window`*"]
    pub fn fetch_with_request(this: &Window, input: &Request) -> ::js_sys::Promise;
    # [ wasm_bindgen ( method , structural , js_name = fetch ) ]
    #[doc = "The `fetch()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/fetch)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn fetch_with_str(this: &Window, input: &str) -> ::js_sys::Promise;
    #[cfg(all(feature = "Request", feature = "RequestInit",))]
    # [ wasm_bindgen ( method , structural , js_name = fetch ) ]
    #[doc = "The `fetch()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/fetch)\n\n*This API requires the following crate features to be activated: `Request`, `RequestInit`, `Window`*"]
    pub fn fetch_with_request_and_init(
        this: &Window,
        input: &Request,
        init: &RequestInit,
    ) -> ::js_sys::Promise;
    #[cfg(feature = "RequestInit")]
    # [ wasm_bindgen ( method , structural , js_name = fetch ) ]
    #[doc = "The `fetch()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/fetch)\n\n*This API requires the following crate features to be activated: `RequestInit`, `Window`*"]
    pub fn fetch_with_str_and_init(
        this: &Window,
        input: &str,
        init: &RequestInit,
    ) -> ::js_sys::Promise;
    # [ wasm_bindgen ( catch , method , structural , js_name = setInterval ) ]
    #[doc = "The `setInterval()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/setInterval)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_interval_with_callback(
        this: &Window,
        handler: &::js_sys::Function,
    ) -> Result<i32, JsValue>;
    # [ wasm_bindgen ( catch , method , structural , variadic , js_name = setInterval ) ]
    #[doc = "The `setInterval()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/setInterval)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_interval_with_callback_and_timeout_and_arguments(
        this: &Window,
        handler: &::js_sys::Function,
        timeout: i32,
        arguments: &::wasm_bindgen::JsValue,
    ) -> Result<i32, JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = setInterval ) ]
    #[doc = "The `setInterval()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/setInterval)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_interval_with_callback_and_timeout_and_arguments_0(
        this: &Window,
        handler: &::js_sys::Function,
        timeout: i32,
    ) -> Result<i32, JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = setInterval ) ]
    #[doc = "The `setInterval()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/setInterval)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_interval_with_callback_and_timeout_and_arguments_1(
        this: &Window,
        handler: &::js_sys::Function,
        timeout: i32,
        arguments_1: &::wasm_bindgen::JsValue,
    ) -> Result<i32, JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = setInterval ) ]
    #[doc = "The `setInterval()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/setInterval)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_interval_with_callback_and_timeout_and_arguments_2(
        this: &Window,
        handler: &::js_sys::Function,
        timeout: i32,
        arguments_1: &::wasm_bindgen::JsValue,
        arguments_2: &::wasm_bindgen::JsValue,
    ) -> Result<i32, JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = setInterval ) ]
    #[doc = "The `setInterval()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/setInterval)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_interval_with_callback_and_timeout_and_arguments_3(
        this: &Window,
        handler: &::js_sys::Function,
        timeout: i32,
        arguments_1: &::wasm_bindgen::JsValue,
        arguments_2: &::wasm_bindgen::JsValue,
        arguments_3: &::wasm_bindgen::JsValue,
    ) -> Result<i32, JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = setInterval ) ]
    #[doc = "The `setInterval()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/setInterval)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_interval_with_callback_and_timeout_and_arguments_4(
        this: &Window,
        handler: &::js_sys::Function,
        timeout: i32,
        arguments_1: &::wasm_bindgen::JsValue,
        arguments_2: &::wasm_bindgen::JsValue,
        arguments_3: &::wasm_bindgen::JsValue,
        arguments_4: &::wasm_bindgen::JsValue,
    ) -> Result<i32, JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = setInterval ) ]
    #[doc = "The `setInterval()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/setInterval)\n\n*This API requires the following crate features to be activated: `Window`*"]
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
    # [ wasm_bindgen ( catch , method , structural , js_name = setInterval ) ]
    #[doc = "The `setInterval()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/setInterval)\n\n*This API requires the following crate features to be activated: `Window`*"]
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
    # [ wasm_bindgen ( catch , method , structural , js_name = setInterval ) ]
    #[doc = "The `setInterval()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/setInterval)\n\n*This API requires the following crate features to be activated: `Window`*"]
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
    # [ wasm_bindgen ( catch , method , structural , js_name = setInterval ) ]
    #[doc = "The `setInterval()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/setInterval)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_interval_with_str(this: &Window, handler: &str) -> Result<i32, JsValue>;
    # [ wasm_bindgen ( catch , method , structural , variadic , js_name = setInterval ) ]
    #[doc = "The `setInterval()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/setInterval)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_interval_with_str_and_timeout_and_unused(
        this: &Window,
        handler: &str,
        timeout: i32,
        unused: &::wasm_bindgen::JsValue,
    ) -> Result<i32, JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = setInterval ) ]
    #[doc = "The `setInterval()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/setInterval)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_interval_with_str_and_timeout_and_unused_0(
        this: &Window,
        handler: &str,
        timeout: i32,
    ) -> Result<i32, JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = setInterval ) ]
    #[doc = "The `setInterval()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/setInterval)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_interval_with_str_and_timeout_and_unused_1(
        this: &Window,
        handler: &str,
        timeout: i32,
        unused_1: &::wasm_bindgen::JsValue,
    ) -> Result<i32, JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = setInterval ) ]
    #[doc = "The `setInterval()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/setInterval)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_interval_with_str_and_timeout_and_unused_2(
        this: &Window,
        handler: &str,
        timeout: i32,
        unused_1: &::wasm_bindgen::JsValue,
        unused_2: &::wasm_bindgen::JsValue,
    ) -> Result<i32, JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = setInterval ) ]
    #[doc = "The `setInterval()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/setInterval)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_interval_with_str_and_timeout_and_unused_3(
        this: &Window,
        handler: &str,
        timeout: i32,
        unused_1: &::wasm_bindgen::JsValue,
        unused_2: &::wasm_bindgen::JsValue,
        unused_3: &::wasm_bindgen::JsValue,
    ) -> Result<i32, JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = setInterval ) ]
    #[doc = "The `setInterval()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/setInterval)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_interval_with_str_and_timeout_and_unused_4(
        this: &Window,
        handler: &str,
        timeout: i32,
        unused_1: &::wasm_bindgen::JsValue,
        unused_2: &::wasm_bindgen::JsValue,
        unused_3: &::wasm_bindgen::JsValue,
        unused_4: &::wasm_bindgen::JsValue,
    ) -> Result<i32, JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = setInterval ) ]
    #[doc = "The `setInterval()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/setInterval)\n\n*This API requires the following crate features to be activated: `Window`*"]
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
    # [ wasm_bindgen ( catch , method , structural , js_name = setInterval ) ]
    #[doc = "The `setInterval()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/setInterval)\n\n*This API requires the following crate features to be activated: `Window`*"]
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
    # [ wasm_bindgen ( catch , method , structural , js_name = setInterval ) ]
    #[doc = "The `setInterval()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/setInterval)\n\n*This API requires the following crate features to be activated: `Window`*"]
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
    # [ wasm_bindgen ( catch , method , structural , js_name = setTimeout ) ]
    #[doc = "The `setTimeout()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/setTimeout)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_timeout_with_callback(
        this: &Window,
        handler: &::js_sys::Function,
    ) -> Result<i32, JsValue>;
    # [ wasm_bindgen ( catch , method , structural , variadic , js_name = setTimeout ) ]
    #[doc = "The `setTimeout()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/setTimeout)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_timeout_with_callback_and_timeout_and_arguments(
        this: &Window,
        handler: &::js_sys::Function,
        timeout: i32,
        arguments: &::wasm_bindgen::JsValue,
    ) -> Result<i32, JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = setTimeout ) ]
    #[doc = "The `setTimeout()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/setTimeout)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_timeout_with_callback_and_timeout_and_arguments_0(
        this: &Window,
        handler: &::js_sys::Function,
        timeout: i32,
    ) -> Result<i32, JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = setTimeout ) ]
    #[doc = "The `setTimeout()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/setTimeout)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_timeout_with_callback_and_timeout_and_arguments_1(
        this: &Window,
        handler: &::js_sys::Function,
        timeout: i32,
        arguments_1: &::wasm_bindgen::JsValue,
    ) -> Result<i32, JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = setTimeout ) ]
    #[doc = "The `setTimeout()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/setTimeout)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_timeout_with_callback_and_timeout_and_arguments_2(
        this: &Window,
        handler: &::js_sys::Function,
        timeout: i32,
        arguments_1: &::wasm_bindgen::JsValue,
        arguments_2: &::wasm_bindgen::JsValue,
    ) -> Result<i32, JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = setTimeout ) ]
    #[doc = "The `setTimeout()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/setTimeout)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_timeout_with_callback_and_timeout_and_arguments_3(
        this: &Window,
        handler: &::js_sys::Function,
        timeout: i32,
        arguments_1: &::wasm_bindgen::JsValue,
        arguments_2: &::wasm_bindgen::JsValue,
        arguments_3: &::wasm_bindgen::JsValue,
    ) -> Result<i32, JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = setTimeout ) ]
    #[doc = "The `setTimeout()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/setTimeout)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_timeout_with_callback_and_timeout_and_arguments_4(
        this: &Window,
        handler: &::js_sys::Function,
        timeout: i32,
        arguments_1: &::wasm_bindgen::JsValue,
        arguments_2: &::wasm_bindgen::JsValue,
        arguments_3: &::wasm_bindgen::JsValue,
        arguments_4: &::wasm_bindgen::JsValue,
    ) -> Result<i32, JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = setTimeout ) ]
    #[doc = "The `setTimeout()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/setTimeout)\n\n*This API requires the following crate features to be activated: `Window`*"]
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
    # [ wasm_bindgen ( catch , method , structural , js_name = setTimeout ) ]
    #[doc = "The `setTimeout()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/setTimeout)\n\n*This API requires the following crate features to be activated: `Window`*"]
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
    # [ wasm_bindgen ( catch , method , structural , js_name = setTimeout ) ]
    #[doc = "The `setTimeout()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/setTimeout)\n\n*This API requires the following crate features to be activated: `Window`*"]
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
    # [ wasm_bindgen ( catch , method , structural , js_name = setTimeout ) ]
    #[doc = "The `setTimeout()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/setTimeout)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_timeout_with_str(this: &Window, handler: &str) -> Result<i32, JsValue>;
    # [ wasm_bindgen ( catch , method , structural , variadic , js_name = setTimeout ) ]
    #[doc = "The `setTimeout()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/setTimeout)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_timeout_with_str_and_timeout_and_unused(
        this: &Window,
        handler: &str,
        timeout: i32,
        unused: &::wasm_bindgen::JsValue,
    ) -> Result<i32, JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = setTimeout ) ]
    #[doc = "The `setTimeout()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/setTimeout)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_timeout_with_str_and_timeout_and_unused_0(
        this: &Window,
        handler: &str,
        timeout: i32,
    ) -> Result<i32, JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = setTimeout ) ]
    #[doc = "The `setTimeout()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/setTimeout)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_timeout_with_str_and_timeout_and_unused_1(
        this: &Window,
        handler: &str,
        timeout: i32,
        unused_1: &::wasm_bindgen::JsValue,
    ) -> Result<i32, JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = setTimeout ) ]
    #[doc = "The `setTimeout()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/setTimeout)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_timeout_with_str_and_timeout_and_unused_2(
        this: &Window,
        handler: &str,
        timeout: i32,
        unused_1: &::wasm_bindgen::JsValue,
        unused_2: &::wasm_bindgen::JsValue,
    ) -> Result<i32, JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = setTimeout ) ]
    #[doc = "The `setTimeout()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/setTimeout)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_timeout_with_str_and_timeout_and_unused_3(
        this: &Window,
        handler: &str,
        timeout: i32,
        unused_1: &::wasm_bindgen::JsValue,
        unused_2: &::wasm_bindgen::JsValue,
        unused_3: &::wasm_bindgen::JsValue,
    ) -> Result<i32, JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = setTimeout ) ]
    #[doc = "The `setTimeout()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/setTimeout)\n\n*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_timeout_with_str_and_timeout_and_unused_4(
        this: &Window,
        handler: &str,
        timeout: i32,
        unused_1: &::wasm_bindgen::JsValue,
        unused_2: &::wasm_bindgen::JsValue,
        unused_3: &::wasm_bindgen::JsValue,
        unused_4: &::wasm_bindgen::JsValue,
    ) -> Result<i32, JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = setTimeout ) ]
    #[doc = "The `setTimeout()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/setTimeout)\n\n*This API requires the following crate features to be activated: `Window`*"]
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
    # [ wasm_bindgen ( catch , method , structural , js_name = setTimeout ) ]
    #[doc = "The `setTimeout()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/setTimeout)\n\n*This API requires the following crate features to be activated: `Window`*"]
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
    # [ wasm_bindgen ( catch , method , structural , js_name = setTimeout ) ]
    #[doc = "The `setTimeout()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/setTimeout)\n\n*This API requires the following crate features to be activated: `Window`*"]
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
