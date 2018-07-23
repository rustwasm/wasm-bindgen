use super::project;

static SHAPE_IDL: &'static str = r#"
    enum ShapeType { "circle", "square" };
    
    [Constructor(ShapeType kind)]
    interface Shape {
        [Pure]
        boolean isSquare();

        [Pure]
        boolean isCircle();

        [Pure]
        ShapeType getShape();
    };
"#;

#[test]
fn top_level_enum() {
    project()
        .file("shape.webidl", SHAPE_IDL)
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

                getShape() {
                    return this.kind;
                }
            }
            "#,
        )
        .file(
            "src/lib.rs",
            r#"
            #![feature(use_extern_macros)]

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

#[test]
fn valid_enum_return() {
    project()
        .file("shape.webidl", SHAPE_IDL)
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

                getShape() {
                    return this.kind;
                }
            }
            "#,
        )
        .file(
            "src/lib.rs",
            r#"
            #![feature(use_extern_macros)]

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
                assert_eq!(circle.get_shape(), ShapeType::Circle);
                assert!(square.is_square());
                assert!(!square.is_circle());
                assert_eq!(square.get_shape(), ShapeType::Square);
            }
        "#,
        )
        .test();
}

#[test]
fn invalid_enum_return() {
    project()
        .file("shape.webidl", SHAPE_IDL)
        .file(
            "shape.mjs",
            r#"
            export class Shape {
                constructor(kind) {
                    this.kind = 'triangle'; // <-- invalid ShapeType
                }

                isSquare() {
                    return this.kind === 'square';
                }

                isCircle() {
                    return this.kind === 'circle';
                }

                getShape() {
                    return this.kind;
                }
            }
            "#,
        )
        .file(
            "src/lib.rs",
            r#"
            #![feature(use_extern_macros)]

            extern crate wasm_bindgen;

            use wasm_bindgen::prelude::*;

            pub mod shape;

            use shape::{Shape, ShapeType};

            #[wasm_bindgen]
            pub fn test() {
                let actually_a_triangle = Shape::new(ShapeType::Circle).unwrap();
                assert!(!actually_a_triangle.is_circle());
                assert!(!actually_a_triangle.is_square());
                match actually_a_triangle.get_shape() {
                    ShapeType::Circle | ShapeType::Square => assert!(false),
                    _ => {} // Success
                };
            }
        "#,
        )
        .test();
}
