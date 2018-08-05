const assert = require('assert');

exports.works_call = a => {
    a();
};

exports.works_thread = a => a(2);

let CANNOT_REUSE_CACHE = null;

exports.cannot_reuse_call = a => {
    CANNOT_REUSE_CACHE = a;
};

exports.cannot_reuse_call_again = () => {
    CANNOT_REUSE_CACHE();
};

exports.long_lived_call1 = a => {
    a();
};

exports.long_lived_call2 = a => a(2);

exports.many_arity_call1 = a => {
    a();
};
exports.many_arity_call2 = a => {
    a(1);
};
exports.many_arity_call3 = a => {
    a(1, 2);
};
exports.many_arity_call4 = a => {
    a(1, 2, 3);
};
exports.many_arity_call5 = a => {
    a(1, 2, 3, 4);
};
exports.many_arity_call6 = a => {
    a(1, 2, 3, 4, 5);
};
exports.many_arity_call7 = a => {
    a(1, 2, 3, 4, 5, 6);
};
exports.many_arity_call8 = a => {
    a(1, 2, 3, 4, 5, 6, 7);
};

let LONG_LIVED_DROPPING_CACHE = null;

exports.long_lived_dropping_cache = a => {
    LONG_LIVED_DROPPING_CACHE = a;
};
exports.long_lived_dropping_call = () => {
    LONG_LIVED_DROPPING_CACHE();
};

let LONG_FNMUT_RECURSIVE_CACHE = null;

exports.long_fnmut_recursive_cache = a => {
    LONG_FNMUT_RECURSIVE_CACHE = a;
};
exports.long_fnmut_recursive_call = () => {
    LONG_FNMUT_RECURSIVE_CACHE();
};

exports.fnmut_call = a => {
    a();
};

exports.fnmut_thread = a => a(2);

let FNMUT_BAD_F = null;

exports.fnmut_bad_call = a => {
    FNMUT_BAD_F = a;
    a();
};

exports.fnmut_bad_again = x => {
    if (x) {
        FNMUT_BAD_F();
    }
};

exports.string_arguments_call = a => {
    a('foo');
};

exports.string_ret_call = a => {
    assert.strictEqual(a('foo'), 'foobar');
};