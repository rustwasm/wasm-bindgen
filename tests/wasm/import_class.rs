//! dox

#![deny(missing_docs)] // test that documenting public bindings is enough

use wasm_bindgen_test::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "tests/wasm/import_class.js", version = "*")]
extern {
    fn math_log(f: f64) -> f64;

    #[wasm_bindgen(js_namespace = StaticFunction)]
    fn bar() -> u32;

    #[derive(Clone)]
    type Construct;
    #[wasm_bindgen(js_namespace = Construct)]
    fn create() -> Construct;
    #[wasm_bindgen(method)]
    fn get_internal_string(this: &Construct) -> String;
    #[wasm_bindgen(method)]
    fn append_to_internal_string(this: &Construct, s: &str);
    #[wasm_bindgen(method)]
    fn assert_internal_string(this: &Construct, s: &str);

    type NewConstructors;
    #[wasm_bindgen(constructor)]
    fn new(arg: i32) -> NewConstructors;
    #[wasm_bindgen(method)]
    fn get(this: &NewConstructors) -> i32;

    fn switch_methods_a();
    fn switch_methods_b();
    type SwitchMethods;
    #[wasm_bindgen(constructor)]
    fn new() -> SwitchMethods;
    #[wasm_bindgen(js_namespace = SwitchMethods)]
    fn a();
    fn switch_methods_called() -> bool;
    #[wasm_bindgen(method)]
    fn b(this: &SwitchMethods);

    type Properties;
    #[wasm_bindgen(constructor)]
    fn new() -> Properties;
    #[wasm_bindgen(getter, method)]
    fn a(this: &Properties) -> i32;
    #[wasm_bindgen(setter, method)]
    fn set_a(this: &Properties, a: i32);

    type RenameProperties;
    #[wasm_bindgen(constructor)]
    fn new() -> RenameProperties;
    #[wasm_bindgen(getter = a, method)]
    fn test(this: &RenameProperties) -> i32;
    #[wasm_bindgen(setter = a, method)]
    fn another(this: &RenameProperties, a: i32);

    /// dox
    pub type AssertImportDenyDocsWorks;
    /// dox
    #[wasm_bindgen(constructor)]
    pub fn new() -> AssertImportDenyDocsWorks;
    /// dox
    #[wasm_bindgen(getter = a, method)]
    pub fn test(this: &AssertImportDenyDocsWorks) -> i32;
    /// dox
    pub fn foo();

    pub type Options;
    #[wasm_bindgen(constructor)]
    fn new() -> Options;
    fn take_none(val: Option<Options>);
    fn take_some(val: Option<Options>);
    fn return_null() -> Option<Options>;
    fn return_undefined() -> Option<Options>;
    fn return_some() -> Option<Options>;
    fn run_rust_option_tests();
}

#[wasm_bindgen]
extern {
    #[wasm_bindgen(js_namespace = Math)]
    fn random() -> f64;
    #[wasm_bindgen(js_namespace = Math)]
    fn log(a: f64) -> f64;
}

#[wasm_bindgen_test]
fn simple() {
    random();
    assert_eq!(log(1.0), math_log(1.0));
}

#[wasm_bindgen_test]
fn import_class() {
    assert_eq!(bar(), 2);
}

#[wasm_bindgen_test]
fn construct() {
    let f = Construct::create();
    assert_eq!(f.get_internal_string(), "this");
    assert_eq!(f.clone().get_internal_string(), "this");
    f.append_to_internal_string(" foo");
    f.assert_internal_string("this foo");
}

#[wasm_bindgen_test]
fn new_constructors() {
    let f = NewConstructors::new(1);
    assert_eq!(f.get(), 2);
}

#[wasm_bindgen_test]
fn switch_methods() {
    assert!(!switch_methods_called());
    SwitchMethods::a();
    assert!(switch_methods_called());

    switch_methods_a();

    assert!(!switch_methods_called());
    SwitchMethods::a();
    assert!(switch_methods_called());

    assert!(!switch_methods_called());
    SwitchMethods::new().b();
    assert!(switch_methods_called());

    switch_methods_a();

    assert!(!switch_methods_called());
    SwitchMethods::new().b();
    assert!(switch_methods_called());
}

#[wasm_bindgen_test]
fn properties() {
    let a = Properties::new();
    assert_eq!(a.a(), 1);
    a.set_a(2);
    assert_eq!(a.a(), 2);
}

#[wasm_bindgen_test]
fn rename_setter_getter() {
    let x: fn() -> RenameProperties = RenameProperties::new;
    let a = x();
    assert_eq!(a.test(), 1);
    a.another(2);
    assert_eq!(a.test(), 2);
}

/// dox
#[wasm_bindgen]
pub struct AssertDenyDocsWorks {
    /// dox
    pub a: u32,
    _b: i64,
}

/// dox
#[wasm_bindgen]
pub fn assert_deny_docs_works() {
}


#[wasm_bindgen_test]
fn options() {
    take_none(None);
    take_some(Some(Options::new()));
    assert!(return_null().is_none());
    assert!(return_undefined().is_none());
    assert!(return_some().is_some());
    run_rust_option_tests();
}

/// doc
#[wasm_bindgen]
pub fn rust_take_none(a: Option<Options>) {
    assert!(a.is_none());
}

/// doc
#[wasm_bindgen]
pub fn rust_take_some(a: Option<Options>) {
    assert!(a.is_some());
}

/// doc
#[wasm_bindgen]
pub fn rust_return_none() -> Option<Options> {
    None
}

/// doc
#[wasm_bindgen]
pub fn rust_return_some() -> Option<Options> {
    Some(Options::new())
}
