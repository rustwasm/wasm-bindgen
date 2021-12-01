import * as wasm from './reference_test_bg.wasm';

const heap = new Array(32).fill(undefined);

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

let cachegetInt32Memory0 = null;
function getInt32Memory0() {
    if (cachegetInt32Memory0 === null || cachegetInt32Memory0.buffer !== wasm.memory.buffer) {
        cachegetInt32Memory0 = new Int32Array(wasm.memory.buffer);
    }
    return cachegetInt32Memory0;
}
/**
*/
export function exported() {
    try {
        const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
        wasm.exported(retptr);
        var r0 = getInt32Memory0()[retptr / 4 + 0];
        var r1 = getInt32Memory0()[retptr / 4 + 1];
        var is_ok0 = r0;
        var err1 = r1;
    if (is_ok0 === 0) { throw takeObject(err1); }
} finally {
    wasm.__wbindgen_add_to_stack_pointer(16);
}
}

export function __wbg_foo_8d66ddef0ff279d6() { return handleError(function () {
    foo();
}, arguments) };

