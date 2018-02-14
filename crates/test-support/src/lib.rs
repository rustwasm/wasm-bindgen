extern crate wasm_bindgen_cli_support as cli;

use std::env;
use std::fs::{self, File};
use std::io::{Write, Read};
use std::path::{PathBuf, Path};
use std::process::Command;
use std::sync::atomic::*;
use std::sync::{Once, ONCE_INIT};
use std::time::Instant;

static CNT: AtomicUsize = ATOMIC_USIZE_INIT;
thread_local!(static IDX: usize = CNT.fetch_add(1, Ordering::SeqCst));

pub struct Project {
    files: Vec<(String, String)>,
    debug: bool,
    js: bool,
}

pub fn project() -> Project {
    let dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let dir = dir.parent().unwrap() // chop off `test-support`
        .parent().unwrap(); // chop off `crates`

    let mut lockfile = String::new();
    fs::File::open(&dir.join("Cargo.lock")).unwrap()
        .read_to_string(&mut lockfile).unwrap();
    Project {
        debug: true,
        js: false,
        files: vec![
            ("Cargo.toml".to_string(), format!(r#"
                [package]
                name = "test{}"
                version = "0.0.1"
                authors = []

                [workspace]

                [lib]
                crate-type = ["cdylib"]

                [dependencies]
                wasm-bindgen = {{ path = '{}' }}

                [profile.dev]
                opt-level = 2 # TODO: decrease when upstream is not buggy
                incremental = false
            "#, IDX.with(|x| *x), dir.display())),

            ("Cargo.lock".to_string(), lockfile),

            ("run.ts".to_string(), r#"
                import * as process from "process";

                import * as out from "./out_wasm";
                import * as test from "./test";

                out.booted.then(() => {
                  test.test();
                  if ((out as any).assertHeapAndStackEmpty)
                    (out as any).assertHeapAndStackEmpty();
                }).catch(error => {
                  console.error(error);
                  process.exit(1);
                });
            "#.to_string()),

            ("rollup.config.js".to_string(), r#"
                import typescript from 'rollup-plugin-typescript2';

                export default {
                    input: './run.ts',

                    plugins: [
                        typescript()
                    ],
                    output: {
                        file: 'bundle.js',
                        format: 'cjs'
                    }
                }
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
                        "target": "es5"
					}
				}
			"#.to_string()),
        ],
    }
}

pub fn root() -> PathBuf {
    let idx = IDX.with(|x| *x);

    let mut me = env::current_exe().unwrap();
    me.pop(); // chop off exe name
    me.pop(); // chop off `deps`
    me.pop(); // chop off `debug` / `release`
    me.push("generated-tests");
    me.push(&format!("test{}", idx));
    return me
}

fn rollup() -> PathBuf {
    static INIT: Once = ONCE_INIT;

    let mut me = env::current_exe().unwrap();
    me.pop(); // chop off exe name
    me.pop(); // chop off `deps`
    me.pop(); // chop off `debug` / `release`
    let install_dir = me.clone();
    me.push("node_modules/rollup/bin/rollup");

    INIT.call_once(|| {
        if !me.exists() {
            let mut npm = if cfg!(windows) {
                let mut n = Command::new("cmd");
                n.arg("/c").arg("npm");
                n
            } else {
                Command::new("npm")
            };
            run(npm
                .arg("install")
                .arg("rollup")
                .arg("rollup-plugin-typescript2")
                .arg("typescript")
                .arg("@types/node")
                //.arg("@types/webassembly-js-api")
                .current_dir(&install_dir), "npm");
            assert!(me.exists());
        }
    });

    return me
}

impl Project {
    pub fn file(&mut self, name: &str, contents: &str) -> &mut Project {
        self.files.push((name.to_string(), contents.to_string()));
        self
    }

    pub fn debug(&mut self, debug: bool) -> &mut Project {
        self.debug = debug;
        self
    }

    pub fn js(&mut self, js: bool) -> &mut Project {
        self.js = js;
        self
    }

    pub fn test(&mut self) {
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
            .env("CARGO_TARGET_DIR", &target_dir);
        run(&mut cmd, "cargo");

        let idx = IDX.with(|x| *x);
        let mut out = target_dir.join(&format!("wasm32-unknown-unknown/debug/test{}.wasm", idx));
        if Command::new("wasm-gc").output().is_ok() {
            let tmp = out;
            out = tmp.with_extension("gc.wasm");
            let mut cmd = Command::new("wasm-gc");
            cmd.arg(&tmp).arg(&out);
            run(&mut cmd, "wasm-gc");
        }

        let as_a_module = root.join("out.wasm");
        fs::copy(&out, &as_a_module).unwrap();

        cli::Bindgen::new()
            .input_path(&as_a_module)
            .nodejs(true)
            .typescript(true)
            .debug(self.debug)
            .generate(&root)
            .expect("failed to run bindgen");

        let mut wasm = Vec::new();
        File::open(root.join("out_wasm.wasm")).unwrap()
            .read_to_end(&mut wasm).unwrap();
        let obj = cli::wasm2es6js::Config::new()
            .base64(true)
            .generate(&wasm)
            .expect("failed to convert wasm to js");
        File::create(root.join("out_wasm.d.ts")).unwrap()
            .write_all(obj.typescript().as_bytes()).unwrap();
        File::create(root.join("out_wasm.js")).unwrap()
            .write_all(obj.js().as_bytes()).unwrap();

        let mut cmd = Command::new("node");
        cmd.arg(rollup())
            .current_dir(&root)
            .arg("-c");
        run(&mut cmd, "node");

        let mut cmd = Command::new("node");
        cmd.arg(root.join("bundle.js"))
            .current_dir(&root);
        run(&mut cmd, "node");
    }
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
