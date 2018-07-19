use super::project;

#[test]
fn works() {
    let mut p = project();
    p.file(
        "src/lib.rs",
        r#"
            #![feature(use_extern_macros, wasm_import_module)]

            extern crate wasm_bindgen;

            use wasm_bindgen::prelude::*;

            #[wasm_bindgen]
            /// This comment should exist
            pub fn annotated() -> String {
                String::new()
            }

            #[wasm_bindgen]
            /// This comment should exist
            pub struct Annotated {
                /// This comment should not exist
                a: String,
                /// This comment should exist
                pub b: u32
            }

            #[wasm_bindgen]
            impl Annotated {
                #[wasm_bindgen(method)]
                /// This comment should exist
                pub fn get_a(&self) -> String {
                    self.a.clone()
                }
            }
        "#,
    );

    p.gen_bindings();
    let js = p.read_js();
    let comments = extract_doc_comments(&js);
    assert!(comments.iter().all(|c| c == "This comment should exist" ||
                                    c.starts_with("@")));
}

/// Pull out all lines in a js string that start with
/// '* ', all other lines will either be comment start, comment
/// end or actual js lines.
fn extract_doc_comments(js: &str) -> Vec<String> {
    js.lines()
        .filter_map(|l| {
            let trimmed = l.trim();
            if trimmed.starts_with("* ") {
                Some(trimmed[2..].to_owned())
            } else {
                None
            }
        })
        .collect()
}
