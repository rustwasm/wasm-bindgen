use crate::__steps__::assembly::given_there_is_an_assembly_with_one_successful_level_1_test;
use crate::__steps__::standard_error::then_the_standard_error_should_be_empty;
use crate::__steps__::standard_output::then_the_standard_output_should_have;
use crate::__steps__::standard_output::then_the_standard_output_should_not_have;
use crate::__steps__::success::then_success_should_have_been_returned;
use crate::__steps__::wasm_bindgen_test_runner::when_wasm_bindgen_test_runner_is_invoked_with_the_assembly_for_test_mode_and_the_arguments;
use crate::__steps__::Context;
use crate::__steps__::TestMode;
use auroka_morpheus_macros_feature::feature;

feature! {
    test_mode: TestMode

    given_there_is_an_assembly_with_one_successful_level_1_test();
    when_wasm_bindgen_test_runner_is_invoked_with_the_assembly_for_test_mode_and_the_arguments(test_mode, "--skip ss");

    "Outputs no information about the skipped test" {
        then_the_standard_output_should_not_have("test assembly_with_one_successful_test::pass");
    }

    "Outputs its running 1 test" {
        then_the_standard_output_should_have("running 1 test");
    }

    "Outputs the assembly test summary" {
        then_the_standard_output_should_have("test result: ok. 0 passed; 0 failed; 0 ignored; 1 filtered out");
    }

    "Outputs no error" {
        then_the_standard_error_should_be_empty();
    }

    "Returns Success" {
        then_success_should_have_been_returned();
    }
}
