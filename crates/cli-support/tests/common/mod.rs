#[macro_export]
#[allow(unused)]
macro_rules! test_output_snapshot {
    ($name:ident, $start_output:literal, $init:expr) => {
        #[test]
        fn $name() {
            use std::path::PathBuf;
            use wasm_bindgen_cli_support::Bindgen;

            let mut bindgen = Bindgen::new();
            $init(&mut bindgen);
            bindgen.input_path(WASM.concat());

            let output = bindgen.generate_output().unwrap();
            let mut settings = insta::Settings::clone_current();
            settings.set_snapshot_suffix(
                PathBuf::from(WASM[1])
                    .file_stem()
                    .unwrap()
                    .to_str()
                    .unwrap(),
            );
            settings.set_prepend_module_to_snapshot(false);
            if $start_output {
                settings.bind(|| insta::assert_snapshot!(output.start().unwrap()));
            } else {
                settings.bind(|| insta::assert_snapshot!(output.js()));
            }
        }
    };
}
#[macro_export]
#[allow(unused)]
macro_rules! test_output_empty {
    ($name:ident, $start_output:literal, $init:expr) => {
        #[test]
        fn $name() {
            use wasm_bindgen_cli_support::Bindgen;

            let mut bindgen = Bindgen::new();
            $init(&mut bindgen);
            bindgen.input_path(WASM.concat());

            let output = bindgen.generate_output().unwrap();
            if $start_output {
                assert!(output.start().is_none());
            } else {
                assert!(output.js().is_empty());
            }
        }
    };
}
#[macro_export]
#[allow(unused)]
macro_rules! test_output_error {
    ($name:ident, $init:expr) => {
        #[test]
        fn $name() {
            use wasm_bindgen_cli_support::Bindgen;

            let mut bindgen = Bindgen::new();
            $init(&mut bindgen);
            bindgen.input_path(WASM.concat());

            let output = bindgen.generate_output();
            assert!(output.is_err());
        }
    };
}
