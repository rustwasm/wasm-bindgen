let wasm;
export function __wbg_set_wasm(val) {
    wasm = val;
}


let cachedDataViewMemory0 = null;

function getDataViewMemory0() {
    if (cachedDataViewMemory0 === null || cachedDataViewMemory0.buffer.detached === true || (cachedDataViewMemory0.buffer.detached === undefined && cachedDataViewMemory0.buffer !== wasm.memory.buffer)) {
        cachedDataViewMemory0 = new DataView(wasm.memory.buffer);
    }
    return cachedDataViewMemory0;
}
/**
 * @param {bigint} a
 * @returns {bigint}
 */
export function echo_i128(a) {
    try {
        const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
        wasm.echo_i128(retptr, a, a >> 64n);
        var r0 = getDataViewMemory0().getBigInt64(retptr + 8 * 0, true);
        var r2 = getDataViewMemory0().getBigInt64(retptr + 8 * 1, true);
        return (BigInt.asUintN(64, r0) | (r2 << 64n));
    } finally {
        wasm.__wbindgen_add_to_stack_pointer(16);
    }
}

/**
 * @param {bigint} a
 * @returns {bigint}
 */
export function echo_u128(a) {
    try {
        const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
        wasm.echo_u128(retptr, a, a >> 64n);
        var r0 = getDataViewMemory0().getBigInt64(retptr + 8 * 0, true);
        var r2 = getDataViewMemory0().getBigInt64(retptr + 8 * 1, true);
        return (BigInt.asUintN(64, r0) | (BigInt.asUintN(64, r2) << 64n));
    } finally {
        wasm.__wbindgen_add_to_stack_pointer(16);
    }
}

function isLikeNone(x) {
    return x === undefined || x === null;
}
/**
 * @param {bigint | undefined} [a]
 * @returns {bigint | undefined}
 */
export function echo_option_i128(a) {
    try {
        const retptr = wasm.__wbindgen_add_to_stack_pointer(-32);
        wasm.echo_option_i128(retptr, !isLikeNone(a), isLikeNone(a) ? 0n : a, isLikeNone(a) ? 0n : a >> 64n);
        var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
        var r2 = getDataViewMemory0().getBigInt64(retptr + 8 * 1, true);
        var r4 = getDataViewMemory0().getBigInt64(retptr + 8 * 2, true);
        return r0 === 0 ? undefined : (BigInt.asUintN(64, r2) | (r4 << 64n));
    } finally {
        wasm.__wbindgen_add_to_stack_pointer(32);
    }
}

/**
 * @param {bigint | undefined} [a]
 * @returns {bigint | undefined}
 */
export function echo_option_u128(a) {
    try {
        const retptr = wasm.__wbindgen_add_to_stack_pointer(-32);
        wasm.echo_option_u128(retptr, !isLikeNone(a), isLikeNone(a) ? 0n : a, isLikeNone(a) ? 0n : a >> 64n);
        var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
        var r2 = getDataViewMemory0().getBigInt64(retptr + 8 * 1, true);
        var r4 = getDataViewMemory0().getBigInt64(retptr + 8 * 2, true);
        return r0 === 0 ? undefined : (BigInt.asUintN(64, r2) | (BigInt.asUintN(64, r4) << 64n));
    } finally {
        wasm.__wbindgen_add_to_stack_pointer(32);
    }
}

const heap = new Array(128).fill(undefined);

heap.push(undefined, null, true, false);

function getObject(idx) { return heap[idx]; }

let heap_next = heap.length;

function dropObject(idx) {
    if (idx < 132) return;
    heap[idx] = heap_next;
    heap_next = idx;
}

function takeObject(idx) {
    const ret = getObject(idx);
    dropObject(idx);
    return ret;
}
/**
 * @returns {bigint}
 */
export function throw_i128() {
    try {
        const retptr = wasm.__wbindgen_add_to_stack_pointer(-32);
        wasm.throw_i128(retptr);
        var r0 = getDataViewMemory0().getBigInt64(retptr + 8 * 0, true);
        var r2 = getDataViewMemory0().getBigInt64(retptr + 8 * 1, true);
        var r4 = getDataViewMemory0().getInt32(retptr + 4 * 4, true);
        var r5 = getDataViewMemory0().getInt32(retptr + 4 * 5, true);
        if (r5) {
            throw takeObject(r4);
        }
        return (BigInt.asUintN(64, r0) | (r2 << 64n));
    } finally {
        wasm.__wbindgen_add_to_stack_pointer(32);
    }
}

