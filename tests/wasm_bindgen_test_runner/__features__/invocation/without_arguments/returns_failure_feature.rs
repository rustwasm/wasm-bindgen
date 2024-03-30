use crate::__steps__::error_code::then_failure_should_have_been_returned;
use crate::__steps__::wasm_bindgen_test_runner::when_wasm_bindgen_test_runner_is_invoked_without_arguments;
use crate::__steps__::Context;

#[test]
fn returns_an_error_code_feature() {
    let mut context = Context::new();
    when_wasm_bindgen_test_runner_is_invoked_without_arguments(&mut context);
    then_failure_should_have_been_returned(context);
}
