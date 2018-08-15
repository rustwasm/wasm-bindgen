use futures::Future;
use js_sys::*;
use wasm_bindgen::{JsCast, prelude::*};
use wasm_bindgen_futures::JsFuture;
use wasm_bindgen_test::*;

#[wasm_bindgen(module = "tests/wasm/WebAssembly.js")]
extern {
    #[wasm_bindgen(js_name = getWasmArray)]
    fn get_wasm_array() -> Uint8Array;

    #[wasm_bindgen(js_name = getTableObject)]
    fn get_table_object() -> Object;
}

fn get_invalid_wasm() -> JsValue {
    ArrayBuffer::new(42).into()
}

fn get_bad_type_wasm() -> JsValue {
    2.into()
}

fn get_valid_wasm() -> JsValue {
    get_wasm_array().into()
}

#[wasm_bindgen_test]
fn validate() {
    assert!(!WebAssembly::validate(&get_invalid_wasm()).unwrap());

    assert!(WebAssembly::validate(&get_bad_type_wasm()).is_err());
}

#[wasm_bindgen_test(async)]
fn compile_compile_error() -> impl Future<Item = (), Error = JsValue> {
    let p = WebAssembly::compile(&get_invalid_wasm());
    JsFuture::from(p)
        .map(|_| unreachable!())
        .or_else(|e| {
            assert!(e.is_instance_of::<WebAssembly::CompileError>());
            Ok(())
        })
}

#[wasm_bindgen_test(async)]
fn compile_type_error() -> impl Future<Item = (), Error = JsValue> {
    let p = WebAssembly::compile(&get_bad_type_wasm());
    JsFuture::from(p)
        .map(|_| unreachable!())
        .or_else(|e| {
            assert!(e.is_instance_of::<TypeError>());
            Ok(())
        })
}

#[wasm_bindgen_test(async)]
fn compile_valid() -> impl Future<Item = (), Error = JsValue> {
    let p = WebAssembly::compile(&get_valid_wasm());
    JsFuture::from(p)
        .map(|module| {
            assert!(module.is_instance_of::<WebAssembly::Module>());
        })
        .map_err(|_| unreachable!())
}

#[wasm_bindgen_test]
fn module_inheritance() {
    let module = WebAssembly::Module::new(&get_valid_wasm());
    assert!(module.is_instance_of::<WebAssembly::Module>());
    assert!(module.is_instance_of::<Object>());

    let _: &Object = module.as_ref();
}

#[wasm_bindgen_test]
fn module_custom_sections() {
    let module = WebAssembly::Module::new(&get_valid_wasm());
    let cust_sec = WebAssembly::Module::custom_sections(&module, "abcd").unwrap();
    assert_eq!(cust_sec.length(), 0);
}

#[wasm_bindgen_test]
fn module_exports() {
    let module = WebAssembly::Module::new(&get_valid_wasm());
    let exports = WebAssembly::Module::exports(&module).unwrap();
    assert_eq!(exports.length(), 1);
}

#[wasm_bindgen_test]
fn module_imports() {
    let module = WebAssembly::Module::new(&get_valid_wasm());
    let imports = WebAssembly::Module::imports(&module).unwrap();
    assert_eq!(imports.length(), 1);
}

#[wasm_bindgen_test]
fn table_inheritance() {
    let table = WebAssembly::Table::new(&get_table_object().into());
    assert!(table.is_instance_of::<WebAssembly::Table>());
    assert!(table.is_instance_of::<Object>());

    let _: &Object = table.as_ref();
}

#[wasm_bindgen_test]
fn table() {
    let table = WebAssembly::Table::new(&get_table_object().into());
    assert_eq!(table.length(), 1);
}

#[wasm_bindgen_test]
fn compile_error_inheritance() {
    let error = WebAssembly::CompileError::new("");
    assert!(error.is_instance_of::<WebAssembly::CompileError>());
    assert!(error.is_instance_of::<Error>());

    let _: &Error = error.as_ref();
}

#[wasm_bindgen_test]
fn link_error_inheritance() {
    let error = WebAssembly::LinkError::new("");
    assert!(error.is_instance_of::<WebAssembly::LinkError>());
    assert!(error.is_instance_of::<Error>());

    let _: &Error = error.as_ref();
}

#[wasm_bindgen_test]
fn runtime_error_inheritance() {
    let error = WebAssembly::RuntimeError::new("");
    assert!(error.is_instance_of::<WebAssembly::RuntimeError>());
    assert!(error.is_instance_of::<Error>());

    let _: &Error = error.as_ref();
}
