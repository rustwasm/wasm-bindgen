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

const TestFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_test_free(ptr >>> 0, 1));

export class Test {

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        TestFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_test_free(ptr, 0);
    }
    constructor() {
        const ret = wasm.test_new();
        this.__wbg_ptr = ret >>> 0;
        TestFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    consume_self() {
        const ptr = this.__destroy_into_raw();
        wasm.test_consume_self(ptr);
    }
    ref_self() {
        wasm.test_ref_self(this.__wbg_ptr);
    }
    ref_mut_self() {
        wasm.test_ref_mut_self(this.__wbg_ptr);
    }
    self_Self() {
        const ptr = this.__destroy_into_raw();
        wasm.test_self_Self(ptr);
    }
    self_ref_Self() {
        wasm.test_self_ref_Self(this.__wbg_ptr);
    }
    self_ref_mut_Self() {
        wasm.test_self_ref_mut_Self(this.__wbg_ptr);
    }
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

export function __wbindgen_throw(arg0, arg1) {
    throw new Error(getStringFromWasm0(arg0, arg1));
};

