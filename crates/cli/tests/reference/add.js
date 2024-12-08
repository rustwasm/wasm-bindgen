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

/**
 * @param {bigint} a
 * @param {bigint} b
 * @returns {bigint}
 */
export function add_u64(a, b) {
    const bigint0 = typeof a === 'number' ? BigInt(Math.trunc(a)) : a;
    const bigint1 = typeof b === 'number' ? BigInt(Math.trunc(b)) : b;
    const ret = wasm.add_u64(bigint0, bigint1);
    return BigInt.asUintN(64, ret);
}

/**
 * @param {bigint} a
 * @param {bigint} b
 * @returns {bigint}
 */
export function add_i64(a, b) {
    const bigint0 = typeof a === 'number' ? BigInt(Math.trunc(a)) : a;
    const bigint1 = typeof b === 'number' ? BigInt(Math.trunc(b)) : b;
    const ret = wasm.add_i64(bigint0, bigint1);
    return ret;
}

function isLikeNone(x) {
    return x === undefined || x === null;
}
/**
 * @param {bigint | null | undefined} a
 * @param {bigint} b
 * @returns {bigint | undefined}
 */
export function add_option_u64(a, b) {
    const bigint0 = typeof a === 'number' ? BigInt(Math.trunc(a)) : a;
    const bigint1 = typeof b === 'number' ? BigInt(Math.trunc(b)) : b;
    const ret = wasm.add_option_u64(!isLikeNone(bigint0), isLikeNone(bigint0) ? BigInt(0) : bigint0, bigint1);
    return ret[0] === 0 ? undefined : BigInt.asUintN(64, ret[1]);
}

/**
 * @param {bigint | null | undefined} a
 * @param {bigint} b
 * @returns {bigint | undefined}
 */
export function add_option_i64(a, b) {
    const bigint0 = typeof a === 'number' ? BigInt(Math.trunc(a)) : a;
    const bigint1 = typeof b === 'number' ? BigInt(Math.trunc(b)) : b;
    const ret = wasm.add_option_i64(!isLikeNone(bigint0), isLikeNone(bigint0) ? BigInt(0) : bigint0, bigint1);
    return ret[0] === 0 ? undefined : ret[1];
}

export function __wbindgen_init_externref_table() {
    const table = wasm.__wbindgen_export_0;
    const offset = table.grow(4);
    table.set(0, undefined);
    table.set(offset + 0, undefined);
    table.set(offset + 1, null);
    table.set(offset + 2, true);
    table.set(offset + 3, false);
    ;
};

