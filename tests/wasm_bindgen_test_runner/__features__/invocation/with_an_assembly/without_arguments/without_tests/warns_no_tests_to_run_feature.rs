use crate::__steps__::assembly::given_there_is_an_assembly_without_anything;
use crate::__steps__::standard_error::then_the_standard_error_should_be_empty;
use crate::__steps__::standard_output::then_the_standard_output_should_have;
use crate::__steps__::success::then_success_should_have_been_returned;
use crate::__steps__::wasm_bindgen_test_runner::when_wasm_bindgen_test_runner_is_invoked_with_the_assembly;
use crate::__steps__::Context;
use auroka_morpheus_macros_feature::feature;

feature! {
    given_there_is_an_assembly_without_anything();
    when_wasm_bindgen_test_runner_is_invoked_with_the_assembly();

    "Outputs no tests to run warning" {
        then_the_standard_output_should_have("no tests to run!");
    }

    "Outputs no error" {
        then_the_standard_error_should_be_empty();
    }

    "Returns an error code" {
        then_success_should_have_been_returned();
    }
}
