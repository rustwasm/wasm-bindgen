use crate::__steps__::assembly::given_there_is_an_assembly_with_one_failing_test;
use crate::__steps__::standard_output::then_the_standard_output_should_have;
use crate::__steps__::success::then_success_should_have_been_returned;
use crate::__steps__::wasm_bindgen_test_runner::when_wasm_bindgen_test_runner_is_invoked_with_the_option;
use crate::__steps__::Context;
use auroka_morpheus_macros_feature::feature;

feature! {
    given_there_is_an_assembly_with_one_failing_test();
    when_wasm_bindgen_test_runner_is_invoked_with_the_option("--help");

    "Outputs the wasm-bindgen-test-runner help information" {
        then_the_standard_output_should_have(
            r#"Execute all wasm bindgen unit and integration tests and build examples of a local package

Usage:
    wasm-bindgen-test-runner [options] <input> [<testname>] [--include-ignored] [(--skip PATTERN)...] [--nocapture]
    wasm-bindgen-test-runner [options] <input> <testname> [--nocapture] --exact
    wasm-bindgen-test-runner [options] <input> --list [--format FORMAT] [--ignored]
    wasm-bindgen-test-runner -h | --help
    wasm-bindgen-test-runner -V | --version

Options:
    -h, --help         Show this screen.
    -V, --version      Print the version number of wasm-bindgen-test-runner

    <input>            The wasm file to test
    <testname>         If specified, only executes the tests containing <testname> in their names

Arguments:
    --include-ignored  Include ignored tests in the test run
    --skip PATTERN     Skip tests whose names match the given pattern
    --nocapture        Disables the tests output capture
    --exact            Run only the test with the exact name

    --list             List all tests that would be run
    --format FORMAT    Format of the tests listing output, valid values are [terse, json]
    --ignored          Restricts the listing to only consider the ignored tests

Additional documentation: https://rustwasm.github.io/wasm-bindgen/wasm-bindgen-test/usage.html
"#,
        );
    }

    "Returns success" {
        then_success_should_have_been_returned();
    }
}
