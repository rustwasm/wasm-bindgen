use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct ColorWithGetter {
    r: f64,
    _g: f64,
    _b: f64,
    _a: u8,
}

#[wasm_bindgen]
impl ColorWithGetter {
    #[wasm_bindgen(getter)]
    pub fn r(&self) -> f64 {
        self.r
    }
}

#[wasm_bindgen]
pub struct ColorWithSetter {
    r: f64,
    _g: f64,
    _b: f64,
    a: u8,
}

#[wasm_bindgen]
impl ColorWithSetter {
    #[wasm_bindgen(setter)]
    pub fn set_r(&mut self, r: f64) {
        self.r = r;
        self.a = if self.r > 1.0 {
            255
        } else if self.r < 0.0 {
            0
        } else {
            (self.r * 255.0) as u8
        };
    }
}

#[wasm_bindgen]
pub struct ColorWithGetterAndSetter {
    r: f64,
    _g: f64,
    _b: f64,
    a: u8,
}

#[wasm_bindgen]
impl ColorWithGetterAndSetter {
    #[wasm_bindgen(getter)]
    pub fn r(&self) -> f64 {
        self.r
    }

    #[wasm_bindgen(setter)]
    pub fn set_r(&mut self, r: f64) {
        self.r = r;
        self.a = if self.r > 1.0 {
            255
        } else if self.r < 0.0 {
            0
        } else {
            (self.r * 255.0) as u8
        };
    }
}
