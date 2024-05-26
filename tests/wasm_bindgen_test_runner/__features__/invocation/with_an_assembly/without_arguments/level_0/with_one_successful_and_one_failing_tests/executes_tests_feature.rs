use crate::__steps__::assembly::given_there_is_an_assembly_with_one_successful_and_one_failing_tests;
use crate::__steps__::error_code::then_failure_should_have_been_returned;
use crate::__steps__::standard_error::then_the_standard_error_should_have;
use crate::__steps__::standard_output::then_the_standard_output_should_have;
use crate::__steps__::wasm_bindgen_test_runner::when_wasm_bindgen_test_runner_is_invoked_with_the_assembly;
use crate::__steps__::Context;
use auroka_morpheus_macros_feature::feature;

feature! {
    given_there_is_an_assembly_with_one_successful_and_one_failing_tests();
    when_wasm_bindgen_test_runner_is_invoked_with_the_assembly();

    "Outputs its running 2 tests" {
        then_the_standard_output_should_have("running 2 tests");
    }

    "Outputs the failed test summary" {
        then_the_standard_output_should_have("test assembly_with_one_successful_and_one_failing_tests::fail ... FAIL");
    }

    "Outputs the successful test summary" {
        then_the_standard_output_should_have("test assembly_with_one_successful_and_one_failing_tests::pass ... ok");
    }

    "Outputs the failed test assertion error" {
        then_the_standard_error_should_have("assertion `left == right` failed\n  left: 1\n right: 2");
    }

    "Outpus the assembly failure summary" {
        then_the_standard_output_should_have("failures:\n\n    assembly_with_one_successful_and_one_failing_tests::fail\n");
    }

    "Outputs the assembly test summary" {
        then_the_standard_output_should_have("test result: FAILED. 1 passed; 1 failed; 0 ignored; 0 filtered out");
    }

    "Returns failure" {
        then_failure_should_have_been_returned();
    }
}
