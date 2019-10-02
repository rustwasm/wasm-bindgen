use wasm_bindgen::prelude::*;

// lifted from the `console_log` example
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
#[derive(Debug)]
pub struct Counter {
    key: char,
    count: i32,
}

#[wasm_bindgen]
impl Counter {
    pub fn default() -> WasmType<Counter> {
        log("Counter::default");
        Self::new("a", 0)
    }
    #[wasm_bindgen(constructor)]
    pub fn new(key: &str, count: i32) -> WasmType<Counter> {
        log(&format!("Counter::new({}, {})", key, count));
        instantiate! {
            Counter {
                key: key.chars().next().unwrap(),
                count: count,
            }
        }
    }

    pub fn key(&self) -> char {
        log("Counter.key()");
        self.key
    }

    pub fn count(&self) -> i32 {
        log("Counter.count");
        self.count
    }

    pub fn increment(&mut self) {
        log("Counter.increment");
        self.count += 1;
    }

    pub fn update_key(&mut self, key: char) {
        self.key = key;
    }
}
