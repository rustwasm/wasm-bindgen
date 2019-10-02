#[allow(unused_imports)] // test for #919
use std::borrow::BorrowMut;

use wasm_bindgen::prelude::*;
use wasm_bindgen_test::*;

#[wasm_bindgen(module = "tests/wasm/classes.js")]
extern "C" {
    fn js_simple();
    fn js_strings();
    fn js_exceptions();
    fn js_pass_one_to_another();
    fn take_class(foo: WasmType<ClassesIntoJs>);
    #[wasm_bindgen(js_name = take_class)]
    fn take_class_as_jsvalue(foo: JsValue);
    fn js_constructors();
    fn js_empty_structs();
    fn js_public_fields();
    fn js_using_self();
    fn js_readonly_fields();
    fn js_double_consume();
    fn js_js_rename();
    fn js_access_fields();
    fn js_renamed_export();
    fn js_conditional_bindings();

    fn js_assert_none(a: Option<WasmType<OptionClass>>);
    fn js_assert_some(a: Option<WasmType<OptionClass>>);
    fn js_return_none1() -> Option<WasmType<OptionClass>>;
    fn js_return_none2() -> Option<WasmType<OptionClass>>;
    fn js_return_some(a: WasmType<OptionClass>) -> Option<WasmType<OptionClass>>;
    fn js_test_option_classes();

    pub type ExportedClass;
    fn js_exported_class_inheritance();
    fn js_exported_super_constructors();
}

#[wasm_bindgen_test]
fn simple() {
    js_simple();
}

#[wasm_bindgen]
pub struct ClassesSimple {
    contents: u32,
}

#[wasm_bindgen]
impl ClassesSimple {
    pub fn new() -> WasmType<ClassesSimple> {
        ClassesSimple::with_contents(0)
    }

    #[wasm_bindgen(constructor)]
    pub fn with_contents(a: u32) -> WasmType<ClassesSimple> {
        instantiate! { ClassesSimple { contents: a } }
    }

    pub fn add(&mut self, amt: u32) -> u32 {
        self.contents += amt;
        self.contents
    }

    pub fn consume(_self: WasmType<ClassesSimple>) -> u32 {
        _self.borrow().contents
    }
}

#[wasm_bindgen_test]
fn strings() {
    js_strings()
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
    #[wasm_bindgen(constructor)]
    pub fn new() -> WasmType<ClassesStrings1> {
        instantiate! { ClassesStrings1 { name: 0 } }
    }

    pub fn set(&mut self, amt: u32) {
        self.name = amt;
    }

    pub fn bar(&self, mix: &str) -> WasmType<ClassesStrings2> {
        ClassesStrings2::new(format!("foo-{}-{}", mix, self.name))
    }
}

#[wasm_bindgen]
impl ClassesStrings2 {
    #[wasm_bindgen(constructor)]
    pub fn new(contents: String) -> WasmType<ClassesStrings2> {
        instantiate! { ClassesStrings2 { contents } }
    }

    pub fn name(&self) -> String {
        self.contents.clone()
    }
}

#[wasm_bindgen_test]
fn exceptions() {
    js_exceptions();
}

#[wasm_bindgen]
pub struct ClassesExceptions1 {}

#[wasm_bindgen]
impl ClassesExceptions1 {
    #[wasm_bindgen(constructor)]
    pub fn new() -> WasmType<ClassesExceptions1> {
        instantiate! { ClassesExceptions1 {} }
    }

    pub fn foo(&self, _: &ClassesExceptions1) {}

    pub fn bar(&mut self, _: &mut ClassesExceptions1) {}
}

#[wasm_bindgen]
pub struct ClassesExceptions2 {}

#[wasm_bindgen]
impl ClassesExceptions2 {
    #[wasm_bindgen(constructor)]
    pub fn new() -> WasmType<ClassesExceptions2> {
        instantiate! { ClassesExceptions2 {} }
    }
}

#[wasm_bindgen_test]
fn pass_one_to_another() {
    js_pass_one_to_another();
}

#[wasm_bindgen]
pub struct ClassesPassA {}

#[wasm_bindgen]
impl ClassesPassA {
    #[wasm_bindgen(constructor)]
    pub fn new() -> WasmType<ClassesPassA> {
        instantiate! { ClassesPassA {} }
    }

    pub fn foo(&self, _other: &ClassesPassB) {}

    pub fn bar(&self, _other: WasmType<ClassesPassB>) {}
}

#[wasm_bindgen]
pub struct ClassesPassB {}

#[wasm_bindgen]
impl ClassesPassB {
    #[wasm_bindgen(constructor)]
    pub fn new() -> WasmType<ClassesPassB> {
        instantiate! { ClassesPassB {} }
    }
}

#[wasm_bindgen_test]
fn pass_into_js() {
    take_class(ClassesIntoJs::new(13));
}

#[wasm_bindgen]
pub struct ClassesIntoJs(i32);

#[wasm_bindgen]
impl ClassesIntoJs {
    #[wasm_bindgen(constructor)]
    pub fn new(val: i32) -> WasmType<ClassesIntoJs> {
        instantiate! { ClassesIntoJs(val) }
    }

