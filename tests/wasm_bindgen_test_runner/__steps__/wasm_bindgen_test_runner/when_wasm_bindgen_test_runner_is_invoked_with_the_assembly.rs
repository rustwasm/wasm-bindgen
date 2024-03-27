use crate::__steps__::{Context, wasm_bindgen_test_runner::wasm_bindgen_test_runner_command};

pub fn when_wasm_bindgen_test_runner_is_invoked_with_the_assembly(context: &mut Context) {
    let mut aux = wasm_bindgen_test_runner_command();

    if let Some(ref assembly) = context.assembly() {
        aux.arg(assembly);
    }

    context.command_set(aux);
}
