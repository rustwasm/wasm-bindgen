use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = Presentation , typescript_name = Presentation ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `Presentation` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Presentation)\n\n*This API requires the following crate features to be activated: `Presentation`*"]
    pub type Presentation;
    # [ wasm_bindgen ( structural , method , getter , js_name = defaultRequest ) ]
    #[cfg(feature = "PresentationRequest")]
    #[doc = "Getter for the `defaultRequest` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Presentation/defaultRequest)\n\n*This API requires the following crate features to be activated: `Presentation`, `PresentationRequest`*"]
    pub fn default_request(this: &Presentation) -> Option<PresentationRequest>;
    # [ wasm_bindgen ( structural , method , setter , js_name = defaultRequest ) ]
    #[cfg(feature = "PresentationRequest")]
    #[doc = "Setter for the `defaultRequest` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Presentation/defaultRequest)\n\n*This API requires the following crate features to be activated: `Presentation`, `PresentationRequest`*"]
    pub fn set_default_request(this: &Presentation, value: Option<PresentationRequest>);
    # [ wasm_bindgen ( structural , method , getter , js_name = receiver ) ]
    #[cfg(feature = "PresentationReceiver")]
    #[doc = "Getter for the `receiver` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Presentation/receiver)\n\n*This API requires the following crate features to be activated: `Presentation`, `PresentationReceiver`*"]
    pub fn receiver(this: &Presentation) -> Option<PresentationReceiver>;
}
