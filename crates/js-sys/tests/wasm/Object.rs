use wasm_bindgen::prelude::*;
use wasm_bindgen_test::*;
use js_sys::*;

#[wasm_bindgen]
extern {
    type Foo42;
    #[wasm_bindgen(method, setter, structural)]
    fn set_foo(this: &Foo42, val: JsValue);
}

#[wasm_bindgen(module = "tests/wasm/Object.js", version = "*")]
extern {
    fn map_with_symbol_key() -> Object;
    fn symbol_key() -> JsValue;

    type Foo;
    #[wasm_bindgen(constructor)]
    fn new() -> Foo;

    #[wasm_bindgen(js_name = prototype, js_namespace = Foo)]
    static FOO_PROTOTYPE: Object;
    #[wasm_bindgen(js_name = prototype, js_namespace = Bar)]
    static BAR_PROTOTYPE: Object;
}

fn foo_42() -> Object {
    let foo = Foo42::from(JsValue::from(Object::new()));
    foo.set_foo(42.into());
    JsValue::from(foo).into()
}

#[wasm_bindgen_test]
fn new() {
    assert!(JsValue::from(Object::new()).is_object());
}

#[wasm_bindgen_test]
fn has_own_property() {
    assert!(foo_42().has_own_property(&"foo".into()));
    assert!(!foo_42().has_own_property(&"bar".into()));
    assert!(map_with_symbol_key().has_own_property(&symbol_key()));
}

#[wasm_bindgen_test]
fn to_string() {
    assert_eq!(Object::new().to_string(), "[object Object]");
    assert_eq!(foo_42().to_string(), "[object Object]");
}

#[wasm_bindgen_test]
fn is_extensible() {
    let object = Object::new();
    assert!(Object::is_extensible(&object));
    Object::prevent_extensions(&object);
    assert!(!Object::is_extensible(&object));
}

#[wasm_bindgen_test]
fn is_frozen() {
    let object = Object::new();
    assert!(!Object::is_frozen(&object));
    Object::freeze(&object);
    assert!(Object::is_frozen(&object));
}

#[wasm_bindgen_test]
fn is_sealed() {
    let object = Object::new();
    assert!(!Object::is_sealed(&object));
    Object::seal(&object);
    assert!(Object::is_sealed(&object));
}

#[wasm_bindgen_test]
fn is_prototype_of() {
    let foo = JsValue::from(Foo::new());
    assert!(FOO_PROTOTYPE.is_prototype_of(&foo));
    assert!(!BAR_PROTOTYPE.is_prototype_of(&foo));
}

#[wasm_bindgen_test]
fn keys() {
    let keys = Object::keys(&foo_42());
    assert_eq!(keys.length(), 1);
    keys.for_each(&mut |x, _, _| {
        assert_eq!(x, "foo");
    });
}

#[wasm_bindgen_test]
fn values() {
    let values = Object::values(&foo_42());
    assert_eq!(values.length(), 1);
    values.for_each(&mut |x, _, _| {
        assert_eq!(x, 42);
    });
}

#[wasm_bindgen_test]
fn property_is_enumerable() {
    assert!(foo_42().property_is_enumerable(&"foo".into()));
    assert!(!foo_42().property_is_enumerable(&42.into()));
    assert!(!Object::new().property_is_enumerable(&"foo".into()));
}

#[wasm_bindgen_test]
fn set_prototype_of() {
    let a = foo_42();
    let b = foo_42();
    Object::set_prototype_of(&a, &b);
    assert!(b.is_prototype_of(&a.into()));
}

#[wasm_bindgen_test]
fn to_locale_string() {
    assert_eq!(Object::new().to_locale_string(), "[object Object]");
}

#[wasm_bindgen_test]
fn value_of() {
    let a = JsValue::from(foo_42());
    let b = JsValue::from(foo_42());
    let a2 = JsValue::from(Object::from(a.clone()).value_of());
    assert_eq!(a, a);
    assert_eq!(a, a2);
    assert_ne!(a, b);
    assert_ne!(a2, b);
}
