let wasm;
export function __wbg_set_wasm(val) {
    wasm = val;
}


const lTextDecoder = typeof TextDecoder === 'undefined' ? (0, module.require)('util').TextDecoder : TextDecoder;

let cachedTextDecoder = new lTextDecoder('utf-8', { ignoreBOM: true, fatal: true });

cachedTextDecoder.decode();

let cachedUint8ArrayMemory0 = null;

function getUint8ArrayMemory0() {
    if (cachedUint8ArrayMemory0 === null || cachedUint8ArrayMemory0.byteLength === 0) {
        cachedUint8ArrayMemory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachedUint8ArrayMemory0;
}

function getStringFromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return cachedTextDecoder.decode(getUint8ArrayMemory0().subarray(ptr, ptr + len));
}

const heap = new Array(128).fill(undefined);

heap.push(undefined, null, true, false);

let heap_next = heap.length;

function addHeapObject(obj) {
    if (heap_next === heap.length) heap.push(heap.length + 1);
    const idx = heap_next;
    heap_next = heap[idx];

    heap[idx] = obj;
    return idx;
}

function getObject(idx) { return heap[idx]; }

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
 * @returns {string}
 */
export function result_string() {
    try {
        const ret = wasm.result_string();
        var ptr = ret[0];
        var len = ret[1];
        if (ret[3]) {
            ptr = 0; len = 0;
            throw takeObject(ret[2]);
        }
        var deferred0 = ptr;
        var deferred1 = len;
        return getStringFromWasm0(ptr, len);
    } finally {
        wasm.__wbindgen_free(deferred0, deferred1, 1);
    }
}

export function result_void() {
    const ret = wasm.result_void();
    if (ret[1]) {
        throw takeObject(ret[0]);
    }
}

/**
 * @returns {number}
 */
export function result_i32() {
    const ret = wasm.result_i32();
    if (ret[2]) {
        throw takeObject(ret[1]);
    }
    return ret[0];
}

export function __wbindgen_error_new(arg0, arg1) {
    const ret = new Error(getStringFromWasm0(arg0, arg1));
    return addHeapObject(ret);
};

export function __wbindgen_number_new(arg0) {
    const ret = arg0;
    return addHeapObject(ret);
};

