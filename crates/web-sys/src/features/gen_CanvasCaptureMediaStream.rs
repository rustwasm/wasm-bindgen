use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = MediaStream , extends = EventTarget , extends = :: js_sys :: Object , js_name = CanvasCaptureMediaStream , typescript_name = CanvasCaptureMediaStream ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `CanvasCaptureMediaStream` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasCaptureMediaStream)
    ///
    ///*This API requires the following crate features to be activated: `CanvasCaptureMediaStream`*
    pub type CanvasCaptureMediaStream;

    #[cfg(feature = "HtmlCanvasElement")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "CanvasCaptureMediaStream" , js_name = canvas ) ]
    ///Getter for the `canvas` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasCaptureMediaStream/canvas)
    ///
    ///*This API requires the following crate features to be activated: `CanvasCaptureMediaStream`, `HtmlCanvasElement`*
    pub fn canvas(this: &CanvasCaptureMediaStream) -> HtmlCanvasElement;

    # [ wasm_bindgen ( method , structural , js_class = "CanvasCaptureMediaStream" , js_name = requestFrame ) ]
    ///The `requestFrame()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasCaptureMediaStream/requestFrame)
    ///
    ///*This API requires the following crate features to be activated: `CanvasCaptureMediaStream`*
    pub fn request_frame(this: &CanvasCaptureMediaStream);

}
