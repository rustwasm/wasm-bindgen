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

export function exported() {
    wasm.exported();
}

export function __wbg_a_266c81b129cbc216(arg0) {
    const ret = getObject(arg0).a;
    return ret;
};

export function __wbg_bar2_38c86771c0e03476() {
    const ret = Bar.bar2();
    return ret;
};

export function __wbg_bar_690459206923b526() {
    const ret = Bar.bar();
    return ret;
};

export function __wbg_new_98ff9abc2a3e2736() {
    const ret = new SomeClass();
    return addHeapObject(ret);
};

export function __wbg_prop2_79dcbfe47962d7a7(arg0) {
    const ret = getObject(arg0).prop2;
    return ret;
};

export function __wbg_seta_eda0c18669c4ad53(arg0, arg1) {
    getObject(arg0).a = arg1 >>> 0;
};

export function __wbg_setbar2_d99cb80edd0e1959(arg0) {
    Bar.set_bar2(arg0 >>> 0);
};

export function __wbg_setbar_029452b4d4645d79(arg0) {
    Bar.set_bar(arg0 >>> 0);
};

export function __wbg_setprop2_51e596d4d035bc4d(arg0, arg1) {
    getObject(arg0).prop2 = arg1 >>> 0;
};

export function __wbg_setsignal_bd536e517c35da41(arg0, arg1) {
    getObject(arg0).signal = arg1 >>> 0;
};

export function __wbg_setsomeprop_965004b0138eb32c(arg0, arg1) {
    getObject(arg0).some_prop = arg1 >>> 0;
};

export function __wbg_signal_89fe6c5b19fec3df(arg0) {
    const ret = getObject(arg0).signal;
    return ret;
};

export function __wbg_someprop_fd4fc05f44bf5de2(arg0) {
    const ret = getObject(arg0).some_prop;
    return ret;
};

export function __wbindgen_object_drop_ref(arg0) {
    takeObject(arg0);
};

