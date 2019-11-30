//! Support for actually generating a JS function shim.
//!
//! This `Builder` type is used to generate JS function shims which sit between
//! exported functions, table elements, imports, etc. All function shims
//! generated by `wasm-bindgen` run through this type.

use crate::js::Context;
use crate::wit::InstructionData;
use crate::wit::{Adapter, AdapterId, AdapterKind, AdapterType, Instruction};
use anyhow::{anyhow, bail, Error};
use std::collections::HashSet;
use walrus::Module;

/// A one-size-fits-all builder for processing WebIDL bindings and generating
/// JS.
pub struct Builder<'a, 'b> {
    /// Parent context used to expose helper functions and such.
    pub cx: &'a mut Context<'b>,
    /// The TypeScript definition for each argument to this function.
    pub ts_args: Vec<TypescriptArg>,
    /// The TypeScript return value for this function.
    pub ts_ret: Option<TypescriptArg>,
    /// Whether or not this is building a constructor for a Rust class, and if
    /// so what class it's constructing.
    constructor: Option<String>,
    /// Whether or not this is building a method of a Rust class instance, and
    /// whether or not the method consumes `self` or not.
    method: Option<bool>,
    /// Whether or not we're catching exceptions from the main function
    /// invocation. Currently only used for imports.
    catch: bool,
    /// Whether or not we're logging the error coming out of this intrinsic
    log_error: bool,
}

/// Helper struct used in incoming/outgoing to generate JS.
pub struct JsBuilder<'a, 'b> {
    cx: &'a mut Context<'b>,
    typescript: Vec<TypescriptArg>,
    prelude: String,
    finally: String,
    tmp: usize,
    args: Vec<String>,
    stack: Vec<String>,
}

pub struct TypescriptArg {
    pub ty: String,
    pub name: String,
    pub optional: bool,
}

impl<'a, 'b> Builder<'a, 'b> {
    pub fn new(cx: &'a mut Context<'b>) -> Builder<'a, 'b> {
        Builder {
            log_error: cx.config.debug,
            cx,
            ts_args: Vec::new(),
            ts_ret: None,
            constructor: None,
            method: None,
            catch: false,
        }
    }

    pub fn method(&mut self, consumed: bool) {
        self.method = Some(consumed);
    }

    pub fn constructor(&mut self, class: &str) {
        self.constructor = Some(class.to_string());
    }

    pub fn catch(&mut self, catch: bool) {
        self.catch = catch;
    }

    pub fn disable_log_error(&mut self, disable: bool) {
        if disable {
            self.log_error = false;
        }
    }

    pub fn process(
        &mut self,
        adapter: &Adapter,
        instructions: &[InstructionData],
        explicit_arg_names: &Option<Vec<String>>,
    ) -> Result<String, Error> {
        let mut params = adapter.params.iter();
        let mut function_args = Vec::new();

        // If this is a method then we're generating this as part of a class
        // method, so the leading parameter is the this pointer stored on
        // the JS object, so synthesize that here.
        let mut js = JsBuilder::new(self.cx);
        match self.method {
            Some(consumes_self) => {
                drop(params.next());
                if js.cx.config.debug {
                    js.prelude(
                        "if (this.ptr == 0) throw new Error('Attempt to use a moved value');\n",
                    );
                }
                if consumes_self {
                    js.prelude("var ptr = this.ptr;");
                    js.prelude("this.ptr = 0;");
                    js.args.push("ptr".to_string());
                } else {
                    js.args.push("this.ptr".to_string());
                }
            }
            None => {}
        }
        for (i, _param) in params.enumerate() {
            let arg = match explicit_arg_names {
                Some(list) => list[i].clone(),
                None => format!("arg{}", i),
            };
            js.args.push(arg.clone());
            function_args.push(arg);
        }

        for instr in instructions {
            instruction(&mut js, &instr.instr)?;
        }

        assert_eq!(js.stack.len(), adapter.results.len());
        match js.stack.len() {
            0 => {}
            1 => {
                self.ts_ret = js.typescript.pop();
                let val = js.pop();
                js.prelude(&format!("return {};", val));
            }
            _ => {
                if true {
                    panic!()
                }
                let expr = js.stack.join(", ");
                js.stack.truncate(0);
                js.prelude(&format!("return [{}];", expr));
            }
        }
        assert!(js.stack.is_empty());
        self.ts_args = js.typescript;

        // Remove extraneous typescript args which were synthesized and aren't
        // part of our function shim.
        while self.ts_args.len() > function_args.len() {
            self.ts_args.remove(0);
        }

        let mut ret = String::new();
        ret.push_str("(");
        ret.push_str(&function_args.join(", "));
        ret.push_str(") {\n");

        let mut call = js.prelude;
        if js.finally.len() != 0 {
            call = format!("try {{\n{}}} finally {{\n{}}}\n", call, js.finally);
        }

        if self.catch {
            js.cx.expose_handle_error()?;
            call = format!("try {{\n{}}} catch (e) {{\n handleError(e)\n}}\n", call);
        }

        // Generate a try/catch block in debug mode which handles unexpected and
        // unhandled exceptions, typically used on imports. This currently just
        // logs what happened, but keeps the exception being thrown to propagate
        // elsewhere.
        if self.log_error {
            js.cx.expose_log_error();
            call = format!("try {{\n{}}} catch (e) {{\n logError(e)\n}}\n", call);
        }

        ret.push_str(&call);
        ret.push_str("}");

        return Ok(ret);
    }

