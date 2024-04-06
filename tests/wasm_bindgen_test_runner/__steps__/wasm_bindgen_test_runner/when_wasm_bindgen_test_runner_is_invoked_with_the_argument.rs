use crate::__steps__::{wasm_bindgen_test_runner::wasm_bindgen_test_runner_command, Context};

pub fn when_wasm_bindgen_test_runner_is_invoked_with_the_argument(
    context: &mut Context,
    argument: &str,
) {
    let mut command = wasm_bindgen_test_runner_command();

    command.arg(argument);

    context.output_set(command.output());
}
