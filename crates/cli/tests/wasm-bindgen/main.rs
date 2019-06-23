//! A small test suite for the `wasm-bindgen` CLI command itself
//!
//! This test suite is intended to exercise functionality of the CLI in terms of
//! errors and such. It is not intended for comprehensive behavior testing, as
//! that should all be placed in the top-level `tests` directory for the
//! `wasm-bindgen` crate itself.
//!
//! Assertions about errors in `wasm-bindgen` or assertions about the output of
//! `wasm-bindgen` should all be placed in this test suite, however. Currently
//! it is largely based off actually running `cargo build` at test time which is
//! quite expensive, so it's recommended that this test suite doesn't become too
//! large!

use assert_cmd::prelude::*;
use predicates::str;
use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

fn target_dir() -> PathBuf {
    let mut dir = PathBuf::from(env::current_exe().unwrap());
    dir.pop(); // current exe
    if dir.ends_with("deps") {
        dir.pop();
    }
    dir.pop(); // debug and/or release
    return dir;
}

fn repo_root() -> PathBuf {
    let mut repo_root = env::current_dir().unwrap();
    repo_root.pop(); // remove 'cli'
    repo_root.pop(); // remove 'crates'
    repo_root
}

struct Project {
    root: PathBuf,
    name: &'static str,
}

impl Project {
    fn new(name: &'static str) -> Project {
        let root = target_dir().join("cli-tests").join(name);
        drop(fs::remove_dir_all(&root));
        fs::create_dir_all(&root).unwrap();
        Project { root, name }
    }

    fn file(&mut self, name: &str, contents: &str) -> &mut Project {
        let dst = self.root.join(name);
        fs::create_dir_all(dst.parent().unwrap()).unwrap();
        fs::write(&dst, contents).unwrap();
        self
    }

    fn wasm_bindgen(&mut self, args: &str) -> (Command, PathBuf) {
        let wasm = self.build();
        let output = self.root.join("pkg");
        fs::create_dir_all(&output).unwrap();
        let mut cmd = Command::cargo_bin("wasm-bindgen").unwrap();
        cmd.arg("--out-dir").arg(&output);
        cmd.arg(&wasm);
        for arg in args.split_whitespace() {
            cmd.arg(arg);
        }
        (cmd, output)
    }

    fn build(&mut self) -> PathBuf {
        if !self.root.join("Cargo.toml").is_file() {
            self.file(
                "Cargo.toml",
                &format!(
                    "
                        [package]
                        name = \"{}\"
                        authors = []
                        version = \"1.0.0\"
                        edition = '2018'

                        [dependencies]
                        wasm-bindgen = {{ path = '{}' }}

                        [lib]
                        crate-type = ['cdylib']

                        [workspace]
                    ",
                    self.name,
                    repo_root().display(),
                ),
            );
        }

        let target_dir = target_dir();
        Command::new("cargo")
            .current_dir(&self.root)
            .arg("build")
            .arg("--target")
            .arg("wasm32-unknown-unknown")
            .env("CARGO_TARGET_DIR", &target_dir)
            .assert()
            .success();

        target_dir
            .join("wasm32-unknown-unknown")
            .join("debug")
            .join(self.name)
            .with_extension("wasm")
    }
}

#[test]
fn version_useful() {
    Command::cargo_bin("wasm-bindgen")
        .unwrap()
        .arg("-V")
        .assert()
        .stdout(str::ends_with("\n"))
        .stdout(str::starts_with("wasm-bindgen "))
        .success();
}

#[test]
fn works_on_empty_project() {
    let (mut cmd, _out_dir) = Project::new("works_on_empty_project")
        .file(
            "src/lib.rs",
            r#"
            "#,
        )
        .wasm_bindgen("");
    cmd.assert().success();
}

mod npm;

#[test]
fn one_export_works() {
    let (mut cmd, _out_dir) = Project::new("one_export_works")
        .file(
            "src/lib.rs",
            r#"
                use wasm_bindgen::prelude::*;
                #[wasm_bindgen]
                pub fn foo() {}
            "#,
        )
        .wasm_bindgen("");
    cmd.assert().success();
}
