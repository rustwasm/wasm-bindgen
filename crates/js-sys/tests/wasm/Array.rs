use js_sys::*;
use std::iter::FromIterator;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use wasm_bindgen_test::*;

macro_rules! js_array {
    ($($e:expr),*) => ({
        let __x = Array::new();
        $(__x.push(&JsValue::from($e));)*
        __x
    })
}

macro_rules! array {
    ($($e:expr),*) => ({
        let mut __x = Vec::new();
        $(__x.push(JsValue::from($e));)*
        __x
    })
}

fn to_rust(arr: &Array) -> Vec<JsValue> {
    let mut result = Vec::with_capacity(arr.length() as usize);
    arr.for_each(&mut |x, _, _| result.push(x));
    result
}

#[wasm_bindgen_test]
fn from_iter() {
    assert_eq!(
        to_rust(&vec![
            JsValue::from("a"),
            JsValue::from("b"),
            JsValue::from("c"),
        ].into_iter().collect()),
        vec!["a", "b", "c"],
    );

    assert_eq!(
        to_rust(&vec![
            JsValue::from("a"),
            JsValue::from("b"),
            JsValue::from("c"),
        ].iter().collect()),
        vec!["a", "b", "c"],
    );

    let array = js_array![1u32, 2u32, 3u32];

    assert_eq!(
        to_rust(&vec![
            array.clone(),
        ].into_iter().collect()),
        vec![JsValue::from(array.clone())],
    );

    assert_eq!(
        to_rust(&vec![
            array.clone(),
        ].iter().collect()),
        vec![JsValue::from(array)],
    );

    assert_eq!(
        to_rust(&vec![
            5,
            10,
            20,
        ].into_iter().map(JsValue::from).collect()),
        vec![5, 10, 20],
    );

    assert_eq!(
        to_rust(&Array::from_iter(&[
            JsValue::from("a"),
            JsValue::from("b"),
            JsValue::from("c"),
        ])),
        vec!["a", "b", "c"],
    );

    let v = vec![
        "a",
        "b",
        "c",
    ];

    assert_eq!(
        to_rust(&Array::from_iter(v.into_iter().map(|s| JsValue::from(s)))),
        vec!["a", "b", "c"],
    );
}

#[wasm_bindgen_test]
fn new_with_length() {
    let array = Array::new_with_length(5);
    assert_eq!(array.length(), 5);
    assert_eq!(array.get(4), JsValue::undefined());
    array.set(4, JsValue::from("a"));
    assert_eq!(array.get(4), "a");
    assert_eq!(array.length(), 5);
}

#[wasm_bindgen_test]
fn get() {
    let array = js_array!["a", "c", "x", "n"];
    assert_eq!(array.length(), 4);
    assert_eq!(array.get(0), "a");
    assert_eq!(array.get(3), "n");
    assert_eq!(array.get(4), JsValue::undefined());
}

#[wasm_bindgen_test]
fn set() {
    let array = js_array!["a", "c", "x", "n"];
    assert_eq!(array.length(), 4);
    assert_eq!(array.get(0), "a");
    array.set(0, JsValue::from("b"));
    assert_eq!(array.get(0), "b");

    assert_eq!(array.get(4), JsValue::undefined());
    assert_eq!(array.length(), 4);
    array.set(4, JsValue::from("d"));
    assert_eq!(array.length(), 5);
    assert_eq!(array.get(4), "d");

    assert_eq!(array.get(10), JsValue::undefined());
    assert_eq!(array.length(), 5);
    array.set(10, JsValue::from("z"));
    assert_eq!(array.length(), 11);
    assert_eq!(array.get(10), "z");
    assert_eq!(array.get(9), JsValue::undefined());
}

#[wasm_bindgen_test]
fn delete() {
    let array = js_array!["a", "c", "x", "n"];
    assert_eq!(array.length(), 4);
    assert_eq!(array.get(0), "a");
    array.delete(0);
    assert_eq!(array.get(0), JsValue::undefined());
}

