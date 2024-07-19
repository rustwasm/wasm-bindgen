use super::super::wasm_bindgen_test_runner::wasm_bindgen_test_runner_env_set;
use super::super::TestMode;
use std::process::Command;

pub fn cargo_nextest_command(mode: TestMode) -> Command {
    let mut command = Command::new("cargo");

    command.args(&["nextest", "run"]);

    return wasm_bindgen_test_runner_env_set(mode, command);
}
