use crate::__steps__::assembly::given_there_is_an_assembly_with_one_failing_test;
use crate::__steps__::standard_output::then_the_standard_output_should_have;
use crate::__steps__::wasm_bindgen_test_runner::when_wasm_bindgen_test_runner_is_invoked_with_the_argument;
use crate::__steps__::Context;

#[test]
fn outputs_the_wasm_bindgen_test_runner_version_information_feature() {
    let mut context = Context::new();
    given_there_is_an_assembly_with_one_failing_test(&mut context);
    when_wasm_bindgen_test_runner_is_invoked_with_the_argument(&mut context, "-V");
    then_the_standard_output_should_have(
        context,
        &format!("wasm-bindgen-test-runner {}", env!("CARGO_PKG_VERSION")),
    );
}