#[wasm_bindgen_test]
fn filter() {
    let array = js_array!["a", "c", "x", "n"];
    assert!(array.filter(&mut |x, _, _| x.as_f64().is_some()).length() == 0);

    let array = js_array![1, 2, 3, 4];
    assert_eq!(
        array.filter(&mut |x, _, _| x.as_f64().is_some()).length(),
        4
    );

    let array = js_array!["a", 1, "b", 2];
    assert_eq!(
        array.filter(&mut |x, _, _| x.as_f64().is_some()).length(),
        2
    );
}

#[wasm_bindgen_test]
fn flat() {
    let array = js_array![
        js_array!["a", "b", "c"],
        "d",
        js_array!["e", js_array!["f", "g"]]
    ];

    assert_eq!(
        to_rust(&array.flat(1).slice(0, 5)),
        vec!["a", "b", "c", "d", "e"]
    );

    assert_eq!(array.flat(1).length(), 6);

    assert_eq!(
        to_rust(&array.flat(2)),
        vec!["a", "b", "c", "d", "e", "f", "g"]
    );
}

#[wasm_bindgen_test]
fn flat_map() {
    let array = js_array![1, 2, 3, 1];

    assert_eq!(
        to_rust(
            &array.flat_map(&mut |val, _, _| match val.as_f64().map(|v| v as i32) {
                Some(1) => vec![JsString::from("x").into(), JsString::from("x").into()],
                Some(2) => vec![],
                Some(3) => vec![JsString::from("z").into()],
                _ => panic!("Unexpected conversion"),
            })
        ),
        vec!["x", "x", "z", "x", "x"]
    );
}

#[wasm_bindgen_test]
fn index_of() {
    let chars = js_array!["a", "c", "x", "n"];
    assert_eq!(chars.index_of(&"x".into(), 0), 2);
    assert_eq!(chars.index_of(&"z".into(), 0), -1);
    assert_eq!(chars.index_of(&"x".into(), -3), 2);
    assert_eq!(chars.index_of(&"z".into(), -2), -1);
}

#[wasm_bindgen_test]
fn is_array() {
    assert!(Array::is_array(&Array::new().into()));
    assert!(Array::is_array(&js_array![1].into()));
    assert!(!Array::is_array(&JsValue::null()));
    assert!(!Array::is_array(&JsValue::undefined()));
    assert!(!Array::is_array(&10.into()));
    assert!(!Array::is_array(&"x".into()));
    assert!(!Array::is_array(&true.into()));
    assert!(!Array::is_array(&false.into()));
}

#[wasm_bindgen_test]
fn sort() {
    let array = js_array![3, 1, 6, 2];
    let sorted = array.sort();
    assert_eq!(to_rust(&sorted), array![1, 2, 3, 6]);
}

#[wasm_bindgen_test]
fn some() {
    let array = js_array!["z", 1, "y", 2];
    assert!(array.some(&mut |e| e == JsValue::from(2)));
    assert!(array.some(&mut |e| e == JsValue::from("y")));
    assert!(!array.some(&mut |e| e == JsValue::from("nope")));
}

#[wasm_bindgen_test]
fn last_index_of() {
    let characters = js_array!["a", "x", "c", "x", "n"];
    assert_eq!(characters.last_index_of(&"x".into(), 5), 3);
    assert_eq!(characters.last_index_of(&"z".into(), 5), -1);
    assert_eq!(characters.last_index_of(&"x".into(), 2), 1);
    assert_eq!(characters.last_index_of(&"x".into(), 0), -1);
}

#[wasm_bindgen_test]
fn join() {
    let characters = js_array!["a", "c", "x", "n"];
    assert_eq!(String::from(characters.join(", ")), "a, c, x, n");
    assert_eq!(String::from(characters.join("/")), "a/c/x/n");
}

#[wasm_bindgen_test]
fn slice() {
    let characters = js_array!["a", "c", "x", "n", 1, "8"];
    let subset = characters.slice(1, 3);

    assert_eq!(to_rust(&subset), array!["c", "x"]);
}

