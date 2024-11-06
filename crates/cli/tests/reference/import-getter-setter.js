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
    wasm.exported();
}

export function __wbg_bar2_84566b6bcf547b19() {
    const ret = Foo.bar2();
    return ret;
};

export function __wbg_getfoo_eae0175044c62418() {
    const ret = Foo.get_foo();
    return ret;
};

export function __wbg_new_d728785ba7e8df96() {
    const ret = new SomeClass();
    return addHeapObject(ret);
};

export function __wbg_setbar2_c8b4a150c6accea2(arg0) {
    Foo.set_bar2(arg0 >>> 0);
};

export function __wbg_setfoo_6c6b6af72f779234(arg0) {
    Foo.set_foo(arg0 >>> 0);
};

export function __wbg_setsignal_d386d151e7775c3f(arg0, arg1) {
    getObject(arg0).signal = arg1 >>> 0;
};

export function __wbg_signal_b82e5486ce265c55(arg0) {
    const ret = getObject(arg0).signal;
    return ret;
};

export function __wbindgen_object_drop_ref(arg0) {
    takeObject(arg0);
};

