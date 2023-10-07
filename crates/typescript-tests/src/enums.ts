import * as wbg from '../pkg/typescript_tests';

const a1: wbg.Foo = wbg.Foo.A;
const a2: wbg.Foo.A = wbg.Foo.A;
const a3: wbg.Foo.A = 1;
const b1: wbg.Foo = wbg.Foo.B;
const b2: wbg.Foo.B = wbg.Foo.B;
const b3: wbg.Foo.B = 3;

const fn_expects_enum: (_: wbg.Foo) => void = wbg.fn_expects_enum;
const fn_returns_enum: () => wbg.Foo = wbg.fn_returns_enum;
const fn_expects_option_enum: (_?: wbg.Foo | undefined) => void = wbg.fn_expects_option_enum;
const fn_returns_option_enum: () => wbg.Foo | undefined = wbg.fn_returns_option_enum;
