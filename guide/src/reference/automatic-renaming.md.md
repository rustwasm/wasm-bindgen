# Automatic renaming

`wasm-bindgen` encourages users to follow [Rust naming conventions](https://rust-lang.github.io/api-guidelines/naming.html) in their projects. However, while JavaScript has no official naming conventions, the JavaScript community typically uses camelCase for function, method, and field/property names. This makes it quite tedious to define JavaScript APIs in Rust (both importing and exporting), since a lot of functions and properties require a `js_name` attribute to explicitly specify the camelCase JavaScript name.

Here's an overview comparing Rust and JavaScript naming conventions:

| Feature                                 | Rust                   | JavaScript                          |
| --------------------------------------- | ---------------------- | ----------------------------------- |
| `class`/`struct`                        | `PascalCase`           | `PascalCase`                        |
| Anything type-like, e.g. `type`, `enum` | `PascalCase`           | `PascalCase`                        |
| Enum variants                           | `PascalCase`           | `PascalCase` (in TS)                |
| Functions                               | `snake_case`           | `camelCase`                         |
| Methods                                 | `snake_case`           | `camelCase`                         |
| Fields, properties (getter/setter)      | `snake_case`           | `camelCase`                         |
| Local variables                         | `snake_case`           | `camelCase`                         |
| Constants                               | `SCREAMING_SNAKE_CASE` | `SCREAMING_SNAKE_CASE` (very often) |

(While some JavaScript libraries may use different conventions, this table still holds true for most JavaScript code.)

TODO: write docs
