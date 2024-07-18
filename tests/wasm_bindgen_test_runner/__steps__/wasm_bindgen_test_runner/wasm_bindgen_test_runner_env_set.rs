use super::super::TestMode;
use std::process::Command;

pub fn wasm_bindgen_test_runner_env_set(mode: TestMode, mut command: Command) -> Command {
    match mode {
        TestMode::Default => &mut command,
        TestMode::Deno => command.env("WASM_BINDGEN_USE_DENO", "true"),
        TestMode::Node => command.env("WASM_BINDGEN_TEST_ONLY_NODE", "true"),
        TestMode::BrowserDefault => command.env("WASM_BINDGEN_USE_BROWSER", "true"),
        TestMode::BrowserChrome => command.env("WASM_BINDGEN_USE_BROWSER", "chrome"),
        TestMode::BrowserEdge => command.env("WASM_BINDGEN_USE_BROWSER", "edge"),
        TestMode::BrowserFirefox => command.env("WASM_BINDGEN_USE_BROWSER", "firefox"),
        TestMode::BrowserSafari => command.env("WASM_BINDGEN_USE_BROWSER", "safari"),
        TestMode::DedicatedWorkerDefault => {
            command.env("WASM_BINDGEN_USE_DEDICATED_WORKER", "true")
        }
        TestMode::DedicatedWorkerChrome => {
            command.env("WASM_BINDGEN_USE_DEDICATED_WORKER", "chrome")
        }
        TestMode::DedicatedWorkerEdge => command.env("WASM_BINDGEN_USE_DEDICATED_WORKER", "edge"),
        TestMode::DedicatedWorkerFirefox => {
            command.env("WASM_BINDGEN_USE_DEDICATED_WORKER", "firefox")
        }
        TestMode::DedicatedWorkerSafari => {
            command.env("WASM_BINDGEN_USE_DEDICATED_WORKER", "safari")
        }
        TestMode::ServiceWorkerDefault => command.env("WASM_BINDGEN_USE_SERVICE_WORKER", "true"),
        TestMode::ServiceWorkerChrome => command.env("WASM_BINDGEN_USE_SERVICE_WORKER", "chrome"),
        TestMode::ServiceWorkerEdge => command.env("WASM_BINDGEN_USE_SERVICE_WORKER", "edge"),
        TestMode::ServiceWorkerFirefox => command.env("WASM_BINDGEN_USE_SERVICE_WORKER", "firefox"),
        TestMode::ServiceWorkerSafari => command.env("WASM_BINDGEN_USE_SERVICE_WORKER", "safari"),
        TestMode::SharedWorkerDefault => command.env("WASM_BINDGEN_USE_SHARED_WORKER", "true"),
        TestMode::SharedWorkerChrome => command.env("WASM_BINDGEN_USE_SHARED_WORKER", "chrome"),
        TestMode::SharedWorkerEdge => command.env("WASM_BINDGEN_USE_SHARED_WORKER", "edge"),
        TestMode::SharedWorkerFirefox => command.env("WASM_BINDGEN_USE_SHARED_WORKER", "firefox"),
        TestMode::SharedWorkerSafari => command.env("WASM_BINDGEN_USE_SHARED_WORKER", "safari"),
    };

    command
}
