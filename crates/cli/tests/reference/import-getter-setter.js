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

export function __wbg_a_f17802a22332a667(arg0) {
    const ret = getObject(arg0).a;
    return ret;
};

export function __wbg_bar2_84566b6bcf547b19() {
    const ret = Foo.bar2();
    return ret;
};

export function __wbg_bar_eae0175044c62418() {
    const ret = Foo.bar();
    return ret;
};

export function __wbg_new_d728785ba7e8df96() {
    const ret = new SomeClass();
    return addHeapObject(ret);
};

export function __wbg_prop2_edf1002f8e41a5da(arg0) {
    const ret = getObject(arg0).prop2;
    return ret;
};

export function __wbg_seta_40f909ae19a05c10(arg0, arg1) {
    getObject(arg0).a = arg1 >>> 0;
};

export function __wbg_setbar2_c8b4a150c6accea2(arg0) {
    Foo.set_bar2(arg0 >>> 0);
};

export function __wbg_setbar_6c6b6af72f779234(arg0) {
    Foo.set_bar(arg0 >>> 0);
};

export function __wbg_setprop2_2d160a2b6600e990(arg0, arg1) {
    getObject(arg0).prop2 = arg1 >>> 0;
};

export function __wbg_setsignal_d386d151e7775c3f(arg0, arg1) {
    getObject(arg0).signal = arg1 >>> 0;
};

export function __wbg_setsomeprop_afbca63b5d0f4c92(arg0, arg1) {
    getObject(arg0).some_prop = arg1 >>> 0;
};

export function __wbg_signal_b82e5486ce265c55(arg0) {
    const ret = getObject(arg0).signal;
    return ret;
};

export function __wbg_someprop_26178791e2719528(arg0) {
    const ret = getObject(arg0).some_prop;
    return ret;
};

export function __wbindgen_object_drop_ref(arg0) {
    takeObject(arg0);
};

