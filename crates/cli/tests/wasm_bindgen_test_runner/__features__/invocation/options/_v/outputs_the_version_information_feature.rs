use crate::__steps__::standard_output::then_the_standard_output_should_have;
use crate::__steps__::success::then_success_should_have_been_returned;
use crate::__steps__::wasm_bindgen_test_runner::when_wasm_bindgen_test_runner_is_invoked_with_the_option;
use crate::__steps__::Context;
use auroka_morpheus_macros_feature::feature;

feature! {
    when_wasm_bindgen_test_runner_is_invoked_with_the_option("-V");

    "Outputs the wasm-bindgen-test-runner version information" {
        then_the_standard_output_should_have(
            &format!("wasm-bindgen-test-runner {}", env!("CARGO_PKG_VERSION")),
        );
    }

    "Returns success" {
        then_success_should_have_been_returned();
    }
}
