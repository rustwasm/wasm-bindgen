use wasm_bindgen::prelude::*;

#[wasm_bindgen]
///The `RtcDataChannelType` enum.
///
///*This API requires the following crate features to be activated: `RtcDataChannelType`*
#[derive(Debug, Clone, Copy, PartialEq, Eq)]

pub enum RtcDataChannelType {
    Arraybuffer = "arraybuffer",
    Blob = "blob",
}
