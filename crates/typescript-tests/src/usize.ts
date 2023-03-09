import * as wbg from "../pkg/typescript_tests";

const _f1: (x: number) => number = wbg.usize_identity;
const _f2: (x: number) => number = wbg.isize_identity;
const _f3: (x: number) => Promise<number> = wbg.async_usize_identity;
const _f4: (x: number) => Promise<number> = wbg.async_isize_identity;
