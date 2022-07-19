import * as wbg from "../pkg/typescript_tests";

const bar: wbg.Bar = new wbg.Bar(1);
const barAsJson: Object = bar.toJSON();
const barAsString: string = bar.toString();
const _ = bar.foo;
