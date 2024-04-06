use crate::__steps__::assembly::given_there_is_an_assembly_with_one_failing_test;
use crate::__steps__::standard_error::then_the_standard_error_should_have;
use crate::__steps__::wasm_bindgen_test_runner::when_wasm_bindgen_test_runner_is_invoked_without_arguments;
use crate::__steps__::Context;

#[test]
fn outputs_the_wasm_bindgen_test_runner_usage_information_feature() {
    let mut context = Context::new();
    given_there_is_an_assembly_with_one_failing_test(&mut context);
    when_wasm_bindgen_test_runner_is_invoked_without_arguments(&mut context);
    then_the_standard_error_should_have(
        context,
        "Usage:\n    wasm-bindgen-test-runner [options] <input>\n    wasm-bindgen-test-runner -h | --help\n    wasm-bindgen-test-runner -V | --version",
    );
}
