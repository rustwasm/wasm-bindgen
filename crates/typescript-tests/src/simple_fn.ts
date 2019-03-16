import * as wbg from '../pkg/typescript_tests';
import * as wasm from '../pkg/typescript_tests_bg';

const wbg_greet: (a: string) => void = wbg.greet;
const wasm_greet: (a: number, b: number) => void = wasm.greet;