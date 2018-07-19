use super::project;

#[test]
fn works() {
    project()
        .file(
            "src/lib.rs",
            r#"
            #![feature(use_extern_macros, wasm_import_module)]
            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            #[wasm_bindgen]
            pub struct Fruit {
                name: String,
            }
            #[wasm_bindgen]
            impl Fruit {
                #[wasm_bindgen(method)]
                pub fn name(&self) -> String {
                    self.name.clone()
                }
                #[wasm_bindgen(constructor)]
                pub fn new(name: String) -> Self {
                    Fruit {
                        name,
                    }
                }
            }
            #[wasm_bindgen]
            pub fn eat(_fruit: Fruit) { }
        "#,
        )
        .file(
            "test.js",
            r#"
            import * as wasm from './out';
            const targetMessage = 'Attempt to use a moved value';
            function assertEq(a, b) {
                console.log(a, '?=', b);
                if (a === b)
                return;
                throw new Error('not equal');
            }
            export function test() {
                useMoved();
                moveMoved();
            }
            export function useMoved() {
                // create a new struct
                let apple = new wasm.Fruit('apple');
                // sanity check that this method works
                let name = apple.name();
                // consume the struct
                wasm.eat(apple);
                // try and use the moved apple again
                try {
                    let movedName = apple.name();
                } catch (e) {
                    assertEq(e.message, targetMessage);
                }
            }
            export function moveMoved() {
                let pear = new wasm.Fruit('pear');
                let name = pear.name();
                wasm.eat(pear);
                try {
                    wasm.eat(pear);
                } catch (e) {
                    assertEq(e.message, targetMessage);
                }
            }
        "#,
        )
        .test();
}
