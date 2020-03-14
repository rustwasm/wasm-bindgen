import * as wbg from '../pkg/typescript_tests';

const style: wbg.TextStyle = new wbg.TextStyle({
  bold: true,
  italic: true,
  size: 42,
});

const optional_style: wbg.TextStyle = wbg.TextStyle.optional_new();
