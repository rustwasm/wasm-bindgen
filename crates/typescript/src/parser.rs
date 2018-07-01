use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

use backend;
use backend::ast::{BindgenAttrs, Export, Function};
use proc_macro2;
use serde_json;
use syn;

use api_extractor;
use definitions::*;

pub fn ts_to_program(file_name: &str) -> backend::ast::Program {
    api_extractor::run();

    let ts_package = parse_json(file_name);

    let mut program = backend::ast::Program::default();

    for (name, export) in ts_package.exports {
        match export {
            TsExport::TsClass { members } => {
                for (member_name, member) in members {
                    match member {
                        TsClassMembers::TsProperty { .. } => {}
                        TsClassMembers::TsConstructor { .. } => {}
                        TsClassMembers::TsMethod {
                            parameters,
                            return_value,
                            ..
                        } => {
                            let function = build_function(member_name, parameters, return_value);

                            program.exports.push(Export {
                                class: Some(syn::Ident::new(&name, proc_macro2::Span::call_site())),
                                method: true,
                                mutable: false,
                                constructor: None,
                                function,
                            });
                        }
                    }
                }
            }

            TsExport::TsFunction {
                parameters,
                return_value,
            } => {
                let function = build_function(name, parameters, return_value);

                program.exports.push(Export {
                    class: None,
                    method: false,
                    mutable: false,
                    constructor: None,
                    function,
                });
            }
        }
    }

    program
}

fn parse_json(file_name: &str) -> TsPackage {
    let mut file = File::open(file_name).unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();

    serde_json::from_str(&data).unwrap()
}

fn build_function(
    name: String,
    parameters: HashMap<String, TsMethodProperty>,
    return_value: TsReturnValue,
) -> Function {
    let arguments = parameters
        .iter()
        .map(|(_name, property)| {
            let mut segments = syn::punctuated::Punctuated::new();
            segments.push(syn::PathSegment {
                ident: syn::Ident::new(&property.property_type, proc_macro2::Span::call_site()),
                arguments: syn::PathArguments::None,
            });

            syn::Type::Path(syn::TypePath {
                qself: None,
                path: syn::Path {
                    leading_colon: None,
                    segments,
                },
            })
        })
        .collect::<Vec<_>>();

    let mut ret_segments = syn::punctuated::Punctuated::new();
    ret_segments.push(syn::PathSegment {
        ident: syn::Ident::new(&return_value.property_type, proc_macro2::Span::call_site()),
        arguments: syn::PathArguments::None,
    });

    let ret = syn::Type::Path(syn::TypePath {
        qself: None,
        path: syn::Path {
            leading_colon: None,
            segments: ret_segments,
        },
    });

    let rust_decl = Box::new(syn::FnDecl {
        fn_token: Default::default(),
        generics: Default::default(),
        paren_token: Default::default(),
        //TODO investigate if inputs should be taken from arguments
        inputs: Default::default(),
        variadic: None,
        output: syn::ReturnType::Type(Default::default(), Box::new(ret.clone())),
    });

    Function {
        name: syn::Ident::new(&name, proc_macro2::Span::call_site()),
        arguments,
        ret: Some(ret),
        opts: BindgenAttrs::default(),
        rust_attrs: Vec::new(),
        rust_decl,
        rust_vis: syn::Visibility::Public(syn::VisPublic {
            pub_token: Default::default(),
        }),
    }
}
