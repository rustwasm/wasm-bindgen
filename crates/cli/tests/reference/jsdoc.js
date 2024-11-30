let wasm;
export function __wbg_set_wasm(val) {
    wasm = val;
}

/**
 * Manually documented function
 *
 * @param {number} arg - This is my arg. It is mine.
 * @returns {number} to whence I came
 */
export function docme(arg) {
    const ret = wasm.docme(arg);
    return ret >>> 0;
}

/**
 * Manually documented function
 *
 * @param {number} arg - This is my arg. It is mine.
 * @returns to whence I came
 */
export function docme_skip(arg) {
    const ret = wasm.docme_skip(arg);
    return ret >>> 0;
}

/**
 * Regular documentation.
 * @param {number} arg
 * @returns {number}
 */
export function i_has_docs(arg) {
    const ret = wasm.i_has_docs(arg);
    return ret >>> 0;
}

function isLikeNone(x) {
    return x === undefined || x === null;
}
/**
 * Regular documentation.
 *
 * @param {number | undefined} [b=0] Description of `arg`.
 * @param {number | undefined} [d] Another description.
 * @param {number} a
 * @param {number | undefined} [c]
 * @returns {number}
 */
export function add(a, b, c, d) {
    const ret = wasm.add(a, isLikeNone(b) ? 0x100000001 : (b) >>> 0, isLikeNone(c) ? 0x100000001 : (c) >>> 0, isLikeNone(d) ? 0x100000001 : (d) >>> 0);
    return ret >>> 0;
}

/**
 * ```js
 * function foo() {
 *     return 1;
 * }
 * ```
 * @param {number} arg
 */
export function indent_test1(arg) {
    wasm.indent_test1(arg);
}

/**
 * ```js
 * function foo() {
 *     return 1;
 * }
 * ```
 */
export function indent_test2(arg) {
    wasm.indent_test2(arg);
}

export function __wbindgen_init_externref_table() {
    const table = wasm.__wbindgen_export_0;
    const offset = table.grow(4);
    table.set(0, undefined);
    table.set(offset + 0, undefined);
    table.set(offset + 1, null);
    table.set(offset + 2, true);
    table.set(offset + 3, false);
    ;
};

