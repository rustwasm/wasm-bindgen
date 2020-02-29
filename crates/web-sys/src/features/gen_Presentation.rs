use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = Presentation , typescript_name = Presentation ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `Presentation` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Presentation)
    ///
    ///*This API requires the following crate features to be activated: `Presentation`*
    pub type Presentation;

    #[cfg(feature = "PresentationRequest")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "Presentation" , js_name = defaultRequest ) ]
    ///Getter for the `defaultRequest` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Presentation/defaultRequest)
    ///
    ///*This API requires the following crate features to be activated: `Presentation`, `PresentationRequest`*
    pub fn default_request(this: &Presentation) -> Option<PresentationRequest>;

    #[cfg(feature = "PresentationRequest")]
    # [ wasm_bindgen ( structural , method , setter , js_class = "Presentation" , js_name = defaultRequest ) ]
    ///Setter for the `defaultRequest` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Presentation/defaultRequest)
    ///
    ///*This API requires the following crate features to be activated: `Presentation`, `PresentationRequest`*
    pub fn set_default_request(this: &Presentation, value: Option<&PresentationRequest>);

    #[cfg(feature = "PresentationReceiver")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "Presentation" , js_name = receiver ) ]
    ///Getter for the `receiver` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Presentation/receiver)
    ///
    ///*This API requires the following crate features to be activated: `Presentation`, `PresentationReceiver`*
    pub fn receiver(this: &Presentation) -> Option<PresentationReceiver>;

}
