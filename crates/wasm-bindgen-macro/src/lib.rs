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

use std::borrow::Cow;
use std::env;
use std::sync::atomic::*;
use std::collections::HashSet;

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::{Tokens, ToTokens};

macro_rules! my_quote {
    ($($t:tt)*) => (quote_spanned!(Span::call_site() => $($t)*))
}

mod ast;
mod literal;

#[proc_macro_attribute]
pub fn wasm_bindgen(attr: TokenStream, input: TokenStream) -> TokenStream {
    let item = syn::parse::<syn::Item>(input.clone())
        .expect("expected a valid Rust item");
    let opts = syn::parse::<ast::BindgenAttrs>(attr)
        .expect("invalid arguments to #[wasm_bindgen]");

    let mut ret = Tokens::new();
    let mut program = ast::Program::default();
    program.push_item(item, Some(opts), &mut ret);
    program.to_tokens(&mut ret);

    // println!("{}", ret);

    ret.into()
}

fn to_ident_name(s: &str) -> Cow<str> {
    if s.chars().all(|c| match c {
        'a'...'z' | 'A'...'Z' | '0'...'9' | '_' => true,
        _ => false,
    }) {
        return Cow::from(s);
    }

    Cow::from(
        s.chars()
            .map(|c| match c {
                'a'...'z' | 'A'...'Z' | '0'...'9' | '_' => c,
                _ => '_',
            })
            .collect::<String>(),
    )
}

