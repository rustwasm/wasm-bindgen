use super::super::TestMode;
use assert_cmd::cargo::{CargoError, CommandCargoExt};
use lazy_static::lazy_static;
use std::process::Command;

lazy_static! {
    static ref COMMAND: Result<Command, CargoError> = wasm_bindgen_test_runner_command_get();
}

fn wasm_bindgen_test_runner_command_get() -> Result<Command, CargoError> {
    let result = Command::cargo_bin("wasm-bindgen-test-runner");
    if result.is_ok() {
        return result;
    }

    Command::new("cargo")
        .args(&["build", "--package", "wasm-bindgen-cli"])
        .output()
        .expect("Failed to build wasm-bindgen-cli");

    Command::cargo_bin("wasm-bindgen-test-runner")
}

pub fn wasm_bindgen_test_runner_command(mode: TestMode) -> Command {
    if COMMAND.is_ok() {
        let mut command = Command::cargo_bin("wasm-bindgen-test-runner").unwrap();

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
            TestMode::DedicatedWorkerEdge => {
                command.env("WASM_BINDGEN_USE_DEDICATED_WORKER", "edge")
            }
            TestMode::DedicatedWorkerFirefox => {
                command.env("WASM_BINDGEN_USE_DEDICATED_WORKER", "firefox")
            }
            TestMode::DedicatedWorkerSafari => {
                command.env("WASM_BINDGEN_USE_DEDICATED_WORKER", "safari")
            }
            TestMode::ServiceWorkerDefault => {
                command.env("WASM_BINDGEN_USE_SERVICE_WORKER", "true")
            }
            TestMode::ServiceWorkerChrome => {
                command.env("WASM_BINDGEN_USE_SERVICE_WORKER", "chrome")
            }
            TestMode::ServiceWorkerEdge => command.env("WASM_BINDGEN_USE_SERVICE_WORKER", "edge"),
            TestMode::ServiceWorkerFirefox => {
                command.env("WASM_BINDGEN_USE_SERVICE_WORKER", "firefox")
            }
            TestMode::ServiceWorkerSafari => {
                command.env("WASM_BINDGEN_USE_SERVICE_WORKER", "safari")
            }
            TestMode::SharedWorkerDefault => command.env("WASM_BINDGEN_USE_SHARED_WORKER", "true"),
            TestMode::SharedWorkerChrome => command.env("WASM_BINDGEN_USE_SHARED_WORKER", "chrome"),
            TestMode::SharedWorkerEdge => command.env("WASM_BINDGEN_USE_SHARED_WORKER", "edge"),
            TestMode::SharedWorkerFirefox => {
                command.env("WASM_BINDGEN_USE_SHARED_WORKER", "firefox")
            }
            TestMode::SharedWorkerSafari => command.env("WASM_BINDGEN_USE_SHARED_WORKER", "safari"),
        };

        return command;
    }

    panic!("Failed to find wasm-bindgen-test-runner binary")
}
