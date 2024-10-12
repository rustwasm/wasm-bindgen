let wasm;
export function __wbg_set_wasm(val) {
    wasm = val;
}


/** @type {TextDecoder} */
let cachedTextDecoder = new (typeof TextDecoder === 'undefined' ? (0, module.require)('util').TextDecoder : TextDecoder)('utf-8', { ignoreBOM: true, fatal: true });

cachedTextDecoder.decode();

/** @type {Uint8Array | null} */
let cachedUint8ArrayMemory0 = null;
function getUint8ArrayMemory0() {
    if (cachedUint8ArrayMemory0 === null || cachedUint8ArrayMemory0.byteLength === 0) {
        cachedUint8ArrayMemory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachedUint8ArrayMemory0;
}

/**
 * @param {number} ptr
 * @param {number} len
 * @returns {string}
 */
function getStringFromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return cachedTextDecoder.decode(getUint8ArrayMemory0().subarray(ptr, ptr + len));
}

const ClassConstructorFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_classconstructor_free(ptr >>> 0, 1));

export class ClassConstructor {

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        ClassConstructorFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_classconstructor_free(ptr, 0);
    }
    constructor() {
        const ret = wasm.classconstructor_new();
        this.__wbg_ptr = ret >>> 0;
        ClassConstructorFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
}

export function __wbindgen_throw(arg0, arg1) {
    throw new Error(getStringFromWasm0(arg0, arg1));
};

