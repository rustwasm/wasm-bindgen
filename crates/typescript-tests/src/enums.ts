import * as wbg from '../pkg/typescript_tests';

const wbg_accepts_enum: (a: wbg.Shape) => void = wbg.accepts_enum;
const wbg_take_and_return_enum: (a: wbg.Shape) => wbg.Shape = wbg.take_and_return_enum;

const wbg_accepts_option_enum: (a: wbg.Shape | undefined) => void = wbg.accepts_option_enum;
const wbg_take_and_return_option_enum: (a: wbg.Shape | undefined) => wbg.Shape | undefined = wbg.take_and_return_option_enum;
