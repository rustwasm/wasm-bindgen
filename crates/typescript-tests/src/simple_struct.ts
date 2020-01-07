import * as wbg from '../pkg/typescript_tests';

const a = new wbg.A();
wbg.A.other();
a.foo();
a.free();
const b: boolean = a.ret_bool()
a.take_bool(b);
a.take_many(b, 1, 2);