#[wasm_bindgen_test]
fn splice() {
    let characters = js_array!["a", "c", "x", "n", 1, "8"];
    let removed = characters.splice(1, 3, &"b".into());

    assert_eq!(to_rust(&removed), array!["c", "x", "n"]);
    assert_eq!(to_rust(&characters), array!["a", "b", 1, "8"]);
}

#[wasm_bindgen_test]
fn fill() {
    let characters = js_array!["a", "c", "x", "n", 1, "8"];
    let subset = characters.fill(&0.into(), 0, 3);

    assert_eq!(to_rust(&subset), array![0, 0, 0, "n", 1, "8"]);
}

#[wasm_bindgen_test]
fn copy_within() {
    let characters = js_array![8, 5, 4, 3, 1, 2];
    characters.copy_within(1, 4, 5);

    assert_eq!(to_rust(&characters)[1], JsValue::from(1));

    // if negatives were used
    characters.copy_within(-1, -3, -2);
    assert_eq!(to_rust(&characters)[5], JsValue::from(3));
}

#[wasm_bindgen_test]
fn of() {
    let a = JsValue::from("a");
    let b = JsValue::from("b");
    let c = JsValue::from("c");
    let arr = Array::of3(&a, &b, &c);
    let vec = to_rust(&arr);
    assert_eq!(vec.len(), 3);
    assert_eq!(vec[0], a);
    assert_eq!(vec[1], b);
    assert_eq!(vec[2], c);
}

#[wasm_bindgen_test]
fn pop() {
    let characters = js_array![8, 5, 4, 3, 1, 2];
    let item = characters.pop();
    assert_eq!(item, JsValue::from(2));
    assert_eq!(characters.length(), 5);
}

#[wasm_bindgen_test]
fn push() {
    let characters = js_array![8, 5, 4, 3, 1, 2];
    let length = characters.push(&"a".into());
    assert_eq!(length, 7);
    assert_eq!(to_rust(&characters)[6], "a");
}

#[wasm_bindgen_test]
fn reverse() {
    let characters = js_array![8, 5, 4, 3, 1, 2];
    let reversed = characters.reverse();
    assert_eq!(to_rust(&reversed), array![2, 1, 3, 4, 5, 8]);
}

#[wasm_bindgen_test]
fn shift() {
    let characters = js_array![8, 5, 4, 3, 1, 2];
    let shiftedItem = characters.shift();

    assert_eq!(shiftedItem, 8);
    assert_eq!(characters.length(), 5);
}

#[wasm_bindgen_test]
fn unshift() {
    let characters = js_array![8, 5, 4, 3, 1, 2];
    let length = characters.unshift(&"abba".into());

    assert_eq!(length, 7);
    assert_eq!(to_rust(&characters)[0], "abba");
}

#[wasm_bindgen_test]
fn to_string() {
    let characters = js_array![8, 5, 4, 3, 1, 2];
    assert_eq!(String::from(characters.to_string()), "8,5,4,3,1,2");
}

#[wasm_bindgen_test]
fn includes() {
    let characters = js_array![8, 5, 4, 3, 1, 2];
    assert!(characters.includes(&2.into(), 0));
    assert!(!characters.includes(&9.into(), 0));
    assert!(!characters.includes(&3.into(), 4));
}

#[wasm_bindgen_test]
fn concat() {
    let arr1 = js_array![1, 2, 3];
    let arr2 = js_array![4, 5, 6];

    let new_array = arr1.concat(&arr2);
    assert_eq!(to_rust(&new_array), array![1, 2, 3, 4, 5, 6]);
}

#[wasm_bindgen_test]
fn length() {
    let characters = js_array![8, 5, 4, 3, 1, 2];
    assert_eq!(characters.length(), 6);
    assert_eq!(Array::new().length(), 0);
}

