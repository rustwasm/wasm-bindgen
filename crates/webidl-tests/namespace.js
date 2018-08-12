const strictEqual = require('assert').strictEqual;

global.mathtest = {};

global.mathtest.powf = function powf(base, exp) {
    return Math.pow(base, exp);
}

global.mathtest.add_one = function add_one(val) {
    return val + 1;
}
