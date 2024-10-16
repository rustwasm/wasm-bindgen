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

let cachedDataViewMemory0 = null;

function getDataViewMemory0() {
    if (cachedDataViewMemory0 === null || cachedDataViewMemory0.buffer.detached === true || (cachedDataViewMemory0.buffer.detached === undefined && cachedDataViewMemory0.buffer !== wasm.memory.buffer)) {
        cachedDataViewMemory0 = new DataView(wasm.memory.buffer);
    }
    return cachedDataViewMemory0;
}
/**
 * @param {bigint | undefined} a
 * @param {bigint} b
 * @returns {bigint | undefined}
 */
export function add_option_u64(a, b) {
    try {
        const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
        const bigint0 = typeof a === 'number' ? BigInt(Math.trunc(a)) : a;
        const bigint1 = typeof b === 'number' ? BigInt(Math.trunc(b)) : b;
        wasm.add_option_u64(retptr, !isLikeNone(bigint0), isLikeNone(bigint0) ? BigInt(0) : bigint0, bigint1);
        var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
        var r2 = getDataViewMemory0().getBigInt64(retptr + 8 * 1, true);
        return r0 === 0 ? undefined : BigInt.asUintN(64, r2);
    } finally {
        wasm.__wbindgen_add_to_stack_pointer(16);
    }
}

/**
 * @param {bigint | undefined} a
 * @param {bigint} b
 * @returns {bigint | undefined}
 */
export function add_option_i64(a, b) {
    try {
        const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
        const bigint0 = typeof a === 'number' ? BigInt(Math.trunc(a)) : a;
        const bigint1 = typeof b === 'number' ? BigInt(Math.trunc(b)) : b;
        wasm.add_option_i64(retptr, !isLikeNone(bigint0), isLikeNone(bigint0) ? BigInt(0) : bigint0, bigint1);
        var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
        var r2 = getDataViewMemory0().getBigInt64(retptr + 8 * 1, true);
        return r0 === 0 ? undefined : r2;
    } finally {
        wasm.__wbindgen_add_to_stack_pointer(16);
    }
}

