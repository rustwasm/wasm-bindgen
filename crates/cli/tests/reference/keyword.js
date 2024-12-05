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

export function exported() {
    wasm.exported();
}

export function _function() {
    wasm._function();
}

export function _var() {
    wasm._var();
}

/**
 * @param {number} _new
 * @param {number} _var
 */
export function weird_arguments(_new, _var) {
    wasm.weird_arguments(_new, _var);
}

/**
 * @enum {0 | 1}
 */
export const switch = Object.freeze({
    A: 0, "0": "A",
    B: 1, "1": "B",
});

const classFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_class_free(ptr >>> 0, 1));

export class class {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(class.prototype);
        obj.__wbg_ptr = ptr;
        classFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        classFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_class_free(ptr, 0);
    }
    /**
     * @returns {class}
     */
    static new() {
        const ret = wasm.class_new();
        return class.__wrap(ret);
    }
}

export function __wbg_await_e0a0e75be8b6fef6() {
    await();
};

export function __wbg_new_64246b0041120fec() {
    A.new();
};

export function __wbg_new_d4bfd9add722b492() {
    const ret = window.__TAURI__.menu.Menu.new();
    return ret;
};

export function __wbg_new_e17dd7c5a1cd57d8() {
    B.new();
};

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

