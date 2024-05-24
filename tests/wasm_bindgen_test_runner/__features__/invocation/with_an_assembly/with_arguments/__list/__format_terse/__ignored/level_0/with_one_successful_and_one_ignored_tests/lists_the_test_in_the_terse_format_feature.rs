use crate::__steps__::assembly::given_there_is_an_assembly_with_one_successful_and_one_ignored_tests;
use crate::__steps__::standard_output::then_the_standard_output_should_have;
use crate::__steps__::success::then_success_should_have_been_returned;
use crate::__steps__::wasm_bindgen_test_runner::when_wasm_bindgen_test_runner_is_invoked_with_the_assembly_and_the_arguments;
use crate::__steps__::Context;
use auroka_morpheus_macros_feature::feature;

feature! {
    given_there_is_an_assembly_with_one_successful_and_one_ignored_tests();
    when_wasm_bindgen_test_runner_is_invoked_with_the_assembly_and_the_arguments("--list --format terse --ignored");

    "Outputs the test in the terse format" {
        then_the_standard_output_should_have(r#"ignored: test"#);
    }

    "Returns success" {
        then_success_should_have_been_returned();
    }
}
