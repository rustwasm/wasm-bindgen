use crate::__steps__::assembly::given_there_is_an_assembly_with;
use crate::__steps__::error_code::then_failure_should_have_been_returned;
use crate::__steps__::standard_output::then_the_standard_output_should_have;
use crate::__steps__::wasm_bindgen_test_runner::when_wasm_bindgen_test_runner_is_invoked_with_the_assembly_for_test_mode_and_the_arguments;
use crate::__steps__::Context;
use crate::__steps__::TestMode;
use auroka_morpheus_macros_feature::feature;

feature! {
    test_mode: TestMode

    given_there_is_an_assembly_with(r#"
#[wasm_bindgen_test]
fn fail() {
    assert_eq!(1, 2);
}
"#);
    when_wasm_bindgen_test_runner_is_invoked_with_the_assembly_for_test_mode_and_the_arguments(test_mode, "--include-ignored");

    "Outputs its running 1 test" {
        then_the_standard_output_should_have("running 1 test");
    }

    "Outputs the failed test summary" {
        then_the_standard_output_should_have("test assembly::fail ... FAIL");
    }

    "Outputs the failed test assertion error" {
        then_the_standard_output_should_have("assertion `left == right` failed\n  left: 1\n right: 2");
    }

    "Outputs the assembly failure summary" {
        then_the_standard_output_should_have("failures:\n\n    assembly::fail");
    }

    "Outputs the assembly test summary" {
        then_the_standard_output_should_have("test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 filtered out");
    }

    "Returns failure" {
        then_failure_should_have_been_returned();
    }
}
