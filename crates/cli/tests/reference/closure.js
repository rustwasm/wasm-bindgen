let wasm;
export function __wbg_set_wasm(val) {
    wasm = val;
}


const CLOSURE_DTORS = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(state => {
    wasm.__wbindgen_export_0.get(state.dtor)(state.a, state.b)
});

function makeMutClosure(arg0, arg1, dtor, f) {
    const state = { a: arg0, b: arg1, cnt: 1, dtor };
    const real = (...args) => {
        // First up with a closure we increment the internal reference
        // count. This ensures that the Rust closure environment won't
        // be deallocated while we're invoking it.
        state.cnt++;
        const a = state.a;
        state.a = 0;
        try {
            return f(a, state.b, ...args);
        } finally {
            if (--state.cnt === 0) {
                wasm.__wbindgen_export_0.get(state.dtor)(a, state.b);
                CLOSURE_DTORS.unregister(state);
            } else {
                state.a = a;
            }
        }
    };
    real.original = state;
    CLOSURE_DTORS.register(real, state, state);
    return real;
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

function __wbg_adapter_8(arg0, arg1) {
    wasm._dyn_core__ops__function__FnMut_____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h08d04a92620849c7(arg0, arg1);
}

function __wbg_adapter_9(arg0, arg1) {
    wasm.wasm_bindgen__convert__closures__invoke0__h0558a7c8fc0aaaa4(arg0, arg1);
}

export function __wbg_setInterval_dcabc54c1b7f2a8c(arg0, arg1) {
    const ret = setInterval(arg0, arg1 >>> 0);
    return ret;
};

export function __wbg_takesimmutableclosure_19e990abb85ab0b6(arg0, arg1) {
    try {
        var state = {a: arg0, b: arg1};
        const cb = () => __wbg_adapter_9(state.a, state.b, );
        takes_immutable_closure(cb);
    } finally {
        state.a = state.b = 0;
    }
};

export function __wbg_takesmutableclosure_792efe9eb99346a2(arg0, arg1) {
    try {
        var state = {a: arg0, b: arg1};
        const cb = () => {
            const a = state.a;
            state.a = 0;
            try {
                return __wbg_adapter_8(a, state.b, );
            } finally {
                state.a = a;
            }
        };
        takes_mutable_closure(cb);
    } finally {
        state.a = state.b = 0;
    }
};

export function __wbindgen_cb_drop(arg0) {
    const obj = arg0.original;
    if (obj.cnt-- == 1) {
        obj.a = 0;
        return true;
    }
    const ret = false;
    return ret;
};

export function __wbindgen_closure_wrapper57(arg0, arg1, arg2) {
    const ret = makeMutClosure(arg0, arg1, 4, __wbg_adapter_8);
    return ret;
};

export function __wbindgen_init_externref_table() {
    const table = wasm.__wbindgen_export_1;
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

