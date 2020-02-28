use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = RTCStatsReport , typescript_name = RTCStatsReport ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RtcStatsReport` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCStatsReport)\n\n*This API requires the following crate features to be activated: `RtcStatsReport`*"]
    pub type RtcStatsReport;
}
