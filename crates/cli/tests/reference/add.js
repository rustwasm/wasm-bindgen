let wasm;
export function __wbg_set_wasm(val) {
    wasm = val;
}

/**
* @param {number} a
* @param {number} b
* @returns {number}
*/
export function add_u32(a, b) {
    const ret = wasm.add_u32(a, b);
    return ret >>> 0;
}

/**
* @param {number} a
* @param {number} b
* @returns {number}
*/
export function add_i32(a, b) {
    const ret = wasm.add_i32(a, b);
    return ret;
}

