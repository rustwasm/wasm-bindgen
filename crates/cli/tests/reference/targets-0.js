let wasm;
export function __wbg_set_wasm(val) {
    wasm = val;
}

/**
 * @param {number} a
 * @param {number} b
 * @returns {number}
 */
export function add_that_might_fail(a, b) {
    const ret = wasm.add_that_might_fail(a, b);
    return ret >>> 0;
}

export function __wbg_random_5d40be260a2cfbac() {
    const ret = Math.random();
    return ret;
};

