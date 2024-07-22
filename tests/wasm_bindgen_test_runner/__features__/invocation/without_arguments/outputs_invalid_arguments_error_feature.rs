use crate::__steps__::standard_error::then_the_standard_error_should_have;
use crate::__steps__::wasm_bindgen_test_runner::when_wasm_bindgen_test_runner_is_invoked_without_arguments;
use crate::__steps__::Context;

#[test]
fn outputs_test_file_missing_error_feature() {
    let mut context = Context::new();
    when_wasm_bindgen_test_runner_is_invoked_without_arguments(&mut context);
    then_the_standard_error_should_have(&context, "Invalid arguments.");
}
