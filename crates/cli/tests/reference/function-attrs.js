let wasm;
export function __wbg_set_wasm(val) {
    wasm = val;
}


const heap = new Array(128).fill(undefined);

heap.push(undefined, null, true, false);

function getObject(idx) { return heap[idx]; }

let heap_next = heap.length;

function addHeapObject(obj) {
    if (heap_next === heap.length) heap.push(heap.length + 1);
    const idx = heap_next;
    heap_next = heap[idx];

    heap[idx] = obj;
    return idx;
}

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

let cachedDataViewMemory0 = null;

function getDataViewMemory0() {
    if (cachedDataViewMemory0 === null || cachedDataViewMemory0.buffer.detached === true || (cachedDataViewMemory0.buffer.detached === undefined && cachedDataViewMemory0.buffer !== wasm.memory.buffer)) {
        cachedDataViewMemory0 = new DataView(wasm.memory.buffer);
    }
    return cachedDataViewMemory0;
}
/**
 * Description for fn_with_attr
 * @param {number} firstArg - some number
 * @param {boolean} [secondArg]
 * @returns {number} returns 1 if arg2 is true, or arg1 if arg2 is undefined or false
 */
export function fn_with_attr(firstArg, secondArg) {
    try {
        const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
        wasm.fn_with_attr(retptr, firstArg, addHeapObject(secondArg));
        var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
        var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
        var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
        if (r2) {
            throw takeObject(r1);
        }
        return takeObject(r0);
    } finally {
        wasm.__wbindgen_add_to_stack_pointer(16);
    }
}

const HoldsNumberFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_holdsnumber_free(ptr >>> 0, 1));
/**
 * Description for HoldsNumber
 */
export class HoldsNumber {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(HoldsNumber.prototype);
        obj.__wbg_ptr = ptr;
        HoldsNumberFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        HoldsNumberFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_holdsnumber_free(ptr, 0);
    }
    /**
     * Inner value
     * @returns {number}
     */
    get inner() {
        const ret = wasm.holdsnumber_get_inner(this.__wbg_ptr);
        return takeObject(ret);
    }
    /**
     * Description for static_fn_with_attr
     * @param {number} firstArg - some number
     * @param {number} [secondArg]
     * @returns {HoldsNumber} returns an instance of HoldsNumber, holding arg1 if arg2 is undefined and holding arg2 if not
     */
    static static_fn_with_attr(firstArg, secondArg) {
        const ret = wasm.holdsnumber_static_fn_with_attr(firstArg, addHeapObject(secondArg));
        return HoldsNumber.__wrap(ret);
    }
    /**
     * Description for method_with_attr
     * @param {number} firstArg - some number
     * @param {boolean} [secondArg]
     * @returns {number} returns arg1 if arg2 is true, or holding value of self if arg2 is undefined or false
     */
    method_with_attr(firstArg, secondArg) {
        const ret = wasm.holdsnumber_method_with_attr(this.__wbg_ptr, firstArg, addHeapObject(secondArg));
        return takeObject(ret);
    }
}

export function __wbindgen_is_falsy(arg0) {
    const ret = !getObject(arg0);
    return ret;
};

export function __wbindgen_is_undefined(arg0) {
    const ret = getObject(arg0) === undefined;
    return ret;
};

export function __wbindgen_number_new(arg0) {
    const ret = arg0;
    return addHeapObject(ret);
};

export function __wbindgen_object_clone_ref(arg0) {
    const ret = getObject(arg0);
    return addHeapObject(ret);
};

export function __wbindgen_object_drop_ref(arg0) {
    takeObject(arg0);
};

export function __wbindgen_throw(arg0, arg1) {
    throw new Error(getStringFromWasm0(arg0, arg1));
};

