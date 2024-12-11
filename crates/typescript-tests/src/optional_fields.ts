import * as wbg from '../pkg/typescript_tests';

const fields: wbg.Fields = null as unknown as wbg.Fields;

// optional fields read T | undefined
const _hallo: boolean | undefined = fields.hallo;

// and allow writing T | null | undefined
fields.hallo = undefined;
fields.hallo = null;
