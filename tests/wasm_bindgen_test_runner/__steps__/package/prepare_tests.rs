pub fn prepare_tests(tests: &str) -> String {
    let mut final_content = String::from(
        r#"
#[cfg(test)]
use wasm_bindgen_test::*;
"#,
    );

    for line in tests.lines() {
        if line.starts_with("#[wasm_bindgen_test]") || line.starts_with("mod") {
            final_content.push_str("#[cfg(test)]\n");
        }
        final_content.push_str(line);
        final_content.push_str("\n");
    }

    final_content
}
