use crate::__steps__::standard_output::then_the_standard_output_should_have;
use crate::__steps__::success::then_success_should_have_been_returned;
use crate::__steps__::wasm_bindgen_test_runner::when_wasm_bindgen_test_runner_is_invoked_with_the_option;
use crate::__steps__::Context;
use auroka_morpheus_macros_feature::feature;

feature! {
    when_wasm_bindgen_test_runner_is_invoked_with_the_option("--help");

    "Outputs the wasm-bindgen-test-runner help information" {
        then_the_standard_output_should_have(
            r#"Execute all wasm bindgen unit and integration tests and build examples of a local package

Usage: wasm-bindgen-test-runner [OPTIONS] [INPUT] [TESTNAME]

Arguments:
  [INPUT]     The wasm file to test
  [TESTNAME]  If specified, only executes the tests containing [testname] in their names

Options:
      --exact                Run only the test with the exact name
      --include-ignored      Include ignored tests in the test run
      --nocapture            Disables the tests output capture
      --skip [<PATTERN>...]  Skip tests whose names match the given pattern
      --list                 List all tests that would be run
      --format <FORMAT>      Format of the tests listing output [possible values: terse, json]
      --ignored              Restricts the listing to only consider the ignored tests
  -V, --version              Print version
  -h, --help                 Print help

Additional documentation: https://rustwasm.github.io/wasm-bindgen/wasm-bindgen-test/usage.html
"#,
        );
    }

    "Returns success" {
        then_success_should_have_been_returned();
    }
}
