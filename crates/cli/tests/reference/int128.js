let wasm;
export function __wbg_set_wasm(val) {
    wasm = val;
}

/**
 * @param {bigint} a
 * @returns {bigint}
 */
export function echo_i128(a) {
    const ret = wasm.echo_i128(a, a >> 64n);
    return (BigInt.asUintN(64, ret[0]) | (ret[1] << 64n));
}

/**
 * @param {bigint} a
 * @returns {bigint}
 */
export function echo_u128(a) {
    const ret = wasm.echo_u128(a, a >> 64n);
    return (BigInt.asUintN(64, ret[0]) | (BigInt.asUintN(64, ret[1]) << 64n));
}

function isLikeNone(x) {
    return x === undefined || x === null;
}
/**
 * @param {bigint | undefined} [a]
 * @returns {bigint | undefined}
 */
export function echo_option_i128(a) {
    const ret = wasm.echo_option_i128(!isLikeNone(a), isLikeNone(a) ? 0n : a, isLikeNone(a) ? 0n : a >> 64n);
    return ret[0] === 0 ? undefined : (BigInt.asUintN(64, ret[1]) | (ret[2] << 64n));
}

/**
 * @param {bigint | undefined} [a]
 * @returns {bigint | undefined}
 */
export function echo_option_u128(a) {
    const ret = wasm.echo_option_u128(!isLikeNone(a), isLikeNone(a) ? 0n : a, isLikeNone(a) ? 0n : a >> 64n);
    return ret[0] === 0 ? undefined : (BigInt.asUintN(64, ret[1]) | (BigInt.asUintN(64, ret[2]) << 64n));
}

const heap = new Array(128).fill(undefined);

heap.push(undefined, null, true, false);

function getObject(idx) { return heap[idx]; }

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
/**
 * @returns {bigint}
 */
export function throw_i128() {
    const ret = wasm.throw_i128();
    if (ret[3]) {
        throw takeObject(ret[2]);
    }
    return (BigInt.asUintN(64, ret[0]) | (ret[1] << 64n));
}
