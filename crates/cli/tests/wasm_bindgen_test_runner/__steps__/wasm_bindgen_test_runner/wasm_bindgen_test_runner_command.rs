use super::super::TestMode;
use super::wasm_bindgen_test_runner_env_set;
use assert_cmd::cargo::CommandCargoExt;
use std::process::Command;

pub fn wasm_bindgen_test_runner_command(mode: TestMode) -> Command {
    let command = Command::cargo_bin("wasm-bindgen-test-runner").unwrap();

    return wasm_bindgen_test_runner_env_set(mode, command);
}