    /// Returns the typescript signature of the binding that this has described.
    /// This is used to generate all the TypeScript definitions later on.
    ///
    /// Note that the TypeScript returned here is just the argument list and the
    /// return value, it doesn't include the function name in any way.
    pub fn typescript_signature(&self) -> String {
        // Build up the typescript signature as well
        let mut omittable = true;
        let mut ts_args = Vec::new();
        for arg in self.ts_args.iter().rev() {
            // In TypeScript, we can mark optional parameters as omittable
            // using the `?` suffix, but only if they're not followed by
            // non-omittable parameters. Therefore iterate the parameter list
            // in reverse and stop using the `?` suffix for optional params as
            // soon as a non-optional parameter is encountered.
            if arg.optional {
                if omittable {
                    ts_args.push(format!("{}?: {}", arg.name, arg.ty));
                } else {
                    ts_args.push(format!("{}: {} | undefined", arg.name, arg.ty));
                }
            } else {
                omittable = false;
                ts_args.push(format!("{}: {}", arg.name, arg.ty));
            }
        }
        ts_args.reverse();
        let mut ts = format!("({})", ts_args.join(", "));

        // Constructors have no listed return type in typescript
        if self.constructor.is_none() {
            ts.push_str(": ");
            if let Some(ty) = &self.ts_ret {
                ts.push_str(&ty.ty);
                if ty.optional {
                    ts.push_str(" | undefined");
                }
            } else {
                ts.push_str("void");
            }
        }
        return ts;
    }

    /// Returns a helpful JS doc comment which lists types for all parameters
    /// and the return value.
    pub fn js_doc_comments(&self) -> String {
        let mut ret: String = self
            .ts_args
            .iter()
            .map(|a| {
                if a.optional {
                    format!("@param {{{} | undefined}} {}\n", a.ty, a.name)
                } else {
                    format!("@param {{{}}} {}\n", a.ty, a.name)
                }
            })
            .collect();
        if let Some(ts) = &self.ts_ret {
            ret.push_str(&format!("@returns {{{}}}", ts.ty));
        }
        ret
    }
}

impl<'a, 'b> JsBuilder<'a, 'b> {
    pub fn new(cx: &'a mut Context<'b>) -> JsBuilder<'a, 'b> {
        JsBuilder {
            cx,
            args: Vec::new(),
            tmp: 0,
            finally: String::new(),
            prelude: String::new(),
            typescript: Vec::new(),
            stack: Vec::new(),
        }
    }

    pub fn typescript_len(&self) -> usize {
        self.typescript.len()
    }

    pub fn arg(&self, idx: u32) -> &str {
        &self.args[idx as usize]
    }

    pub fn typescript_required(&mut self, ty: &str) {
        let name = self.arg_name();
        self.typescript.push(TypescriptArg {
            ty: ty.to_string(),
            optional: false,
            name,
        });
    }

    pub fn typescript_optional(&mut self, ty: &str) {
        let name = self.arg_name();
        self.typescript.push(TypescriptArg {
            ty: ty.to_string(),
            optional: true,
            name,
        });
    }

    fn arg_name(&self) -> String {
        self.args
            .get(self.typescript.len())
            .cloned()
            .unwrap_or_else(|| format!("arg{}", self.typescript.len()))
    }

    pub fn prelude(&mut self, prelude: &str) {
        for line in prelude.trim().lines().map(|l| l.trim()) {
            if !line.is_empty() {
                self.prelude.push_str(line);
                self.prelude.push_str("\n");
            }
        }
    }

