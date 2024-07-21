use crate::__steps__::TestMode;
use crate::__steps__::{wasm_bindgen_test_runner::wasm_bindgen_test_runner_command, Context};

pub fn when_wasm_bindgen_test_runner_is_invoked_with_the_assembly_for_test_mode(
    context: &mut Context,
    mode: TestMode,
) {
    let mut command = wasm_bindgen_test_runner_command(mode);

    command.arg(context.sandbox_mut().assembly());

    context.output_set(command.output());
}
