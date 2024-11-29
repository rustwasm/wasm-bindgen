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

export function __wbg_another_79dcbfe47962d7a7(arg0) {
    const ret = arg0.prop2;
    return ret;
};

export function __wbg_b_266c81b129cbc216(arg0) {
    const ret = arg0.a;
    return ret;
};

export function __wbg_bar2_38c86771c0e03476() {
    const ret = Bar.bar2();
    return ret;
};

export function __wbg_getfoo_690459206923b526() {
    const ret = Bar.get_foo();
    return ret;
};

export function __wbg_new_98ff9abc2a3e2736() {
    const ret = new SomeClass();
    return ret;
};

export function __wbg_setanother_51e596d4d035bc4d(arg0, arg1) {
    arg0.prop2 = arg1 >>> 0;
};

export function __wbg_setb_eda0c18669c4ad53(arg0, arg1) {
    arg0.a = arg1 >>> 0;
};

export function __wbg_setbar2_d99cb80edd0e1959(arg0) {
    Bar.set_bar2(arg0 >>> 0);
};

export function __wbg_setfoo_029452b4d4645d79(arg0) {
    Bar.set_foo(arg0 >>> 0);
};

export function __wbg_setsignal_bd536e517c35da41(arg0, arg1) {
    arg0.signal = arg1 >>> 0;
};

export function __wbg_setsomeprop_965004b0138eb32c(arg0, arg1) {
    arg0.some_prop = arg1 >>> 0;
};

export function __wbg_signal_89fe6c5b19fec3df(arg0) {
    const ret = arg0.signal;
    return ret;
};

export function __wbg_someprop_fd4fc05f44bf5de2(arg0) {
    const ret = arg0.some_prop;
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

