extern crate proc_macro2;
extern crate syn;
extern crate wasm_bindgen_backend as backend;
extern crate wasm_bindgen_webidl as wb_webidl;

pub fn assert_parse(webidl: &str, expected: backend::ast::Program) {
    let actual = wb_webidl::parse(webidl).expect("should parse the webidl source OK");
    assert_eq!(expected, actual);
}

macro_rules! assert_parse {
    ($test_name:ident, $webidl_source:expr, $expected_ast:expr) => {
        #[test]
        fn $test_name() {
            $crate::assert_parse($webidl_source, $expected_ast);
        }
    };
}

mod event;
mod simple;
