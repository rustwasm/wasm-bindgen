use crate::__steps__::assembly::given_there_is_an_assembly_with;
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

    given_there_is_an_assembly_with(r#"
mod level_1 {
    use wasm_bindgen_test::*;

    #[wasm_bindgen_test]
    fn pass_1() {
        console_log!("pass_1 standard output");
        assert_eq!(1, 1);
    }

    #[wasm_bindgen_test]
    fn pass_2() {
        console_log!("pass_2 standard output");
        assert_eq!(1, 1);
    }
}
"#);
    when_wasm_bindgen_test_runner_is_invoked_with_the_assembly_for_test_mode_and_the_arguments(test_mode, "--skip pass_1 --skip level_1");

    "Outputs no information about the skipped test 1" {
        then_the_standard_output_should_not_have("pass_1");
    }

    "Outputs no information about the skipped test 2" {
        then_the_standard_output_should_not_have("pass_2");
    }

    "Outputs its running 2 tests" {
        then_the_standard_output_should_have("running 2 tests");
    }

    "Outputs the assembly test summary" {
        then_the_standard_output_should_have("test result: ok. 0 passed; 0 failed; 0 ignored; 2 filtered out");
    }

    "Outputs no error" {
        then_the_standard_error_should_be_empty();
    }

    "Returns Success" {
        then_success_should_have_been_returned();
    }
}
