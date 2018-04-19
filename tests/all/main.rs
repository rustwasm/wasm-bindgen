extern crate wasm_bindgen_cli_support as cli;

use std::env;
use std::fs::{self, File};
use std::io::{self, Write, Read};
use std::path::{PathBuf, Path};
use std::process::Command;
use std::sync::atomic::*;
use std::time::Instant;

static CNT: AtomicUsize = ATOMIC_USIZE_INIT;
thread_local!(static IDX: usize = CNT.fetch_add(1, Ordering::SeqCst));

struct Project {
    files: Vec<(String, String)>,
    debug: bool,
    node: bool,
    no_std: bool,
}

fn project() -> Project {
    let dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let mut lockfile = String::new();
    fs::File::open(&dir.join("Cargo.lock")).unwrap()
        .read_to_string(&mut lockfile).unwrap();
    Project {
        debug: true,
        node: false,
        no_std: false,
        files: vec![
            ("Cargo.toml".to_string(), format!(r#"
                [package]
                name = "test{}"
                version = "0.0.1"
                authors = []

                [workspace]

                [lib]
                crate-type = ["cdylib"]

                # XXX: It is important that `[dependencies]` is the last section
                # here, so that `add_local_dependency` functions correctly!
                [dependencies]
            "#, IDX.with(|x| *x))),

            ("Cargo.lock".to_string(), lockfile),

            ("run.js".to_string(), r#"
                import * as process from "process";

                const test = import("./test");

                test.then(test => {
                  test.test();
                }).catch(error => {
                  console.error(error);
                  process.exit(1);
                });
            "#.to_string()),

            ("run-node.js".to_string(), r#"
                require("./test").test();
            "#.to_string()),

            ("webpack.config.js".to_string(), r#"
                const path = require('path');

                module.exports = {
                  entry: './run.js',
                  mode: "development",
                  devtool: "source-map",
                  module: {
                    rules: [
                      {
                        test: /\.ts$/,
                        use: 'ts-loader',
                        exclude: /node_modules/
                      }
                    ]
                  },
                  resolve: {
                    extensions: [ '.ts', '.js', '.wasm' ]
                  },
                  output: {
                    filename: 'bundle.js',
                    path: path.resolve(__dirname, '.')
                  },
                  target: 'node'
                };
            "#.to_string()),
            ("tsconfig.json".to_string(), r#"
                {
                  "compilerOptions": {
                    "noEmitOnError": true,
                    "noImplicitAny": true,
                    "noImplicitThis": true,
                    "noUnusedParameters": true,
                    "noUnusedLocals": true,
                    "noImplicitReturns": true,
                    "strictFunctionTypes": true,
                    "strictNullChecks": true,
                    "alwaysStrict": true,
                    "strict": true,
                    "target": "es5",
                    "lib": ["es2015"]
                  }
                }
            "#.to_string()),
        ],
    }
}

fn root() -> PathBuf {
    let idx = IDX.with(|x| *x);

    let mut me = env::current_exe().unwrap();
    me.pop(); // chop off exe name
    me.pop(); // chop off `deps`
    me.pop(); // chop off `debug` / `release`
    me.push("generated-tests");
    me.push(&format!("test{}", idx));
    return me
}

impl Project {
    fn file(&mut self, name: &str, contents: &str) -> &mut Project {
        self.files.push((name.to_string(), contents.to_string()));
        self
    }

    fn debug(&mut self, debug: bool) -> &mut Project {
        self.debug = debug;
        self
    }

    fn node(&mut self, node: bool) -> &mut Project {
        self.node = node;
        self
    }

    fn no_std(&mut self, no_std: bool) -> &mut Project {
        self.no_std = no_std;
        self
    }

    fn add_local_dependency(&mut self, name: &str, path: &str) -> &mut Project {
        {
            let cargo_toml = self.files
                .iter_mut()
                .find(|f| f.0 == "Cargo.toml")
                .expect("should have Cargo.toml file!");
            cargo_toml.1.push_str(name);
            cargo_toml.1.push_str(" = { path = '");
            cargo_toml.1.push_str(path);
            cargo_toml.1.push_str("' }\n");
        }
        self
    }

    fn test(&mut self) {
        {
            let cargo_toml = self.files
                .iter_mut()
                .find(|f| f.0 == "Cargo.toml")
                .expect("should have Cargo.toml file!");
            cargo_toml.1.push_str("wasm-bindgen = { path = '");
            cargo_toml.1.push_str(env!("CARGO_MANIFEST_DIR"));
            if self.no_std {
                cargo_toml.1.push_str("', default-features = false");
            } else {
                cargo_toml.1.push_str("'");
            }
            cargo_toml.1.push_str(" }\n");
        }
        let root = root();
        drop(fs::remove_dir_all(&root));
        for &(ref file, ref contents) in self.files.iter() {
            let dst = root.join(file);
            fs::create_dir_all(dst.parent().unwrap()).unwrap();
            fs::File::create(&dst).unwrap().write_all(contents.as_ref()).unwrap();
        }

        let target_dir = root.parent().unwrap() // chop off test name
            .parent().unwrap(); // chop off `generated-tests`

        let mut cmd = Command::new("cargo");
        cmd.arg("build")
            .arg("--target")
            .arg("wasm32-unknown-unknown")
            .current_dir(&root)
            .env("CARGO_TARGET_DIR", &target_dir)
            // Catch any warnings in generated code because we don't want any
            .env("RUSTFLAGS", "-Dwarnings");
        run(&mut cmd, "cargo");

        let idx = IDX.with(|x| *x);
        let out = target_dir.join(&format!("wasm32-unknown-unknown/debug/test{}.wasm", idx));

        let as_a_module = root.join("out.wasm");
        fs::copy(&out, &as_a_module).unwrap();

        cli::Bindgen::new()
            .input_path(&as_a_module)
            .typescript(true)
            .nodejs(self.node)
            .debug(self.debug)
            .generate(&root)
            .expect("failed to run bindgen");

        let mut wasm = Vec::new();
        File::open(root.join("out_bg.wasm")).unwrap()
            .read_to_end(&mut wasm).unwrap();
        let obj = cli::wasm2es6js::Config::new()
            .base64(true)
            .generate(&wasm)
            .expect("failed to convert wasm to js");
        File::create(root.join("out_bg.d.ts")).unwrap()
            .write_all(obj.typescript().as_bytes()).unwrap();


        // move files from the root into each test, it looks like this may be
        // needed for webpack to work well when invoked concurrently.
        fs::hard_link("package.json", root.join("package.json")).unwrap();
        if !Path::new("yarn.lock").exists() {
            panic!("\n\nfailed to find `yarn.lock`, have you run `yarn` yet?\n\n");
        }
        fs::hard_link("yarn.lock", root.join("yarn.lock")).unwrap();
        let cwd = env::current_dir().unwrap();
        symlink_dir(&cwd.join("node_modules"), &root.join("node_modules")).unwrap();

        if self.node {
            let mut cmd = Command::new("node");
            cmd.arg(root.join("run-node.js"))
                .current_dir(&root);
            run(&mut cmd, "node");
        } else {
            let mut cmd = if cfg!(windows) {
                let mut c = Command::new("cmd");
                c.arg("/c");
                c.arg("yarn");
                c
            } else {
                Command::new("yarn")
            };
            cmd.arg("webpack").current_dir(&root);
            run(&mut cmd, "yarn");

            let mut cmd = Command::new("node");
            cmd.arg(root.join("bundle.js"))
                .current_dir(&root);
            run(&mut cmd, "node");
        }
    }
}

#[cfg(unix)]
fn symlink_dir(a: &Path, b: &Path) -> io::Result<()> {
    use std::os::unix::fs::symlink;
    symlink(a, b)
}

#[cfg(windows)]
fn symlink_dir(a: &Path, b: &Path) -> io::Result<()> {
    use std::os::windows::fs::symlink_dir;
    symlink_dir(a, b)
}

fn run(cmd: &mut Command, program: &str) {
    println!("···················································");
    println!("running {:?}", cmd);
    let start = Instant::now();
    let output = match cmd.output() {
        Ok(output) => output,
        Err(err) => panic!("failed to spawn `{}`: {}", program, err),
    };
    println!("exit: {}", output.status);
    let dur = start.elapsed();
    println!("dur: {}.{:03}ms", dur.as_secs(), dur.subsec_nanos() / 1_000_000);
    if output.stdout.len() > 0 {
        println!("stdout ---\n{}", String::from_utf8_lossy(&output.stdout));
    }
    if output.stderr.len() > 0 {
        println!("stderr ---\n{}", String::from_utf8_lossy(&output.stderr));
    }
    assert!(output.status.success());
}

mod api;
mod classes;
mod closures;
mod dependencies;
mod enums;
mod import_class;
mod imports;
mod jsobjects;
mod math;
mod node;
mod non_debug;
mod simple;
mod slice;
mod structural;
