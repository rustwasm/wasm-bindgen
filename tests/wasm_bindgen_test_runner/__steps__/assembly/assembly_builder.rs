use predicates::str;
use regex::Regex;
use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::process::Output;

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

pub struct AssemblyBuilder {
    root: PathBuf,
    name: String,
}

impl AssemblyBuilder {
    pub fn new(name: &'static str) -> AssemblyBuilder {
        let root = target_dir()
            .join("wasm-bindgen-test-runner-tests")
            .join(name);
        drop(fs::remove_dir_all(&root));
        fs::create_dir_all(&root).unwrap();
        AssemblyBuilder {
            root,
            name: name.to_string(),
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

    pub fn build(&mut self) -> PathBuf {
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

        let output = Command::new("cargo")
            .current_dir(&self.root)
            .arg("test")
            .arg("--target")
            .arg("wasm32-unknown-unknown")
            .arg("--no-run")
            .env("CARGO_TARGET_DIR", target_dir())
            .output()
            .expect("Failed to build test assembly");

        let assembly = extract_assembly_from_output(output);

        self.root.join(assembly)
    }
}

fn extract_assembly_from_output(output: Output) -> String {
    let error_str = std::str::from_utf8(&output.stderr).unwrap();
    let last = error_str.lines().last().unwrap();

    if last.starts_with("error") {
        panic!("Failed to generate assembly\n{}", error_str);
    }

    let re = Regex::new(r"\((.*?)\)").unwrap();
    let captures = re
        .captures(last)
        .expect(&format!("Failed to generate assembly\n{}", error_str));

    captures.get(1).unwrap().as_str().to_string()
}
