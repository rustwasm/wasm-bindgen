use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[doc = "The `XmlHttpRequestResponseType` enum."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `XmlHttpRequestResponseType`*"]
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum XmlHttpRequestResponseType {
    None = "",
    Arraybuffer = "arraybuffer",
    Blob = "blob",
    Document = "document",
    Json = "json",
    Text = "text",
}
