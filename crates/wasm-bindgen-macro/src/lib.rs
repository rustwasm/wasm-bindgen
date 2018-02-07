#![recursion_limit = "128"]
#![feature(proc_macro)]

#[macro_use]
extern crate syn;
#[macro_use]
extern crate quote;
extern crate proc_macro;
extern crate proc_macro2;
extern crate serde_json;
extern crate wasm_bindgen_shared as shared;

use std::sync::atomic::*;

use proc_macro::TokenStream;
use proc_macro2::{Span, TokenNode, Delimiter, TokenTree};
use quote::{Tokens, ToTokens};

macro_rules! my_quote {
    ($($t:tt)*) => (quote_spanned!(Span::call_site() => $($t)*))
}

mod ast;

#[proc_macro]
pub fn wasm_bindgen(input: TokenStream) -> TokenStream {
    // Parse the input as a list of Rust items, reusing the `syn::File` parser.
    let file = syn::parse::<ast::File>(input)
        .expect("expected a set of valid Rust items");

    let mut ret = Tokens::new();

    let mut program = ast::Program {
        structs: Vec::new(),
        free_functions: Vec::new(),
        imports: Vec::new(),
        imported_structs: Vec::new(),
    };

    // Translate all input items into our own internal representation (the `ast`
    // module). We'll be panicking here on anything that we can't process

    for item in file.items.iter() {
        let item = match *item {
            ast::MyItem::ExternClass(ref c) => {
                program.push_extern_class(c);
                continue
            }
            ast::MyItem::Normal(ref item) => item,
        };

        match *item {
            syn::Item::Fn(ref f) => {
                item.to_tokens(&mut ret);
                program.free_functions.push(ast::Function::from(f));
            }
            syn::Item::Struct(ref s) => {
                item.to_tokens(&mut ret);
                let s = ast::Struct::from(s);
                if program.structs.iter().any(|a| a.name == s.name) {
                    panic!("redefinition of struct: {}", s.name);
                }
                program.structs.push(s);
            }
            syn::Item::Impl(ref i) => {
                item.to_tokens(&mut ret);
                program.push_impl(i);
            }
            syn::Item::ForeignMod(ref f) => {
                program.push_foreign_mod(f);
            }
            _ => panic!("unexpected item in bindgen macro"),
        }
    }

    // Generate wrappers for all the items that we've found

    for function in program.free_functions.iter() {
        bindgen_fn(function, &mut ret);
    }
    for s in program.structs.iter() {
        bindgen_struct(s, &mut ret);
    }
    for i in program.imports.iter() {
        bindgen_import(i, &mut ret);
    }
    for i in program.imported_structs.iter() {
        bindgen_imported_struct(i, &mut ret);
    }

    // Finally generate a static which will eventually be what lives in a custom
    // section of the wasm executable. For now it's just a plain old static, but
    // we'll eventually have it actually in its own section.

    static CNT: AtomicUsize = ATOMIC_USIZE_INIT;
    let generated_static_name = format!("__WASM_BINDGEN_GENERATED{}",
                                        CNT.fetch_add(1, Ordering::SeqCst));
    let generated_static_name = syn::Ident::from(generated_static_name);
    let mut generated_static_value = Tokens::new();
    let generated_static_length = program.wbg_literal(&mut generated_static_value);

    (my_quote! {
        #[no_mangle]
        #[allow(non_upper_case_globals)]
        pub static #generated_static_name: [u32; #generated_static_length] =
            [#generated_static_value];
    }).to_tokens(&mut ret);

    // println!("{}", ret);

    ret.into()
}

fn bindgen_fn(function: &ast::Function, into: &mut Tokens) {
    bindgen(&function.free_function_export_name(),
            function.rust_symbol(None),
            Receiver::FreeFunction(function.name),
            &function.arguments,
            function.ret.as_ref(),
            into)
}

fn bindgen_struct(s: &ast::Struct, into: &mut Tokens) {
    for f in s.functions.iter() {
        bindgen_struct_fn(s, f, into);
    }
    for f in s.methods.iter() {
        bindgen_struct_method(s, f, into);
    }

    let name = &s.name;
    let free_fn = s.free_function();
    let c = shared::name_to_descriptor(name.as_ref()) as u32;
    (my_quote! {
        impl ::wasm_bindgen::convert::WasmBoundary for #name {
            type Js = u32;
            const DESCRIPTOR: u32 = #c;

            fn into_js(self) -> u32 {
                Box::into_raw(Box::new(::wasm_bindgen::__rt::WasmRefCell::new(self))) as u32
            }

            unsafe fn from_js(js: u32) -> Self {
                let js = js as *mut ::wasm_bindgen::__rt::WasmRefCell<#name>;
                ::wasm_bindgen::__rt::assert_not_null(js);
                let js = Box::from_raw(js);
                js.borrow_mut(); // make sure no one's borrowing
                js.into_inner()
            }
        }

        impl ::wasm_bindgen::convert::FromRefWasmBoundary for #name {
            type RefAnchor = ::wasm_bindgen::__rt::Ref<'static, #name>;
            unsafe fn from_js_ref(js: Self::Js) -> Self::RefAnchor {
                let js = js as *mut ::wasm_bindgen::__rt::WasmRefCell<#name>;
                ::wasm_bindgen::__rt::assert_not_null(js);
                (*js).borrow()
            }
        }

        impl ::wasm_bindgen::convert::FromRefMutWasmBoundary for #name {
            type RefAnchor = ::wasm_bindgen::__rt::RefMut<'static, #name>;

            unsafe fn from_js_ref_mut(js: Self::Js) -> Self::RefAnchor {
                let js = js as *mut ::wasm_bindgen::__rt::WasmRefCell<#name>;
                ::wasm_bindgen::__rt::assert_not_null(js);
                (*js).borrow_mut()
            }
        }

        #[no_mangle]
        pub unsafe extern fn #free_fn(ptr: u32) {
            <#name as ::wasm_bindgen::convert::WasmBoundary>::from_js(ptr);
        }
    }).to_tokens(into);
}

