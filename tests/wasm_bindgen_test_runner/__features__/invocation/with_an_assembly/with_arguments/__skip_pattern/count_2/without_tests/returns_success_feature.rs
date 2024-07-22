use crate::__steps__::assembly::given_there_is_an_assembly_without_anything;
use crate::__steps__::success::then_success_should_have_been_returned;
use crate::__steps__::wasm_bindgen_test_runner::when_wasm_bindgen_test_runner_is_invoked_with_the_assembly_and_the_arguments;
use crate::__steps__::Context;

#[test]
fn returns_success_feature() {
    let mut context = Context::new();
    given_there_is_an_assembly_without_anything(&mut context);
    when_wasm_bindgen_test_runner_is_invoked_with_the_assembly_and_the_arguments(
        &mut context,
        "--skip pattern1 --skip pattern2",
    );
    then_success_should_have_been_returned(&context);
}
