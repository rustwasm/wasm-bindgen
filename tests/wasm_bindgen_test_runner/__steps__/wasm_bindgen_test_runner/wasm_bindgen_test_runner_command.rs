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

    println!("YO");

    Command::new("cargo")
        .args(&["build", "--package", "wasm-bindgen-cli"])
        .output()
        .expect("Failed to build wasm-bindgen-cli");

    Command::cargo_bin("wasm-bindgen-test-runner")
}

pub fn wasm_bindgen_test_runner_command() -> Command {
    if COMMAND.is_ok() {
        return Command::cargo_bin("wasm-bindgen-test-runner").unwrap();
    }
    panic!("Failed to find wasm-bindgen-test-runner binary")
}
