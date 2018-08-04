use wasm_bindgen_test::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "tests/wasm/classes.js", version = "*")]
extern {
    fn test_simple();
    fn test_strings();
    fn test_exceptions();
    fn test_pass_one_to_another();
    fn take_class(foo: ClassesIntoJs);
    #[wasm_bindgen(js_name = take_class)]
    fn take_class_as_jsvalue(foo: JsValue);
    fn test_constructors();
    fn test_empty_structs();
    fn test_public_fields();
    fn test_using_self();
    fn test_readonly_fields();
    fn test_double_consume();
    fn test_js_rename();
}

#[wasm_bindgen_test]
fn simple() {
    test_simple();
}

#[wasm_bindgen]
pub struct ClassesSimple {
    contents: u32,
}

#[wasm_bindgen]
impl ClassesSimple {
    #[wasm_bindgen(constructor)]
    pub fn new() -> ClassesSimple {
        ClassesSimple::with_contents(0)
    }

    pub fn with_contents(a: u32) -> ClassesSimple {
        ClassesSimple { contents: a }
    }

    pub fn add(&mut self, amt: u32) -> u32 {
        self.contents += amt;
        self.contents
    }

    pub fn consume(self) -> u32 {
        self.contents
    }
}

#[wasm_bindgen_test]
fn strings() {
    test_strings()
}

#[wasm_bindgen]
pub struct ClassesStrings1 {
    name: u32,
}

#[wasm_bindgen]
pub struct ClassesStrings2 {
    contents: String,
}

#[wasm_bindgen]
impl ClassesStrings1 {
    pub fn new() -> ClassesStrings1 {
        ClassesStrings1 { name: 0 }
    }

    pub fn set(&mut self, amt: u32) {
        self.name = amt;
    }

    pub fn bar(&self, mix: &str) -> ClassesStrings2 {
        ClassesStrings2 { contents: format!("foo-{}-{}", mix, self.name) }
    }
}

#[wasm_bindgen]
impl ClassesStrings2 {
    pub fn name(&self) -> String {
        self.contents.clone()
    }
}

#[wasm_bindgen_test]
fn exceptions() {
    test_exceptions();
}
#[wasm_bindgen]
pub struct ClassesExceptions1 {
}

#[wasm_bindgen]
impl ClassesExceptions1 {
    pub fn new() -> ClassesExceptions1 {
        ClassesExceptions1 {}
    }

    pub fn foo(&self, _: &ClassesExceptions1) {
    }

    pub fn bar(&mut self, _: &mut ClassesExceptions1) {
    }
}

#[wasm_bindgen]
pub struct ClassesExceptions2 {
}

#[wasm_bindgen]
impl ClassesExceptions2 {
    pub fn new() -> ClassesExceptions2 {
        ClassesExceptions2 {}
    }
}

#[wasm_bindgen_test]
fn pass_one_to_another() {
    test_pass_one_to_another();
}

#[wasm_bindgen]
pub struct ClassesPassA {}

#[wasm_bindgen]
impl ClassesPassA {
    pub fn new() -> ClassesPassA {
        ClassesPassA {}
    }

    pub fn foo(&self, _other: &ClassesPassB) {
    }

    pub fn bar(&self, _other: ClassesPassB) {
    }
}

#[wasm_bindgen]
pub struct ClassesPassB {}

#[wasm_bindgen]
impl ClassesPassB {
    pub fn new() -> ClassesPassB {
        ClassesPassB {}
    }
}

#[wasm_bindgen_test]
fn pass_into_js() {
    take_class(ClassesIntoJs(13));
}

#[wasm_bindgen]
pub struct ClassesIntoJs(i32);

#[wasm_bindgen]
impl ClassesIntoJs {
    pub fn inner(&self) -> i32 {
        self.0
    }
}

#[wasm_bindgen]
pub struct Issue27Context {}

