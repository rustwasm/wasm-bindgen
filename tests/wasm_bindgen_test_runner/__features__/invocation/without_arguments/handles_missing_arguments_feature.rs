use crate::__steps__::error_code::then_failure_should_have_been_returned;
use crate::__steps__::standard_error::then_the_standard_error_should_have;
use crate::__steps__::wasm_bindgen_test_runner::when_wasm_bindgen_test_runner_is_invoked_without_arguments;
use crate::__steps__::Context;
use auroka_morpheus_macros_feature::feature;

feature! {
    when_wasm_bindgen_test_runner_is_invoked_without_arguments();

    "Outputs test file missing error" {
        then_the_standard_error_should_have("Error: must have a file to test as first argument");
    }

    "Returns an error code" {
        then_failure_should_have_been_returned();
    }
}
