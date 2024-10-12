let wasm;
export function __wbg_set_wasm(val) {
    wasm = val;
}


const heap = new Array(128).fill(undefined);

heap.push(undefined, null, true, false);

function getObject(idx) { return heap[idx]; }

function debugString(val) {
    // primitive types
    const type = typeof val;
    if (type == 'number' || type == 'boolean' || val == null) {
        return  `${val}`;
    }
    if (type == 'string') {
        return `"${val}"`;
    }
    if (type == 'symbol') {
        const description = val.description;
        if (description == null) {
            return 'Symbol';
        } else {
            return `Symbol(${description})`;
        }
    }
    if (type == 'function') {
        const name = val.name;
        if (typeof name == 'string' && name.length > 0) {
            return `Function(${name})`;
        } else {
            return 'Function';
        }
    }
    // objects
    if (Array.isArray(val)) {
        const length = val.length;
        let debug = '[';
        if (length > 0) {
            debug += debugString(val[0]);
        }
        for(let i = 1; i < length; i++) {
            debug += ', ' + debugString(val[i]);
        }
        debug += ']';
        return debug;
    }
    // Test for built-in
    const builtInMatches = /\[object ([^\]]+)\]/.exec(toString.call(val));
    let className;
    if (builtInMatches.length > 1) {
        className = builtInMatches[1];
    } else {
        // Failed to match the standard '[object ClassName]'
        return toString.call(val);
    }
    if (className == 'Object') {
        // we're a user defined class or Object
        // JSON.stringify avoids problems with cycles, and is generally much
        // easier than looping through ownProperties of `val`.
        try {
            return 'Object(' + JSON.stringify(val) + ')';
        } catch (_) {
            return 'Object';
        }
    }
    // errors
    if (val instanceof Error) {
        return `${val.name}: ${val.message}\n${val.stack}`;
    }
    // TODO we could test for more things here, like `Set`s and `Map`s.
    return className;
}

let WASM_VECTOR_LEN = 0;

let cachedUint8ArrayMemory0 = null;

function getUint8ArrayMemory0() {
    if (cachedUint8ArrayMemory0 === null || cachedUint8ArrayMemory0.byteLength === 0) {
        cachedUint8ArrayMemory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachedUint8ArrayMemory0;
}

const lTextEncoder = typeof TextEncoder === 'undefined' ? (0, module.require)('util').TextEncoder : TextEncoder;

let cachedTextEncoder = new lTextEncoder('utf-8');

const encodeString = (typeof cachedTextEncoder.encodeInto === 'function'
    ? function (arg, view) {
    return cachedTextEncoder.encodeInto(arg, view);
}
    : function (arg, view) {
    const buf = cachedTextEncoder.encode(arg);
    view.set(buf);
    return {
        read: arg.length,
        written: buf.length
    };
});

function passStringToWasm0(arg, malloc, realloc) {

    if (realloc === undefined) {
        const buf = cachedTextEncoder.encode(arg);
        const ptr = malloc(buf.length, 1) >>> 0;
        getUint8ArrayMemory0().subarray(ptr, ptr + buf.length).set(buf);
        WASM_VECTOR_LEN = buf.length;
        return ptr;
    }

    let len = arg.length;
    let ptr = malloc(len, 1) >>> 0;

    const mem = getUint8ArrayMemory0();

    let offset = 0;

    for (; offset < len; offset++) {
        const code = arg.charCodeAt(offset);
        if (code > 0x7F) break;
        mem[ptr + offset] = code;
    }

    if (offset !== len) {
        if (offset !== 0) {
            arg = arg.slice(offset);
        }
        ptr = realloc(ptr, len, len = offset + arg.length * 3, 1) >>> 0;
        const view = getUint8ArrayMemory0().subarray(ptr + offset, ptr + len);
        const ret = encodeString(arg, view);

        offset += ret.written;
        ptr = realloc(ptr, len, offset, 1) >>> 0;
    }

    WASM_VECTOR_LEN = offset;
    return ptr;
}

let cachedDataViewMemory0 = null;

function getDataViewMemory0() {
    if (cachedDataViewMemory0 === null || cachedDataViewMemory0.buffer.detached === true || (cachedDataViewMemory0.buffer.detached === undefined && cachedDataViewMemory0.buffer !== wasm.memory.buffer)) {
        cachedDataViewMemory0 = new DataView(wasm.memory.buffer);
    }
    return cachedDataViewMemory0;
}

const lTextDecoder = typeof TextDecoder === 'undefined' ? (0, module.require)('util').TextDecoder : TextDecoder;

let cachedTextDecoder = new lTextDecoder('utf-8', { ignoreBOM: true, fatal: true });

cachedTextDecoder.decode();

function getStringFromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return cachedTextDecoder.decode(getUint8ArrayMemory0().subarray(ptr, ptr + len));
}

