import * as wbg from '../pkg/typescript_tests';

const a2 = new wbg.A();
wbg.A[wbg.B.other]();
a2[wbg.B.greet]("Hello World!");
const b2: boolean = a2[wbg.B.take_and_return](true)