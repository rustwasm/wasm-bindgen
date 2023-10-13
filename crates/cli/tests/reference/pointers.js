let wasm;
export function __wbg_set_wasm(val) {
    wasm = val;
}

/**
* @param {number} input
* @returns {number}
*/
export function const_pointer(input) {
    const ret = wasm.const_pointer(input);
    return ret >>> 0;
}

/**
* @param {number} input
* @returns {number}
*/
export function mut_pointer(input) {
    const ret = wasm.mut_pointer(input);
    return ret >>> 0;
}

