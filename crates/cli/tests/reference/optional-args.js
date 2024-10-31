let wasm;
export function __wbg_set_wasm(val) {
    wasm = val;
}


function isLikeNone(x) {
    return x === undefined || x === null;
}
/**
 * @param {number | null} [a]
 * @param {number | null} [b]
 * @param {number | null} [c]
 */
export function all_optional(a, b, c) {
    wasm.all_optional(isLikeNone(a) ? 0x100000001 : (a) >>> 0, isLikeNone(b) ? 0x100000001 : (b) >>> 0, isLikeNone(c) ? 0x100000001 : (c) >>> 0);
}

/**
 * @param {number | null | undefined} a
 * @param {number} b
 * @param {number | null} [c]
 */
export function some_optional(a, b, c) {
    wasm.some_optional(isLikeNone(a) ? 0x100000001 : (a) >>> 0, b, isLikeNone(c) ? 0x100000001 : (c) >>> 0);
}

