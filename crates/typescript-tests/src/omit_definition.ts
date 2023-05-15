import * as wbg from '../pkg/typescript_tests';

wbg.take_function((value) => {
    // `value` should be inferred as `number` because of the
    // custom typescript section. If `typescript = false` does
    // not prevent the generation of a signature that takes any
    // function, then this will trigger a noImplicitAny error.
    console.log(value);
});

declare function assert<T>(message: T): void;

type EnableIfEnum = "MyEnum" extends keyof typeof wbg ? never : string;
assert<EnableIfEnum>("`MyEnum` type should not be exported.");

type EnableIfStruct = "MyStruct" extends keyof typeof wbg ? never : string;
assert<EnableIfStruct>("`MyStruct` type should not be exported.");

type EnableIfField = "field" extends keyof wbg.MyExportedStruct ? never : string;
assert<EnableIfField>("`field` should not exist on `MyExportedStruct`.");

type EnableIfMethod = "method" extends keyof wbg.MyExportedStruct ? never : string;
assert<EnableIfMethod>("`method` should not exist on `MyExportedStruct`.");

type EnableIfStaticMethod = "static_func" extends keyof typeof wbg.MyExportedStruct ? never : string;
assert<EnableIfStaticMethod>("`static_func` should not exist on `MyExportedStruct`.");
