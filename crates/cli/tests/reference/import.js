import { default as default1 } from 'tests/wasm/import_class.js';

let wasm;
export function __wbg_set_wasm(val) {
    wasm = val;
}


function addToExternrefTable0(obj) {
    const idx = wasm.__externref_table_alloc();
    wasm.__wbindgen_export_2.set(idx, obj);
    return idx;
}

function handleError(f, args) {
    try {
        return f.apply(this, args);
    } catch (e) {
        const idx = addToExternrefTable0(e);
        wasm.__wbindgen_exn_store(idx);
    }
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

function takeFromExternrefTable0(idx) {
    const value = wasm.__wbindgen_export_2.get(idx);
    wasm.__externref_table_dealloc(idx);
    return value;
}

export function exported() {
    const ret = wasm.exported();
    if (ret[1]) {
        throw takeFromExternrefTable0(ret[0]);
    }
}

export function __wbg_add_7fbfb2c172506d12(arg0, arg1) {
    const ret = add(arg0, arg1);
    return ret;
};

export function __wbg_barfromfoo_29614885590bfb6f() {
    bar_from_foo();
};

export function __wbg_catchme_f7d87ea824a61e87() { return handleError(function () {
    catch_me();
}, arguments) };

export function __wbg_get_56ba567010fb9959(arg0) {
    const ret = arg0.get();
    return ret;
};

export function __wbg_myfunction_8c7b624429f78550() {
    b.my_function();
};

export function __wbg_new_d21827b66c7fd25d(arg0) {
    const ret = new default1(arg0);
    return ret;
};

export function __wbg_nocatch_be850a8dddd9599d() {
    no_catch();
};

export function __wbg_reload_84c12f152ad689f0() {
    window.location.reload();
};

export function __wbg_static_accessor_CONST_9e9d5ae758197645() {
    const ret = a.CONST;
    return ret;
};

export function __wbg_write_c2ce0ce33a6087d5(arg0, arg1) {
    window.document.write(getStringFromWasm0(arg0, arg1));
};

export function __wbindgen_init_externref_table() {
    const table = wasm.__wbindgen_export_2;
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

