use super::project;

#[test]
fn top_level_enum() {
    project()
        .file(
            "shape.webidl",
            r#"
            enum ShapeType { "circle", "square" };
            
            [Constructor(ShapeType kind)]
            interface Shape {
                [Pure]
                boolean isSquare();

                [Pure]
                boolean isCircle();
            };
        "#,
        )
        .file(
            "shape.mjs",
            r#"
            export class Shape {
                constructor(kind) {
                    this.kind = kind;
                }

                isSquare() {
                    return this.kind === 'square';
                }

                isCircle() {
                    return this.kind === 'circle';
                }
            }
            "#,
        )
        .file(
            "src/lib.rs",
            r#"
            #![feature(use_extern_macros, wasm_import_module)]

            extern crate wasm_bindgen;

            use wasm_bindgen::prelude::*;

            pub mod shape;

            use shape::{Shape, ShapeType};

            #[wasm_bindgen]
            pub fn test() {
                let circle = Shape::new(ShapeType::Circle).unwrap();
                let square = Shape::new(ShapeType::Square).unwrap();
                assert!(circle.is_circle());
                assert!(!circle.is_square());
                assert!(square.is_square());
                assert!(!square.is_circle());
            }
        "#,
        )
        .test();
}
