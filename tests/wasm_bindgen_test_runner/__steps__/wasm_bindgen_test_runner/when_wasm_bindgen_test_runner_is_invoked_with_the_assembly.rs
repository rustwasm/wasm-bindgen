use crate::__steps__::Context;
use assert_cmd::prelude::*;
use std::process::Command;

pub fn when_wasm_bindgen_test_runner_is_invoked_with_the_assembly(context: &mut Context) {
    let mut aux = Command::cargo_bin("wasm-bindgen-test-runner").unwrap();

    if let Some(ref assembly) = context.assembly() {
        aux.arg(assembly);
    }

    context.command_set(aux);
}
