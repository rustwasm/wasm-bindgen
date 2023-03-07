use anyhow::{Context, Result};
use std::{fs, path::PathBuf};

pub fn update_cargo_toml_features(
    cargo_toml_path: &PathBuf,
    generated_features: &str,
) -> Result<()> {
    let mut cargo_toml_text = fs::read_to_string(cargo_toml_path).context("reading Cargo.toml")?;
    let comment = "# This list is auto-generated by the wasm-bindgen-webidl program";
    let features_head = "[features]";
    let features_table = format!(
        r"{comment}
{features_head}
{generated_features}
"
    );

    match cargo_toml_text.find(features_head) {
        Some(features_start) => {
            let comment_start = cargo_toml_text[..features_start]
                .rfind(comment)
                .unwrap_or(0);

            let end = cargo_toml_text[features_start..]
                .find("\n[") // find start of next toml table
                .map(|i| i + features_start)
                .unwrap_or_else(|| cargo_toml_text.len()); // or end of file

            cargo_toml_text.replace_range(comment_start..end, &features_table);
        }
        None => {
            cargo_toml_text.push_str(&features_table);
        }
    }

    fs::write(cargo_toml_path, cargo_toml_text).context("writing features to current directory")?;

    Ok(())
}
