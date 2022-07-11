import * as wbg from '../pkg/typescript_tests';

const colorWithGetters: wbg.ColorWithGetters = new wbg.ColorWithGetters;
const _a = colorWithGetters.r;
const _b = wbg.ColorWithGetters.color_space;

const colorWithSetters: wbg.ColorWithSetters = new wbg.ColorWithSetters;
colorWithSetters.r = 1;
wbg.ColorWithSetters.color_space = "Linear sRGB";

const colorWithGetterAndSetter: wbg.ColorWithGetterAndSetter = new wbg.ColorWithGetterAndSetter;
colorWithGetterAndSetter.r = 1;
const _c = colorWithGetterAndSetter.r;

const colorWithReadonly: wbg.ColorWithReadonly = new wbg.ColorWithReadonly(1, 2, 3);
const _r: number = colorWithReadonly.r;
colorWithReadonly.a = 4;
