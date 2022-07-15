use anyhow::{bail, Result};
use assert_cmd::prelude::*;
use rayon::prelude::*;
use std::env;
use std::path::Path;
use std::process::Command;

fn main() -> Result<()> {
    let filter = env::args().nth(1);

    let mut tests = Vec::new();
    let dir = env::current_dir()?.join("tests/interface-types");
    for entry in dir.read_dir()? {
        let path = entry?.path();
        if path.extension().and_then(|s| s.to_str()) != Some("wat") {
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
    let js = test.with_extension("js");
    let td = tempfile::TempDir::new()?;

    let mut bindgen = Command::cargo_bin("wasm-bindgen")?;
    bindgen
        .arg("--out-dir")
        .arg(td.path())
        .arg(test)
        .arg("--out-name=wasm")
        .arg("--nodejs")
        .arg("--no-typescript");
    exec(&mut bindgen)?;

    exec(
        Command::new("node")
            .arg("--experimental-wasm-reftypes")
            .arg(&js)
            .env("NODE_PATH", td.path()),
    )
    .or_else(|_| {
        // The `--experimental-wasm-reftypes` flag was removed in Node 18, so that might
        // have caused the error; try again without it.
        exec(Command::new("node").arg(&js).env("NODE_PATH", td.path()))
    })
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
