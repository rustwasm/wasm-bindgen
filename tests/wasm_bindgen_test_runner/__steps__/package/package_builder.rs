use crate::__steps__::wasm_bindgen_test_runner::wasm_bindgen_test_runner_command;
use crate::__steps__::TestMode;
use predicates::str;
use rand::Rng;
use std::env;
use std::fs;
use std::path::PathBuf;

fn build_dir() -> PathBuf {
    target_dir()
        .join("wasm32-unknown-unknown")
        .join("debug")
        .join("deps")
}

fn target_dir() -> PathBuf {
    let mut dir = env::current_exe().unwrap();
    dir.pop(); // current exe
    if dir.ends_with("deps") {
        dir.pop();
    }
    dir.pop(); // debug and/or release
    dir
}

fn repo_root() -> PathBuf {
    env::current_dir().unwrap()
}

pub struct PackageBuilder {
    root: PathBuf,
    name: String,
}

impl PackageBuilder {
    pub fn new(name: &'static str) -> Self {
        let mut rng = rand::thread_rng();

        let root = target_dir()
            .join("wasm-bindgen-test-runner-tests")
            .join(format!("{}-{}", name, rng.gen_range(10000..99999)));

        drop(fs::remove_dir_all(&root));
        fs::create_dir_all(&root).unwrap();

        Self {
            root,
            name: name.to_string(),
        }
    }

    pub fn check_cargo(&mut self) {
        if !self.root.join("Cargo.toml").is_file() {
            self.file(
                "Cargo.toml",
                &format!(
                    "[package]
name = '{}'
authors = []
version = '1.0.0'
edition = '2021'

[dev-dependencies]
wasm-bindgen-test = {{ path = '{}/crates/test' }}

[patch.crates-io]
wasm-bindgen = {{ path = '{}' }}

[workspace]
",
                    self.name,
                    repo_root().display(),
                    repo_root().display(),
                ),
            );
        }
    }

    pub fn check_cargo_config(&mut self) {
        if !self.root.join(".cargo/config.toml").is_file() {
            // to make sure that the wasm-bindgen-test-runner is compiled
            wasm_bindgen_test_runner_command(TestMode::Default);
            // gets the current test executable
            let runner = env::current_exe().unwrap();
            // drops the executable name
            let runner = runner.parent().unwrap();
            // drops deps directory
            let runner = runner.parent().unwrap();
            let runner = runner
                .join("wasm-bindgen-test-runner")
                .display()
                .to_string();
            self.file(
                ".cargo/config.toml",
                format!(
                    r#"[build]
target = "wasm32-unknown-unknown"

[target.wasm32-unknown-unknown]
runner = '{}'
"#,
                    runner
                )
                .as_str(),
            );
        }
    }

    pub fn clean(&mut self) {
        drop(fs::remove_dir_all(&self.root));
    }

    pub fn file(&mut self, name: &str, contents: &str) -> &mut Self {
        let dst = self.root.join(name);
        fs::create_dir_all(dst.parent().unwrap()).unwrap();
        fs::write(&dst, contents).unwrap();
        self
    }

    pub fn finalize(&mut self) -> PathBuf {
        self.check_cargo();
        self.check_cargo_config();
        self.root.clone()
    }
}
