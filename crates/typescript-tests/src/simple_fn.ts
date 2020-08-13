import * as wbg from '../pkg/typescript_tests';
import * as wasm from '../pkg/typescript_tests_bg.wasm';

const wbg_greet: (a: string) => void = wbg.greet;
const wasm_greet: (a: number, b: number) => void = wasm.greet;
const take_and_return_bool: (a: boolean) => boolean = wbg.take_and_return_bool;
