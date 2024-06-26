use crate::__steps__::Runtime;
use crate::__steps__::{wasm_bindgen_test_runner::wasm_bindgen_test_runner_command, Context};

pub fn when_wasm_bindgen_test_runner_is_invoked_with_the_assembly_for_runtime(
    context: &mut Context,
    runtime: Runtime,
) {
    let mut command = wasm_bindgen_test_runner_command();

    match runtime {
        Runtime::Chrome => command.env("WASM_BINDGEN_USE_BROWSER", "chrome"),
        Runtime::Deno => command.env("WASM_BINDGEN_USE_DENO", "true"),
        Runtime::Edge => command.env("WASM_BINDGEN_USE_BROWSER", "edge"),
        Runtime::Node => command.env("WASM_BINDGEN_TEST_ONLY_NODE", "true"),
        Runtime::Firefox => command.env("WASM_BINDGEN_USE_BROWSER", "firefox"),
        Runtime::Safari => command.env("WASM_BINDGEN_USE_BROWSER", "safari"),
        Runtime::Default => &mut command,
    };

    command.arg(context.sandbox_mut().assembly());

    context.output_set(command.output());
}
