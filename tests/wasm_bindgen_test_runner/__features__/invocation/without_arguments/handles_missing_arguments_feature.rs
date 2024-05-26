use crate::__steps__::assembly::given_there_is_an_assembly_with_one_failing_test;
use crate::__steps__::error_code::then_failure_should_have_been_returned;
use crate::__steps__::standard_error::then_the_standard_error_should_have;
use crate::__steps__::wasm_bindgen_test_runner::when_wasm_bindgen_test_runner_is_invoked_without_arguments;
use crate::__steps__::Context;
use auroka_morpheus_macros_feature::feature;

feature! {
    given_there_is_an_assembly_with_one_failing_test();
    when_wasm_bindgen_test_runner_is_invoked_without_arguments();

    "Outputs the wasm-bindgen-test-runner usage information" {
        then_the_standard_error_should_have(r#"
Usage:
    wasm-bindgen-test-runner [options] <input> [<testname>] [--include-ignored] [(--skip PATTERN)...] [--nocapture]
    wasm-bindgen-test-runner [options] <input> <testname> [--nocapture] --exact
    wasm-bindgen-test-runner [options] <input> --list [--format FORMAT] [--ignored]
    wasm-bindgen-test-runner -h | --help
    wasm-bindgen-test-runner -V | --version"#,
      );
    }

    "Outputs test file missing error" {
        then_the_standard_error_should_have("Invalid arguments.");
    }

    "Returns an error code" {
        then_failure_should_have_been_returned();
    }
}