fn bindgen_struct_fn(s: &ast::Struct, f: &ast::Function, into: &mut Tokens) {
    bindgen(&f.struct_function_export_name(s.name),
            f.rust_symbol(Some(s.name)),
            Receiver::StructFunction(s.name, f.name),
            &f.arguments,
            f.ret.as_ref(),
            into)
}

fn bindgen_struct_method(s: &ast::Struct, m: &ast::Method, into: &mut Tokens) {
    bindgen(&m.function.struct_function_export_name(s.name),
            m.function.rust_symbol(Some(s.name)),
            Receiver::StructMethod(s.name, m.mutable, m.function.name),
            &m.function.arguments,
            m.function.ret.as_ref(),
            into)
}

enum Receiver {
    FreeFunction(syn::Ident),
    StructFunction(syn::Ident, syn::Ident),
    StructMethod(syn::Ident, bool, syn::Ident),
}

fn bindgen(export_name: &syn::LitStr,
           generated_name: syn::Ident,
           receiver: Receiver,
           arguments: &[ast::Type],
           ret_type: Option<&ast::Type>,
           into: &mut Tokens) {
    let mut args = vec![];
    let mut arg_conversions = vec![];
    let mut converted_arguments = vec![];
    let ret = syn::Ident::from("_ret");

    let mut offset = 0;
    if let Receiver::StructMethod(class, _, _) = receiver {
        args.push(my_quote! { me: *mut ::wasm_bindgen::__rt::WasmRefCell<#class> });
        arg_conversions.push(my_quote! {
            ::wasm_bindgen::__rt::assert_not_null(me);
            let me = unsafe { &*me };
        });
        offset = 1;
    }

    for (i, ty) in arguments.iter().enumerate() {
        let i = i + offset;
        let ident = syn::Ident::from(format!("arg{}", i));
        match *ty {
            ast::Type::BorrowedStr => {
                let ptr = syn::Ident::from(format!("arg{}_ptr", i));
                let len = syn::Ident::from(format!("arg{}_len", i));
                args.push(my_quote! { #ptr: *const u8 });
                args.push(my_quote! { #len: usize });
                arg_conversions.push(my_quote! {
                    let #ident = unsafe {
                        let slice = ::std::slice::from_raw_parts(#ptr, #len);
                        ::std::str::from_utf8_unchecked(slice)
                    };
                });
            }
            ast::Type::String => {
                let ptr = syn::Ident::from(format!("arg{}_ptr", i));
                let len = syn::Ident::from(format!("arg{}_len", i));
                args.push(my_quote! { #ptr: *mut u8 });
                args.push(my_quote! { #len: usize });
                arg_conversions.push(my_quote! {
                    let #ident = unsafe {
                        let vec = ::std::vec::Vec::from_raw_parts(#ptr, #len, #len);
                        ::std::string::String::from_utf8_unchecked(vec)
                    };
                });
            }
            ast::Type::ByValue(ref t) => {
                args.push(my_quote! {
                    #ident: <#t as ::wasm_bindgen::convert::WasmBoundary >::Js
                });
                arg_conversions.push(my_quote! {
                    let #ident = unsafe {
                        <#t as ::wasm_bindgen::convert::WasmBoundary>
                            ::from_js(#ident)
                    };
                });
            }
            ast::Type::ByRef(ref ty) => {
                args.push(my_quote! {
                    #ident: <#ty as ::wasm_bindgen::convert::WasmBoundary>::Js
                });
                arg_conversions.push(my_quote! {
                    let #ident = unsafe {
                        <#ty as ::wasm_bindgen::convert::FromRefWasmBoundary>
                            ::from_js_ref(#ident)
                    };
                    let #ident = &*#ident;
                });
            }
            ast::Type::ByMutRef(ref ty) => {
                args.push(my_quote! {
                    #ident: <#ty as ::wasm_bindgen::convert::WasmBoundary>::Js
                });
                arg_conversions.push(my_quote! {
                    let mut #ident = unsafe {
                        <#ty as ::wasm_bindgen::convert::FromRefMutWasmBoundary>
                            ::from_js_ref_mut(#ident)
                    };
                    let #ident = &mut *#ident;
                });
            }
        }
        converted_arguments.push(my_quote! { #ident });
    }
    let ret_ty;
    let convert_ret;
    match ret_type {
        Some(&ast::Type::String) => {
            ret_ty = my_quote! { -> *mut String };
            convert_ret = my_quote! { Box::into_raw(Box::new(#ret)) };
        }
        Some(&ast::Type::ByValue(ref t)) => {
            ret_ty = my_quote! {
                -> <#t as ::wasm_bindgen::convert::WasmBoundary>::Js
            };
            convert_ret = my_quote! {
                <#t as ::wasm_bindgen::convert::WasmBoundary>::into_js(#ret)
            };
        }
        Some(&ast::Type::BorrowedStr) |
        Some(&ast::Type::ByMutRef(_)) |
        Some(&ast::Type::ByRef(_)) => {
            panic!("can't return a borrowed ref");
        }
        None => {
            ret_ty = my_quote! {};
            convert_ret = my_quote! {};
        }
    }

    let tokens = my_quote! {
        #[export_name = #export_name]
        #[allow(non_snake_case)]
        pub extern fn #generated_name(#(#args),*) #ret_ty {
            #(#arg_conversions)*
            let #ret = #receiver(#(#converted_arguments),*);
            #convert_ret
        }
    };
    tokens.to_tokens(into);
}

impl ToTokens for Receiver {
    fn to_tokens(&self, tokens: &mut Tokens) {
        match *self {
            Receiver::FreeFunction(name) => name.to_tokens(tokens),
            Receiver::StructFunction(s, name) => {
                s.to_tokens(tokens);
                syn::token::Colon2::default().to_tokens(tokens);
                name.to_tokens(tokens);
            }
            Receiver::StructMethod(_, mutable, name) => {
                (my_quote! { me }).to_tokens(tokens);
                syn::token::Dot::default().to_tokens(tokens);
                if mutable {
                    syn::Ident::from("borrow_mut").to_tokens(tokens);
                } else {
                    syn::Ident::from("borrow").to_tokens(tokens);
                }
                tokens.append(TokenTree {
                    span: Span::def_site(),
                    kind: TokenNode::Group(Delimiter::Parenthesis,
                                           proc_macro2::TokenStream::empty()),
                });
                syn::token::Dot::default().to_tokens(tokens);
                name.to_tokens(tokens);
            }
        }
    }
}

fn bindgen_import(import: &ast::Import, tokens: &mut Tokens) {
    let import_name = shared::mangled_import_name(
        None,
        import.function.wasm_function.name.as_ref(),
    );
    bindgen_import_function(&import.function, &import_name, tokens);
}

fn bindgen_imported_struct(import: &ast::ImportStruct, tokens: &mut Tokens) {
    let name = import.name;

    let mut methods = Tokens::new();

    for &(_, ref f) in import.functions.iter() {
        let import_name = shared::mangled_import_name(
            Some(&import.name.to_string()),
            f.wasm_function.name.as_ref(),
        );
        bindgen_import_function(f, &import_name, &mut methods);
    }

    (my_quote! {
        pub struct #name {
            obj: ::wasm_bindgen::JsValue,
        }

        impl #name {
            #methods
        }

        impl ::wasm_bindgen::convert::WasmBoundary for #name {
            type Js = <::wasm_bindgen::JsValue as
                ::wasm_bindgen::convert::WasmBoundary>::Js;
            const DESCRIPTOR: u32 = <::wasm_bindgen::JsValue as
                ::wasm_bindgen::convert::WasmBoundary>::DESCRIPTOR;

            fn into_js(self) -> Self::Js {
                self.obj.into_js()
            }

            unsafe fn from_js(js: Self::Js) -> Self {
                #name { obj: ::wasm_bindgen::JsValue::from_js(js) }
            }
        }
    }).to_tokens(tokens);
}

fn bindgen_import_function(import: &ast::ImportFunction,
                           import_name: &str,
                           tokens: &mut Tokens) {
    let vis = &import.rust_vis;
    let ret = &import.rust_decl.output;
    let fn_token = &import.rust_decl.fn_token;
    let arguments = &import.rust_decl.inputs;

    let mut abi_argument_names = Vec::new();
    let mut abi_arguments = Vec::new();
    let mut arg_conversions = Vec::new();
    let ret_ident = syn::Ident::from("_ret");

    let inputs = import.rust_decl.inputs.iter().collect::<Vec<_>>();
    let (is_method, inputs) = match inputs.get(0) {
        Some(&&syn::FnArg::Captured(_)) => (false, &inputs[..]),
        Some(_) => (true, &inputs[1..]),
        None => (false, &inputs[..]),
    };

    if is_method {
        let ptr = syn::Ident::from("ptr");
        abi_argument_names.push(ptr);
        abi_arguments.push(my_quote! { #ptr: u32 });
        arg_conversions.push(my_quote! {
            let #ptr = ::wasm_bindgen::convert::ToRefWasmBoundary::to_js_ref(&self.obj);
        });
    }

    let names = inputs
        .iter()
        .map(|arg| {
            match **arg {
                syn::FnArg::Captured(ref c) => c,
                _ => panic!("arguments cannot be `self` or ignored"),
            }
        })
        .map(|arg| {
            match arg.pat {
                syn::Pat::Ident(syn::PatIdent {
                    by_ref: None,
                    ident,
                    subpat: None,
                    ..
                }) => {
                    ident
                }
                _ => panic!("unsupported pattern in foreign function"),
            }
        });

    for (ty, name) in import.wasm_function.arguments.iter().zip(names) {
        match *ty {
            ast::Type::BorrowedStr => {
                let ptr = syn::Ident::from(format!("{}_ptr", name));
                let len = syn::Ident::from(format!("{}_len", name));
                abi_argument_names.push(ptr);
                abi_argument_names.push(len);
                abi_arguments.push(my_quote! { #ptr: *const u8 });
                abi_arguments.push(my_quote! { #len: usize });
                arg_conversions.push(my_quote! {
                    let #ptr = #name.as_ptr();
                    let #len = #name.len();
                });
            }
            ast::Type::ByValue(ref t) => {
                abi_argument_names.push(name);
                abi_arguments.push(my_quote! {
                    #name: <#t as ::wasm_bindgen::convert::WasmBoundary>::Js
                });
                arg_conversions.push(my_quote! {
                    let #name = <#t as ::wasm_bindgen::convert::WasmBoundary>
                        ::into_js(#name);
                });
            }
            ast::Type::ByMutRef(_) => panic!("urgh mut"),
            ast::Type::ByRef(ref t) => {
                abi_argument_names.push(name);
                abi_arguments.push(my_quote! { #name: u32 });
                arg_conversions.push(my_quote! {
                    let #name = <#t as ::wasm_bindgen::convert::ToRefWasmBoundary>
                        ::to_js_ref(#name);
                });
            }
            // TODO: need to test this
            ast::Type::String => {
                let ptr = syn::Ident::from(format!("{}_ptr", name));
                let len = syn::Ident::from(format!("{}_len", name));
                abi_argument_names.push(ptr);
                abi_argument_names.push(len);
                abi_arguments.push(my_quote! { #ptr: *const u8 });
                abi_arguments.push(my_quote! { #len: usize });
                arg_conversions.push(my_quote! {
                    let #ptr = #name.as_ptr();
                    let #len = #name.len();
                    ::std::mem::forget(#name);
                });
            }
        }
    }
    let abi_ret;
    let convert_ret;
    match import.wasm_function.ret {
        Some(ast::Type::ByValue(ref t)) => {
            abi_ret = my_quote! {
                <#t as ::wasm_bindgen::convert::WasmBoundary>::Js
            };
            convert_ret = my_quote! {
                <#t as ::wasm_bindgen::convert::WasmBoundary>::from_js(#ret_ident)
            };
        }

        // TODO: add a test for this
        Some(ast::Type::String) => {
            let name = syn::Ident::from("__ret_strlen");
            let name_ptr = syn::Ident::from("__ret_strlen_ptr");
            abi_argument_names.push(name_ptr);
            abi_arguments.push(my_quote! { #name_ptr: *mut usize });
            arg_conversions.push(my_quote! {
                let mut #name = 0;
                let mut #name_ptr = &mut #name as *mut usize;
            });
            abi_ret = my_quote! { *mut u8 };
            convert_ret = my_quote! {
                String::from_utf8_unchecked(
                    Vec::from_raw_parts(#ret_ident, #name, #name)
                )
            };
        }
        Some(ast::Type::BorrowedStr) |
        Some(ast::Type::ByRef(_)) |
        Some(ast::Type::ByMutRef(_)) => panic!("can't return a borrowed ref"),
        None => {
            abi_ret = my_quote! { () };
            convert_ret = my_quote! {};
        }
    }

    let name = import.ident;
    let import_name = syn::Ident::from(import_name);
    (quote! {
        #vis #fn_token #name(#arguments) #ret {
            extern {
                fn #import_name(#(#abi_arguments),*) -> #abi_ret;
            }
            unsafe {
                #(#arg_conversions)*
                let #ret_ident = #import_name(#(#abi_argument_names),*);
                #convert_ret
            }
        }
    }).to_tokens(tokens);
}
