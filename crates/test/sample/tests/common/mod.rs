use std::time::Duration;
use sample::Timeout;
use wasm_bindgen_test::*;

#[wasm_bindgen_test]
fn pass() {
    console_log!("DO NOT SEE ME");
}

#[wasm_bindgen_test]
async fn pass_after_2s() {
    console_log!("immediate log");
    Timeout::new(Duration::new(1, 0)).await;
    console_log!("log after 1s");
    Timeout::new(Duration::new(1, 0)).await;
    console_log!("log at end");
}

#[wasm_bindgen_test]
fn fail() {
    console_log!("helpful messsage, please see me");
    panic!("this is a failing test");
}

#[wasm_bindgen_test]
async fn fail_after_3s() {
    console_log!("immediate log");
    Timeout::new(Duration::new(1, 0)).await;
    console_log!("log after 1s");
    Timeout::new(Duration::new(1, 0)).await;
    console_log!("log after 2s");
    Timeout::new(Duration::new(1, 0)).await;
    panic!("end");
}
