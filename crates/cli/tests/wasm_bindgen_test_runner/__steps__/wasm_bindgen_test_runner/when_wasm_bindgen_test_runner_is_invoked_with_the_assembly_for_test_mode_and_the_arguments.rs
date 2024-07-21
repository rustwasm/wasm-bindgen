use crate::__steps__::TestMode;
use crate::__steps__::{wasm_bindgen_test_runner::wasm_bindgen_test_runner_command, Context};

pub fn when_wasm_bindgen_test_runner_is_invoked_with_the_assembly_for_test_mode_and_the_arguments(
    context: &mut Context,
    mode: TestMode,
    arguments: &str,
) {
    let mut command = wasm_bindgen_test_runner_command(mode);

    if arguments.starts_with("--list") && arguments.contains("--ignored") {
        command.arg(context.sandbox().original());
    } else {
        command.arg(context.sandbox_mut().assembly());
    }

    command.args(arguments.split_whitespace());

    context.output_set(command.output());
}
