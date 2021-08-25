import * as wbg from '../pkg/typescript_tests';
import * as wasm from '../pkg/typescript_tests_bg.wasm';

const wbg_async_greet: (a: string) => Promise<void> = wbg.async_greet;
const wasm_async_greet: (a: number, b: number) => number = wasm.async_greet;
const async_take_and_return_bool: (a: boolean) => Promise<boolean> = wbg.async_take_and_return_bool;
