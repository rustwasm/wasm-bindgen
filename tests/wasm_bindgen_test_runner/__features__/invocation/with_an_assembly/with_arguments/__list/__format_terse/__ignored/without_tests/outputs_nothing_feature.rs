use crate::__steps__::assembly::given_there_is_an_assembly_without_anything;
use crate::__steps__::standard_output::then_the_standard_output_should_be_empty;
use crate::__steps__::wasm_bindgen_test_runner::when_wasm_bindgen_test_runner_is_invoked_with_the_assembly_and_the_arguments;
use crate::__steps__::Context;

#[test]
fn outputs_nothing_feature() {
    let mut context = Context::new();
    given_there_is_an_assembly_without_anything(&mut context);
    when_wasm_bindgen_test_runner_is_invoked_with_the_assembly_and_the_arguments(
        &mut context,
        "--list --format terse --ignored",
    );
    then_the_standard_output_should_be_empty(&context);
}
