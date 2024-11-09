use wasm_bindgen::prelude::*;

/// C-style enum
#[wasm_bindgen]
#[derive(Copy, Clone)]
pub enum ImageFormat {
    PNG,
    JPEG,
    GIF,
}

#[wasm_bindgen]
impl ImageFormat {
    pub fn from_str(s: &str) -> ImageFormat {
        match s {
            "PNG" => ImageFormat::PNG,
            "JPEG" => ImageFormat::JPEG,
            "GIF" => ImageFormat::GIF,
            _ => panic!("unknown image format: {}", s),
        }
    }
}

/// String enum
#[wasm_bindgen]
#[derive(Copy, Clone)]
pub enum Status {
    Success = "success",
    Failure = "failure",
}

#[wasm_bindgen]
impl Status {
    /// I have documentation.
    pub fn from_bool(success: bool) -> Status {
        if success {
            Status::Success
        } else {
            Status::Failure
        }
    }
}
