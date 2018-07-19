use super::project;

#[test]
fn works() {
    project()
        .requires_bigint()
        .file(
            "src/lib.rs",
            r#"
            #![feature(use_extern_macros, wasm_import_module)]

            extern crate wasm_bindgen;

            use wasm_bindgen::prelude::*;

            #[wasm_bindgen(module = "./test")]
            extern {
                fn js_i64_round(a: i64) -> i64;
                fn js_u64_round(a: u64) -> u64;
            }

            #[wasm_bindgen]
            pub fn zero() -> u64 { 0 }
            #[wasm_bindgen]
            pub fn one() -> u64 { 1 }
            #[wasm_bindgen]
            pub fn neg_one() -> i64 { -1 }
            #[wasm_bindgen]
            pub fn u32_max() -> u64 { u32::max_value() as u64 }
            #[wasm_bindgen]
            pub fn i32_min() -> i64 { i32::min_value() as i64 }
            #[wasm_bindgen]
            pub fn u64_max() -> u64 { u64::max_value() }
            #[wasm_bindgen]
            pub fn i64_min() -> i64 { i64::min_value() }

            #[wasm_bindgen]
            pub fn i64_round(a: i64) -> i64 { js_i64_round(a) }
            #[wasm_bindgen]
            pub fn u64_round(a: u64) -> u64 { js_u64_round(a) }

            #[wasm_bindgen]
            pub fn i64_slice(a: &[i64]) -> Vec<i64> { a.to_vec() }

            #[wasm_bindgen]
            pub fn u64_slice(a: &[u64]) -> Vec<u64> { a.to_vec() }
        "#,
        )
        .file(
            "test.js",
            r#"
            import * as wasm from './out';

            function assertEq(a, b) {
              console.log(a, '?=', b);
              if (a === b)
                return;
              throw new Error('not equal');
            }

            function assertArrayEq(a, b) {
              console.log(a, '?=', b);
              if (a.length !== b.length)
                throw new Error('not equal');
              for (let i = 0; i < a.length; i++)
                assertEq(a[i], b[i]);
            }

            export function test() {
              assertEq(wasm.zero(), BigInt(0));
              assertEq(wasm.one(), BigInt(1));
              assertEq(wasm.neg_one(), BigInt(-1));
              assertEq(wasm.u32_max(), BigInt(4294967295));
              assertEq(wasm.i32_min(), BigInt(-2147483648));
              assertEq(wasm.u64_max(), BigInt('18446744073709551615'));
              assertEq(wasm.i64_min(), BigInt('-9223372036854775808'));

              assertEq(wasm.i64_round(BigInt(0)), BigInt(0));
              assertEq(wasm.i64_round(BigInt(1)), BigInt(1));
              assertEq(wasm.i64_round(BigInt(-1)), BigInt(-1));
              assertEq(wasm.u64_round(BigInt(0)), BigInt(0));
              assertEq(wasm.u64_round(BigInt(1)), BigInt(1));
              assertEq(wasm.u64_round(BigInt(1) << BigInt(64)), BigInt(0));

              const u64_max = BigInt('18446744073709551615');
              const i64_min = BigInt('-9223372036854775808');
              assertEq(wasm.i64_round(i64_min), i64_min);
              assertEq(wasm.u64_round(u64_max), u64_max);

              assertArrayEq(wasm.u64_slice([]), new BigUint64Array());
              assertArrayEq(wasm.i64_slice([]), new BigInt64Array());
              const arr1 = new BigUint64Array([BigInt(1), BigInt(2)]);
              assertArrayEq(wasm.u64_slice([BigInt(1), BigInt(2)]), arr1);
              const arr2 = new BigInt64Array([BigInt(1), BigInt(2)]);
              assertArrayEq(wasm.i64_slice([BigInt(1), BigInt(2)]), arr2);

              assertArrayEq(wasm.i64_slice([i64_min]), new BigInt64Array([i64_min]));
              assertArrayEq(wasm.u64_slice([u64_max]), new BigUint64Array([u64_max]));
            }

            export function js_i64_round(a) { return a; }
            export function js_u64_round(a) { return a; }
        "#,
        )
        .test();
}
