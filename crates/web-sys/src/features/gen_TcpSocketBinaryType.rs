use wasm_bindgen::prelude::*;

#[wasm_bindgen]
///The `TcpSocketBinaryType` enum.
///
///*This API requires the following crate features to be activated: `TcpSocketBinaryType`*
#[derive(Debug, Clone, Copy, PartialEq, Eq)]

pub enum TcpSocketBinaryType {
    Arraybuffer = "arraybuffer",
    String = "string",
}
