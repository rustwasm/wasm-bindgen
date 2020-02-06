import * as wasm from './reference_test_bg.wasm';

const heap = [];

heap.length = 32;

heap.push(undefined, null, true, false);

function getObject(idx) { return heap[idx]; }

let heap_next = heap.length;

function dropObject(idx) {
    if (idx < 36) return;
    heap[idx] = heap_next;
    heap_next = idx;
}

function takeObject(idx) {
    const ret = getObject(idx);
    dropObject(idx);
    return ret;
}

function addHeapObject(obj) {
    if (heap_next === heap.length) heap.push(heap.length + 1);
    const idx = heap_next;
    heap_next = heap[idx];

    heap[idx] = obj;
    return idx;
}

function handleError(e) {
    wasm.__wbindgen_exn_store(addHeapObject(e));
}
/**
*/
export function exported() {
    wasm.exported();
}

export const __wbindgen_object_drop_ref = function(arg0) {
    takeObject(arg0);
};

export const __wbg_foo_8d66ddef0ff279d6 = function() {
    try {
        foo();
    } catch (e) {
        handleError(e)
    }
};

export const __wbindgen_rethrow = function(arg0) {
    throw takeObject(arg0);
};

