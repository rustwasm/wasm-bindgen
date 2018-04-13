use std::borrow::Cow;
use std::collections::HashSet;
use std::env;
use std::sync::atomic::{ATOMIC_USIZE_INIT, AtomicUsize, Ordering};

use ast;
use proc_macro2::Span;
use quote::{ToTokens, Tokens};
use serde_json;
use shared;
use syn;

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
            DescribeImport(&i.kind).to_tokens(tokens);
        }
        for e in self.enums.iter() {
            e.to_tokens(tokens);
        }

        // Generate a static which will eventually be what lives in a custom section
        // of the wasm executable. For now it's just a plain old static, but we'll
        // eventually have it actually in its own section.

        static CNT: AtomicUsize = ATOMIC_USIZE_INIT;

        let crate_name = env::var("CARGO_PKG_NAME").expect("should have CARGO_PKG_NAME env var");
        let crate_vers =
            env::var("CARGO_PKG_VERSION").expect("should have CARGO_PKG_VERSION env var");

        let generated_static_name = format!(
            "__WASM_BINDGEN_GENERATED_{}_{}_{}",
            to_ident_name(&crate_name),
            to_ident_name(&crate_vers),
            CNT.fetch_add(1, Ordering::SeqCst)
        );
        let generated_static_name = syn::Ident::from(generated_static_name);

        let description = serde_json::to_string(&self.shared()).unwrap();

        // Each JSON blob is prepended with the length of the JSON blob so when
        // all these sections are concatenated in the final wasm file we know
        // how to extract all the JSON pieces, so insert the byte length here.
        let generated_static_length = description.len() + 4;
        let mut bytes = vec![
            (description.len() >> 0) as u8,
            (description.len() >> 8) as u8,
            (description.len() >> 16) as u8,
            (description.len() >> 24) as u8,
        ];
        bytes.extend_from_slice(description.as_bytes());
        let generated_static_value = syn::LitByteStr::new(&bytes, Span::call_site());

        (quote! {
            #[allow(non_upper_case_globals)]
            #[wasm_custom_section = "__wasm_bindgen_unstable"]
            const #generated_static_name: [u8; #generated_static_length] =
                *#generated_static_value;
        }).to_tokens(tokens);
    }
}

impl ToTokens for ast::Struct {
    fn to_tokens(&self, tokens: &mut Tokens) {
        let name = &self.name;
        let name_len = name.as_ref().len() as u32;
        let name_chars = name.as_ref().chars().map(|c| c as u32);
        let new_fn = syn::Ident::from(shared::new_function(self.name.as_ref()));
        let free_fn = syn::Ident::from(shared::free_function(self.name.as_ref()));
        (quote! {
            impl ::wasm_bindgen::describe::WasmDescribe for #name {
                fn describe() {
                    use wasm_bindgen::describe::*;
                    inform(RUST_STRUCT);
                    inform(#name_len);
                    #(inform(#name_chars);)*
                }
            }

            impl ::wasm_bindgen::convert::WasmBoundary for #name {
                type Abi = u32;

                fn into_abi(self, _extra: &mut ::wasm_bindgen::convert::Stack)
                    -> u32
                {
                    Box::into_raw(Box::new(::wasm_bindgen::__rt::WasmRefCell::new(self))) as u32
                }

                unsafe fn from_abi(js: u32, _extra: &mut ::wasm_bindgen::convert::Stack)
                    -> Self
                {
                    let js = js as *mut ::wasm_bindgen::__rt::WasmRefCell<#name>;
                    ::wasm_bindgen::__rt::assert_not_null(js);
                    let js = Box::from_raw(js);
                    js.borrow_mut(); // make sure no one's borrowing
                    js.into_inner()
                }
            }

            impl ::wasm_bindgen::convert::FromRefWasmBoundary for #name {
                type Abi = u32;
                type RefAnchor = ::wasm_bindgen::__rt::Ref<'static, #name>;

                unsafe fn from_abi_ref(
                    js: Self::Abi,
                    _extra: &mut ::wasm_bindgen::convert::Stack,
                ) -> Self::RefAnchor {
                    let js = js as *mut ::wasm_bindgen::__rt::WasmRefCell<#name>;
                    ::wasm_bindgen::__rt::assert_not_null(js);
                    (*js).borrow()
                }
            }

            impl ::wasm_bindgen::convert::FromRefMutWasmBoundary for #name {
                type Abi = u32;
                type RefAnchor = ::wasm_bindgen::__rt::RefMut<'static, #name>;

                unsafe fn from_abi_ref_mut(
                    js: Self::Abi,
                    _extra: &mut ::wasm_bindgen::convert::Stack,
                ) -> Self::RefAnchor {
                    let js = js as *mut ::wasm_bindgen::__rt::WasmRefCell<#name>;
                    ::wasm_bindgen::__rt::assert_not_null(js);
                    (*js).borrow_mut()
                }
            }

