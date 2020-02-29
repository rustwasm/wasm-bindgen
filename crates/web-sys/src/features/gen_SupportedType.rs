use wasm_bindgen::prelude::*;

#[wasm_bindgen]
///The `SupportedType` enum.
///
///*This API requires the following crate features to be activated: `SupportedType`*
#[derive(Debug, Clone, Copy, PartialEq, Eq)]

pub enum SupportedType {
    TextHtml = "text/html",
    TextXml = "text/xml",
    ApplicationXml = "application/xml",
    ApplicationXhtmlXml = "application/xhtml+xml",
    ImageSvgXml = "image/svg+xml",
}
