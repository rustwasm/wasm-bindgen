let wasm;
export function __wbg_set_wasm(val) {
    wasm = val;
}


function notDefined(what) { return () => { throw new Error(`${what} is not defined`); }; }

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
/**
 * @param {number} a
 * @returns {number}
 */
export function export_from_rust(a) {
    const ret = wasm.export_from_rust(a);
    return ret >>> 0;
}

function isLikeNone(x) {
    return x === undefined || x === null;
}

const RustStructFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_ruststruct_free(ptr >>> 0, 1));

export class RustStruct {

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        RustStructFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_ruststruct_free(ptr, 0);
    }
    /**
     * @returns {number}
     */
    get foo() {
        const ret = wasm.__wbg_get_ruststruct_foo(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @param {number} arg0
     */
    set foo(arg0) {
        wasm.__wbg_set_ruststruct_foo(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {number}
     */
    get someCoolField() {
        const ret = wasm.__wbg_get_ruststruct_someCoolField(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @param {number} arg0
     */
    set someCoolField(arg0) {
        wasm.__wbg_set_ruststruct_someCoolField(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {number}
     */
    get another_field_for_you() {
        const ret = wasm.__wbg_get_ruststruct_another_field_for_you(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @param {number} arg0
     */
    set another_field_for_you(arg0) {
        wasm.__wbg_set_ruststruct_another_field_for_you(this.__wbg_ptr, arg0);
    }
    static i_dont_get_renamed() {
        wasm.ruststruct_i_dont_get_renamed();
    }
    /**
     * @param {number | undefined} [amount]
     */
    incrementFoo(amount) {
        wasm.ruststruct_incrementFoo(this.__wbg_ptr, isLikeNone(amount) ? 0x100000001 : (amount) >>> 0);
    }
    /**
     * @param {number} foo
     */
    setFoo(foo) {
        wasm.ruststruct_setFoo(this.__wbg_ptr, foo);
    }
    /**
     * @returns {number}
     */
    get_another() {
        const ret = wasm.ruststruct_get_another(this.__wbg_ptr);
        return ret >>> 0;
    }
}

export const __wbg_foobar_aa5072d28246f9cb = typeof foo_bar == 'function' ? foo_bar : notDefined('foo_bar');

export const __wbg_quxCorge_d8ec2d56c00b013f = typeof quxCorge == 'function' ? quxCorge : notDefined('quxCorge');

export const __wbg_yesNo_1feba4b061143a4c = typeof Baz.yesNo == 'function' ? Baz.yesNo : notDefined('Baz.yesNo');

export function __wbindgen_throw(arg0, arg1) {
    throw new Error(getStringFromWasm0(arg0, arg1));
};

