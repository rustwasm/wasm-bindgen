#![feature(use_extern_macros, wasm_import_module)]

extern crate humantime;
extern crate wasm_bindgen;

use std::time::{Duration, SystemTime, UNIX_EPOCH};

use wasm_bindgen::prelude::*;

// Like with the `dom` example this block will eventually be auto-generated, but
// for now we can write it by hand to get by!
#[wasm_bindgen]
extern "C" {
    type Performance;
    static performance: Performance;
    #[wasm_bindgen(method)]
    fn now(this: &Performance) -> f64;
    #[wasm_bindgen(method, getter)]
    fn timing(this: &Performance) -> PerformanceTiming;

    type PerformanceTiming;
    #[wasm_bindgen(method, getter)]
    fn requestStart(this: &PerformanceTiming) -> f64;
    #[wasm_bindgen(method, getter)]
    fn responseEnd(this: &PerformanceTiming) -> f64;

    #[wasm_bindgen(js_namespace = console)]
    fn log(a: &str);
}

macro_rules! println {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

// Called by our JS entry point to run the example
#[wasm_bindgen]
pub fn run() {
    println!("the current time is {}", performance.now());

    let start = perf_to_system(performance.timing().requestStart());
    let end = perf_to_system(performance.timing().responseEnd());

    println!("request started at {}", humantime::format_rfc3339(start));
    println!("request ended at {}", humantime::format_rfc3339(end));
}

fn perf_to_system(amt: f64) -> SystemTime {
    let secs = (amt as u64) / 1_000;
    let nanos = ((amt as u32) % 1_000) * 1_000_000;
    UNIX_EPOCH + Duration::new(secs, nanos)
}
