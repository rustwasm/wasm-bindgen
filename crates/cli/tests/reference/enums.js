let wasm;
export function __wbg_set_wasm(val) {
    wasm = val;
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
/**
 * @param {Color} color
 * @returns {Color}
 */
export function enum_echo(color) {
    const ret = wasm.enum_echo(color);
    return ret;
}

function isLikeNone(x) {
    return x === undefined || x === null;
}
/**
 * @param {Color | undefined} [color]
 * @returns {Color | undefined}
 */
export function option_enum_echo(color) {
    const ret = wasm.option_enum_echo(isLikeNone(color) ? 3 : color);
    return ret === 3 ? undefined : ret;
}

/**
 * @param {Color} color
 * @returns {ColorName}
 */
export function get_name(color) {
    const ret = wasm.get_name(color);
    return __wbindgen_enum_ColorName[ret];
}

/**
 * @param {ColorName | undefined} [color]
 * @returns {ColorName | undefined}
 */
export function option_string_enum_echo(color) {
    const ret = wasm.option_string_enum_echo(isLikeNone(color) ? 4 : ((__wbindgen_enum_ColorName.indexOf(color) + 1 || 4) - 1));
    return __wbindgen_enum_ColorName[ret];
}

/**
 * A color.
 * @enum {0|1|2}
 */
export const Color = Object.freeze({ Green:0,"0":"Green",Yellow:1,"1":"Yellow",Red:2,"2":"Red", });

/**
 * The name of a color.
 * @typedef {"green" | "yellow" | "red"} ColorName
 * @type {ColorName[]}
 */
const __wbindgen_enum_ColorName = ["green", "yellow", "red"];

/**
 * An unused string enum.
 * @typedef {"foo" | "bar"} FooBar
 * @type {FooBar[]}
 */
const __wbindgen_enum_FooBar = ["foo", "bar"];

/**
 * @typedef {"foo" | "bar"} PrivateStringEnum
 * @type {PrivateStringEnum[]}
 */
const __wbindgen_enum_PrivateStringEnum = ["foo", "bar"];

export function __wbindgen_throw(arg0, arg1) {
    throw new Error(getStringFromWasm0(arg0, arg1));
};