    pub fn finally(&mut self, finally: &str) {
        for line in finally.trim().lines().map(|l| l.trim()) {
            if !line.is_empty() {
                self.finally.push_str(line);
                self.finally.push_str("\n");
            }
        }
    }

    pub fn tmp(&mut self) -> usize {
        let ret = self.tmp;
        self.tmp += 1;
        return ret;
    }

    fn pop(&mut self) -> String {
        self.stack.pop().unwrap()
    }

    fn push(&mut self, arg: String) {
        self.stack.push(arg);
    }

    fn assert_class(&mut self, arg: &str, class: &str) {
        self.cx.expose_assert_class();
        self.prelude(&format!("_assertClass({}, {});", arg, class));
    }

    fn assert_number(&mut self, arg: &str) {
        if !self.cx.config.debug {
            return;
        }
        self.cx.expose_assert_num();
        self.prelude(&format!("_assertNum({});", arg));
    }

    fn assert_bool(&mut self, arg: &str) {
        if !self.cx.config.debug {
            return;
        }
        self.cx.expose_assert_bool();
        self.prelude(&format!("_assertBoolean({});", arg));
    }

    fn assert_optional_number(&mut self, arg: &str) {
        if !self.cx.config.debug {
            return;
        }
        self.cx.expose_is_like_none();
        self.prelude(&format!("if (!isLikeNone({})) {{", arg));
        self.assert_number(arg);
        self.prelude("}");
    }

    fn assert_optional_bool(&mut self, arg: &str) {
        if !self.cx.config.debug {
            return;
        }
        self.cx.expose_is_like_none();
        self.prelude(&format!("if (!isLikeNone({})) {{", arg));
        self.assert_bool(arg);
        self.prelude("}");
    }

    fn assert_not_moved(&mut self, arg: &str) {
        if !self.cx.config.debug {
            return;
        }
        self.prelude(&format!(
            "\
                if ({0}.ptr === 0) {{
                    throw new Error('Attempt to use a moved value');
                }}
            ",
            arg,
        ));
    }
}

