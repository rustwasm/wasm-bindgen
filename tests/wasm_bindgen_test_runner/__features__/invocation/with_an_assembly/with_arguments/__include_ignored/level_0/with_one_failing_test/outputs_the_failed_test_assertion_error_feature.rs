use crate::__steps__::assembly::given_there_is_an_assembly_with_one_failing_test;
use crate::__steps__::standard_error::then_the_standard_error_should_have;
use crate::__steps__::wasm_bindgen_test_runner::when_wasm_bindgen_test_runner_is_invoked_with_the_assembly_and_the_arguments;
use crate::__steps__::Context;

#[test]
fn outputs_the_failed_test_assertion_error_feature() {
    let mut context = Context::new();
    given_there_is_an_assembly_with_one_failing_test(&mut context);
    when_wasm_bindgen_test_runner_is_invoked_with_the_assembly_and_the_arguments(
        &mut context,
        "--include-ignored",
    );
    then_the_standard_error_should_have(
        &context,
        "assertion `left == right` failed\n  left: 1\n right: 2",
    );
}
