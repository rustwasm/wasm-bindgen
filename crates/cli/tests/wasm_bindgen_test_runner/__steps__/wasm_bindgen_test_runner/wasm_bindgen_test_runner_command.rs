use super::super::TestMode;
use super::wasm_bindgen_test_runner_env_set;
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
        let command = Command::cargo_bin("wasm-bindgen-test-runner").unwrap();

        return wasm_bindgen_test_runner_env_set(mode, command);
    }

    panic!("Failed to find wasm-bindgen-test-runner binary")
}
