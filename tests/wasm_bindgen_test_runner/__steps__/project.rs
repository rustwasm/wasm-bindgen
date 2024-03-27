use assert_cmd::prelude::*;
use predicates::str;
use rand::Rng;
use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

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

pub struct Project {
    root: PathBuf,
    name: String,
}

impl Project {
    pub fn new(name: &'static str) -> Project {
        let mut rng = rand::thread_rng();
        let name = format!("{}_{}", name, rng.gen_range(1000..9999));
        let root = target_dir()
            .join("wasm-bindgen-test-runner-tests")
            .join(&name);
        println!("root: {:?}", root);
        drop(fs::remove_dir_all(&root));
        fs::create_dir_all(&root).unwrap();
        Project { root, name }
    }

    pub fn file(&mut self, name: &str, contents: &str) -> &mut Project {
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

    pub fn build(&mut self) -> PathBuf {
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
            .join(&self.name)
            .with_extension("wasm")
    }
}
