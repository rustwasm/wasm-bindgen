use super::wasm_bindgen_test_runner::Sandbox;
use std::process::Output;

pub struct Context {
    output: Option<Result<Output, std::io::Error>>,
    sandbox: Option<Sandbox>,
}

impl Context {
    pub fn new() -> Self {
        Context {
            output: None,
            sandbox: None,
        }
    }

    pub fn output(&self) -> Result<Output, &std::io::Error> {
        Ok(self.output.as_ref().unwrap().as_ref()?.clone())
    }

    pub fn output_set(&mut self, output: Result<Output, std::io::Error>) {
        self.output = Some(output);
    }

    pub fn sandbox(&self) -> &Sandbox {
        self.sandbox.as_ref().unwrap()
    }

    pub fn sandbox_mut(&mut self) -> &mut Sandbox {
        self.sandbox.as_mut().unwrap()
    }

    pub fn sandbox_set(&mut self, sandbox: Sandbox) {
        self.sandbox = Some(sandbox);
    }
}
