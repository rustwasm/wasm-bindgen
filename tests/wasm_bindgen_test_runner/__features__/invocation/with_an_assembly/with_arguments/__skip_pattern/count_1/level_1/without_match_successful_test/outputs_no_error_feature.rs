use crate::__steps__::assembly::given_there_is_an_assembly_with_one_successful_level_1_test;
use crate::__steps__::standard_error::then_the_standard_error_should_be_empty;
use crate::__steps__::wasm_bindgen_test_runner::when_wasm_bindgen_test_runner_is_invoked_with_the_assembly_and_the_arguments;
use crate::__steps__::Context;

#[test]
fn outputs_no_error_feature() {
    let mut context = Context::new();
    given_there_is_an_assembly_with_one_successful_level_1_test(&mut context);
    when_wasm_bindgen_test_runner_is_invoked_with_the_assembly_and_the_arguments(
        &mut context,
        "--skip pattern",
    );
    then_the_standard_error_should_be_empty(context);
}