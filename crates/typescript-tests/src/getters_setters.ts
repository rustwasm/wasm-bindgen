import * as wbg from '../pkg/typescript_tests';

const colorWithGetter: wbg.ColorWithGetter = new wbg.ColorWithGetter;
const _a = colorWithGetter.r;

const colorWithSetter: wbg.ColorWithSetter = new wbg.ColorWithSetter;
colorWithSetter.r = 1;

const colorWithGetterAndSetter: wbg.ColorWithGetterAndSetter = new wbg.ColorWithGetterAndSetter;
colorWithGetterAndSetter.r = 1;
const _b = colorWithGetterAndSetter.r;
