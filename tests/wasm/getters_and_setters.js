const wasm = require('wasm-bindgen-test.js');
const assert = require('assert');

exports._1_js = (rules) => {
    assert.equal(rules.field, 1);
    rules.field *= 2;
    return rules;
}

exports._2_js = (rules) => {
    let value = rules.no_js_name__no_getter_with_name__no_getter_without_name();
    assert.equal(value, 2);
    rules.set_no_js_name__no_setter_with_name__no_setter_without_name(value * 2);
    return rules;
}

exports._3_js = (rules) => {
    let value = rules.no_js_name__no_getter_with_name__getter_without_name;
    assert.equal(value, 3);
    rules.no_js_name__no_setter_with_name__setter_without_name = value * 2;
    return rules;
}

exports._4_js = (rules) => {
    let value = rules.new_no_js_name__getter_with_name__getter_without_name;
    assert.equal(value, 4);
    rules.new_no_js_name__setter_with_name__setter_without_name = value * 2;
    return rules;
}

exports._5_js = (rules) => {
    let value = rules.new_js_name__no_getter_with_name__no_getter_without_name();
    assert.equal(value, 5);
    rules.new_js_name__no_setter_with_name__no_setter_without_name(value * 2);
    return rules;
}

exports._6_js = (rules) => {
    let value = rules.new_js_name__no_getter_with_name__getter_without_name;
    assert.equal(value, 6);
    rules.new_js_name__no_setter_with_name__setter_without_name = value * 2;
    return rules;
}

exports._7_js = (rules) => {
    let value = rules.new_js_name__getter_with_name__no_getter_without_name_for_field;
    assert.equal(value, 7);
    rules.new_js_name__setter_with_name__no_setter_without_name_for_field = value * 2;
    return rules;
}

exports.test_getter_compute = x => {
  assert.equal(x.foo, 3)
};

exports.test_setter_compute = x => {
  x.foo = 97;
};
