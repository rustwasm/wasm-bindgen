//! A test suite to check the reference JS and wasm output of the `wasm-bindgen`
//! library.
//!
//! This is intended as an end-to-end integration test where we can track
//! changes to the JS and wasm output.
//!
//! Tests are located in `reference/*.rs` files and are accompanied with sibling
//! `*.js` files and `*.wat` files with the expected output of the `*.rs`
//! compilation. Use `BLESS=1` in the environment to automatically update all
//! tests.

use anyhow::{bail, Result};
use assert_cmd::prelude::*;
use rayon::prelude::*;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

fn main() -> Result<()> {
    let filter = env::args().nth(1);

    let mut tests = Vec::new();
    let dir = env::current_dir()?.join("tests/reference");
    for entry in dir.read_dir()? {
        let path = entry?.path();
        if path.extension().and_then(|s| s.to_str()) != Some("rs") {
            continue;
        }
        if let Some(filter) = &filter {
            if !path.display().to_string().contains(filter) {
                continue;
            }
        }
        tests.push(path);
    }
    tests.sort();

    let errs = tests
        .par_iter()
        .filter_map(|t| runtest(t).err().map(|e| (t, e)))
        .collect::<Vec<_>>();

    if errs.len() == 0 {
        println!("{} tests passed", tests.len());
        return Ok(());
    }
    eprintln!("failed tests:\n");
    for (test, err) in errs {
        eprintln!("{} failure\n{}", test.display(), tab(&format!("{:?}", err)));
    }
    bail!("tests failed");
}

fn runtest(test: &Path) -> Result<()> {
    let contents = fs::read_to_string(test)?;
    let td = tempfile::TempDir::new()?;

    let manifest = format!(
        "
            [package]
            name = \"reference-test\"
            authors = []
            version = \"1.0.0\"
            edition = '2018'

            [dependencies]
            wasm-bindgen = {{ path = '{}' }}

            [lib]
            crate-type = ['cdylib']
            path = '{}'
        ",
        repo_root().display(),
        test.display(),
    );
    let interface_types = contents.contains("// interface-types");

    fs::write(td.path().join("Cargo.toml"), manifest)?;
    let target_dir = target_dir();
    let mut cargo = Command::new("cargo");
    cargo
        .current_dir(td.path())
        .arg("build")
        .arg("--target")
        .arg("wasm32-unknown-unknown")
        .env("CARGO_TARGET_DIR", &target_dir);
    exec(&mut cargo)?;

    let wasm = target_dir
        .join("wasm32-unknown-unknown")
        .join("debug")
        .join("reference_test.wasm");

    let mut bindgen = Command::cargo_bin("wasm-bindgen")?;
    bindgen
        .arg("--out-dir")
        .arg(td.path())
        .arg(&wasm)
        .arg("--no-typescript");
    if contents.contains("// enable-externref") {
        bindgen.env("WASM_BINDGEN_EXTERNREF", "1");
    }
    if interface_types {
        bindgen.env("WASM_INTERFACE_TYPES", "1");
    }
    exec(&mut bindgen)?;

    if interface_types {
        let wasm = td.path().join("reference_test.wasm");
        wit_validator::validate(&fs::read(&wasm)?)?;
        let wat = sanitize_wasm(&wasm)?;
        assert_same(&wat, &test.with_extension("wat"))?;
    } else {
        let js = fs::read_to_string(td.path().join("reference_test_bg.js"))?;
        assert_same(&js, &test.with_extension("js"))?;
        let wat = sanitize_wasm(&td.path().join("reference_test_bg.wasm"))?;
        assert_same(&wat, &test.with_extension("wat"))?;
    }

    Ok(())
}

fn assert_same(output: &str, expected: &Path) -> Result<()> {
    if env::var("BLESS").is_ok() {
        fs::write(expected, output)?;
    } else {
        let expected = fs::read_to_string(&expected)?;
        diff(&expected, output)?;
    }
    Ok(())
}

fn sanitize_wasm(wasm: &Path) -> Result<String> {
    // Clean up the wasm module by removing all function
    // implementations/instructions, data sections, etc. This'll help us largely
    // only deal with exports/imports which is all we're really interested in.
    let mut module = walrus::ModuleConfig::new()
        .on_parse(wit_walrus::on_parse)
        .parse_file(wasm)?;
    for func in module.funcs.iter_mut() {
        let local = match &mut func.kind {
            walrus::FunctionKind::Local(l) => l,
            _ => continue,
        };
        local.block_mut(local.entry_block()).instrs.truncate(0);
    }
    let ids = module.data.iter().map(|d| d.id()).collect::<Vec<_>>();
    for id in ids {
        module.data.delete(id);
    }
    for mem in module.memories.iter_mut() {
        mem.data_segments.drain();
    }
    let ids = module.elements.iter().map(|d| d.id()).collect::<Vec<_>>();
    for id in ids {
        module.elements.delete(id);
    }
    for table in module.tables.iter_mut() {
        table.elem_segments.drain();
    }
    let ids = module
        .exports
        .iter()
        .filter(|e| match e.item {
            walrus::ExportItem::Global(_) => true,
            _ => false,
        })
        .map(|d| d.id())
        .collect::<Vec<_>>();
    for id in ids {
        module.exports.delete(id);
    }
    walrus::passes::gc::run(&mut module);
    let mut wat = wit_printer::print_bytes(&module.emit_wasm())?;
    wat.push_str("\n");
    Ok(wat)
}

fn diff(a: &str, b: &str) -> Result<()> {
    if a == b {
        return Ok(());
    }
    let mut s = String::new();
    for result in diff::lines(a, b) {
        match result {
            diff::Result::Both(l, _) => {
                s.push_str(" ");
                s.push_str(l);
            }
            diff::Result::Left(l) => {
                s.push_str("-");
                s.push_str(l);
            }
            diff::Result::Right(l) => {
                s.push_str("+");
                s.push_str(l);
            }
        }
        s.push_str("\n");
    }
    bail!("found a difference:\n\n{}", s);
}

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

fn exec(cmd: &mut Command) -> Result<()> {
    let output = cmd.output()?;
    if output.status.success() {
        return Ok(());
    }
    let mut err = format!("command failed {:?}", cmd);
    err.push_str(&format!("\nstatus: {}", output.status));
    err.push_str(&format!(
        "\nstderr:\n{}",
        tab(&String::from_utf8_lossy(&output.stderr))
    ));
    err.push_str(&format!(
        "\nstdout:\n{}",
        tab(&String::from_utf8_lossy(&output.stdout))
    ));
    bail!("{}", err);
}

fn tab(s: &str) -> String {
    format!("    {}", s.replace("\n", "\n    "))
}
