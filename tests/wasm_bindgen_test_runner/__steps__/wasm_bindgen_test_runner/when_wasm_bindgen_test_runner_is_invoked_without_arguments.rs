use crate::__steps__::Context;
use assert_cmd::prelude::*;
use std::process::Command;

pub fn when_wasm_bindgen_test_runner_is_invoked_without_arguments(context: &mut Context) {
    let aux = Command::cargo_bin("wasm-bindgen-test-runner").unwrap();

    context.command_set(aux);
}
