use std::process;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(main)]
fn main() -> Test {
    unimplemented!()
}

struct Test;

impl process::Termination for Test {
    fn report(self) -> process::ExitCode {
        unimplemented!()
    }
}

#[wasm_bindgen(main)]
fn fail() {}
