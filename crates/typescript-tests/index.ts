import * as wbg from './pkg/typescript_tests';
import * as wasm from './pkg/typescript_tests_bg';

const a1: (a: string) => void = wbg.greet;
const a2: (a: number, b: number) => void = wasm.greet;
const a3: WebAssembly.Memory = wasm.memory;

const c = new wbg.A();
wbg.A.other();
c.foo();
c.free();