let heap_next = heap.length;

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

function addHeapObject(obj) {
    if (heap_next === heap.length) heap.push(heap.length + 1);
    const idx = heap_next;
    heap_next = heap[idx];

    heap[idx] = obj;
    return idx;
}
/**
 * @param {number} a
 * @returns {number}
 */
export function echo_u8(a) {
    const ret = wasm.echo_u8(a);
    return ret;
}

/**
 * @param {number} a
 * @returns {number}
 */
export function echo_i8(a) {
    const ret = wasm.echo_i8(a);
    return ret;
}

/**
 * @param {number} a
 * @returns {number}
 */
export function echo_u16(a) {
    const ret = wasm.echo_u16(a);
    return ret;
}

/**
 * @param {number} a
 * @returns {number}
 */
export function echo_i16(a) {
    const ret = wasm.echo_i16(a);
    return ret;
}

/**
 * @param {number} a
 * @returns {number}
 */
export function echo_u32(a) {
    const ret = wasm.echo_u32(a);
    return ret >>> 0;
}

/**
 * @param {number} a
 * @returns {number}
 */
export function echo_i32(a) {
    const ret = wasm.echo_i32(a);
    return ret;
}

/**
 * @param {bigint} a
 * @returns {bigint}
 */
export function echo_u64(a) {
    const ret = wasm.echo_u64(a);
    return BigInt.asUintN(64, ret);
}

/**
 * @param {bigint} a
 * @returns {bigint}
 */
export function echo_i64(a) {
    const ret = wasm.echo_i64(a);
    return ret;
}

/**
 * @param {number} a
 * @returns {number}
 */
export function echo_usize(a) {
    const ret = wasm.echo_usize(a);
    return ret >>> 0;
}

/**
 * @param {number} a
 * @returns {number}
 */
export function echo_isize(a) {
    const ret = wasm.echo_isize(a);
    return ret;
}

/**
 * @param {number} a
 * @returns {number}
 */
export function echo_f32(a) {
    const ret = wasm.echo_f32(a);
    return ret;
}

/**
 * @param {number} a
 * @returns {number}
 */
export function echo_f64(a) {
    const ret = wasm.echo_f64(a);
    return ret;
}

/**
 * @param {boolean} a
 * @returns {boolean}
 */
export function echo_bool(a) {
    const ret = wasm.echo_bool(a);
    return ret !== 0;
}

function _assertChar(c) {
    if (typeof(c) === 'number' && (c >= 0x110000 || (c >= 0xD800 && c < 0xE000))) throw new Error(`expected a valid Unicode scalar value, found ${c}`);
}
/**
 * @param {string} a
 * @returns {string}
 */
export function echo_char(a) {
    const char0 = a.codePointAt(0);
    _assertChar(char0);
    const ret = wasm.echo_char(char0);
    return String.fromCodePoint(ret);
}

/**
 * @param {string} a
 * @returns {string}
 */
