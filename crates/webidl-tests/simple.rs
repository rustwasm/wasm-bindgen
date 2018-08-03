use wasm_bindgen_test::*;

include!(concat!(env!("OUT_DIR"), "/simple.rs"));

#[wasm_bindgen_test]
fn method() {
    let pi = Method::new(3.14159).unwrap();
    let e = Method::new(2.71828).unwrap();
    assert!(pi.my_cmp(&pi));
    assert!(!pi.my_cmp(&e));
    assert!(!e.my_cmp(&pi));
    assert!(e.my_cmp(&e));
}

#[wasm_bindgen_test]
fn property() {
    let x = Property::new(3.14159).unwrap();
    assert_eq!(x.value(), 3.14159);
    assert_ne!(x.value(), 2.71828);
    x.set_value(2.71828);
    assert_ne!(x.value(), 3.14159);
    assert_eq!(x.value(), 2.71828);
}

#[wasm_bindgen_test]
fn named_constructor() {
    let x = NamedConstructor::new(3.14159).unwrap();
    assert_eq!(x.value(), 3.14159);
    assert_ne!(x.value(), 0.);
}

#[wasm_bindgen_test]
fn static_method() {
    assert_eq!(StaticMethod::swap(3.14159), 0.);
    assert_eq!(StaticMethod::swap(2.71828), 3.14159);
    assert_ne!(StaticMethod::swap(2.71828), 3.14159);
    assert_eq!(StaticMethod::swap(3.14159), 2.71828);
    assert_ne!(StaticMethod::swap(3.14159), 2.71828);
}

#[wasm_bindgen_test]
fn static_property() {
    assert_eq!(StaticProperty::value(), 0.);
    StaticProperty::set_value(3.14159);
    assert_eq!(StaticProperty::value(), 3.14159);
    assert_ne!(StaticProperty::value(), 2.71828);
    StaticProperty::set_value(2.71828);
    assert_eq!(StaticProperty::value(), 2.71828);
    assert_ne!(StaticProperty::value(), 3.14159);
}

#[wasm_bindgen_test]
fn one_method_using_an_undefined_import_doesnt_break_all_other_methods() {
    let f = UndefinedMethod::new().unwrap();
    assert!(f.ok_method());
}

#[wasm_bindgen_test]
fn optional_method() {
    let f = OptionalMethod::new().unwrap();
    assert!(f.opt(Some(15)) == Some(16));
    assert!(f.opt(None) == None);
}

#[wasm_bindgen_test]
fn unforgeable_is_structural() {
    let f = Unforgeable::new().unwrap();
    assert_eq!(f.uno(), 1);
    assert_eq!(f.dos(), 2);
}

#[wasm_bindgen_test]
fn partial_interface() {
    let f = PartialInterface::new().unwrap();
    assert_eq!(f.un(), 1);
    assert_eq!(f.deux(), 2);
    assert_eq!(f.trois(), 3);
    assert_eq!(f.quatre(), 4);
}

#[wasm_bindgen_test]
fn mixin() {
    let f = MixinFoo::new(1).unwrap();
    assert_eq!(f.bar(), 1);
    MixinFoo::set_default_bar(7);
    f.add_to_bar(MixinFoo::default_bar());
    assert_eq!(f.bar(), 8);
}
