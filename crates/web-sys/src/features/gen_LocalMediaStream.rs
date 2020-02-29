use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = MediaStream , extends = EventTarget , extends = :: js_sys :: Object , js_name = LocalMediaStream , typescript_type = "LocalMediaStream" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `LocalMediaStream` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/LocalMediaStream)
    ///
    ///*This API requires the following crate features to be activated: `LocalMediaStream`*
    pub type LocalMediaStream;

    # [ wasm_bindgen ( method , structural , js_class = "LocalMediaStream" , js_name = stop ) ]
    ///The `stop()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/LocalMediaStream/stop)
    ///
    ///*This API requires the following crate features to be activated: `LocalMediaStream`*
    pub fn stop(this: &LocalMediaStream);

}
