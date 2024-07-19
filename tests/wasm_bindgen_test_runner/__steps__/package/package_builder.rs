use predicates::str;
use rand::Rng;
use regex::Regex;
use std::env;
use std::fs;
use std::path::PathBuf;
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
            self.file(
                ".cargo/config.toml",
                r#"[build]
target = "wasm32-unknown-unknown"

[target.wasm32-unknown-unknown]
runner = 'wasm-bindgen-test-runner'
"#,
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