export function echo_string(a) {
    let deferred2_0;
    let deferred2_1;
    try {
        const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
        const ptr0 = passStringToWasm0(a, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.echo_string(retptr, ptr0, len0);
        var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
        var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
        deferred2_0 = r0;
        deferred2_1 = r1;
        return getStringFromWasm0(r0, r1);
    } finally {
        wasm.__wbindgen_add_to_stack_pointer(16);
        wasm.__wbindgen_free(deferred2_0, deferred2_1, 1);
    }
}

function passArray8ToWasm0(arg, malloc) {
    const ptr = malloc(arg.length * 1, 1) >>> 0;
    getUint8ArrayMemory0().set(arg, ptr / 1);
    WASM_VECTOR_LEN = arg.length;
    return ptr;
}

function getArrayU8FromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return getUint8ArrayMemory0().subarray(ptr / 1, ptr / 1 + len);
}
/**
 * @param {Uint8Array} a
 * @returns {Uint8Array}
 */
export function echo_vec_u8(a) {
    try {
        const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
        const ptr0 = passArray8ToWasm0(a, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.echo_vec_u8(retptr, ptr0, len0);
        var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
        var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
        var v2 = getArrayU8FromWasm0(r0, r1).slice();
        wasm.__wbindgen_free(r0, r1 * 1, 1);
        return v2;
    } finally {
        wasm.__wbindgen_add_to_stack_pointer(16);
    }
}

function _assertClass(instance, klass) {
    if (!(instance instanceof klass)) {
        throw new Error(`expected instance of ${klass.name}`);
    }
    return instance.ptr;
}
/**
 * @param {Foo} a
 * @returns {Foo}
 */
export function echo_struct(a) {
    _assertClass(a, Foo);
    var ptr0 = a.__destroy_into_raw();
    const ret = wasm.echo_struct(ptr0);
    return Foo.__wrap(ret);
}

function passArrayJsValueToWasm0(array, malloc) {
    const ptr = malloc(array.length * 4, 4) >>> 0;
    const mem = getDataViewMemory0();
    for (let i = 0; i < array.length; i++) {
        mem.setUint32(ptr + 4 * i, addHeapObject(array[i]), true);
    }
    WASM_VECTOR_LEN = array.length;
    return ptr;
}

function getArrayJsValueFromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    const mem = getDataViewMemory0();
    const result = [];
    for (let i = ptr; i < ptr + 4 * len; i += 4) {
        result.push(takeObject(mem.getUint32(i, true)));
    }
    return result;
}
/**
 * @param {(Foo)[]} a
 * @returns {(Foo)[]}
 */
