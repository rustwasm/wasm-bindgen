const strictEqual = require('assert').strictEqual;

global.math = class {
    powf(base, exp) {
        return Math.pow(base, exp);
    }

    add_one(val) {
        return val + 1;
    }
};
