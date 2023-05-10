let wasm;
export function __wbg_set_wasm(val) {
    wasm = val;
}


const heap = new Array(32).fill(undefined);

heap.push(undefined, null, true, false);

let stack_pointer = 32;

function addBorrowedObject(obj) {
    if (stack_pointer == 1) throw new Error('out of js stack');
    heap[--stack_pointer] = obj;
    return stack_pointer;
}
/**
* @param {import("somepackage").SomeType} a
*/
export function something(a) {
    try {
        wasm.something(addBorrowedObject(a));
    } finally {
        heap[stack_pointer++] = undefined;
    }
}

