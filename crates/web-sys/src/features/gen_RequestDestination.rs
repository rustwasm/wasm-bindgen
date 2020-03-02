use wasm_bindgen::prelude::*;

#[wasm_bindgen]
///The `RequestDestination` enum.
///
///*This API requires the following crate features to be activated: `RequestDestination`*
#[derive(Debug, Clone, Copy, PartialEq, Eq)]

pub enum RequestDestination {
    None = "",
    Audio = "audio",
    Audioworklet = "audioworklet",
    Document = "document",
    Embed = "embed",
    Font = "font",
    Image = "image",
    Manifest = "manifest",
    Object = "object",
    Paintworklet = "paintworklet",
    Report = "report",
    Script = "script",
    Sharedworker = "sharedworker",
    Style = "style",
    Track = "track",
    Video = "video",
    Worker = "worker",
    Xslt = "xslt",
}