fn instruction(js: &mut JsBuilder, instr: &Instruction) -> Result<(), Error> {
    // Here first properly aligned nonzero address is chosen to be the
    // out-pointer. We use the address for a BigInt64Array sometimes which
    // means it needs to be 8-byte aligned. Otherwise valid code is
    // unlikely to ever be working around address 8, so this should be a
    // safe address to use for returning data through.
    let retptr_val = 8;

    match instr {
        Instruction::Standard(wit_walrus::Instruction::ArgGet(n)) => {
            let arg = js.arg(*n).to_string();
            js.push(arg);
        }

        Instruction::Standard(wit_walrus::Instruction::CallAdapter(_)) => {
            panic!("standard call adapter functions should be mapped to our adapters");
        }

        Instruction::Standard(wit_walrus::Instruction::CallCore(_))
        | Instruction::CallExport(_)
        | Instruction::CallAdapter(_)
        | Instruction::CallTableElement(_)
        | Instruction::Standard(wit_walrus::Instruction::DeferCallCore(_)) => {
            let invoc = Invocation::from(instr, js.cx.module)?;
            let (params, results) = invoc.params_results(js.cx);

            // Pop off the number of parameters for the function we're calling
            let mut args = Vec::new();
            for _ in 0..params {
                args.push(js.pop());
            }
            args.reverse();

            // Call the function through an export of the underlying module.
            let call = invoc.invoke(js.cx, &args, &mut js.prelude)?;

            // And then figure out how to actually handle where the call
            // happens. This is pretty conditional depending on the number of
            // return values of the function.
            match (invoc.defer(), results) {
                (true, 0) => {
                    js.finally(&format!("{};", call));
                    js.stack.extend(args);
                }
                (true, _) => panic!("deferred calls must have no results"),
                (false, 0) => js.prelude(&format!("{};", call)),
                (false, n) => {
                    js.prelude(&format!("var ret = {};", call));
                    if n == 1 {
                        js.push("ret".to_string());
                    } else {
                        for i in 0..n {
                            js.push(format!("ret[{}]", i));
                        }
                    }
                }
            }
        }

        Instruction::Standard(wit_walrus::Instruction::IntToWasm { trap: false, .. }) => {
            js.typescript_required("number");
            let val = js.pop();
            js.assert_number(&val);
            js.push(val);
        }

        // When converting to a JS number we need to specially handle the `u32`
        // case because if the high bit is set then it comes out as a negative
        // number, but we want to switch that to an unsigned representation.
        Instruction::Standard(wit_walrus::Instruction::WasmToInt {
            trap: false,
            output,
            ..
        }) => {
            js.typescript_required("number");
            let val = js.pop();
            match output {
                wit_walrus::ValType::U32 => js.push(format!("{} >>> 0", val)),
                _ => js.push(val),
            }
        }

        Instruction::Standard(wit_walrus::Instruction::WasmToInt { trap: true, .. })
        | Instruction::Standard(wit_walrus::Instruction::IntToWasm { trap: true, .. }) => {
            bail!("trapping wasm-to-int and int-to-wasm instructions not supported")
        }

        Instruction::Standard(wit_walrus::Instruction::MemoryToString(mem)) => {
            js.typescript_required("string");
            let len = js.pop();
            let ptr = js.pop();
            let get = js.cx.expose_get_string_from_wasm(*mem)?;
            js.push(format!("{}({}, {})", get, ptr, len));
        }

        Instruction::Standard(wit_walrus::Instruction::StringToMemory { mem, malloc }) => {
            js.typescript_required("string");
            let pass = js.cx.expose_pass_string_to_wasm(*mem)?;
            let val = js.pop();
            let malloc = js.cx.export_name_of(*malloc);
            let i = js.tmp();
            js.prelude(&format!(
                "var ptr{i} = {f}({0}, wasm.{malloc});",
                val,
                i = i,
                f = pass,
                malloc = malloc,
            ));
            js.prelude(&format!("var len{} = WASM_VECTOR_LEN;", i));
            js.push(format!("ptr{}", i));
            js.push(format!("len{}", i));
        }

        Instruction::Retptr => js.stack.push(retptr_val.to_string()),

        Instruction::StoreRetptr { ty, offset, mem } => {
            let (mem, size) = match ty {
                AdapterType::I32 => (js.cx.expose_int32_memory(*mem), 4),
                AdapterType::F32 => (js.cx.expose_f32_memory(*mem), 4),
                AdapterType::F64 => (js.cx.expose_f64_memory(*mem), 8),
                other => bail!("invalid aggregate return type {:?}", other),
            };
            // Note that we always assume the return pointer is argument 0,
            // which is currently the case for LLVM.
            let val = js.pop();
            let expr = format!(
                "{}()[{} / {} + {}] = {};",
                mem,
                js.arg(0),
                size,
                offset,
                val,
            );
            js.prelude(&expr);
        }

        Instruction::LoadRetptr { ty, offset, mem } => {
            let (mem, size) = match ty {
                AdapterType::I32 => (js.cx.expose_int32_memory(*mem), 4),
                AdapterType::F32 => (js.cx.expose_f32_memory(*mem), 4),
                AdapterType::F64 => (js.cx.expose_f64_memory(*mem), 8),
                other => bail!("invalid aggregate return type {:?}", other),
            };
            // If we're loading from the return pointer then we must have pushed
            // it earlier, and we always push the same value, so load that value
            // here
            let expr = format!("{}()[{} / {} + {}]", mem, retptr_val, size, offset,);
            js.prelude(&format!("var r{} = {};", offset, expr));
            js.push(format!("r{}", offset));
        }

        Instruction::I32FromBool => {
            js.typescript_required("boolean");
            let val = js.pop();
            js.assert_bool(&val);
            // JS will already coerce booleans into numbers for us
            js.push(val);
        }

        Instruction::I32FromStringFirstChar => {
            js.typescript_required("string");
            let val = js.pop();
            js.push(format!("{}.codePointAt(0)", val));
        }

        Instruction::I32FromAnyrefOwned => {
            js.typescript_required("any");
            js.cx.expose_add_heap_object();
            let val = js.pop();
            js.push(format!("addHeapObject({})", val));
        }

        Instruction::I32FromAnyrefBorrow => {
            js.typescript_required("any");
            js.cx.expose_borrowed_objects();
            js.cx.expose_global_stack_pointer();
            let val = js.pop();
            js.push(format!("addBorrowedObject({})", val));
            js.finally("heap[stack_pointer++] = undefined;");
        }

        Instruction::I32FromAnyrefRustOwned { class } => {
            js.typescript_required(class);
            let val = js.pop();
            js.assert_class(&val, &class);
            js.assert_not_moved(&val);
            let i = js.tmp();
            js.prelude(&format!("var ptr{} = {}.ptr;", i, val));
            js.prelude(&format!("{}.ptr = 0;", val));
            js.push(format!("ptr{}", i));
        }

        Instruction::I32FromAnyrefRustBorrow { class } => {
            js.typescript_required(class);
            let val = js.pop();
            js.assert_class(&val, &class);
            js.assert_not_moved(&val);
            js.push(format!("{}.ptr", val));
        }

        Instruction::I32FromOptionRust { class } => {
            js.typescript_optional(class);
            let val = js.pop();
            js.cx.expose_is_like_none();
            let i = js.tmp();
            js.prelude(&format!("let ptr{} = 0;", i));
            js.prelude(&format!("if (!isLikeNone({0})) {{", val));
            js.assert_class(&val, class);
            js.assert_not_moved(&val);
            js.prelude(&format!("ptr{} = {}.ptr;", i, val));
            js.prelude(&format!("{}.ptr = 0;", val));
            js.prelude("}");
            js.push(format!("ptr{}", i));
        }

        Instruction::I32Split64 { signed } => {
            js.typescript_required("BigInt");
            let val = js.pop();
            let f = if *signed {
                js.cx.expose_int64_cvt_shim()
            } else {
                js.cx.expose_uint64_cvt_shim()
            };
            let i = js.tmp();
            js.prelude(&format!(
                "
                 {f}[0] = {val};
                 const low{i} = u32CvtShim[0];
                 const high{i} = u32CvtShim[1];
                 ",
                i = i,
                f = f,
                val = val,
            ));
            js.push(format!("low{}", i));
            js.push(format!("high{}", i));
        }

        Instruction::I32SplitOption64 { signed } => {
            js.typescript_optional("BigInt");
            let val = js.pop();
            js.cx.expose_is_like_none();
            let f = if *signed {
                js.cx.expose_int64_cvt_shim()
            } else {
                js.cx.expose_uint64_cvt_shim()
            };
            let i = js.tmp();
            js.prelude(&format!(
                "\
                    {f}[0] = isLikeNone({val}) ? BigInt(0) : {val};
                    const low{i} = u32CvtShim[0];
                    const high{i} = u32CvtShim[1];
                ",
                i = i,
                f = f,
                val = val,
            ));
            js.push(format!("!isLikeNone({0})", val));
            js.push(format!("low{}", i));
            js.push(format!("high{}", i));
        }

        Instruction::I32FromOptionAnyref => {
            js.typescript_optional("any");
            let val = js.pop();
            js.cx.expose_is_like_none();
            // TODO: would be great to handle this in the anyref pass so we
            // don't have to worry about it here, shouldn't have an extra
            // switch.
            if js.cx.config.anyref {
                js.cx.expose_add_to_anyref_table()?;
                js.push(format!("isLikeNone({0}) ? 0 : addToAnyrefTable({0})", val));
            } else {
                js.cx.expose_add_heap_object();
                js.push(format!("isLikeNone({0}) ? 0 : addHeapObject({0})", val));
            }
        }

        Instruction::I32FromOptionU32Sentinel => {
            js.typescript_optional("number");
            let val = js.pop();
            js.cx.expose_is_like_none();
            js.assert_optional_number(&val);
            js.push(format!("isLikeNone({0}) ? 0xFFFFFF : {0}", val));
        }

        Instruction::I32FromOptionBool => {
            js.typescript_optional("boolean");
            let val = js.pop();
            js.cx.expose_is_like_none();
            js.assert_optional_bool(&val);
            js.push(format!("isLikeNone({0}) ? 0xFFFFFF : {0} ? 1 : 0", val));
        }

        Instruction::I32FromOptionChar => {
            js.typescript_optional("string");
            let val = js.pop();
            js.cx.expose_is_like_none();
            js.push(format!(
                "isLikeNone({0}) ? 0xFFFFFF : {0}.codePointAt(0)",
                val
            ));
        }

        Instruction::I32FromOptionEnum { hole } => {
            js.typescript_optional("number");
            let val = js.pop();
            js.cx.expose_is_like_none();
            js.assert_optional_number(&val);
            js.push(format!("isLikeNone({0}) ? {1} : {0}", val, hole));
        }

        Instruction::FromOptionNative { .. } => {
            js.typescript_optional("number");
            let val = js.pop();
            js.cx.expose_is_like_none();
            js.assert_optional_number(&val);
            js.push(format!("!isLikeNone({0})", val));
            js.push(format!("isLikeNone({0}) ? 0 : {0}", val));
        }

        Instruction::VectorToMemory { kind, malloc, mem } => {
            js.typescript_required(kind.js_ty());
            let val = js.pop();
            let func = js.cx.pass_to_wasm_function(*kind, *mem)?;
            let malloc = js.cx.export_name_of(*malloc);
            let i = js.tmp();
            js.prelude(&format!(
                "var ptr{i} = {f}({0}, wasm.{malloc});",
                val,
                i = i,
                f = func,
                malloc = malloc,
            ));
            js.prelude(&format!("var len{} = WASM_VECTOR_LEN;", i));
            js.push(format!("ptr{}", i));
            js.push(format!("len{}", i));
        }

        Instruction::OptionVector { kind, mem, malloc } => {
            js.typescript_optional(kind.js_ty());
            let func = js.cx.pass_to_wasm_function(*kind, *mem)?;
            js.cx.expose_is_like_none();
            let i = js.tmp();
            let malloc = js.cx.export_name_of(*malloc);
            let val = js.pop();
            js.prelude(&format!(
                "var ptr{i} = isLikeNone({0}) ? 0 : {f}({0}, wasm.{malloc});",
                val,
                i = i,
                f = func,
                malloc = malloc,
            ));
            js.prelude(&format!("var len{} = WASM_VECTOR_LEN;", i));
            js.push(format!("ptr{}", i));
            js.push(format!("len{}", i));
        }

        Instruction::MutableSliceToMemory {
            kind,
            malloc,
            mem,
            free,
        } => {
            js.typescript_required(kind.js_ty());
            // First up, pass the JS value into wasm, getting out a pointer and
            // a length. These two pointer/length values get pushed onto the
            // value stack.
            let val = js.pop();
            let func = js.cx.pass_to_wasm_function(*kind, *mem)?;
            let malloc = js.cx.export_name_of(*malloc);
            let i = js.tmp();
            js.prelude(&format!(
                "var ptr{i} = {f}({val}, wasm.{malloc});",
                val = val,
                i = i,
                f = func,
                malloc = malloc,
            ));
            js.prelude(&format!("var len{} = WASM_VECTOR_LEN;", i));
            js.push(format!("ptr{}", i));
            js.push(format!("len{}", i));

            // Next we set up a `finally` clause which will both update the
            // original mutable slice with any modifications, and then free the
            // Rust-backed memory.
            let free = js.cx.export_name_of(*free);
            let get = js.cx.memview_function(*kind, *mem);
            js.finally(&format!(
                "
                    {val}.set({get}().subarray(ptr{i} / {size}, ptr{i} / {size} + len{i}));
                    wasm.{free}(ptr{i}, len{i} * {size});
                ",
                val = val,
                get = get,
                free = free,
                size = kind.size(),
                i = i,
            ));
        }

        Instruction::BoolFromI32 => {
            js.typescript_required("bool");
            let val = js.pop();
            js.push(format!("{} !== 0", val));
        }

        Instruction::AnyrefLoadOwned => {
            js.typescript_required("any");
            js.cx.expose_take_object();
            let val = js.pop();
            js.push(format!("takeObject({})", val));
        }

        Instruction::StringFromChar => {
            js.typescript_required("string");
            let val = js.pop();
            js.push(format!("String.fromCodePoint({})", val));
        }

        Instruction::I64FromLoHi { signed } => {
            js.typescript_required("BigInt");
            let f = if *signed {
                js.cx.expose_int64_cvt_shim()
            } else {
                js.cx.expose_uint64_cvt_shim()
            };
            let i = js.tmp();
            let high = js.pop();
            let low = js.pop();
            js.prelude(&format!(
                "\
                     u32CvtShim[0] = {low};
                     u32CvtShim[1] = {high};
                     const n{i} = {f}[0];
                 ",
                low = low,
                high = high,
                f = f,
                i = i,
            ));
            js.push(format!("n{}", i))
        }

        Instruction::RustFromI32 { class } => {
            js.typescript_required(class);
            js.cx.require_class_wrap(class);
            let val = js.pop();
            js.push(format!("{}.__wrap({})", class, val));
        }

        Instruction::OptionRustFromI32 { class } => {
            js.typescript_optional(class);
            js.cx.require_class_wrap(class);
            let val = js.pop();
            js.push(format!(
                "{0} === 0 ? undefined : {1}.__wrap({0})",
                val, class,
            ))
        }

        Instruction::CachedStringLoad {
            owned,
            optional,
            mem,
            free,
        } => {
            if *optional {
                js.typescript_optional("string");
            } else {
                js.typescript_required("string");
            }

            let len = js.pop();
            let ptr = js.pop();
            let tmp = js.tmp();

            let get = js.cx.expose_get_cached_string_from_wasm(*mem)?;

            js.prelude(&format!("var v{} = {}({}, {});", tmp, get, ptr, len));

            if *owned {
                let free = js.cx.export_name_of(*free);
                js.prelude(&format!(
                    "if ({ptr} !== 0) {{ wasm.{}({ptr}, {len}); }}",
                    free,
                    ptr = ptr,
                    len = len,
                ));
            }

            js.push(format!("v{}", tmp));
        }

        Instruction::TableGet => {
            js.typescript_required("any");
            let val = js.pop();
            js.cx.expose_get_object();
            js.push(format!("getObject({})", val));
        }

        Instruction::StackClosure {
            adapter,
            nargs,
            mutable,
        } => {
            js.typescript_optional("any");
            let i = js.tmp();
            let b = js.pop();
            let a = js.pop();
            js.prelude(&format!("var state{} = {{a: {}, b: {}}};", i, a, b,));
            let args = (0..*nargs)
                .map(|i| format!("arg{}", i))
                .collect::<Vec<_>>()
                .join(", ");
            let wrapper = js.cx.adapter_name(*adapter);
            if *mutable {
                // Mutable closures need protection against being called
                // recursively, so ensure that we clear out one of the
                // internal pointers while it's being invoked.
                js.prelude(&format!(
                    "var cb{i} = ({args}) => {{
                        const a = state{i}.a;
                        state{i}.a = 0;
                        try {{
                            return {name}(a, state{i}.b, {args});
                        }} finally {{
                            state{i}.a = a;
                        }}
                    }};",
                    i = i,
                    args = args,
                    name = wrapper,
                ));
            } else {
                js.prelude(&format!(
                    "var cb{i} = ({args}) => {wrapper}(state{i}.a, state{i}.b, {args});",
                    i = i,
                    args = args,
                    wrapper = wrapper,
                ));
            }

            // Make sure to null out our internal pointers when we return
            // back to Rust to ensure that any lingering references to the
            // closure will fail immediately due to null pointers passed in
            // to Rust.
            js.finally(&format!("state{}.a = state{0}.b = 0;", i));
            js.push(format!("cb{}", i));
        }

        Instruction::VectorLoad { kind, mem, free } => {
            js.typescript_required(kind.js_ty());
            let len = js.pop();
            let ptr = js.pop();
            let f = js.cx.expose_get_vector_from_wasm(*kind, *mem)?;
            let i = js.tmp();
            let free = js.cx.export_name_of(*free);
            js.prelude(&format!("var v{} = {}({}, {}).slice();", i, f, ptr, len));
            js.prelude(&format!(
                "wasm.{}({}, {} * {});",
                free,
                ptr,
                len,
                kind.size()
            ));
            js.push(format!("v{}", i))
        }

        Instruction::OptionVectorLoad { kind, mem, free } => {
            js.typescript_optional(kind.js_ty());
            let len = js.pop();
            let ptr = js.pop();
            let f = js.cx.expose_get_vector_from_wasm(*kind, *mem)?;
            let i = js.tmp();
            let free = js.cx.export_name_of(*free);
            js.prelude(&format!("let v{};", i));
            js.prelude(&format!("if ({} !== 0) {{", ptr));
            js.prelude(&format!("v{} = {}({}, {}).slice();", i, f, ptr, len));
            js.prelude(&format!(
                "wasm.{}({}, {} * {});",
                free,
                ptr,
                len,
                kind.size()
            ));
            js.prelude("}");
            js.push(format!("v{}", i));
        }

        Instruction::View { kind, mem } => {
            js.typescript_required(kind.js_ty());
            let len = js.pop();
            let ptr = js.pop();
            let f = js.cx.expose_get_vector_from_wasm(*kind, *mem)?;
            js.push(format!("{f}({ptr}, {len})", ptr = ptr, len = len, f = f));
        }

        Instruction::OptionView { kind, mem } => {
            js.typescript_optional(kind.js_ty());
            let len = js.pop();
            let ptr = js.pop();
            let f = js.cx.expose_get_vector_from_wasm(*kind, *mem)?;
            js.push(format!(
                "{ptr} === 0 ? undefined : {f}({ptr}, {len})",
                ptr = ptr,
                len = len,
                f = f
            ));
        }

        Instruction::OptionU32Sentinel => {
            js.typescript_optional("number");
            let val = js.pop();
            js.push(format!("{0} === 0xFFFFFF ? undefined : {0}", val));
        }

        Instruction::ToOptionNative { ty: _, signed } => {
            js.typescript_optional("number");
            let val = js.pop();
            let present = js.pop();
            js.push(format!(
                "{} === 0 ? undefined : {}{}",
                present,
                val,
                if *signed { "" } else { " >>> 0" },
            ));
        }

        Instruction::OptionBoolFromI32 => {
            js.typescript_optional("boolean");
            let val = js.pop();
            js.push(format!("{0} === 0xFFFFFF ? undefined : {0} !== 0", val));
        }

        Instruction::OptionCharFromI32 => {
            js.typescript_optional("string");
            let val = js.pop();
            js.push(format!(
                "{0} === 0xFFFFFF ? undefined : String.fromCodePoint({0})",
                val,
            ));
        }

        Instruction::OptionEnumFromI32 { hole } => {
            js.typescript_optional("number");
            let val = js.pop();
            js.push(format!("{0} === {1} ? undefined : {0}", val, hole));
        }

        Instruction::Option64FromI32 { signed } => {
            js.typescript_optional("BigInt");
            let f = if *signed {
                js.cx.expose_int64_cvt_shim()
            } else {
                js.cx.expose_uint64_cvt_shim()
            };
            let i = js.tmp();
            let high = js.pop();
            let low = js.pop();
            let present = js.pop();
            js.prelude(&format!(
                "
                    u32CvtShim[0] = {low};
                    u32CvtShim[1] = {high};
                    const n{i} = {present} === 0 ? undefined : {f}[0];
                ",
                present = present,
                low = low,
                high = high,
                f = f,
                i = i,
            ));
            js.push(format!("n{}", i));
        }
    }
    Ok(())
}

