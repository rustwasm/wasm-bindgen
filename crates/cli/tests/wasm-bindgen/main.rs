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

#[test]
fn namespace_global_and_noglobal_works() {
    let (mut cmd, _out_dir) = Project::new("namespace_global_and_noglobal_works")
        .file(
            "src/lib.rs",
            r#"
                use wasm_bindgen::prelude::*;
                #[wasm_bindgen(module = "fs")]
                extern "C" {
                    #[wasm_bindgen(js_namespace = window)]
                    fn t1();
                }
                #[wasm_bindgen]
                extern "C" {
                    #[wasm_bindgen(js_namespace = window)]
                    fn t2();
                }
                #[wasm_bindgen]
                pub fn test() {
                    t1();
                    t2();
                }
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

#[test]
fn bin_crate_works() {
    let (mut cmd, out_dir) = Project::new("bin_crate_works")
        .file(
            "src/main.rs",
            r#"
                use wasm_bindgen::prelude::*;
                #[wasm_bindgen]
                extern "C" {
                    #[wasm_bindgen(js_namespace = console)]
                    fn log(data: &str);
                }

                fn main() {
                    log("hello, world");
                }
            "#,
        )
        .file(
            "Cargo.toml",
            &format!(
                "
                    [package]
                    name = \"bin_crate_works\"
                    authors = []
                    version = \"1.0.0\"
                    edition = '2018'

                    [dependencies]
                    wasm-bindgen = {{ path = '{}' }}

                    [workspace]
                ",
                repo_root().display(),
            ),
        )
        .wasm_bindgen("--target nodejs");
    cmd.assert().success();
    Command::new("node")
        .arg("bin_crate_works.js")
        .current_dir(out_dir)
        .assert()
        .success()
        .stdout("hello, world\n");
}

#[test]
fn default_module_path_target_web() {
    let (mut cmd, out_dir) = Project::new("default_module_path_target_web")
        .file(
            "src/lib.rs",
            r#"
            "#,
        )
        .wasm_bindgen("--target web");
    cmd.assert().success();
    let contents = fs::read_to_string(out_dir.join("default_module_path_target_web.js")).unwrap();
    assert!(contents.contains(
        "\
async function init(input) {
    if (typeof input === 'undefined') {
        input = new URL('default_module_path_target_web_bg.wasm', import.meta.url);
    }",
    ));
}

#[test]
fn default_module_path_target_no_modules() {
    let (mut cmd, out_dir) = Project::new("default_module_path_target_no_modules")
        .file(
            "src/lib.rs",
            r#"
            "#,
        )
        .wasm_bindgen("--target no-modules");
    cmd.assert().success();
    let contents =
        fs::read_to_string(out_dir.join("default_module_path_target_no_modules.js")).unwrap();
    assert!(contents.contains(
        "\
    async function init(input) {
        if (typeof input === 'undefined') {
            let src;
            if (typeof document === 'undefined') {
                src = location.href;
            } else {
                src = document.currentScript.src;
            }
            input = src.replace(/\\.js$/, '_bg.wasm');
        }",
    ));
}

#[test]
fn omit_default_module_path_target_web() {
    let (mut cmd, out_dir) = Project::new("omit_default_module_path_target_web")
        .file(
            "src/lib.rs",
            r#"
            "#,
        )
        .wasm_bindgen("--target web --omit-default-module-path");
    cmd.assert().success();
    let contents =
        fs::read_to_string(out_dir.join("omit_default_module_path_target_web.js")).unwrap();
    assert!(contents.contains(
        "\
async function init(input) {

    const imports = getImports();",
    ));
}

#[test]
fn omit_default_module_path_target_no_modules() {
    let (mut cmd, out_dir) = Project::new("omit_default_module_path_target_no_modules")
        .file(
            "src/lib.rs",
            r#"
            "#,
        )
        .wasm_bindgen("--target no-modules --omit-default-module-path");
    cmd.assert().success();
    let contents =
        fs::read_to_string(out_dir.join("omit_default_module_path_target_no_modules.js")).unwrap();
    assert!(contents.contains(
        "\
    async function init(input) {

        const imports = getImports();",
    ));
}

#[test]
fn empty_interface_types() {
    let (mut cmd, _out_dir) = Project::new("empty_interface_types")
        .file(
            "src/lib.rs",
            r#"
                #[no_mangle]
                pub extern fn foo() {}
            "#,
        )
        .file(
            "Cargo.toml",
            &format!(
                "
                    [package]
                    name = \"empty_interface_types\"
                    authors = []
                    version = \"1.0.0\"
                    edition = '2018'

                    [dependencies]
                    wasm-bindgen = {{ path = '{}' }}

                    [lib]
                    crate-type = ['cdylib']

                    [workspace]
                ",
                repo_root().display(),
            ),
        )
        .wasm_bindgen("");
    cmd.env("WASM_INTERFACE_TYPES", "1");
    cmd.assert().success();
}

#[test]
fn bad_interface_types_export() -> anyhow::Result<()> {
    let (mut cmd, _out_dir) = Project::new("bad_interface_types_export")
        .file(
            "src/lib.rs",
            r#"
                use wasm_bindgen::prelude::*;

                #[wasm_bindgen]
                pub fn foo(a: Vec<u8>) {}
            "#,
        )
        .file(
            "Cargo.toml",
            &format!(
                "
                    [package]
                    name = \"bad_interface_types_export\"
                    authors = []
                    version = \"1.0.0\"
                    edition = '2018'

                    [lib]
                    crate-type = [\"cdylib\"]

                    [dependencies]
                    wasm-bindgen = {{ path = '{}' }}

                    [workspace]
                ",
                repo_root().display(),
            ),
        )
        .wasm_bindgen("");
    cmd.env("WASM_INTERFACE_TYPES", "1");
    cmd.assert().failure().code(1).stderr(str::is_match(
        "\
error: failed to generate a standard interface types section

Caused by:
    0: in function export `foo`
    1: type Vector\\(U8\\) isn't supported in standard interface types
$",
    )?);
    Ok(())
}

#[test]
fn bad_interface_types_import() -> anyhow::Result<()> {
    let (mut cmd, _out_dir) = Project::new("bad_interface_types_import")
        .file(
            "src/lib.rs",
            r#"
                use wasm_bindgen::prelude::*;

                #[wasm_bindgen]
                extern "C" {
                    pub fn foo() -> Vec<u8>;
                }

                #[wasm_bindgen]
                pub fn bar() {
                    foo();
                }
            "#,
        )
        .file(
            "Cargo.toml",
            &format!(
                "
                    [package]
                    name = \"bad_interface_types_import\"
                    authors = []
                    version = \"1.0.0\"
                    edition = '2018'

                    [lib]
                    crate-type = [\"cdylib\"]

                    [dependencies]
                    wasm-bindgen = {{ path = '{}' }}

                    [workspace]
                ",
                repo_root().display(),
            ),
        )
        .wasm_bindgen("");
    cmd.env("WASM_INTERFACE_TYPES", "1");
    cmd.assert().failure().code(1).stderr(str::is_match(
        "\
error: failed to generate a standard interface types section

Caused by:
    0: in adapter function
    1: import of global `foo` requires JS glue
$",
    )?);
    Ok(())
}

#[test]
fn function_table_preserved() {
    let (mut cmd, _out_dir) = Project::new("function_table_preserved")
        .file(
            "src/lib.rs",
            r#"
                use wasm_bindgen::prelude::*;

                #[wasm_bindgen]
                pub fn bar() {
                    Closure::wrap(Box::new(|| {}) as Box<dyn Fn()>);
                }
            "#,
        )
        .wasm_bindgen("");
    cmd.assert().success();
}
