const wasm = require('wasm-bindgen-test.js');
const assert = require('assert');

function variadic_sum() {
    let answer = 0;
    for(var i=0; i<arguments.length; i++) {
        answer += arguments[i];
    }
    return answer;
}

exports.variadic_sum = variadic_sum;