#[wasm_bindgen]
impl Issue27Context {
    pub fn parse(&self, _expr: &str) -> Issue27Expr {
        panic!()
    }
    pub fn eval(&self, _expr: &Issue27Expr) -> f64 {
        panic!()
    }
    pub fn set(&mut self, _var: &str, _val: f64) {
        panic!()
    }
}

#[wasm_bindgen]
pub struct Issue27Expr {}

#[wasm_bindgen_test]
fn pass_into_js_as_js_class() {
    take_class_as_jsvalue(ClassesIntoJs(13).into());
}

#[wasm_bindgen_test]
fn constructors() {
    test_constructors();
}

#[wasm_bindgen]
pub fn cross_item_construction() -> ConstructorsBar {
    ConstructorsBar::other_name(7, 8)
}

#[wasm_bindgen]
pub struct ConstructorsFoo {
    number: u32,
}

#[wasm_bindgen]
impl ConstructorsFoo {
    #[wasm_bindgen(constructor)]
    pub fn new(number: u32) -> ConstructorsFoo {
        ConstructorsFoo { number }
    }

    pub fn get_number(&self) -> u32 {
        self.number
    }
}

#[wasm_bindgen]
pub struct ConstructorsBar {
    number: u32,
    number2: u32,
}

#[wasm_bindgen]
impl ConstructorsBar {
    #[wasm_bindgen(constructor)]
    pub fn other_name(number: u32, number2: u32) -> ConstructorsBar {
        ConstructorsBar { number, number2 }
    }

    pub fn get_sum(&self) -> u32 {
        self.number + self.number2
    }
}

#[wasm_bindgen_test]
fn empty_structs() {
    test_empty_structs();
}

#[wasm_bindgen]
pub struct MissingClass {}

#[wasm_bindgen]
pub struct OtherEmpty {}

#[wasm_bindgen]
impl OtherEmpty {
    pub fn return_a_value() -> MissingClass { MissingClass {} }
}

#[wasm_bindgen_test]
fn public_fields() {
    test_public_fields();
}

#[wasm_bindgen]
#[derive(Default)]
pub struct PublicFields {
    pub a: u32,
    pub b: f32,
    pub c: f64,
    pub d: i32,
}

#[wasm_bindgen]
impl PublicFields {
    pub fn new() -> PublicFields {
        PublicFields::default()
    }
}

#[wasm_bindgen_test]
fn using_self() {
    test_using_self();
}

#[wasm_bindgen]
pub struct UseSelf {
}

#[wasm_bindgen]
impl UseSelf {
    pub fn new() -> Self {
        UseSelf {}
    }
}

#[wasm_bindgen_test]
fn readonly_fields() {
    test_readonly_fields();
}

#[wasm_bindgen]
#[derive(Default)]
pub struct Readonly {
    #[wasm_bindgen(readonly)]
    pub a: u32,
}

#[wasm_bindgen]
impl Readonly {
    pub fn new() -> Readonly {
        Readonly::default()
    }
}

#[wasm_bindgen_test]
fn double_consume() {
    test_double_consume();
}

#[wasm_bindgen]
pub struct DoubleConsume { }

#[wasm_bindgen]
impl DoubleConsume {
    #[wasm_bindgen(constructor)]
    pub fn new() -> DoubleConsume {
        DoubleConsume {}
    }

    pub fn consume(self, other: DoubleConsume) {
        drop(other);
    }
}

#[wasm_bindgen_test]
fn rename_function_for_js() {
    test_js_rename();
    foo();
}

#[wasm_bindgen]
pub struct JsRename { }

#[wasm_bindgen]
impl JsRename {
    #[wasm_bindgen(constructor)]
    pub fn new() -> JsRename {
        let f = JsRename {};
        f.foo();
        return f
    }

    #[wasm_bindgen(js_name = bar)]
    pub fn foo(&self) {
    }

}

#[wasm_bindgen(js_name = classes_foo)]
pub fn foo() {}
