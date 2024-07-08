use predicates::str;
use rand::Rng;
use regex::Regex;
use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::process::Output;

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

pub struct AssemblyBuilder {
    root: PathBuf,
    name: String,
}

impl AssemblyBuilder {
    pub fn new(name: &'static str) -> AssemblyBuilder {
        let mut rng = rand::thread_rng();

        let root = target_dir()
            .join("wasm-bindgen-test-runner-tests")
            .join(format!("{}-{}", name, rng.gen_range(1000..9999)));

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

        remove_files_prefix(&build_dir(), &format!("{}-", self.name));

        let output = Command::new("cargo")
            .current_dir(&self.root)
            .arg("test")
            .arg("--target")
            .arg("wasm32-unknown-unknown")
            .arg("--no-run")
            .env("CARGO_TARGET_DIR", target_dir())
            .output()
            .expect("Failed to build test assembly");

        let assembly = PathBuf::from(extract_assembly_from_output(output));

        let destination = self.root.join(format!("{}.wasm", self.name));

        fs::copy(&assembly, &destination).unwrap();

        fs::remove_file(&assembly).unwrap();

        destination
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

fn remove_files_prefix(dir: &PathBuf, prefix: &str) {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_file()
                && path
                    .file_name()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .starts_with(prefix)
            {
                fs::remove_file(&path).unwrap();
            }
        }
    }
}