enum Invocation {
    Core { id: walrus::FunctionId, defer: bool },
    Adapter(AdapterId),
}

impl Invocation {
    fn from(instr: &Instruction, module: &Module) -> Result<Invocation, Error> {
        use Instruction::*;
        Ok(match instr {
            Standard(wit_walrus::Instruction::CallCore(f)) => Invocation::Core {
                id: *f,
                defer: false,
            },

            Standard(wit_walrus::Instruction::DeferCallCore(f)) => Invocation::Core {
                id: *f,
                defer: true,
            },

            CallExport(e) => match module.exports.get(*e).item {
                walrus::ExportItem::Function(id) => Invocation::Core { id, defer: false },
                _ => panic!("can only call exported function"),
            },

            // The function table never changes right now, so we can statically
            // look up the desired function.
            CallTableElement(idx) => {
                let table = module
                    .tables
                    .main_function_table()?
                    .ok_or_else(|| anyhow!("no function table found"))?;
                let functions = match &module.tables.get(table).kind {
                    walrus::TableKind::Function(f) => f,
                    _ => bail!("should have found a function table"),
                };
                let id = functions
                    .elements
                    .get(*idx as usize)
                    .and_then(|id| *id)
                    .ok_or_else(|| anyhow!("function table wasn't filled in a {}", idx))?;
                Invocation::Core { id, defer: false }
            }

            CallAdapter(id) => Invocation::Adapter(*id),

            // this function is only called for the above instructions
            _ => unreachable!(),
        })
    }

