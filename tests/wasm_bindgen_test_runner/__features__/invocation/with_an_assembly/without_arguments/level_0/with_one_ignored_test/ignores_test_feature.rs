use crate::__steps__::assembly::given_there_is_an_assembly_with;
use crate::__steps__::standard_error::then_the_standard_error_should_be_empty;
use crate::__steps__::standard_output::then_the_standard_output_should_have;
use crate::__steps__::success::then_success_should_have_been_returned;
use crate::__steps__::wasm_bindgen_test_runner::when_wasm_bindgen_test_runner_is_invoked_with_the_assembly_for_test_mode;
use crate::__steps__::Context;
use crate::__steps__::TestMode;
use auroka_morpheus_macros_feature::feature;

feature! {
    test_mode: TestMode

    given_there_is_an_assembly_with(r#"
#[wasm_bindgen_test]
#[ignore]
fn ignored() {
    assert_eq!(1, 1);
}
"#
);
    when_wasm_bindgen_test_runner_is_invoked_with_the_assembly_for_test_mode(test_mode);

    "Outputs its running 1 test" {
        then_the_standard_output_should_have("running 1 test");
    }

    "Outputs the ignored test summary" {
        then_the_standard_output_should_have("test assembly::ignored ... ignored");
    }

    "Outputs no error" {
        then_the_standard_error_should_be_empty();
    }

    "Outputs the assembly test summary" {
        then_the_standard_output_should_have("test result: ok. 0 passed; 0 failed; 1 ignored; 0 filtered out");
    }

    "Returns success" {
        then_success_should_have_been_returned();
    }
}
