use crate::*;

#[test]
fn no_modules_rejects_npm() {
    let (mut cmd, _out_dir) = Project::new("no_modules_rejects_npm")
        .file(
            "src/lib.rs",
            r#"
                use wasm_bindgen::prelude::*;

                #[wasm_bindgen(module = "foo")]
                extern {
                    fn foo();
                }

                #[wasm_bindgen(start)]
                fn main() {
                    foo();
                }
            "#,
        )
        .file("package.json", "")
        .wasm_bindgen("--no-modules");
    cmd.assert()
        .stderr(
            str::is_match(
                "\
error: NPM dependencies have been specified in `.*` but \
this is incompatible with the `no-modules` target
",
            )
            .unwrap(),
        )
        .failure();
}

#[test]
fn more_package_json_fields_ignored() {
    let (mut cmd, _out_dir) = Project::new("more_package_json_fields_ignored")
        .file(
            "src/lib.rs",
            r#"
                use wasm_bindgen::prelude::*;

                #[wasm_bindgen(module = "foo")]
                extern {
                    fn foo();
                }

                #[wasm_bindgen(start)]
                fn main() {
                    foo();
                }
            "#,
        )
        .file(
            "package.json",
            r#"
                {
                    "name": "foo",
                    "dependencies": {}
                }
            "#,
        )
        .wasm_bindgen("");
    cmd.assert().success();
}

#[test]
fn npm_conflict_rejected() {
    let (mut cmd, _out_dir) = Project::new("npm_conflict_rejected")
        .file(
            "Cargo.toml",
            &format!(
                r#"
                [package]
                name = "npm_conflict_rejected"
                authors = []
                version = "1.0.0"
                edition = '2021'

                [dependencies]
                wasm-bindgen = {{ path = '{}' }}
                bar = {{ path = 'bar' }}

                [lib]
                crate-type = ['cdylib']

                [workspace]
            "#,
                repo_root().display()
            ),
        )
        .file(
            "src/lib.rs",
            r#"
                use wasm_bindgen::prelude::*;

                #[wasm_bindgen(module = "bar")]
                extern {
                    fn foo();
                }

                #[wasm_bindgen(start)]
                fn main() {
                    foo();
                    bar::foo();
                }
            "#,
        )
        .file(
            "package.json",
            r#"
                {
                    "dependencies": {"bar": "0.0.0"}
                }
            "#,
        )
        .file(
            "bar/Cargo.toml",
            &format!(
                r#"
                [package]
                name = "bar"
                authors = []
                version = "1.0.0"
                edition = '2021'

                [dependencies]
                wasm-bindgen = {{ path = '{}' }}
            "#,
                repo_root().display()
            ),
        )
        .file(
            "bar/src/lib.rs",
            r#"
                use wasm_bindgen::prelude::*;

                #[wasm_bindgen(module = "bar")]
                extern {
                    pub fn foo();
                }
            "#,
        )
        .file(
            "bar/package.json",
            r#"
                {
                    "dependencies": {"bar": "1.0.0"}
                }
            "#,
        )
        .wasm_bindgen("");
    cmd.assert()
        .stderr(str::is_match("dependency on NPM package `bar` specified in two").unwrap())
        .failure();
}
