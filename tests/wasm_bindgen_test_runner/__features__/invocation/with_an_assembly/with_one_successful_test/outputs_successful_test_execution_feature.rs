use crate::__steps__::assembly::given_there_is_an_assembly_with_one_successful_test;
use crate::__steps__::standard_output::then_the_standard_output_should_have;
use crate::__steps__::wasm_bindgen_test_runner::when_wasm_bindgen_test_runner_is_invoked_with_the_assembly;
use crate::__steps__::Context;

#[test]
fn outputs_successful_test_execution_feature() {
    let mut context = Context::new();
    given_there_is_an_assembly_with_one_successful_test(&mut context);
    when_wasm_bindgen_test_runner_is_invoked_with_the_assembly(&mut context);
    then_the_standard_output_should_have(context, "running 1 test\n\ntest assembly_with_one_successful_test::pass ... ok\n\ntest result: ok. 1 passed; 0 failed; 0 ignored; 0 filtered out");
}
