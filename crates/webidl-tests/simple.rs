use crate::generated::*;
use js_sys::Object;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_test::*;

#[wasm_bindgen_test]
fn interfaces_inherit_from_object() {
    let m = Method::new(42.0).unwrap();
    assert!(m.is_instance_of::<Object>());
    let _: &Object = m.as_ref();
}

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
fn one_method_with_an_undefined_import_doesnt_break_all_other_methods() {
    let f = UndefinedMethod::new().unwrap();
    assert!(f.ok_method());
}

#[wasm_bindgen_test]
fn nullable_method() {
    let f = NullableMethod::new().unwrap();
    assert!(f.opt(Some(15)) == Some(16));
    assert!(f.opt(None) == None);
}

#[wasm_bindgen]
extern "C" {
    fn get_global_method() -> GlobalMethod;
}

#[wasm_bindgen_test]
fn global_method() {
    let x = get_global_method();
    assert_eq!(x.m(), 123);
}

#[wasm_bindgen_test]
fn indexing() {
    let f = Indexing::new().unwrap();
    assert_eq!(f.get(123).unwrap(), -1);
    f.set(123, 456);
    assert_eq!(f.get(123).unwrap(), 456);
    f.delete(123);
    assert_eq!(f.get(123).unwrap(), -1);
}

#[wasm_bindgen_test]
fn optional_and_union_arguments() {
    let f = OptionalAndUnionArguments::new().unwrap();
    assert_eq!(
        f.m("abc"),
        "string, abc, boolean, true, number, 123, number, 456"
    );
    assert_eq!(
        f.m_with_b("abc", false),
        "string, abc, boolean, false, number, 123, number, 456"
    );
    assert_eq!(
        f.m_with_b_and_i16("abc", false, 5),
        "string, abc, boolean, false, number, 5, number, 456"
    );
    assert_eq!(
        f.m_with_b_and_str("abc", false, "5"),
        "string, abc, boolean, false, string, 5, number, 456"
    );
    assert_eq!(
        f.m_with_b_and_i16_and_opt_i32("abc", false, 5, Some(10)),
        "string, abc, boolean, false, number, 5, number, 10"
    );
    assert_eq!(
        f.m_with_b_and_i16_and_opt_f64("abc", false, 5, Some(10.0)),
        "string, abc, boolean, false, number, 5, number, 10"
    );
    assert_eq!(
        f.m_with_b_and_i16_and_opt_bool("abc", false, 5, Some(true)),
        "string, abc, boolean, false, number, 5, boolean, true"
    );
    assert_eq!(
        f.m_with_b_and_str_and_opt_i32("abc", false, "5", Some(10)),
        "string, abc, boolean, false, string, 5, number, 10"
    );
    assert_eq!(
        f.m_with_b_and_str_and_opt_f64("abc", false, "5", Some(12.0)),
        "string, abc, boolean, false, string, 5, number, 12"
    );
    assert_eq!(
        f.m_with_b_and_str_and_opt_bool("abc", false, "5", Some(true)),
        "string, abc, boolean, false, string, 5, boolean, true"
    );
}

#[wasm_bindgen_test]
fn variadic() {
    let f = Variadic::new().unwrap();
    assert_eq!(f.sum_5(1, 2, 3, 4, 5), 15);
    assert_eq!(
        f.sum(&::js_sys::Array::of5(
            &JsValue::from_f64(1f64),
            &JsValue::from_f64(2f64),
            &JsValue::from_f64(3f64),
            &JsValue::from_f64(4f64),
            &JsValue::from_f64(5f64),
        )),
        15
    );
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

#[wasm_bindgen_test]
fn overload_naming() {
    let o = Overloads::new().unwrap();
    o.foo();
    o.foo_with_arg("x");
    o.foo_with_arg_and_i32("x", 3);
    o.foo_with_arg_and_f32("x", 2.0);
    o.foo_with_arg_and_i16("x", 5);
}

#[wasm_bindgen_test]
fn callback() {
    let o = InvokeCallback::new().unwrap();
    {
        static mut HIT: bool = false;
        let cb = Closure::wrap(Box::new(move || unsafe {
            HIT = true;
        }) as Box<dyn FnMut()>);
        o.invoke(cb.as_ref().unchecked_ref());
        assert!(unsafe { HIT });
    }

    let cb = Closure::wrap(Box::new(move |a, b| a + b) as Box<dyn FnMut(u32, u32) -> u32>);
    assert_eq!(o.call_add(cb.as_ref().unchecked_ref()), 3);

    let cb = Closure::wrap(
        Box::new(move |a: String, b| a.repeat(b)) as Box<dyn FnMut(String, usize) -> String>
    );
    assert_eq!(o.call_repeat(cb.as_ref().unchecked_ref()), "abababab");
}
