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

export function __wbg_now_df71ca5a8f45b624() {
    const ret = Date.now();
    const bigint1 = typeof ret === 'number' ? BigInt(Math.trunc(ret)) : ret;
    return bigint1;
};