    pub fn inner(&self) -> i32 {
        self.0
    }
}

#[wasm_bindgen]
pub struct Issue27Context {}

#[wasm_bindgen]
impl Issue27Context {
    pub fn parse(&self, _expr: &str) -> WasmType<Issue27Expr> {
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
    take_class_as_jsvalue((&*ClassesIntoJs::new(13).borrow()).into());
}

#[wasm_bindgen_test]
fn constructors() {
    js_constructors();
}

#[wasm_bindgen]
pub fn cross_item_construction() -> WasmType<ConstructorsBar> {
    ConstructorsBar::other_name(7, 8)
}

#[wasm_bindgen]
pub struct ConstructorsFoo {
    number: u32,
}

#[wasm_bindgen]
impl ConstructorsFoo {
    #[wasm_bindgen(constructor)]
    pub fn new(number: u32) -> WasmType<ConstructorsFoo> {
        instantiate! { ConstructorsFoo { number } }
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
    pub fn other_name(number: u32, number2: u32) -> WasmType<ConstructorsBar> {
        instantiate! { ConstructorsBar { number, number2 } }
    }

    pub fn get_sum(&self) -> u32 {
        self.number + self.number2
    }
}

#[wasm_bindgen_test]
fn empty_structs() {
    js_empty_structs();
}

#[wasm_bindgen]
pub struct MissingClass {}

#[wasm_bindgen]
impl MissingClass {
    #[wasm_bindgen(constructor)]
    pub fn new() -> WasmType<MissingClass> {
        instantiate! { MissingClass {} }
    }
}

#[wasm_bindgen]
pub struct OtherEmpty {}

#[wasm_bindgen]
impl OtherEmpty {
    pub fn return_a_value() -> WasmType<MissingClass> {
        MissingClass::new()
    }
}

#[wasm_bindgen_test]
fn public_fields() {
    js_public_fields();
}

#[wasm_bindgen]
// #[derive(Default)] TODO (ae)
pub struct PublicFields {
    pub a: u32,
    pub b: f32,
    pub c: f64,
    pub d: i32,
    #[wasm_bindgen(skip)]
    pub skipped: u32,
}

#[wasm_bindgen]
impl PublicFields {
    #[wasm_bindgen(constructor)]
    pub fn new() -> WasmType<PublicFields> {
        instantiate! {
            PublicFields {
                a: Default::default(),
                b: Default::default(),
                c: Default::default(),
                d: Default::default(),
                skipped: Default::default(),
            }
        }
    }
}

#[wasm_bindgen_test]
fn using_self() {
    js_using_self();
}

#[wasm_bindgen]
pub struct UseSelf {}

#[wasm_bindgen]
impl UseSelf {
    #[wasm_bindgen(constructor)]
    pub fn new() -> WasmType<Self> {
        instantiate! { UseSelf {} }
    }
}

#[wasm_bindgen_test]
fn readonly_fields() {
    js_readonly_fields();
}

#[wasm_bindgen]
// #[derive(Default)] TODO (ae)
pub struct Readonly {
    #[wasm_bindgen(readonly)]
    pub a: u32,
}

#[wasm_bindgen]
impl Readonly {
    #[wasm_bindgen(constructor)]
    pub fn new() -> WasmType<Readonly> {
        instantiate! {
            Readonly {
                a: Default::default(),
            }
        }
    }
}

#[wasm_bindgen_test]
fn double_consume() {
    js_double_consume();
}

#[wasm_bindgen]
pub struct DoubleConsume {}

#[wasm_bindgen]
impl DoubleConsume {
    #[wasm_bindgen(constructor)]
    pub fn new() -> WasmType<DoubleConsume> {
        instantiate! { DoubleConsume {} }
    }

    pub fn consume(_self: WasmType<DoubleConsume>, other: WasmType<DoubleConsume>) {
        drop(other);
    }
}

#[wasm_bindgen_test]
fn rename_function_for_js() {
    js_js_rename();
    foo();
}

#[wasm_bindgen]
pub struct JsRename {}

#[wasm_bindgen]
impl JsRename {
    #[wasm_bindgen(constructor)]
    pub fn new() -> WasmType<JsRename> {
        let f = instantiate! { JsRename {} };
        f.borrow().foo();
        f
    }

