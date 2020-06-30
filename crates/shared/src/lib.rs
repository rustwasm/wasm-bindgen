#![doc(html_root_url = "https://docs.rs/wasm-bindgen-shared/0.2")]

// The schema is so unstable right now we just force it to change whenever this
// package's version changes, which happens on all publishes.
pub const SCHEMA_VERSION: &str = env!("CARGO_PKG_VERSION");

#[macro_export]
macro_rules! shared_api {
    ($mac:ident) => {
        $mac! {
        struct Program<'a> {
            exports: Vec<Export<'a>>,
            enums: Vec<Enum<'a>>,
            imports: Vec<Import<'a>>,
            structs: Vec<Struct<'a>>,
            typescript_custom_sections: Vec<&'a str>,
            local_modules: Vec<LocalModule<'a>>,
            inline_js: Vec<&'a str>,
            unique_crate_identifier: &'a str,
            package_json: Option<&'a str>,
        }

        struct Import<'a> {
            module: ImportModule<'a>,
            js_namespace: Option<Vec<String>>,
            kind: ImportKind<'a>,
        }

        enum ImportModule<'a> {
            None,
            Named(&'a str),
            RawNamed(&'a str),
            Inline(u32),
        }

        enum ImportKind<'a> {
            Function(ImportFunction<'a>),
            Static(ImportStatic<'a>),
            Type(ImportType<'a>),
            Enum(ImportEnum),
        }

        struct ImportFunction<'a> {
            shim: &'a str,
            catch: bool,
            variadic: bool,
            assert_no_shim: bool,
            method: Option<MethodData<'a>>,
            structural: bool,
            function: Function<'a>,
        }

        struct MethodData<'a> {
            class: &'a str,
            kind: MethodKind<'a>,
        }

        enum MethodKind<'a> {
            Constructor,
            Operation(Operation<'a>),
        }

        struct Operation<'a> {
            is_static: bool,
            kind: OperationKind<'a>,
        }

        enum OperationKind<'a> {
            Regular,
            Getter(&'a str),
            Setter(&'a str),
            IndexingGetter,
            IndexingSetter,
            IndexingDeleter,
        }

        struct ImportStatic<'a> {
            name: &'a str,
            shim: &'a str,
        }

        struct ImportType<'a> {
            name: &'a str,
            instanceof_shim: &'a str,
            vendor_prefixes: Vec<&'a str>,
        }

        struct ImportEnum {}

        struct Export<'a> {
            class: Option<&'a str>,
            comments: Vec<&'a str>,
            consumed: bool,
            function: Function<'a>,
            method_kind: MethodKind<'a>,
            start: bool,
        }

        struct Enum<'a> {
            name: &'a str,
            variants: Vec<EnumVariant<'a>>,
            comments: Vec<&'a str>,
            generate_typescript: bool,
        }

        struct EnumVariant<'a> {
            name: &'a str,
            value: u32,
            comments: Vec<&'a str>,
        }

        struct Function<'a> {
            arg_names: Vec<String>,
            name: &'a str,
            generate_typescript: bool,
        }

        struct Struct<'a> {
            name: &'a str,
            fields: Vec<StructField<'a>>,
            comments: Vec<&'a str>,
            is_inspectable: bool,
            generate_typescript: bool,
        }

        struct StructField<'a> {
            name: &'a str,
            readonly: bool,
            comments: Vec<&'a str>,
            generate_typescript: bool,
        }

        struct LocalModule<'a> {
            identifier: &'a str,
            contents: &'a str,
        }
        }
    }; // end of mac case
} // end of mac definition

pub fn new_function(struct_name: &str) -> String {
    let mut name = format!("__wbg_");
    name.extend(struct_name.chars().flat_map(|s| s.to_lowercase()));
    name.push_str("_new");
    return name;
}

pub fn free_function(struct_name: &str) -> String {
    let mut name = format!("__wbg_");
    name.extend(struct_name.chars().flat_map(|s| s.to_lowercase()));
    name.push_str("_free");
    return name;
}

pub fn free_function_export_name(function_name: &str) -> String {
    function_name.to_string()
}

pub fn struct_function_export_name(struct_: &str, f: &str) -> String {
    let mut name = struct_
        .chars()
        .flat_map(|s| s.to_lowercase())
        .collect::<String>();
    name.push_str("_");
    name.push_str(f);
    return name;
}

pub fn struct_field_get(struct_: &str, f: &str) -> String {
    let mut name = String::from("__wbg_get_");
    name.extend(struct_.chars().flat_map(|s| s.to_lowercase()));
    name.push_str("_");
    name.push_str(f);
    return name;
}

pub fn struct_field_set(struct_: &str, f: &str) -> String {
    let mut name = String::from("__wbg_set_");
    name.extend(struct_.chars().flat_map(|s| s.to_lowercase()));
    name.push_str("_");
    name.push_str(f);
    return name;
}

pub fn version() -> String {
    let mut v = env!("CARGO_PKG_VERSION").to_string();
    if let Some(s) = option_env!("WBG_VERSION") {
        v.push_str(" (");
        v.push_str(s);
        v.push_str(")");
    }
    return v;
}
