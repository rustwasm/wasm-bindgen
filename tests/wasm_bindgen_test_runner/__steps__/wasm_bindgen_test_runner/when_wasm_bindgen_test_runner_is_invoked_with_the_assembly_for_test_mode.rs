use crate::__steps__::TestMode;
use crate::__steps__::{wasm_bindgen_test_runner::wasm_bindgen_test_runner_command, Context};

pub fn when_wasm_bindgen_test_runner_is_invoked_with_the_assembly_for_test_mode(
    context: &mut Context,
    mode: TestMode,
) {
    let mut command = wasm_bindgen_test_runner_command();

    match mode {
        TestMode::Default => &mut command,
        TestMode::Deno => command.env("WASM_BINDGEN_USE_DENO", "true"),
        TestMode::Node => command.env("WASM_BINDGEN_TEST_ONLY_NODE", "true"),
        TestMode::BrowserDefault => command.env("WASM_BINDGEN_USE_BROWSER", "true"),
        TestMode::BrowserChrome => command.env("WASM_BINDGEN_USE_BROWSER", "chrome"),
        TestMode::BrowserEdge => command.env("WASM_BINDGEN_USE_BROWSER", "edge"),
        TestMode::BrowserFirefox => command.env("WASM_BINDGEN_USE_BROWSER", "firefox"),
        TestMode::BrowserSafari => command.env("WASM_BINDGEN_USE_BROWSER", "safari"),
        TestMode::DedicatedWorkerDefault => command.env("WASM_BINDGEN_USE_DEDICATED_WORKER", "true"),
        TestMode::DedicatedWorkerChrome => command.env("WASM_BINDGEN_USE_DEDICATED_WORKER", "chrome"),
        TestMode::DedicatedWorkerEdge => command.env("WASM_BINDGEN_USE_DEDICATED_WORKER", "edge"),
        TestMode::DedicatedWorkerFirefox => command.env("WASM_BINDGEN_USE_DEDICATED_WORKER", "firefox"),
        TestMode::DedicatedWorkerSafari => command.env("WASM_BINDGEN_USE_DEDICATED_WORKER", "safari"),
    };

    command.arg(context.sandbox_mut().assembly());

    context.output_set(command.output());
}
