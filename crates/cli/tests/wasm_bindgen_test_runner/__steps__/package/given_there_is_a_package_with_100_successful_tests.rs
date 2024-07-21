use crate::__steps__::cargo_nextest::Context;
use crate::__steps__::package::given_there_is_a_package_with;

pub fn given_there_is_a_package_with_100_successful_tests(context: &mut Context) {
    let mut tests = String::new();
    for i in 0..100 {
        tests.push_str(&format!(
            r#"
#[wasm_bindgen_test]
fn pass_{}() {{
    assert_eq!(1, 1);
}}
"#,
            i
        ));
    }

    given_there_is_a_package_with(context, &tests);
}
