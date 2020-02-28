use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[doc = "The `CaretChangedReason` enum."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `CaretChangedReason`*"]
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum CaretChangedReason {
    Visibilitychange = "visibilitychange",
    Updateposition = "updateposition",
    Longpressonemptycontent = "longpressonemptycontent",
    Taponcaret = "taponcaret",
    Presscaret = "presscaret",
    Releasecaret = "releasecaret",
    Scroll = "scroll",
}
