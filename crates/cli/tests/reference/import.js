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

function handleError(f, args) {
    try {
        return f.apply(this, args);
    } catch (e) {
        wasm.__wbindgen_exn_store(addHeapObject(e));
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

export function exported() {
    const ret = wasm.exported();
    if (ret[1]) {
        throw takeObject(ret[0]);
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

export function __wbg_nocatch_be850a8dddd9599d() {
    no_catch();
};

export function __wbg_reload_84c12f152ad689f0() {
    window.location.reload();
};

export function __wbg_write_c2ce0ce33a6087d5(arg0, arg1) {
    window.document.write(getStringFromWasm0(arg0, arg1));
};

