const wasm = require('wasm-bindgen-test.js');
const assert = require('assert');

exports.call_ok = function() {
    assert.doesNotThrow(() => {
        let five = wasm.return_ok();
        assert.strictEqual(five, 5);
    })
}

exports.call_err = function() {
    assert.throws(() => wasm.return_err(), {
        message: "MyError::Variant"
    });
}

exports.call_make_an_error = function() {
    assert.doesNotThrow(() => {
        let e = wasm.make_an_error()
        assert.strictEqual(e.message, "un-thrown error");
    });
}

function check_inflight(struct) {
    assert.strictEqual(struct.is_inflight(), false);
}

exports.all_struct_methods = function() {
    let struct;
    assert.throws(() => wasm.MyStruct.new_err(), {
        message: "MyError::Variant"
    });
    assert.doesNotThrow(() => {
        struct = wasm.MyStruct.new();
    });
    check_inflight(struct);
    assert.doesNotThrow(() => {
        let five = struct.return_ok();
        assert.strictEqual(five, 5);
    });
    check_inflight(struct);
    assert.throws(() => struct.return_err(), {
        message: "MyError::Variant"
    });
    check_inflight(struct);
}

exports.call_return_string = function() {
    assert.doesNotThrow(() => {
        let ok = wasm.return_string();
        assert.strictEqual(ok, "string here");
    })
}

exports.call_return_enum = function() {
    assert.doesNotThrow(() => {
        let ok = wasm.return_enum();
        assert.strictEqual(ok, 2);
    })
}

exports.call_jsvalue_ok = function() {
    assert.doesNotThrow(() => {
        let five = wasm.return_jsvalue_ok();
        assert.strictEqual(five, 5);
    })
}

exports.call_jsvalue_err = function() {
    try {
        wasm.return_jsvalue_err();
        assert.fail("should have thrown");
    } catch (e) {
        assert.strictEqual(e, -1);
    }
}

