use crate::__steps__::assembly::given_there_is_an_assembly_with_one_failing_test;
use crate::__steps__::standard_output::then_the_standard_output_should_have;
use crate::__steps__::wasm_bindgen_test_runner::when_wasm_bindgen_test_runner_is_invoked_with_the_option;
use crate::__steps__::Context;

#[test]
fn outputs_the_wasm_bindgen_test_runner_help_information_feature() {
    let mut context = Context::new();
    given_there_is_an_assembly_with_one_failing_test(&mut context);
    when_wasm_bindgen_test_runner_is_invoked_with_the_option(&mut context, "-h");
    then_the_standard_output_should_have(
        context,
        r#"Execute all wasm bindgen unit and integration tests and build examples of a local package

Usage:
    wasm-bindgen-test-runner [options] <input> [arguments]
    wasm-bindgen-test-runner -h | --help
    wasm-bindgen-test-runner -V | --version

Options:
    -h, --help         Show this screen.
    -V, --version      Print the version number of wasm-bindgen-test-runner

Arguments:
    --include-ignored  Include ignored tests in the test run
    --list             List all tests that would be run
    --format FORMAT    Format of the tests listing output, valid values are [terse, json]
    --ignored          Restricts the listing to only consider the ignored tests

Additional documentation: https://rustwasm.github.io/wasm-bindgen/wasm-bindgen-test/usage.html
"#,
    );
}
