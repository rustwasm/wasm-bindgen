use crate::__steps__::{
    wasm_bindgen_test_runner::wasm_bindgen_test_runner_command, Context, TestMode,
};

pub fn when_wasm_bindgen_test_runner_is_invoked_with_the_assembly(context: &mut Context) {
    let mut command = wasm_bindgen_test_runner_command(TestMode::Default);

    command.arg(context.sandbox_mut().assembly());

    context.output_set(command.output());
}