            impl ::std::convert::From<#name> for ::wasm_bindgen::JsValue {
                fn from(value: #name) -> Self {
                    let ptr = ::wasm_bindgen::convert::WasmBoundary::into_abi(
                        value,
                        unsafe { &mut ::wasm_bindgen::convert::GlobalStack::new() },
                    );

                    #[wasm_import_module = "__wbindgen_placeholder__"]
                    extern {
                        fn #new_fn(ptr: u32) -> u32;
                    }

                    unsafe {
                        <::wasm_bindgen::JsValue as ::wasm_bindgen::convert::WasmBoundary>
                            ::from_abi(
                                #new_fn(ptr),
                                &mut ::wasm_bindgen::convert::GlobalStack::new(),
                            )
                    }
                }
            }

            #[no_mangle]
            pub unsafe extern fn #free_fn(ptr: u32) {
                <#name as ::wasm_bindgen::convert::WasmBoundary>::from_abi(
                    ptr,
                    &mut ::wasm_bindgen::convert::GlobalStack::new(),
                );
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
            args.push(quote! { me: *mut ::wasm_bindgen::__rt::WasmRefCell<#class> });
            arg_conversions.push(quote! {
                ::wasm_bindgen::__rt::assert_not_null(me);
                let me = unsafe { &*me };
            });
            offset = 1;
        }

        for (i, ty) in self.function.arguments.iter().enumerate() {
            let i = i + offset;
            let ident = syn::Ident::from(format!("arg{}", i));
            let t = &ty.ty;
            match ty.kind {
                ast::TypeKind::ByValue => {
                    args.push(quote! {
                        #ident: <#t as ::wasm_bindgen::convert::WasmBoundary>::Abi
                    });
                    arg_conversions.push(quote! {
                        let #ident = unsafe {
                            <#t as ::wasm_bindgen::convert::WasmBoundary>
                                ::from_abi(#ident, &mut __stack)
                        };
                    });
                }
                ast::TypeKind::ByRef => {
                    args.push(quote! {
                        #ident: <#t as ::wasm_bindgen::convert::FromRefWasmBoundary>::Abi
                    });
                    arg_conversions.push(quote! {
                        let #ident = unsafe {
                            <#t as ::wasm_bindgen::convert::FromRefWasmBoundary>
                                ::from_abi_ref(#ident, &mut __stack)
                        };
                        let #ident = &*#ident;
                    });
                }
                ast::TypeKind::ByMutRef => {
                    args.push(quote! {
                        #ident: <#t as ::wasm_bindgen::convert::FromRefMutWasmBoundary>::Abi
                    });
                    arg_conversions.push(quote! {
                        let mut #ident = unsafe {
                            <#t as ::wasm_bindgen::convert::FromRefMutWasmBoundary>
                                ::from_abi_ref_mut(#ident, &mut __stack)
                        };
                        let #ident = &mut *#ident;
                    });
                }
            }
            converted_arguments.push(quote! { #ident });
        }
        let ret_ty;
        let convert_ret;
        match self.function.ret {
            Some(ast::Type { ref ty, kind: ast::TypeKind::ByValue, .. }) => {
                ret_ty = quote! {
                    -> <#ty as ::wasm_bindgen::convert::WasmBoundary>::Abi
                };
                convert_ret = quote! {
                    <#ty as ::wasm_bindgen::convert::WasmBoundary>
                        ::into_abi(#ret, &mut unsafe {
                            ::wasm_bindgen::convert::GlobalStack::new()
                        })
                };
            }
            Some(ast::Type { kind: ast::TypeKind::ByMutRef, .. }) |
            Some(ast::Type { kind: ast::TypeKind::ByRef, .. }) => {
                panic!("can't return a borrowed ref");
            }
            None => {
                ret_ty = quote!{};
                convert_ret = quote!{};
            }
        }
        let describe_ret = match self.function.ret {
            Some(ast::Type { ref ty, .. }) => {
                quote! {
                    inform(1);
                    <#ty as WasmDescribe>::describe();
                }
            }
            None => quote! { inform(0); },
        };

        let name = self.function.name;
        let receiver = match self.class {
            Some(_) if self.method => {
                if self.mutable {
                    quote! { me.borrow_mut().#name }
                } else {
                    quote! { me.borrow().#name }
                }
            }
            Some(class) => quote! { #class::#name },
            None => quote!{ #name },
        };
        let descriptor_name = format!("__wbindgen_describe_{}", export_name);
        let descriptor_name = syn::Ident::from(descriptor_name);
        let nargs = self.function.arguments.len() as u32;
        let argtys = self.function.arguments.iter();

        let tokens = quote! {
            #[export_name = #export_name]
            #[allow(non_snake_case)]
            pub extern fn #generated_name(#(#args),*) #ret_ty {
                ::wasm_bindgen::__rt::link_this_library();
                let #ret = {
                    let mut __stack = unsafe {
                        ::wasm_bindgen::convert::GlobalStack::new()
                    };
                    #(#arg_conversions)*
                    #receiver(#(#converted_arguments),*)
                };
                #convert_ret
            }

            // In addition to generating the shim function above which is what
            // our generated JS will invoke, we *also* generate a "descriptor"
            // shim. This descriptor shim uses the `WasmDescribe` trait to
            // programmatically describe the type signature of the generated
            // shim above. This in turn is then used to inform the
            // `wasm-bindgen` CLI tool exactly what types and such it should be
            // using in JS.
            //
            // Note that this descriptor function is a purely an internal detail
            // of `#[wasm_bindgen]` and isn't intended to be exported to anyone
            // or actually part of the final was binary. Additionally, this is
            // literally executed when the `wasm-bindgen` tool executes.
            //
            // In any case, there's complications in `wasm-bindgen` to handle
            // this, but the tl;dr; is that this is stripped from the final wasm
            // binary along with anything it references.
            #[no_mangle]
            pub extern fn #descriptor_name() {
                use wasm_bindgen::describe::*;
                inform(FUNCTION);
                inform(#nargs);
                #(<#argtys as WasmDescribe>::describe();)*
                #describe_ret
            }
        };
        tokens.to_tokens(into);
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

impl ToTokens for ast::ImportType {
    fn to_tokens(&self, tokens: &mut Tokens) {
        let vis = &self.vis;
        let name = &self.name;
        (quote! {
            #[allow(bad_style)]
            #vis struct #name {
                obj: ::wasm_bindgen::JsValue,
            }

            impl ::wasm_bindgen::describe::WasmDescribe for #name {
                fn describe() {
                    ::wasm_bindgen::JsValue::describe();
                }
            }

            impl ::wasm_bindgen::convert::WasmBoundary for #name {
                type Abi = <::wasm_bindgen::JsValue as
                    ::wasm_bindgen::convert::WasmBoundary>::Abi;

                fn into_abi(self, extra: &mut ::wasm_bindgen::convert::Stack) -> Self::Abi {
                    self.obj.into_abi(extra)
                }

                unsafe fn from_abi(
                    js: Self::Abi,
                    extra: &mut ::wasm_bindgen::convert::Stack,
                ) -> Self {
                    #name { obj: ::wasm_bindgen::JsValue::from_abi(js, extra) }
                }
            }

            impl ::wasm_bindgen::convert::ToRefWasmBoundary for #name {
                type Abi = <::wasm_bindgen::JsValue as
                    ::wasm_bindgen::convert::ToRefWasmBoundary>::Abi;

                fn to_abi_ref(&self, extra: &mut ::wasm_bindgen::convert::Stack) -> u32 {
                    self.obj.to_abi_ref(extra)
                }
            }

            impl ::wasm_bindgen::convert::FromRefWasmBoundary for #name {
                type Abi = <::wasm_bindgen::JsValue as
                    ::wasm_bindgen::convert::ToRefWasmBoundary>::Abi;
                type RefAnchor = ::std::mem::ManuallyDrop<#name>;

                unsafe fn from_abi_ref(
                    js: Self::Abi,
                    extra: &mut ::wasm_bindgen::convert::Stack,
                ) -> Self::RefAnchor {
                    let obj = <::wasm_bindgen::JsValue as ::wasm_bindgen::convert::WasmBoundary>
                        ::from_abi(js, extra);
                    ::std::mem::ManuallyDrop::new(#name { obj })
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

impl ToTokens for ast::ImportFunction {
    fn to_tokens(&self, tokens: &mut Tokens) {
        let mut class_ty = None;
        let mut is_method = false;
        match self.kind {
            ast::ImportFunctionKind::Method { ref ty, .. } => {
                is_method = true;
                class_ty = Some(ty);
            }
            ast::ImportFunctionKind::JsConstructor { ref ty, .. } => {
                class_ty = Some(ty);
            }
            ast::ImportFunctionKind::Normal => {}
        }
        let vis = &self.function.rust_vis;
        let ret = &self.function.rust_decl.output;
        let fn_token = &self.function.rust_decl.fn_token;

        let mut abi_argument_names = Vec::new();
        let mut abi_arguments = Vec::new();
        let mut arg_conversions = Vec::new();
        let ret_ident = syn::Ident::from("_ret");

        let names = self.function
            .rust_decl
            .inputs
            .iter()
            .map(|arg| match *arg {
                syn::FnArg::Captured(ref c) => c,
                _ => panic!("arguments cannot be `self` or ignored"),
            })
            .map(|arg| match arg.pat {
                syn::Pat::Ident(syn::PatIdent {
                    by_ref: None,
                    ident,
                    subpat: None,
                    ..
                }) => ident,
                _ => panic!("unsupported pattern in foreign function"),
            });

        for (i, (ty, name)) in self.function.arguments.iter().zip(names).enumerate() {
            let t = &ty.ty;
            match ty.kind {
                ast::TypeKind::ByValue => {
                    abi_argument_names.push(name);
                    abi_arguments.push(quote! {
                        #name: <#t as ::wasm_bindgen::convert::WasmBoundary>::Abi
                    });
                    let var = if i == 0 && is_method {
                        quote! { self }
                    } else {
                        quote! { #name }
                    };
                    arg_conversions.push(quote! {
                        let #name = <#t as ::wasm_bindgen::convert::WasmBoundary>
                            ::into_abi(#var, &mut __stack);
                    });
                }
                ast::TypeKind::ByMutRef => {
                    abi_argument_names.push(name);
                    abi_arguments.push(quote! { #name: u32 });
                    arg_conversions.push(quote! {
                        let #name = <#t as ::wasm_bindgen::convert::ToRefMutWasmBoundary>
                            ::to_abi_ref_mut(#name, &mut __stack);
                    });
                }
                ast::TypeKind::ByRef => {
                    abi_argument_names.push(name);
                    abi_arguments.push(quote! { #name: u32 });
                    let var = if i == 0 && is_method {
                        quote! { self }
                    } else {
                        quote! { #name }
                    };
                    arg_conversions.push(quote! {
                        let #name = <#t as ::wasm_bindgen::convert::ToRefWasmBoundary>
                            ::to_abi_ref(#var, &mut __stack);
                    });
                }
            }
        }
        let abi_ret;
        let mut convert_ret;
        match self.function.ret {
            Some(ast::Type { ref ty, kind: ast::TypeKind::ByValue, .. }) => {
                abi_ret = quote! {
                    <#ty as ::wasm_bindgen::convert::WasmBoundary>::Abi
                };
                convert_ret = quote! {
                    <#ty as ::wasm_bindgen::convert::WasmBoundary>
                        ::from_abi(
                            #ret_ident,
                            &mut ::wasm_bindgen::convert::GlobalStack::new(),
                        )
                };
            }
            Some(ast::Type { kind: ast::TypeKind::ByRef, .. }) |
            Some(ast::Type { kind: ast::TypeKind::ByMutRef, .. }) => {
                panic!("can't return a borrowed ref")
            }
            None => {
                abi_ret = quote! { () };
                convert_ret = quote! { () };
            }
        }

        let mut exceptional_ret = quote!{};
        let exn_data = if self.function.opts.catch() {
            let exn_data = syn::Ident::from("exn_data");
            let exn_data_ptr = syn::Ident::from("exn_data_ptr");
            abi_argument_names.push(exn_data_ptr);
            abi_arguments.push(quote! { #exn_data_ptr: *mut u32 });
            convert_ret = quote! { Ok(#convert_ret) };
            exceptional_ret = quote! {
                if #exn_data[0] == 1 {
                    return Err(
                        <
                            ::wasm_bindgen::JsValue as ::wasm_bindgen::convert::WasmBoundary
                        >::from_abi(#exn_data[1], &mut ::wasm_bindgen::convert::GlobalStack::new()),
                    )
                }
            };
            quote! {
                let mut #exn_data = [0; 2];
                let #exn_data_ptr = #exn_data.as_mut_ptr();
            }
        } else {
            quote! {}
        };

        let rust_name = self.rust_name;
        let import_name = self.shim;
        let attrs = &self.function.rust_attrs;

        let arguments = self.function
            .rust_decl
            .inputs
            .iter()
            .skip(if is_method { 1 } else { 0 })
            .collect::<Vec<_>>();

        let me = if is_method {
            quote! { &self, }
        } else {
            quote!()
        };

        let invocation = quote! {
            #(#attrs)*
            #[allow(bad_style)]
            #vis extern #fn_token #rust_name(#me #(#arguments),*) #ret {
                ::wasm_bindgen::__rt::link_this_library();
                #[wasm_import_module = "__wbindgen_placeholder__"]
                extern {
                    fn #import_name(#(#abi_arguments),*) -> #abi_ret;
                }
                unsafe {
                    #exn_data
                    let #ret_ident = {
                        let mut __stack = ::wasm_bindgen::convert::GlobalStack::new();
                        #(#arg_conversions)*
                        #import_name(#(#abi_argument_names),*)
                    };
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

// See comment above in ast::Export for what's going on here.
struct DescribeImport<'a>(&'a ast::ImportKind);

impl<'a> ToTokens for DescribeImport<'a> {
    fn to_tokens(&self, tokens: &mut Tokens) {
        let f = match *self.0 {
            ast::ImportKind::Function(ref f) => f,
            ast::ImportKind::Static(_) => return,
            ast::ImportKind::Type(_) => return,
        };
        let describe_name = format!("__wbindgen_describe_{}", f.shim);
        let describe_name = syn::Ident::from(describe_name);
        let argtys = f.function.arguments.iter();
        let nargs = f.function.arguments.len() as u32;
        let inform_ret = match f.function.ret {
            Some(ref t) => quote! { inform(1); <#t as WasmDescribe>::describe(); },
            None => quote! { inform(0); },
        };
        (quote! {
            #[no_mangle]
            #[allow(non_snake_case)]
            pub extern fn #describe_name() {
                use wasm_bindgen::describe::*;
                inform(FUNCTION);
                inform(#nargs);
                #(<#argtys as WasmDescribe>::describe();)*
                #inform_ret
            }
        }).to_tokens(tokens);
    }
}

impl ToTokens for ast::Enum {
    fn to_tokens(&self, into: &mut Tokens) {
        let enum_name = &self.name;
        let cast_clauses = self.variants.iter().map(|variant| {
            let variant_name = &variant.name;
            quote! {
                if js == #enum_name::#variant_name as u32 {
                    #enum_name::#variant_name
                }
            }
        });
        (quote! {
            impl ::wasm_bindgen::convert::WasmBoundary for #enum_name {
                type Abi = u32;

                fn into_abi(self, _extra: &mut ::wasm_bindgen::convert::Stack) -> u32 {
                    self as u32
                }

                unsafe fn from_abi(
                    js: u32,
                    _extra: &mut ::wasm_bindgen::convert::Stack,
                ) -> Self {
                    #(#cast_clauses else)* {
                        wasm_bindgen::throw("invalid enum value passed")
                    }
                }
            }

            impl ::wasm_bindgen::describe::WasmDescribe for #enum_name {
                fn describe() {
                    use wasm_bindgen::describe::*;
                    inform(ENUM);
                }
            }
        }).to_tokens(into);
    }
}

impl ToTokens for ast::ImportStatic {
    fn to_tokens(&self, into: &mut Tokens) {
        let name = self.rust_name;
        let ty = &self.ty;
        let shim_name = self.shim;
        let vis = &self.vis;
        (quote! {
            #[allow(bad_style)]
            #vis static #name: ::wasm_bindgen::JsStatic<#ty> = {
                fn init() -> #ty {
                    #[wasm_import_module = "__wbindgen_placeholder__"]
                    extern {
                        fn #shim_name() -> <#ty as ::wasm_bindgen::convert::WasmBoundary>::Abi;
                    }
                    unsafe {
                        ::wasm_bindgen::convert::WasmBoundary::from_abi(
                            #shim_name(),
                            &mut ::wasm_bindgen::convert::GlobalStack::new(),
                        )
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

impl ToTokens for ast::Type {
    fn to_tokens(&self, into: &mut Tokens) {
        match self.kind {
            ast::TypeKind::ByValue => {}
            ast::TypeKind::ByRef => {
                syn::token::And::default().to_tokens(into);
            }
            ast::TypeKind::ByMutRef => {
                syn::token::And::default().to_tokens(into);
                syn::token::Mut::default().to_tokens(into);
            }
        }
        self.ty.to_tokens(into);
    }
}
