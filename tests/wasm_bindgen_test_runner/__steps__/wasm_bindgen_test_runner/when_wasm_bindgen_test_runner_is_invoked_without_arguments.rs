use crate::__steps__::{wasm_bindgen_test_runner::wasm_bindgen_test_runner_command, Context};

pub fn when_wasm_bindgen_test_runner_is_invoked_without_arguments(context: &mut Context) {
    let mut command = wasm_bindgen_test_runner_command();

    context.output_set(command.output());
}