    #[wasm_bindgen(js_name = bar)]
    pub fn foo(&self) {}
}

#[wasm_bindgen(js_name = classes_foo)]
pub fn foo() {}

#[wasm_bindgen]
pub struct AccessFieldFoo {
    pub bar: WasmType<AccessFieldBar>,
}

#[wasm_bindgen]
pub struct AccessField0(pub WasmType<AccessFieldBar>);

#[wasm_bindgen]
#[derive(Clone)]
pub struct AccessFieldBar {
    _value: u32,
}

#[wasm_bindgen]
impl AccessFieldBar {
    #[wasm_bindgen(constructor)]
    pub fn new(_value: u32) -> WasmType<AccessFieldBar> {
        instantiate! { AccessFieldBar { _value } }
    }
}

#[wasm_bindgen]
impl AccessFieldFoo {
    #[wasm_bindgen(constructor)]
    pub fn new() -> WasmType<AccessFieldFoo> {
        instantiate! {
            AccessFieldFoo {
                bar: AccessFieldBar::new(2),
            }
        }
    }
}

#[wasm_bindgen]
impl AccessField0 {
    #[wasm_bindgen(constructor)]
    pub fn new() -> WasmType<AccessField0> {
        instantiate! { AccessField0(AccessFieldBar::new(2)) }
    }
}

#[wasm_bindgen_test]
fn access_fields() {
    js_access_fields();
}

#[wasm_bindgen(js_name = JsRenamedExport)]
pub struct RenamedExport {
    pub x: u32,
}

#[wasm_bindgen(js_class = JsRenamedExport)]
impl RenamedExport {
    #[wasm_bindgen(constructor)]
    pub fn new() -> WasmType<RenamedExport> {
        instantiate! { RenamedExport { x: 3 } }
    }
    pub fn foo(&self) {}

    pub fn bar(&self, other: &RenamedExport) {
        drop(other);
    }
}

#[wasm_bindgen_test]
fn renamed_export() {
    js_renamed_export();
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub struct ConditionalBindings {}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
impl ConditionalBindings {
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(constructor))]
    pub fn new() -> WasmType<ConditionalBindings> {
        instantiate! { ConditionalBindings {} }
    }
}

#[wasm_bindgen_test]
fn conditional_bindings() {
    js_conditional_bindings();
}

#[wasm_bindgen]
pub struct OptionClass(u32);

#[wasm_bindgen]
impl OptionClass {
    #[wasm_bindgen(constructor)]
    pub fn new(value: u32) -> WasmType<OptionClass> {
        instantiate! { OptionClass(value) }
    }
}

#[wasm_bindgen_test]
fn option_class() {
    js_assert_none(None);
    js_assert_some(Some(OptionClass::new(1)));
    assert!(js_return_none1().is_none());
    assert!(js_return_none2().is_none());
    assert_eq!(js_return_some(OptionClass::new(2)).unwrap().borrow().0, 2);
    js_test_option_classes();
}

#[wasm_bindgen]
pub fn option_class_none() -> Option<WasmType<OptionClass>> {
    None
}

#[wasm_bindgen]
pub fn option_class_some() -> Option<WasmType<OptionClass>> {
    Some(OptionClass::new(3))
}

#[wasm_bindgen]
pub fn option_class_assert_none(x: Option<WasmType<OptionClass>>) {
    assert!(x.is_none());
}

#[wasm_bindgen]
pub fn option_class_assert_some(x: Option<WasmType<OptionClass>>) {
    assert_eq!(x.unwrap().borrow().0, 3);
}

mod works_in_module {
    use wasm_bindgen::prelude::wasm_bindgen;
    use wasm_bindgen::prelude::WasmType;
    #[wasm_bindgen]
    pub struct WorksInModule(u32);

    #[wasm_bindgen]
    impl WorksInModule {
        #[wasm_bindgen(constructor)]
        pub fn new() -> WasmType<WorksInModule> {
            instantiate! { WorksInModule(1) }
        }

        pub fn foo(&self) {}
    }
}

#[wasm_bindgen]
pub struct Parent(u32);

#[wasm_bindgen]
impl Parent {
    #[wasm_bindgen(constructor)]
    pub fn new() -> WasmType<Parent> {
        instantiate! { Parent(123) }
    }

    pub fn get_value(&self) -> u32 {
        self.0
    }
}

#[wasm_bindgen(prototype=Parent)]
pub struct Child {}

#[wasm_bindgen]
impl Child
{
    #[wasm_bindgen(constructor)]
    pub fn new() -> WasmType<Child> {
        instantiate! {
            super();
            Child {}
        }
    }
}

#[wasm_bindgen(prototype=ExportedClass)]
pub struct CustomImport {}

#[wasm_bindgen]
impl CustomImport {
    #[wasm_bindgen(constructor)]
    pub fn new() -> WasmType<CustomImport> {
        instantiate! {
            super("abc", 123, 3.141, JsValue::NULL, true);
            CustomImport {}
        }
    }
}

#[wasm_bindgen(prototype=js_sys::Date)]
pub struct CustomDate {}

#[wasm_bindgen]
impl CustomDate {
    #[wasm_bindgen(constructor)]
    pub fn new(date_string: &str) -> WasmType<CustomDate> {
        assert_eq!(date_string, "hello".to_string());

        instantiate! {
            super(2000, 0);
            CustomDate {}
        }
    }
}


#[wasm_bindgen_test]
fn instantiation() {
    // wasm_bindgen::WasmWrappedType::<Child>::new(JsValue::NULL);
}

#[wasm_bindgen_test]
fn inheritance() {
    js_exported_class_inheritance();
}

#[wasm_bindgen_test]
fn super_constructors() {
    js_exported_super_constructors();
}