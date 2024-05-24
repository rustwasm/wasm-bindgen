use crate::__steps__::assembly::given_there_is_an_assembly_with_one_ignored_test;
use crate::__steps__::standard_output::then_the_standard_output_should_have;
use crate::__steps__::wasm_bindgen_test_runner::when_wasm_bindgen_test_runner_is_invoked_with_the_assembly_and_the_arguments;
use crate::__steps__::Context;

#[test]
fn outputs_the_ignored_test_successful_execution_summary_feature() {
    let mut context = Context::new();
    given_there_is_an_assembly_with_one_ignored_test(&mut context);
    when_wasm_bindgen_test_runner_is_invoked_with_the_assembly_and_the_arguments(
        &mut context,
        "--include-ignored",
    );
    then_the_standard_output_should_have(
        &context,
        "test assembly_with_one_ignored_test::ignored ... ok",
    );
}
