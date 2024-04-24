use crate::generated::{CamelCaseMe, ManyTypes, OtherDict, PreserveNames, Required, A, B, C};
use wasm_bindgen::prelude::*;
use wasm_bindgen_test::*;

#[wasm_bindgen]
extern "C" {
    fn assert_dict_c(c: &C);
    #[wasm_bindgen(js_name = assert_dict_c)]
    fn assert_dict_c2(c: C);
    #[wasm_bindgen(js_name = assert_dict_c)]
    fn assert_dict_c3(c: Option<&C>);
    #[wasm_bindgen(js_name = assert_dict_c)]
    fn assert_dict_c4(c: Option<C>);
    fn mk_dict_a() -> A;
    #[wasm_bindgen(js_name = mk_dict_a)]
    fn mk_dict_a2() -> Option<A>;
    fn assert_dict_required(r: &Required);
    fn assert_camel_case(dict: &PreserveNames);
}

#[wasm_bindgen_test]
fn smoke() {
    A::new().c(1).g(2).h(3).d(4);
    B::new().c(1).g(2).h(3).d(4).a(5).b(6);

    let mut c = C::new();
    c.a(1).b(2).c(3).d(4).e(5).f(6).g(7).h(8);
    assert_dict_c(&c);
    assert_dict_c2(c.clone());
    assert_dict_c3(Some(&c));
    assert_dict_c4(Some(c));
}

#[wasm_bindgen_test]
fn get_dict() {
    mk_dict_a();
    assert!(mk_dict_a2().is_some());
}

#[wasm_bindgen_test]
fn casing() {
    CamelCaseMe::new().snake_case_me(3);
}

#[wasm_bindgen_test]
fn many_types() {
    ManyTypes::new().a("a");
}

#[wasm_bindgen_test]
fn required() {
    assert_dict_required(Required::new(3, "a").c(4));
}

#[wasm_bindgen_test]
fn correct_casing_in_js() {
    assert_camel_case(PreserveNames::new().weird_field_name(1));
}

#[wasm_bindgen_test]
fn roundtrip_set_get() {
    let mut many_types = ManyTypes::new();
    many_types.a("a");
    many_types.n1(1);
    many_types.n2(2);
    many_types.n3(3);
    many_types.n4(4);
    many_types.n5(5);
    many_types.n6(6);
    let mut other_dict = OtherDict::new();
    other_dict.a(42);
    many_types.c(&other_dict);

    {
        use crate::generated::{ManyTypesGetters, OtherDictGetters};
        assert_eq!(many_types.a(), "a");
        assert_eq!(many_types.n1(), 1);
        assert_eq!(many_types.n2(), 2);
        assert_eq!(many_types.n3(), 3);
        assert_eq!(many_types.n4(), 4);
        assert_eq!(many_types.n5(), 5);
        assert_eq!(many_types.n6(), 6);
        assert_eq!(many_types.c(), other_dict);
        assert_eq!(many_types.c().a(), 42);
    }

    crate::generated::ManyTypesGetters::c(&many_types).a(1337);

    {
        use crate::generated::{ManyTypesGetters, OtherDictGetters};
        assert_eq!(many_types.c().a(), 1337);
    }
}