#[wasm_bindgen_test]
fn every() {
    let even = js_array![2, 4, 6, 8];
    assert!(even.every(&mut |x, _, _| x.as_f64().unwrap() % 2.0 == 0.0));
    let odd = js_array![1, 3, 5, 7];
    assert!(!odd.every(&mut |x, _, _| x.as_f64().unwrap() % 2.0 == 0.0));
    let mixed = js_array![2, 3, 4, 5];
    assert!(!mixed.every(&mut |x, _, _| x.as_f64().unwrap() % 2.0 == 0.0));
}

#[wasm_bindgen_test]
fn find() {
    let even = js_array![2, 4, 6, 8];
    assert_eq!(
        even.find(&mut |x, _, _| x.as_f64().unwrap() % 2.0 == 0.0),
        2
    );
    let odd = js_array![1, 3, 5, 7];
    assert_eq!(
        odd.find(&mut |x, _, _| x.as_f64().unwrap() % 2.0 == 0.0),
        JsValue::undefined(),
    );
    let mixed = js_array![3, 5, 7, 10];
    assert_eq!(
        mixed.find(&mut |x, _, _| x.as_f64().unwrap() % 2.0 == 0.0),
        10
    );
}

#[wasm_bindgen_test]
fn map() {
    let numbers = js_array![1, 4, 9];
    let sqrt = numbers.map(&mut |x, _, _| x.as_f64().unwrap().sqrt().into());
    assert_eq!(to_rust(&sqrt), array![1, 2, 3]);
}

#[wasm_bindgen_test]
fn reduce() {
    let arr = js_array!["0", "1", "2", "3", "4"].reduce(
        &mut |ac, cr, _, _| {
            format!("{}{}", &ac.as_string().unwrap(), &cr.as_string().unwrap()).into()
        },
        &"".into(),
    );
    assert_eq!(arr, "01234");
}

#[wasm_bindgen_test]
fn reduce_right() {
    let arr = js_array!["0", "1", "2", "3", "4"].reduce_right(
        &mut |ac, cr, _, _| {
            format!("{}{}", &ac.as_string().unwrap(), &cr.as_string().unwrap()).into()
        },
        &"".into(),
    );
    assert_eq!(arr, "43210");
}

#[wasm_bindgen_test]
fn find_index() {
    let even = js_array![2, 4, 6, 8];
    assert_eq!(
        even.find_index(&mut |e, _, _| e.as_f64().unwrap() % 2. == 0.),
        0
    );
    let odd = js_array![1, 3, 5, 7];
    assert_eq!(
        odd.find_index(&mut |e, _, _| e.as_f64().unwrap() % 2. == 0.),
        -1
    );
    let mixed = js_array![3, 5, 7, 10];
    assert_eq!(
        mixed.find_index(&mut |e, _, _| e.as_f64().unwrap() % 2. == 0.),
        3
    );
}

#[wasm_bindgen_test]
fn to_locale_string() {
    let output = js_array![1, "a", Date::new(&"21 Dec 1997 14:12:00 UTC".into())]
        .to_locale_string(&"en".into(), &JsValue::undefined());
    assert!(String::from(output).len() > 0);
}

#[wasm_bindgen_test]
fn for_each() {
    fn sum_indices_of_evens(array: &Array) -> u32 {
        let mut res = 0;
        array.for_each(&mut |elem: JsValue, i, _| match elem.as_f64() {
            Some(val) if val % 2. == 0. => res += i,
            _ => {}
        });
        res
    }

    assert_eq!(sum_indices_of_evens(&js_array![2, 4, 6, 8]), 0 + 1 + 2 + 3);
    assert_eq!(sum_indices_of_evens(&js_array![1, 3, 5, 7]), 0);
    assert_eq!(sum_indices_of_evens(&js_array![3, 5, 7, 10]), 3);
}

#[wasm_bindgen_test]
fn array_inheritance() {
    let array = Array::new();
    assert!(array.is_instance_of::<Array>());
    assert!(array.is_instance_of::<Object>());
    let _: &Object = array.as_ref();
}
