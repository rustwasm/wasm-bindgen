let wasm;
export function __wbg_set_wasm(val) {
    wasm = val;
}

/**
 * @returns {bigint}
 */
export function date_now() {
    const ret = wasm.date_now();
    return BigInt.asUintN(64, ret);
}

export function __wbg_now_179748341dc011f4() {
    const ret = Date.now();
    const bigint1 = typeof ret === 'number' ? BigInt(Math.trunc(ret)) : ret;
    return bigint1;
};

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

