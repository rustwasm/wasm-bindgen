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
 * @param {number} _switch
 * @param {number} _default
 * @param {number} _arguments
 */
export function weird_arguments(_new, _var, _switch, _default, _arguments) {
    wasm.weird_arguments(_new, _var, _switch, _default, _arguments);
}

export function __wbg_await_e0a0e75be8b6fef6() {
    await();
};

export function __wbg_let_8d461e9e0592bd8c(arg0) {
    arg0.let();
};

export function __wbg_new_4b026aaf1c1e4438() {
    const ret = A.new();
    return ret;
};

export function __wbg_new_d4bfd9add722b492() {
    const ret = window.__TAURI__.menu.Menu.new();
    return ret;
};

export function __wbg_new_e17dd7c5a1cd57d8() {
    B.new();
};

export function __wbg_static_accessor_TRUE_c6b68bf8545d99a3() {
    const ret = true;
    return ret;
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

