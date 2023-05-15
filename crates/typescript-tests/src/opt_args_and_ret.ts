import * as wbg from '../pkg/typescript_tests';

const opt_fn_mixed: (a: number | undefined, b: number, c?: number) => number | undefined = wbg.opt_fn_mixed;
const opt_fn_only: (a?: number, b?: number, c?: number) => number | undefined = wbg.opt_fn_only;
