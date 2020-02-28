use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = MediaStream , extends = EventTarget , extends = :: js_sys :: Object , js_name = CanvasCaptureMediaStream , typescript_name = CanvasCaptureMediaStream ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `CanvasCaptureMediaStream` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasCaptureMediaStream)\n\n*This API requires the following crate features to be activated: `CanvasCaptureMediaStream`*"]
    pub type CanvasCaptureMediaStream;
    # [ wasm_bindgen ( structural , method , getter , js_name = canvas ) ]
    #[cfg(feature = "HtmlCanvasElement")]
    #[doc = "Getter for the `canvas` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasCaptureMediaStream/canvas)\n\n*This API requires the following crate features to be activated: `CanvasCaptureMediaStream`, `HtmlCanvasElement`*"]
    pub fn canvas(this: &CanvasCaptureMediaStream) -> HtmlCanvasElement;
    # [ wasm_bindgen ( method , structural , js_name = requestFrame ) ]
    #[doc = "The `requestFrame()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasCaptureMediaStream/requestFrame)\n\n*This API requires the following crate features to be activated: `CanvasCaptureMediaStream`*"]
    pub fn request_frame(this: &CanvasCaptureMediaStream);
}