impl ToTokens for ast::Program {
    // Generate wrappers for all the items that we've found
    fn to_tokens(&self, tokens: &mut Tokens) {
        for export in self.exports.iter() {
            export.to_tokens(tokens);
        }
        for s in self.structs.iter() {
            s.to_tokens(tokens);
        }
        let mut types = HashSet::new();
        for i in self.imports.iter() {
            if let ast::ImportKind::Type(ref t) = i.kind {
                types.insert(t.name);
            }
        }
        for i in self.imports.iter() {
            match i.js_namespace {
                Some(ns) if types.contains(&ns) => {
                    let kind = &i.kind;
                    (quote! { impl #ns { #kind } }).to_tokens(tokens);
                }
                _ => i.kind.to_tokens(tokens),
            }
        }
        for e in self.enums.iter() {
            e.to_tokens(tokens);
        }

        // Generate a static which will eventually be what lives in a custom section
        // of the wasm executable. For now it's just a plain old static, but we'll
        // eventually have it actually in its own section.

        static CNT: AtomicUsize = ATOMIC_USIZE_INIT;

        let crate_name = env::var("CARGO_PKG_NAME").expect("should have CARGO_PKG_NAME env var");
        let crate_vers = env::var("CARGO_PKG_VERSION").expect("should have CARGO_PKG_VERSION env var");

        let generated_static_name = format!(
            "__WASM_BINDGEN_GENERATED_{}_{}_{}",
            to_ident_name(&crate_name),
            to_ident_name(&crate_vers),
            CNT.fetch_add(1, Ordering::SeqCst)
        );
        let generated_static_name = syn::Ident::from(generated_static_name);

        let mut generated_static_value = Tokens::new();
        let generated_static_length = self.literal(&mut generated_static_value);

        (my_quote! {
            #[no_mangle]
            #[allow(non_upper_case_globals)]
            pub static #generated_static_name: [u32; #generated_static_length] =
                [#generated_static_value];
        }).to_tokens(tokens);
    }
}

impl ToTokens for ast::Struct {
    fn to_tokens(&self, tokens: &mut Tokens) {
        let name = &self.name;
        let free_fn = syn::Ident::from(shared::free_function(self.name.as_ref()));
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
        }).to_tokens(tokens);
    }
}

impl ToTokens for ast::Export {
    fn to_tokens(self: &ast::Export, into: &mut Tokens) {
        let generated_name = self.rust_symbol();
        let export_name = self.export_name();
        let mut args = vec![];
        let mut arg_conversions = vec![];
        let mut converted_arguments = vec![];
        let ret = syn::Ident::from("_ret");

        let mut offset = 0;
        if self.method {
            let class = self.class.unwrap();
            args.push(my_quote! { me: *mut ::wasm_bindgen::__rt::WasmRefCell<#class> });
            arg_conversions.push(my_quote! {
                ::wasm_bindgen::__rt::assert_not_null(me);
                let me = unsafe { &*me };
            });
            offset = 1;
        }

        for (i, ty) in self.function.arguments.iter().enumerate() {
            let i = i + offset;
            let ident = syn::Ident::from(format!("arg{}", i));
            match *ty {
                ast::Type::Vector(ref ty, owned) => {
                    let ptr = syn::Ident::from(format!("arg{}_ptr", i));
                    let len = syn::Ident::from(format!("arg{}_len", i));
                    let abi_ty = ty.abi_element();
                    args.push(my_quote! { #ptr: *mut #abi_ty });
                    args.push(my_quote! { #len: usize });
                    if owned {
                        arg_conversions.push(my_quote! {
                            let #ident = unsafe {
                                ::std::vec::Vec::from_raw_parts(#ptr, #len, #len)
                            };
                        });
                    } else {
                        arg_conversions.push(my_quote! {
                            let #ident = unsafe {
                                ::std::slice::from_raw_parts(#ptr as *const #abi_ty, #len)
                            };
                        });
                    }
                    if let ast::VectorType::String = *ty {
                        if owned {
                            arg_conversions.push(my_quote! {
                                let #ident = unsafe {
                                    ::std::string::String::from_utf8_unchecked(#ident)
                                };
                            });
                        } else {
                            arg_conversions.push(my_quote! {
                                let #ident = unsafe {
                                    ::std::str::from_utf8_unchecked(#ident)
                                };
                            });
                        }
                    }
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
        match self.function.ret {
            Some(ast::Type::Vector(ref ty, true)) => {
                ret_ty = my_quote! { -> *mut #ty };
                convert_ret = my_quote! { Box::into_raw(Box::new(#ret)) };
            }
            Some(ast::Type::ByValue(ref t)) => {
                ret_ty = my_quote! {
                    -> <#t as ::wasm_bindgen::convert::WasmBoundary>::Js
                };
                convert_ret = my_quote! {
                    <#t as ::wasm_bindgen::convert::WasmBoundary>::into_js(#ret)
                };
            }
            Some(ast::Type::Vector(_, false)) |
            Some(ast::Type::ByMutRef(_)) |
            Some(ast::Type::ByRef(_)) => {
                panic!("can't return a borrowed ref");
            }
            None => {
                ret_ty = my_quote! {};
                convert_ret = my_quote! {};
            }
        }

        let name = self.function.name;
        let receiver = match self.class {
            Some(_) if self.method => {
                if self.mutable {
                    my_quote! { me.borrow_mut().#name }
                } else {
                    my_quote! { me.borrow().#name }
                }
            }
            Some(class) => my_quote! { #class::#name },
            None => my_quote!{ #name },
        };

        let tokens = my_quote! {
            #[export_name = #export_name]
            #[allow(non_snake_case)]
            pub extern fn #generated_name(#(#args),*) #ret_ty {
                ::wasm_bindgen::__rt::link_this_library();
                #(#arg_conversions)*
                let #ret = #receiver(#(#converted_arguments),*);
                #convert_ret
            }
        };
        tokens.to_tokens(into);
    }
}

impl ToTokens for ast::ImportType {
    fn to_tokens(&self, tokens: &mut Tokens) {
        let vis = &self.vis;
        let name = &self.name;
        (my_quote! {
            #[allow(bad_style)]
            #vis struct #name {
                obj: ::wasm_bindgen::JsValue,
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

            impl ::wasm_bindgen::convert::ToRefWasmBoundary for #name {
                fn to_js_ref(&self) -> u32 {
                    self.obj.to_js_ref()
                }
            }

            impl From<::wasm_bindgen::JsValue> for #name {
                fn from(obj: ::wasm_bindgen::JsValue) -> #name {
                    #name { obj }
                }
            }

            impl From<#name> for ::wasm_bindgen::JsValue {
                fn from(obj: #name) -> ::wasm_bindgen::JsValue {
                    obj.obj
                }
            }
        }).to_tokens(tokens);
    }
}

impl ToTokens for ast::ImportKind {
    fn to_tokens(&self, tokens: &mut Tokens) {
        match *self {
            ast::ImportKind::Function(ref f) => f.to_tokens(tokens),
            ast::ImportKind::Static(ref s) => s.to_tokens(tokens),
            ast::ImportKind::Type(ref t) => t.to_tokens(tokens),
        }
    }
}

impl ToTokens for ast::ImportFunction {
    fn to_tokens(&self, tokens: &mut Tokens) {
        let mut class_ty = None;
        let mut is_method = false;
        let mut class_name = None;
        match self.kind {
            ast::ImportFunctionKind::Method { ref ty, ref class } => {
                is_method = true;
                class_ty = Some(ty);
                class_name = Some(class);
            }
            ast::ImportFunctionKind::JsConstructor { ref ty, ref class } => {
                class_ty = Some(ty);
                class_name = Some(class);
            }
            ast::ImportFunctionKind::Normal => {}
        }
        let import_name = shared::mangled_import_name(
            class_name.map(|s| &**s),
            self.function.name.as_ref(),
        );
        let vis = &self.function.rust_vis;
        let ret = &self.function.rust_decl.output;
        let fn_token = &self.function.rust_decl.fn_token;

        let mut abi_argument_names = Vec::new();
        let mut abi_arguments = Vec::new();
        let mut arg_conversions = Vec::new();
        let ret_ident = syn::Ident::from("_ret");

        let names = self.function.rust_decl.inputs
            .iter()
            .map(|arg| {
                match *arg {
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

        for (i, (ty, name)) in self.function.arguments.iter().zip(names).enumerate() {
            match *ty {
                ast::Type::Vector(ref ty, owned) => {
                    let ptr = syn::Ident::from(format!("{}_ptr", name));
                    let len = syn::Ident::from(format!("{}_len", name));
                    abi_argument_names.push(ptr);
                    abi_argument_names.push(len);
                    let abi_ty = ty.abi_element();
                    abi_arguments.push(my_quote! { #ptr: *const #abi_ty });
                    abi_arguments.push(my_quote! { #len: usize });
                    arg_conversions.push(my_quote! {
                        let #ptr = #name.as_ptr();
                        let #len = #name.len();
                    });
                    if owned {
                        arg_conversions.push(my_quote! { ::std::mem::forget(#name); });
                    }
                }
                ast::Type::ByValue(ref t) => {
                    abi_argument_names.push(name);
                    abi_arguments.push(my_quote! {
                        #name: <#t as ::wasm_bindgen::convert::WasmBoundary>::Js
                    });
                    if i == 0 && is_method {
                        arg_conversions.push(my_quote! {
                            let #name = <#t as ::wasm_bindgen::convert::WasmBoundary>
                                ::into_js(self);
                        });
                    } else {
                        arg_conversions.push(my_quote! {
                            let #name = <#t as ::wasm_bindgen::convert::WasmBoundary>
                                ::into_js(#name);
                        });
                    }
                }
                ast::Type::ByMutRef(_) => panic!("urgh mut"),
                ast::Type::ByRef(ref t) => {
                    abi_argument_names.push(name);
                    abi_arguments.push(my_quote! { #name: u32 });
                    if i == 0 && is_method {
                        arg_conversions.push(my_quote! {
                            let #name = <#t as ::wasm_bindgen::convert::ToRefWasmBoundary>
                                ::to_js_ref(self);
                        });
                    } else {
                        arg_conversions.push(my_quote! {
                            let #name = <#t as ::wasm_bindgen::convert::ToRefWasmBoundary>
                                ::to_js_ref(#name);
                        });
                    }
                }
            }
        }
        let abi_ret;
        let mut convert_ret;
        match self.function.ret {
            Some(ast::Type::ByValue(ref t)) => {
                abi_ret = my_quote! {
                    <#t as ::wasm_bindgen::convert::WasmBoundary>::Js
                };
                convert_ret = my_quote! {
                    <#t as ::wasm_bindgen::convert::WasmBoundary>::from_js(#ret_ident)
                };
            }

            Some(ast::Type::Vector(ref ty, true)) => {
                let name = syn::Ident::from("__ret_len");
                let name_ptr = syn::Ident::from("__ret_len_ptr");
                abi_argument_names.push(name_ptr);
                abi_arguments.push(my_quote! { #name_ptr: *mut usize });
                arg_conversions.push(my_quote! {
                    let mut #name = 0;
                    let mut #name_ptr = &mut #name as *mut usize;
                });
                let abi_ty = ty.abi_element();
                abi_ret = my_quote! { *mut #abi_ty };
                if let ast::VectorType::String = *ty {
                    convert_ret = my_quote! {
                        String::from_utf8_unchecked(
                            Vec::from_raw_parts(#ret_ident, #name, #name)
                        )
                    };
                } else {
                    convert_ret = my_quote! {
                        Vec::from_raw_parts(#ret_ident, #name, #name)
                    };
                }
            }
            Some(ast::Type::ByRef(_)) |
            Some(ast::Type::Vector(_, false)) |
            Some(ast::Type::ByMutRef(_)) => panic!("can't return a borrowed ref"),
            None => {
                abi_ret = my_quote! { () };
                convert_ret = my_quote! { () };
            }
        }

        let mut exceptional_ret = my_quote! {};
        if self.function.opts.catch() {
            let exn_data = syn::Ident::from("exn_data");
            let exn_data_ptr = syn::Ident::from("exn_data_ptr");
            abi_argument_names.push(exn_data_ptr);
            abi_arguments.push(my_quote! { #exn_data_ptr: *mut u32 });
            arg_conversions.push(my_quote! {
                let mut #exn_data = [0; 2];
                let #exn_data_ptr = #exn_data.as_mut_ptr();
            });
            convert_ret = my_quote! { Ok(#convert_ret) };
            exceptional_ret = my_quote! {
                if #exn_data[0] == 1 {
                    return Err(<::wasm_bindgen::JsValue as
                        ::wasm_bindgen::convert::WasmBoundary>::from_js(#exn_data[1]))
                }
            };
        }

        let name = self.function.name;
        let import_name = syn::Ident::from(import_name);
        let attrs = &self.function.rust_attrs;

        let arguments = self.function.rust_decl.inputs
            .iter()
            .skip(if is_method { 1 } else { 0 })
            .collect::<Vec<_>>();

        let me = if is_method {
            my_quote! { &self, }
        } else {
            quote!()
        };

        let invocation = my_quote! {
            #(#attrs)*
            #[allow(bad_style)]
            #vis extern #fn_token #name(#me #(#arguments),*) #ret {
                ::wasm_bindgen::__rt::link_this_library();
                extern {
                    fn #import_name(#(#abi_arguments),*) -> #abi_ret;
                }
                unsafe {
                    #(#arg_conversions)*
                    let #ret_ident = #import_name(#(#abi_argument_names),*);
                    #exceptional_ret
                    #convert_ret
                }
            }
        };

        if let Some(class) = class_ty {
            (quote! {
                impl #class {
                    #invocation
                }
            }).to_tokens(tokens);
        } else {
            invocation.to_tokens(tokens);
        }
    }
}

impl ToTokens for ast::Enum {
    fn to_tokens(&self, into: &mut Tokens) {
        let enum_name = &self.name;
        let c = shared::TYPE_ENUM as u32;
        let incoming_u32 = quote! { n };
        let enum_name_as_string = enum_name.to_string();
        let cast_clauses = self.variants.iter().map(|variant| {
            let variant_name = &variant.name;
            quote! {
                if #incoming_u32 == #enum_name::#variant_name as u32 {
                    #enum_name::#variant_name
                }
            }
        });
        (my_quote! {
            impl #enum_name {
                fn from_u32(#incoming_u32: u32) -> #enum_name {
                    #(#cast_clauses else)* {
                        wasm_bindgen::throw(&format!("Could not cast {} as {}", #incoming_u32, #enum_name_as_string));
                    }
                }
            }

            impl ::wasm_bindgen::convert::WasmBoundary for #enum_name {
                type Js = u32;
                const DESCRIPTOR: u32 = #c;

                fn into_js(self) -> u32 {
                    self as u32
                }

                unsafe fn from_js(js: u32) -> Self {
                    #enum_name::from_u32(js)
                }
            }
        }).to_tokens(into);
    }
}

impl ToTokens for ast::ImportStatic {
    fn to_tokens(&self, into: &mut Tokens) {
        let name = self.name;
        let ty = &self.ty;
        let shim_name = shared::static_import_shim_name(name.as_ref());
        let shim_name = syn::Ident::from(shim_name);
        let vis = &self.vis;
        (my_quote! {
            #[allow(bad_style)]
            #vis static #name: ::wasm_bindgen::JsStatic<#ty> = {
                fn init() -> #ty {
                    extern {
                        fn #shim_name() -> u32;
                    }
                    unsafe {
                        ::wasm_bindgen::convert::WasmBoundary::from_js(#shim_name())
                    }
                }
                ::wasm_bindgen::JsStatic {
                    __inner: ::std::cell::UnsafeCell::new(None),
                    __init: init,
                }
            };
        }).to_tokens(into);
    }
}
