use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct ColorWithGetters {
    r: f64,
    _g: f64,
    _b: f64,
    _a: u8,
}

#[wasm_bindgen]
impl ColorWithGetters {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            r: 0.0,
            _g: 0.0,
            _b: 0.0,
            _a: 0,
        }
    }

    #[wasm_bindgen(getter)]
    pub fn r(&self) -> f64 {
        self.r
    }

    #[wasm_bindgen(getter)]
    pub fn color_space() -> String {
        "sRGB".to_owned()
    }
}

#[wasm_bindgen]
pub struct ColorWithSetters {
    r: f64,
    _g: f64,
    _b: f64,
    a: u8,
}

#[wasm_bindgen]
impl ColorWithSetters {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            r: 0.0,
            _g: 0.0,
            _b: 0.0,
            a: 0,
        }
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

    #[wasm_bindgen(setter)]
    pub fn set_color_space(_: String) {}
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
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            r: 0.0,
            _g: 0.0,
            _b: 0.0,
            a: 0,
        }
    }

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

#[wasm_bindgen]
pub struct ColorWithReadonly {
    #[wasm_bindgen(readonly)]
    pub r: f64,
    #[wasm_bindgen(readonly)]
    pub g: f64,
    #[wasm_bindgen(readonly)]
    pub b: f64,
    pub a: u8,
}

#[wasm_bindgen]
impl ColorWithReadonly {
    #[wasm_bindgen(constructor)]
    pub fn new(r: f64, g: f64, b: f64) -> ColorWithReadonly {
        Self { r, b, g, a: 0 }
    }
}
