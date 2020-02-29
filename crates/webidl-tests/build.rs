use std::io::Result;
use std::process::Command;

fn main() -> Result<()> {
    let result = Command::new("cargo")
        .arg("run")
        .arg("--release")
        .arg("--package")
        .arg("wasm-bindgen-webidl")
        .arg("webidls")
        .arg("generated")
        .arg("--no-features")
        .status()?;

    assert!(result.success());

    Ok(())
}
