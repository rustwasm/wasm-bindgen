let wasm;
export function __wbg_set_wasm(val) {
    wasm = val;
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
 * @param {number} a
 * @returns {number}
 */
export function export_from_rust(a) {
    const ret = wasm.export_from_rust(a);
    return ret >>> 0;
}

function isLikeNone(x) {
    return x === undefined || x === null;
}

const RustStructFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_ruststruct_free(ptr >>> 0, 1));

export class RustStruct {

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        RustStructFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_ruststruct_free(ptr, 0);
    }
    /**
     * @returns {number}
     */
    get foo() {
        const ret = wasm.__wbg_get_ruststruct_foo(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @param {number} arg0
     */
    set foo(arg0) {
        wasm.__wbg_set_ruststruct_foo(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {number}
     */
    get someCoolField() {
        const ret = wasm.__wbg_get_ruststruct_someCoolField(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @param {number} arg0
     */
    set someCoolField(arg0) {
        wasm.__wbg_set_ruststruct_someCoolField(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {number}
     */
    get another_field_for_you() {
        const ret = wasm.__wbg_get_ruststruct_another_field_for_you(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @param {number} arg0
     */
    set another_field_for_you(arg0) {
        wasm.__wbg_set_ruststruct_another_field_for_you(this.__wbg_ptr, arg0);
    }
    static i_dont_get_renamed() {
        wasm.ruststruct_i_dont_get_renamed();
    }
    /**
     * @param {number | undefined} [amount]
     */
    incrementFoo(amount) {
        wasm.ruststruct_incrementFoo(this.__wbg_ptr, isLikeNone(amount) ? 0x100000001 : (amount) >>> 0);
    }
    /**
     * @param {number} foo
     */
    setFoo(foo) {
        wasm.ruststruct_setFoo(this.__wbg_ptr, foo);
    }
    /**
     * @returns {number}
     */
    get_another() {
        const ret = wasm.ruststruct_get_another(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @param {number} a
     */
    static staticMethod(a) {
        wasm.ruststruct_staticMethod(a);
    }
    /**
     * @param {number} a
     */
    static IHaveA_funky_name(a) {
        wasm.ruststruct_IHaveA_funky_name(a);
    }
    /**
     * @returns {number}
     */
    get someOtherProp() {
        const ret = wasm.ruststruct_someOtherProp(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @param {number} value
     */
    set someOtherProp(value) {
        wasm.ruststruct_set_someOtherProp(this.__wbg_ptr, value);
    }
    /**
     * @returns {number}
     */
    get my_own_name() {
        const ret = wasm.ruststruct_someDifferentProp(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @param {number} value
     */
    set my_own_name(value) {
        wasm.ruststruct_set_someDifferentProp(this.__wbg_ptr, value);
    }
    /**
     * @returns {number}
     */
    get my_unique_name() {
        const ret = wasm.ruststruct_my_unique_name(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @param {number} value
     */
    set my_unique_name(value) {
        wasm.ruststruct_set_my_unique_name(this.__wbg_ptr, value);
    }
    /**
     * @returns {number}
     */
    static get someStaticProp() {
        const ret = wasm.ruststruct_someStaticProp();
        return ret >>> 0;
    }
    /**
     * @param {number} value
     */
    static set someStaticProp(value) {
        wasm.ruststruct_set_someStaticProp(value);
    }
}

export function __wbg_documentElement_17a3f0d4e04c6241() {
    const ret = document.documentElement();
    return addHeapObject(ret);
};

export function __wbg_foobar_aa5072d28246f9cb() {
    foo_bar();
};

export function __wbg_querySelector_ab6b6886c63dca45(arg0, arg1) {
    const ret = document.querySelector(getStringFromWasm0(arg0, arg1));
    return addHeapObject(ret);
};

export function __wbg_quxCorge_d8ec2d56c00b013f() {
    quxCorge();
};

export function __wbg_static_accessor_MAX_SAFE_INTEGER_37128e65405df998() {
    const ret = Number.MAX_SAFE_INTEGER;
    return ret;
};

export function __wbg_static_accessor_i_do_not_exist_344091339f70bf24() {
    const ret = Number.i_do_not_exist;
    return ret;
};

export function __wbindgen_object_drop_ref(arg0) {
    takeObject(arg0);
};

export function __wbindgen_throw(arg0, arg1) {
    throw new Error(getStringFromWasm0(arg0, arg1));
};

