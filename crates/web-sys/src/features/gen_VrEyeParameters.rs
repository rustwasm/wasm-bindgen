use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = VREyeParameters , typescript_name = VREyeParameters ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `VrEyeParameters` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VREyeParameters)
    ///
    ///*This API requires the following crate features to be activated: `VrEyeParameters`*
    pub type VrEyeParameters;

    # [ wasm_bindgen ( structural , catch , method , getter , js_class = "VREyeParameters" , js_name = offset ) ]
    ///Getter for the `offset` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VREyeParameters/offset)
    ///
    ///*This API requires the following crate features to be activated: `VrEyeParameters`*
    pub fn offset(this: &VrEyeParameters) -> Result<Vec<f32>, JsValue>;

    #[cfg(feature = "VrFieldOfView")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "VREyeParameters" , js_name = fieldOfView ) ]
    ///Getter for the `fieldOfView` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VREyeParameters/fieldOfView)
    ///
    ///*This API requires the following crate features to be activated: `VrEyeParameters`, `VrFieldOfView`*
    pub fn field_of_view(this: &VrEyeParameters) -> VrFieldOfView;

    # [ wasm_bindgen ( structural , method , getter , js_class = "VREyeParameters" , js_name = renderWidth ) ]
    ///Getter for the `renderWidth` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VREyeParameters/renderWidth)
    ///
    ///*This API requires the following crate features to be activated: `VrEyeParameters`*
    pub fn render_width(this: &VrEyeParameters) -> u32;

    # [ wasm_bindgen ( structural , method , getter , js_class = "VREyeParameters" , js_name = renderHeight ) ]
    ///Getter for the `renderHeight` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VREyeParameters/renderHeight)
    ///
    ///*This API requires the following crate features to be activated: `VrEyeParameters`*
    pub fn render_height(this: &VrEyeParameters) -> u32;

}
