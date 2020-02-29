use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = Touch , typescript_name = Touch ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `Touch` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Touch)
    ///
    ///*This API requires the following crate features to be activated: `Touch`*
    pub type Touch;

    # [ wasm_bindgen ( structural , method , getter , js_class = "Touch" , js_name = identifier ) ]
    ///Getter for the `identifier` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Touch/identifier)
    ///
    ///*This API requires the following crate features to be activated: `Touch`*
    pub fn identifier(this: &Touch) -> i32;

    #[cfg(feature = "EventTarget")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "Touch" , js_name = target ) ]
    ///Getter for the `target` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Touch/target)
    ///
    ///*This API requires the following crate features to be activated: `EventTarget`, `Touch`*
    pub fn target(this: &Touch) -> Option<EventTarget>;

    # [ wasm_bindgen ( structural , method , getter , js_class = "Touch" , js_name = screenX ) ]
    ///Getter for the `screenX` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Touch/screenX)
    ///
    ///*This API requires the following crate features to be activated: `Touch`*
    pub fn screen_x(this: &Touch) -> i32;

    # [ wasm_bindgen ( structural , method , getter , js_class = "Touch" , js_name = screenY ) ]
    ///Getter for the `screenY` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Touch/screenY)
    ///
    ///*This API requires the following crate features to be activated: `Touch`*
    pub fn screen_y(this: &Touch) -> i32;

    # [ wasm_bindgen ( structural , method , getter , js_class = "Touch" , js_name = clientX ) ]
    ///Getter for the `clientX` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Touch/clientX)
    ///
    ///*This API requires the following crate features to be activated: `Touch`*
    pub fn client_x(this: &Touch) -> i32;

    # [ wasm_bindgen ( structural , method , getter , js_class = "Touch" , js_name = clientY ) ]
    ///Getter for the `clientY` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Touch/clientY)
    ///
    ///*This API requires the following crate features to be activated: `Touch`*
    pub fn client_y(this: &Touch) -> i32;

    # [ wasm_bindgen ( structural , method , getter , js_class = "Touch" , js_name = pageX ) ]
    ///Getter for the `pageX` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Touch/pageX)
    ///
    ///*This API requires the following crate features to be activated: `Touch`*
    pub fn page_x(this: &Touch) -> i32;

    # [ wasm_bindgen ( structural , method , getter , js_class = "Touch" , js_name = pageY ) ]
    ///Getter for the `pageY` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Touch/pageY)
    ///
    ///*This API requires the following crate features to be activated: `Touch`*
    pub fn page_y(this: &Touch) -> i32;

    # [ wasm_bindgen ( structural , method , getter , js_class = "Touch" , js_name = radiusX ) ]
    ///Getter for the `radiusX` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Touch/radiusX)
    ///
    ///*This API requires the following crate features to be activated: `Touch`*
    pub fn radius_x(this: &Touch) -> i32;

    # [ wasm_bindgen ( structural , method , getter , js_class = "Touch" , js_name = radiusY ) ]
    ///Getter for the `radiusY` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Touch/radiusY)
    ///
    ///*This API requires the following crate features to be activated: `Touch`*
    pub fn radius_y(this: &Touch) -> i32;

    # [ wasm_bindgen ( structural , method , getter , js_class = "Touch" , js_name = rotationAngle ) ]
    ///Getter for the `rotationAngle` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Touch/rotationAngle)
    ///
    ///*This API requires the following crate features to be activated: `Touch`*
    pub fn rotation_angle(this: &Touch) -> f32;

    # [ wasm_bindgen ( structural , method , getter , js_class = "Touch" , js_name = force ) ]
    ///Getter for the `force` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Touch/force)
    ///
    ///*This API requires the following crate features to be activated: `Touch`*
    pub fn force(this: &Touch) -> f32;

    #[cfg(feature = "TouchInit")]
    #[wasm_bindgen(catch, constructor, js_class = "Touch")]
    ///The `new Touch(..)` constructor, creating a new instance of `Touch`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Touch/Touch)
    ///
    ///*This API requires the following crate features to be activated: `Touch`, `TouchInit`*
    pub fn new(touch_init_dict: &TouchInit) -> Result<Touch, JsValue>;

}
