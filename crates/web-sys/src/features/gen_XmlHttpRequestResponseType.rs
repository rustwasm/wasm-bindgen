use wasm_bindgen::prelude::*;

#[wasm_bindgen]
///The `XmlHttpRequestResponseType` enum.
///
///*This API requires the following crate features to be activated: `XmlHttpRequestResponseType`*
#[derive(Debug, Clone, Copy, PartialEq, Eq)]

pub enum XmlHttpRequestResponseType {
    None = "",
    Arraybuffer = "arraybuffer",
    Blob = "blob",
    Document = "document",
    Json = "json",
    Text = "text",
}