export function echo_vec_struct(a) {
    try {
        const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
        const ptr0 = passArrayJsValueToWasm0(a, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.echo_vec_struct(retptr, ptr0, len0);
        var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
        var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
        var v2 = getArrayJsValueFromWasm0(r0, r1).slice();
        wasm.__wbindgen_free(r0, r1 * 4, 4);
        return v2;
    } finally {
        wasm.__wbindgen_add_to_stack_pointer(16);
    }
}

function isLikeNone(x) {
    return x === undefined || x === null;
}
/**
 * @param {number | undefined | null} [a]
 * @returns {number | undefined}
 */
export function echo_option_u8(a) {
    const ret = wasm.echo_option_u8(isLikeNone(a) ? 0xFFFFFF : a);
    return ret === 0xFFFFFF ? undefined : ret;
}

/**
 * @param {number | undefined | null} [a]
 * @returns {number | undefined}
 */
export function echo_option_i8(a) {
    const ret = wasm.echo_option_i8(isLikeNone(a) ? 0xFFFFFF : a);
    return ret === 0xFFFFFF ? undefined : ret;
}

/**
 * @param {number | undefined | null} [a]
 * @returns {number | undefined}
 */
export function echo_option_u16(a) {
    const ret = wasm.echo_option_u16(isLikeNone(a) ? 0xFFFFFF : a);
    return ret === 0xFFFFFF ? undefined : ret;
}

/**
 * @param {number | undefined | null} [a]
 * @returns {number | undefined}
 */
export function echo_option_i16(a) {
    const ret = wasm.echo_option_i16(isLikeNone(a) ? 0xFFFFFF : a);
    return ret === 0xFFFFFF ? undefined : ret;
}

/**
 * @param {number | undefined | null} [a]
 * @returns {number | undefined}
 */
export function echo_option_u32(a) {
    try {
        const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
        wasm.echo_option_u32(retptr, !isLikeNone(a), isLikeNone(a) ? 0 : a);
        var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
        var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
        return r0 === 0 ? undefined : r1 >>> 0;
    } finally {
        wasm.__wbindgen_add_to_stack_pointer(16);
    }
}

/**
 * @param {number | undefined | null} [a]
 * @returns {number | undefined}
 */
export function echo_option_i32(a) {
    try {
        const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
        wasm.echo_option_i32(retptr, !isLikeNone(a), isLikeNone(a) ? 0 : a);
        var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
        var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
        return r0 === 0 ? undefined : r1;
    } finally {
        wasm.__wbindgen_add_to_stack_pointer(16);
    }
}

/**
 * @param {bigint | undefined | null} [a]
 * @returns {bigint | undefined}
 */
export function echo_option_u64(a) {
    try {
        const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
        wasm.echo_option_u64(retptr, !isLikeNone(a), isLikeNone(a) ? BigInt(0) : a);
        var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
        var r2 = getDataViewMemory0().getBigInt64(retptr + 8 * 1, true);
        return r0 === 0 ? undefined : BigInt.asUintN(64, r2);
    } finally {
        wasm.__wbindgen_add_to_stack_pointer(16);
    }
}

/**
 * @param {bigint | undefined | null} [a]
 * @returns {bigint | undefined}
 */
export function echo_option_i64(a) {
    try {
        const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
        wasm.echo_option_i64(retptr, !isLikeNone(a), isLikeNone(a) ? BigInt(0) : a);
        var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
        var r2 = getDataViewMemory0().getBigInt64(retptr + 8 * 1, true);
        return r0 === 0 ? undefined : r2;
    } finally {
        wasm.__wbindgen_add_to_stack_pointer(16);
    }
}

/**
 * @param {number | undefined | null} [a]
 * @returns {number | undefined}
 */
export function echo_option_usize(a) {
    try {
        const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
        wasm.echo_option_usize(retptr, !isLikeNone(a), isLikeNone(a) ? 0 : a);
        var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
        var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
        return r0 === 0 ? undefined : r1 >>> 0;
    } finally {
        wasm.__wbindgen_add_to_stack_pointer(16);
    }
}

/**
 * @param {number | undefined | null} [a]
 * @returns {number | undefined}
 */
export function echo_option_isize(a) {
    try {
        const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
        wasm.echo_option_isize(retptr, !isLikeNone(a), isLikeNone(a) ? 0 : a);
        var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
        var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
        return r0 === 0 ? undefined : r1;
    } finally {
        wasm.__wbindgen_add_to_stack_pointer(16);
    }
}

/**
 * @param {number | undefined | null} [a]
 * @returns {number | undefined}
 */
export function echo_option_f32(a) {
    try {
        const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
        wasm.echo_option_f32(retptr, !isLikeNone(a), isLikeNone(a) ? 0 : a);
        var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
        var r1 = getDataViewMemory0().getFloat32(retptr + 4 * 1, true);
        return r0 === 0 ? undefined : r1;
    } finally {
        wasm.__wbindgen_add_to_stack_pointer(16);
    }
}

/**
 * @param {number | undefined | null} [a]
 * @returns {number | undefined}
 */
export function echo_option_f64(a) {
    try {
        const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
        wasm.echo_option_f64(retptr, !isLikeNone(a), isLikeNone(a) ? 0 : a);
        var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
        var r2 = getDataViewMemory0().getFloat64(retptr + 8 * 1, true);
        return r0 === 0 ? undefined : r2;
    } finally {
        wasm.__wbindgen_add_to_stack_pointer(16);
    }
}

/**
 * @param {boolean | undefined | null} [a]
 * @returns {boolean | undefined}
 */
export function echo_option_bool(a) {
    const ret = wasm.echo_option_bool(isLikeNone(a) ? 0xFFFFFF : a ? 1 : 0);
    return ret === 0xFFFFFF ? undefined : ret !== 0;
}

/**
 * @param {string | undefined | null} [a]
 * @returns {string | undefined}
 */
export function echo_option_char(a) {
    const char0 = isLikeNone(a) ? 0xFFFFFF : a.codePointAt(0);
if (char0 !== 0xFFFFFF) { _assertChar(char0); }
const ret = wasm.echo_option_char(char0);
return ret === 0xFFFFFF ? undefined : String.fromCodePoint(ret);
}

/**
 * @param {string | undefined | null} [a]
 * @returns {string | undefined}
 */
export function echo_option_string(a) {
    try {
        const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
        var ptr0 = isLikeNone(a) ? 0 : passStringToWasm0(a, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        wasm.echo_option_string(retptr, ptr0, len0);
        var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
        var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
        let v2;
        if (r0 !== 0) {
            v2 = getStringFromWasm0(r0, r1).slice();
            wasm.__wbindgen_free(r0, r1 * 1, 1);
        }
        return v2;
    } finally {
        wasm.__wbindgen_add_to_stack_pointer(16);
    }
}

/**
 * @param {Uint8Array | undefined | null} [a]
 * @returns {Uint8Array | undefined}
 */
export function echo_option_vec_u8(a) {
    try {
        const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
        var ptr0 = isLikeNone(a) ? 0 : passArray8ToWasm0(a, wasm.__wbindgen_malloc);
        var len0 = WASM_VECTOR_LEN;
        wasm.echo_option_vec_u8(retptr, ptr0, len0);
        var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
        var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
        let v2;
        if (r0 !== 0) {
            v2 = getArrayU8FromWasm0(r0, r1).slice();
            wasm.__wbindgen_free(r0, r1 * 1, 1);
        }
        return v2;
    } finally {
        wasm.__wbindgen_add_to_stack_pointer(16);
    }
}

/**
 * @param {Foo | undefined | null} [a]
 * @returns {Foo | undefined}
 */
export function echo_option_struct(a) {
    let ptr0 = 0;
    if (!isLikeNone(a)) {
        _assertClass(a, Foo);
        ptr0 = a.__destroy_into_raw();
    }
    const ret = wasm.echo_option_struct(ptr0);
    return ret === 0 ? undefined : Foo.__wrap(ret);
}

/**
 * @param {(Foo)[] | undefined | null} [a]
 * @returns {(Foo)[] | undefined}
 */
export function echo_option_vec_struct(a) {
    try {
        const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
        var ptr0 = isLikeNone(a) ? 0 : passArrayJsValueToWasm0(a, wasm.__wbindgen_malloc);
        var len0 = WASM_VECTOR_LEN;
        wasm.echo_option_vec_struct(retptr, ptr0, len0);
        var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
        var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
        let v2;
        if (r0 !== 0) {
            v2 = getArrayJsValueFromWasm0(r0, r1).slice();
            wasm.__wbindgen_free(r0, r1 * 4, 4);
        }
        return v2;
    } finally {
        wasm.__wbindgen_add_to_stack_pointer(16);
    }
}

const FooFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_foo_free(ptr >>> 0, 1));

export class Foo {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(Foo.prototype);
        obj.__wbg_ptr = ptr;
        FooFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    static __unwrap(jsValue) {
        if (!(jsValue instanceof Foo)) {
            return 0;
        }
        return jsValue.__destroy_into_raw();
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        FooFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_foo_free(ptr, 0);
    }
}

export function __wbg_foo_new(arg0) {
    const ret = Foo.__wrap(arg0);
    return addHeapObject(ret);
};

export function __wbg_foo_unwrap(arg0) {
    const ret = Foo.__unwrap(takeObject(arg0));
    return ret;
};

export function __wbindgen_debug_string(arg0, arg1) {
    const ret = debugString(getObject(arg1));
    const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len1 = WASM_VECTOR_LEN;
    getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
    getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
};

export function __wbindgen_throw(arg0, arg1) {
    throw new Error(getStringFromWasm0(arg0, arg1));
};

export function __wbindgen_object_drop_ref(arg0) {
    takeObject(arg0);
};

