use crate::__steps__::{wasm_bindgen_test_runner::wasm_bindgen_test_runner_command, Context};

pub fn when_wasm_bindgen_test_runner_is_invoked_with_the_assembly_targeting_node(context: &mut Context) {
    let mut command = wasm_bindgen_test_runner_command();

    command.env("WASM_BINDGEN_TEST_ONLY_NODE", "true");

    command.arg(context.sandbox_mut().assembly());

    context.output_set(command.output());
}
