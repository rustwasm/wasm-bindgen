use wasm_bindgen::prelude::*;
use wasm_bindgen_test::*;
use js_sys::*;

#[wasm_bindgen(module = "tests/wasm/Reflect.js")]
extern {
    fn get_char_at() -> Function;

    #[wasm_bindgen(js_name = Rectangle)]
    static RECTANGLE_CLASS: Function;
    #[wasm_bindgen(js_name = Rectangle2)]
    static RECTANGLE2_CLASS: Function;

    #[derive(Clone)]
    type Rectangle;
    #[wasm_bindgen(constructor)]
    fn new() -> Rectangle;
    #[wasm_bindgen(method, getter, structural)]
    fn x(this: &Rectangle) -> u32;
    #[wasm_bindgen(method, getter, structural, js_name = x)]
    fn x_jsval(this: &Rectangle) -> JsValue;
    #[wasm_bindgen(method, setter, structural)]
    fn set_x(this: &Rectangle, x: u32);
}

#[wasm_bindgen]
extern {
    #[wasm_bindgen(js_name = prototype, js_namespace = Object)]
    static OBJECT_PROTOTYPE: JsValue;
    #[wasm_bindgen(js_name = prototype, js_namespace = Array)]
    static ARRAY_PROTOTYPE: JsValue;

    type DefinePropertyAttrs;
    #[wasm_bindgen(method, setter, structural)]
    fn set_value(this: &DefinePropertyAttrs, val: &JsValue);

    type PropertyDescriptor;
    #[wasm_bindgen(method, getter, structural)]
    fn value(this: &PropertyDescriptor) -> JsValue;
}

#[wasm_bindgen_test]
fn apply() {
    let args = Array::new();
    args.push(&3.into());
    assert_eq!(Reflect::apply(&get_char_at(), &"ponies".into(), &args).unwrap(), "i");
}

#[wasm_bindgen_test]
fn construct() {
    let args = Array::new();
    args.push(&10.into());
    args.push(&10.into());
    let instance = Reflect::construct(&RECTANGLE_CLASS, &args);
    assert_eq!(Rectangle::from(instance).x(), 10);
}

#[wasm_bindgen_test]
fn construct_with_new_target() {
    let args = Array::new();
    args.push(&10.into());
    args.push(&10.into());
    let instance = Reflect::construct_with_new_target(
        &RECTANGLE_CLASS,
        &args,
        &RECTANGLE2_CLASS,
    );
    assert_eq!(Rectangle::from(instance).x(), 10);
}

#[wasm_bindgen_test]
fn define_property() {
    let value = DefinePropertyAttrs::from(JsValue::from(Object::new()));
    value.set_value(&42.into());
    let obj = Object::from(JsValue::from(value));
    assert!(Reflect::define_property(&obj, &"key".into(), &obj));
}

#[wasm_bindgen_test]
fn delete_property() {
    let r = Rectangle::new();
    r.set_x(10);

    let obj = Object::from(JsValue::from(r.clone()));
    Reflect::delete_property(&obj, &"x".into());
    assert!(r.x_jsval().is_undefined());

    let array = Array::new();
    array.push(&1.into());
    let obj = Object::from(JsValue::from(array));
    Reflect::delete_property(&obj, &0.into());
    let array = Array::from(&JsValue::from(obj));
    assert!(array.length() == 1);
    array.for_each(&mut |x, _, _| assert!(x.is_undefined()));
}

#[wasm_bindgen_test]
fn get() {
    let r = Rectangle::new();
    r.set_x(10);

    let obj = JsValue::from(r.clone());
    assert_eq!(Reflect::get(&obj, &"x".into()), 10);
}

#[wasm_bindgen_test]
fn get_own_property_descriptor() {
    let r = Rectangle::new();
    r.set_x(10);

    let obj = Object::from(JsValue::from(r.clone()));
    let desc = Reflect::get_own_property_descriptor(&obj, &"x".into());
    assert_eq!(PropertyDescriptor::from(desc).value(), 10);
    let desc = Reflect::get_own_property_descriptor(&obj, &"foo".into());
    assert!(PropertyDescriptor::from(desc).value().is_undefined());
}

#[wasm_bindgen_test]
fn get_prototype_of() {
    let proto = JsValue::from(Reflect::get_prototype_of(&Object::new().into()));
    assert_eq!(proto, *OBJECT_PROTOTYPE);
    let proto = JsValue::from(Reflect::get_prototype_of(&Array::new().into()));
    assert_eq!(proto, *ARRAY_PROTOTYPE);
}

#[wasm_bindgen_test]
fn has() {
    let obj = JsValue::from(Rectangle::new());
    assert!(Reflect::has(&obj, &"x".into()));
    assert!(!Reflect::has(&obj, &"foo".into()));
}

#[wasm_bindgen_test]
fn is_extensible() {
    let obj = Object::from(JsValue::from(Rectangle::new()));
    assert!(Reflect::is_extensible(&obj));
    Reflect::prevent_extensions(&obj);
    assert!(!Reflect::is_extensible(&obj));
    let obj = Object::seal(&Object::new());
    assert!(!Reflect::is_extensible(&obj));
}

#[wasm_bindgen_test]
fn own_keys() {
    let obj = JsValue::from(Rectangle::new());
    let keys = Reflect::own_keys(&obj);
    assert!(keys.length() == 2);
    keys.for_each(&mut |k, _, _| {
        assert!(k == "x" || k == "y");
    });
}

#[wasm_bindgen_test]
fn prevent_extensions() {
    let obj = Object::new();
    Reflect::prevent_extensions(&obj);
    assert!(!Reflect::is_extensible(&obj));
}

#[wasm_bindgen_test]
fn set() {
    let obj = JsValue::from(Object::new());
    assert!(Reflect::set(&obj, &"key".into(), &"value".into()));
    assert_eq!(Reflect::get(&obj, &"key".into()), "value");
}

#[wasm_bindgen_test]
fn set_with_receiver() {
    let obj1 = JsValue::from(Object::new());
    let obj2 = JsValue::from(Object::new());
    assert!(Reflect::set_with_receiver(&obj2, &"key".into(), &"value".into(), &obj2));
    assert!(Reflect::get(&obj1, &"key".into()).is_undefined());
    assert_eq!(Reflect::get(&obj2, &"key".into()), "value");
}

#[wasm_bindgen_test]
fn set_prototype_of() {
    let obj = Object::new();
    assert!(Reflect::set_prototype_of(&obj, &JsValue::null()));
    let obj = JsValue::from(obj);
    assert_eq!(JsValue::from(Reflect::get_prototype_of(&obj)), JsValue::null());
}
