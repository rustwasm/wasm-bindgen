extern crate wasm_bindgen_cli_support as cli;

use std::env;
use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::atomic::*;
use std::sync::{Once, ONCE_INIT};
use std::time::Instant;

static CNT: AtomicUsize = ATOMIC_USIZE_INIT;
thread_local!(static IDX: usize = CNT.fetch_add(1, Ordering::SeqCst));

struct Project {
    files: Vec<(String, String)>,
    debug: bool,
    node: bool,
    no_std: bool,
    serde: bool,
    rlib: bool,
    node_args: Vec<String>,
    deps: Vec<String>,
}

fn project() -> Project {
    let dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let mut lockfile = String::new();
    fs::File::open(&dir.join("Cargo.lock"))
        .unwrap()
        .read_to_string(&mut lockfile)
        .unwrap();
    Project {
        debug: true,
        node: false,
        no_std: false,
        serde: false,
        rlib: false,
        deps: Vec::new(),
        node_args: Vec::new(),
        files: vec![
            ("Cargo.lock".to_string(), lockfile),
            (
                "run-node.js".to_string(),
                r#"require("./test").test();"#.to_string(),
            ),
            (
                "webpack.config.js".to_string(),
                r#"
                const path = require('path');
                const fs = require('fs');
                
                let nodeModules = {};

                // Webpack bundles the modules from node_modules.
                // For node target, we will not have `fs` module 
                // inside the `node_modules` folder.
                // This reads the directories in `node_modules`
                // and give that to externals and webpack ignores 
                // to bundle the modules listed as external. 
                fs.readdirSync('node_modules')
                    .filter(module => module !== '.bin')
                    .forEach(mod => {
                        // External however,expects browser environment. 
                        // To make it work in `node` target we 
                        // prefix commonjs here.
                        nodeModules[mod] = 'commonjs ' + mod;
                    });

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
                  target: 'node',
                  externals: nodeModules
                };
            "#.to_string(),
            ),
            (
                "tsconfig.json".to_string(),
                r#"
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
            "#.to_string(),
            ),
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
    return me;
}

fn assert_bigint_support() -> Option<&'static str> {
    static BIGINT_SUPPORED: AtomicUsize = ATOMIC_USIZE_INIT;
    static INIT: Once = ONCE_INIT;

    INIT.call_once(|| {
        let mut cmd = Command::new("node");
        cmd.arg("-e").arg("BigInt");
        cmd.stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());
        if cmd.status().unwrap().success() {
            BIGINT_SUPPORED.store(1, Ordering::SeqCst);
            return;
        }

        cmd.arg("--harmony-bigint");
        if cmd.status().unwrap().success() {
            BIGINT_SUPPORED.store(2, Ordering::SeqCst);
            return;
        }
    });

    match BIGINT_SUPPORED.load(Ordering::SeqCst) {
        1 => return None,
        2 => return Some("--harmony-bigint"),
        _ => panic!(
            "the version of node.js that is installed for these tests \
             does not support `BigInt`, you may wish to try installing \
             node 10 to fix this"
        ),
    }
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

    fn serde(&mut self, serde: bool) -> &mut Project {
        self.serde = serde;
        self
    }

    fn rlib(&mut self, rlib: bool) -> &mut Project {
        self.rlib = rlib;
        self
    }

    fn depend(&mut self, dep: &str) -> &mut Project {
        self.deps.push(dep.to_string());
        self
    }

    fn add_local_dependency(&mut self, name: &str, path: &str) -> &mut Project {
        self.deps
            .push(format!("{} = {{ path = '{}' }}", name, path));
        self
    }

    fn crate_name(&self) -> String {
        format!("test{}", IDX.with(|x| *x))
    }

    fn requires_bigint(&mut self) -> &mut Project {
        if let Some(arg) = assert_bigint_support() {
            self.node_args.push(arg.to_string());
        }
        self
    }

    fn generate_webidl_bindings(&mut self) -> Vec<PathBuf> {
        let mut res = Vec::new();
        let mut origpaths = Vec::new();

        for (path, _) in &self.files {
            let path = Path::new(&path);
            let extension = path.extension().map(|x| x.to_str().unwrap());

            if extension != Some("webidl") {
                continue;
            }

            res.push(path.with_extension("rs"));
            origpaths.push(path.to_owned());
        }

        if res.is_empty() {
            return res;
        }

        let mut buildrs = r#"
            extern crate wasm_bindgen_webidl;

            use wasm_bindgen_webidl::compile_file;
            use std::env;
            use std::fs::{self, File};
            use std::io::Write;
            use std::path::Path;

            fn main() {
                let dest = env::var("OUT_DIR").unwrap();
        "#.to_string();

        for (path, origpath) in res.iter().zip(origpaths.iter()) {
            buildrs.push_str(&format!(
                r#"
                fs::create_dir_all("{}").unwrap();
                File::create(&Path::new(&dest).join("{}"))
                    .unwrap()
                    .write_all(
                        compile_file(Path::new("{}"))
                            .unwrap()
                            .as_bytes()
                    )
                    .unwrap();
                "#,
                path.parent().unwrap().to_str().unwrap(),
                path.to_str().unwrap(),
                origpath.to_str().unwrap(),
            ));

            self.files.push((
                Path::new("src").join(path).to_str().unwrap().to_string(),
                format!(
                    r#"include!(concat!(env!("OUT_DIR"), "/{}"));"#,
                    path.display()
                ),
            ));
        }

        buildrs.push('}');

        self.files.push(("build.rs".to_string(), buildrs));

        res
    }

    fn generate_js_entry(&mut self, modules: Vec<PathBuf>) {
        let mut runjs = r#"
            import * as process from "process";
        "#.to_string();

        if !modules.is_empty() {
            runjs.push_str(r#"Promise.all(["#);
            for module in &modules {
                runjs.push_str(&format!(
                    r#"import('./{}'),"#,
                    module.with_extension("").to_str().unwrap()
                ));
            }
            runjs.push_str(
                r#"]).then(results => { results.map(module => Object.assign(global, module));"#,
            );
        }

        runjs.push_str(
            r#"
            let wasm = import('./out');
            const test = import('./test');
            "#,
        );

        if !modules.is_empty() {
            runjs.push_str("return ");
        }

        runjs.push_str(r#"Promise.all([test, wasm])"#);

        if !modules.is_empty() {
            runjs.push_str("})");
        }

        runjs.push_str(
            r#"
            .then(results => {
                let [test, wasm] = results;
                test.test();

                if (wasm.assertStackEmpty)
                    wasm.assertStackEmpty();
                if (wasm.assertSlabEmpty)
                    wasm.assertSlabEmpty();
            })
            .catch(error => {
                  console.error(error);
                  process.exit(1);
                });
            "#,
        );
        self.files.push(("run.js".to_string(), runjs));
    }

    fn ensure_test_entry(&mut self) {
        if !self
            .files
            .iter()
            .any(|(path, _)| path == "test.ts" || path == "test.js")
        {
            self.files.push((
                "test.ts".to_string(),
                r#"export {test} from './out';"#.to_string(),
            ));
        }
    }
    /// build + cargo cmd execution
    fn cargo_build(&mut self) -> (PathBuf, PathBuf) {
        let (root, target_dir) = self.build();
        let mut cmd = Command::new("cargo");
        cmd.arg("build")
            .arg("--target")
            .arg("wasm32-unknown-unknown")
            .current_dir(&root)
            .env("CARGO_TARGET_DIR", &target_dir)
            // Catch any warnings in generated code because we don't want any
            .env("RUSTFLAGS", "-Dwarnings");
        run(&mut cmd, "cargo");
        (root, target_dir)
    }

    fn build(&mut self) -> (PathBuf, PathBuf) {
        self.ensure_test_entry();

        let webidl_modules = self.generate_webidl_bindings();
        self.generate_js_entry(webidl_modules);

        let mut manifest = format!(
            r#"
            [package]
            name = "test{}"
            version = "0.0.1"
            authors = []

            [workspace]

            [lib]
        "#,
            IDX.with(|x| *x)
        );

        if !self.rlib {
            manifest.push_str("crate-type = [\"cdylib\"]\n");
        }

        manifest.push_str("[build-dependencies]\n");
        manifest.push_str("wasm-bindgen-webidl = { path = '");
        manifest.push_str(env!("CARGO_MANIFEST_DIR"));
        manifest.push_str("/crates/webidl' }\n");

        manifest.push_str("[dependencies]\n");
        for dep in self.deps.iter() {
            manifest.push_str(dep);
            manifest.push_str("\n");
        }
        manifest.push_str("wasm-bindgen = { path = '");
        manifest.push_str(env!("CARGO_MANIFEST_DIR"));
        manifest.push_str("'");
        if self.no_std {
            manifest.push_str(", default-features = false");
        }
        if self.serde {
            manifest.push_str(", features = ['serde-serialize']");
        }
        manifest.push_str(" }\n");
        self.files.push(("Cargo.toml".to_string(), manifest));

        let root = root();
        drop(fs::remove_dir_all(&root));
        for &(ref file, ref contents) in self.files.iter() {
            let dst = root.join(file);
            fs::create_dir_all(dst.parent().unwrap()).unwrap();
            fs::File::create(&dst)
                .unwrap()
                .write_all(contents.as_ref())
                .unwrap();
        }
        let target_dir = root.parent().unwrap() // chop off test name
            .parent().unwrap(); // chop off `generated-tests`
        (root.clone(), target_dir.to_path_buf())
    }

    fn test(&mut self) {
        let (root, target_dir) = self.cargo_build();

        self.gen_bindings(&root, &target_dir);
        let mut wasm = Vec::new();
        File::open(root.join("out_bg.wasm"))
            .unwrap()
            .read_to_end(&mut wasm)
            .unwrap();

        {
            let _x = wrap_step("running wasm2es6js");
            let obj = cli::wasm2es6js::Config::new()
                .base64(true)
                .generate(&wasm)
                .expect("failed to convert wasm to js");

            File::create(root.join("out_bg.d.ts"))
                .unwrap()
                .write_all(obj.typescript().as_bytes())
                .unwrap();
        }

        // move files from the root into each test, it looks like this may be
        // needed for webpack to work well when invoked concurrently.
        fs::hard_link("package.json", root.join("package.json")).unwrap();
        if !Path::new("node_modules").exists() {
            panic!("\n\nfailed to find `node_modules`, have you run `npm install` yet?\n\n");
        }
        let cwd = env::current_dir().unwrap();
        symlink_dir(&cwd.join("node_modules"), &root.join("node_modules")).unwrap();

        if self.node {
            let mut cmd = Command::new("node");
            cmd.args(&self.node_args);
            cmd.arg(root.join("run-node.js")).current_dir(&root);
            run(&mut cmd, "node");
        } else {
            let mut cmd = if cfg!(windows) {
                let mut c = Command::new("cmd");
                c.arg("/c");
                c.arg("npm");
                c
            } else {
                Command::new("npm")
            };
            cmd.arg("run").arg("run-webpack").current_dir(&root);
            run(&mut cmd, "npm");

            let mut cmd = Command::new("node");
            cmd.args(&self.node_args);
            cmd.arg(root.join("bundle.js")).current_dir(&root);
            run(&mut cmd, "node");
        }
    }
    /// execute the cli against the current test .wasm
    fn gen_bindings(&self, root: &PathBuf, target_dir: &PathBuf) {
        let idx = IDX.with(|x| *x);
        let out = target_dir.join(&format!("wasm32-unknown-unknown/debug/test{}.wasm", idx));

        let as_a_module = root.join("out.wasm");
        fs::copy(&out, &as_a_module).unwrap();

        let _x = wrap_step("running wasm-bindgen");
        let res = cli::Bindgen::new()
            .input_path(&as_a_module)
            .typescript(true)
            .nodejs(self.node)
            .debug(self.debug)
            .generate(&root);
        if let Err(e) = res {
            for e in e.causes() {
                println!("- {}", e);
            }
            panic!("failed");
        }
    }
    fn read_js(&self) -> String {
        let path = root().join("out.js");
        println!("js, {:?}", &path);
        fs::read_to_string(path).expect("Unable to read js")
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

fn wrap_step(desc: &str) -> WrapStep {
    println!("···················································");
    println!("{}", desc);
    WrapStep { start: Instant::now() }
}

struct WrapStep { start: Instant }

impl Drop for WrapStep {
    fn drop(&mut self) {
        let dur = self.start.elapsed();
        println!(
            "dur: {}.{:03}s",
            dur.as_secs(),
            dur.subsec_nanos() / 1_000_000
        );
    }
}

fn run(cmd: &mut Command, program: &str) {
    let _x = wrap_step(&format!("running {:?}", cmd));
    let output = match cmd.output() {
        Ok(output) => output,
        Err(err) => panic!("failed to spawn `{}`: {}", program, err),
    };
    println!("exit: {}", output.status);
    if output.stdout.len() > 0 {
        println!("stdout ---\n{}", String::from_utf8_lossy(&output.stdout));
    }
    if output.stderr.len() > 0 {
        println!("stderr ---\n{}", String::from_utf8_lossy(&output.stderr));
    }
    assert!(output.status.success());
}

mod api;
mod char;
mod classes;
mod closures;
mod comments;
mod dependencies;
mod enums;
mod import_class;
mod imports;
#[cfg(feature = "js_globals")]
mod js_globals;
mod jsobjects;
mod math;
mod node;
mod non_debug;
mod non_wasm;
mod simple;
mod slice;
mod structural;
mod u64;
mod validate_prt;
mod webidl;