    fn params_results(&self, cx: &Context) -> (usize, usize) {
        match self {
            Invocation::Core { id, .. } => {
                let ty = cx.module.funcs.get(*id).ty();
                let ty = cx.module.types.get(ty);
                (ty.params().len(), ty.results().len())
            }
            Invocation::Adapter(id) => {
                let adapter = &cx.wit.adapters[id];
                (adapter.params.len(), adapter.results.len())
            }
        }
    }

    fn invoke(
        &self,
        cx: &mut Context,
        args: &[String],
        prelude: &mut String,
    ) -> Result<String, Error> {
        match self {
            Invocation::Core { id, .. } => {
                let name = cx.export_name_of(*id);
                Ok(format!("wasm.{}({})", name, args.join(", ")))
            }
            Invocation::Adapter(id) => {
                let adapter = &cx.wit.adapters[id];
                let kind = match adapter.kind {
                    AdapterKind::Import { kind, .. } => kind,
                    AdapterKind::Local { .. } => {
                        bail!("adapter-to-adapter calls not supported yet");
                    }
                };
                let import = &cx.aux.import_map[id];
                let variadic = cx.aux.imports_with_variadic.contains(id);
                cx.invoke_import(import, kind, args, variadic, prelude)
            }
        }
    }

    fn defer(&self) -> bool {
        match self {
            Invocation::Core { defer, .. } => *defer,
            _ => false,
        }
    }
}
