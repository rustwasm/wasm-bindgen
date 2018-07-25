use wasm_bindgen::prelude::*;
use wasm_bindgen_test::*;
use js_sys::*;

#[wasm_bindgen]
extern {
    type GenericIterator;
    #[wasm_bindgen(method, structural)]
    fn next(this: &GenericIterator) -> IteratorNext;

    type IteratorNext;
    #[wasm_bindgen(method, structural, getter)]
    fn value(this: &IteratorNext) -> JsValue;
    #[wasm_bindgen(method, structural, getter)]
    fn done(this: &IteratorNext) -> bool;
}

#[wasm_bindgen_test]
fn entries() {
    let s = Set::new(&JsValue::undefined());
    s.add(&1.into());
    let iter = GenericIterator::from(JsValue::from(s.entries()));
    let obj = iter.next();
    assert!(!obj.done());
    let array = Array::from(&obj.value());
    assert_eq!(array.length(), 2);
    array.for_each(&mut |a, _, _| {
        assert_eq!(a, 1);
    });

    assert!(iter.next().done());
}

#[wasm_bindgen_test]
fn keys() {
    let s = Set::new(&JsValue::undefined());
    s.add(&1.into());
    let iter = GenericIterator::from(JsValue::from(s.keys()));
    let obj = iter.next();
    assert!(!obj.done());
    assert_eq!(obj.value(), 1);
    assert!(iter.next().done());
}

#[wasm_bindgen_test]
fn values() {
    let s = Set::new(&JsValue::undefined());
    s.add(&1.into());
    let iter = GenericIterator::from(JsValue::from(s.values()));
    let obj = iter.next();
    assert!(!obj.done());
    assert_eq!(obj.value(), 1);
    assert!(iter.next().done());
}
