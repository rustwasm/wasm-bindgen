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

export function __wbg_add_0f820bc7ed50a898(arg0, arg1) {
    const ret = add(arg0, arg1);
    return ret;
};

export function __wbg_barfromfoo_fc0e2d223c6eed1f() {
    bar_from_foo();
};

export function __wbg_catchme_6fd452194698de3b() { return handleError(function () {
    catch_me();
}, arguments) };

export function __wbg_nocatch_633680ea5fdb920d() {
    no_catch();
};

export function __wbg_reload_bd59a4b785b50f44() {
    window.location.reload();
};

export function __wbg_write_67852f31952d3412(arg0, arg1) {
    window.document.write(getStringFromWasm0(arg0, arg1));
};

