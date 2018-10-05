//! NO GUARANTEES ARE MADE ABOUT THE BACKWARDS COMPATIBILITY OF THIS LIBRARY
//!
//! Internal library of `wasm-gc` to remove unreferenced items from a wasm
//! executable.

extern crate parity_wasm;
#[macro_use]
extern crate log;
extern crate rustc_demangle;

use std::mem;
use std::collections::HashSet;

use parity_wasm::elements::*;

use bitvec::BitSet;

mod bitvec;

pub struct Config {
    demangle: bool,
    keep_debug: bool,
}

impl Config {
    /// Creates a blank slate of configuration, ready to gc wasm files.
    pub fn new() -> Config {
        Config {
            demangle: true,
            keep_debug: false,
        }
    }

    /// Configures whether or not this will demangle symbols as part of the gc
    /// pass.
    pub fn demangle(&mut self, demangle: bool) -> &mut Self {
        self.demangle = demangle;
        self
    }

    /// Configures whether or not debug sections will be preserved.
    pub fn keep_debug(&mut self, keep_debug: bool) -> &mut Self {
        self.keep_debug = keep_debug;
        self
    }

    pub fn run(&mut self, module: &mut Module) {
        run(self, module);
    }
}

fn run(config: &mut Config, module: &mut Module) {
    let analysis = {
        let mut cx = LiveContext::new(&module);
        cx.blacklist.insert("rust_eh_personality");

        if let Some(section) = module.export_section() {
            for (i, entry) in section.entries().iter().enumerate() {
                cx.add_export_entry(entry, i as u32);
            }
        }
        if let Some(section) = module.import_section() {
            for (i, entry) in section.entries().iter().enumerate() {
                debug!("import {:?}", entry);
                if let External::Memory(_) = *entry.external() {
                    cx.add_import_entry(entry, i as u32);
                }
            }
        }
        if let Some(section) = module.data_section() {
            for entry in section.entries() {
                cx.add_data_segment(entry);
            }
        }
        if let Some(tables) = module.table_section() {
            for i in 0..tables.entries().len() as u32 {
                cx.add_table(i);
            }
        }
        if let Some(elements) = module.elements_section() {
            for seg in elements.entries() {
                cx.add_element_segment(seg);
            }
        }
        if let Some(i) = module.start_section() {
            cx.add_function(i);
        }
        cx.analysis
    };

    let cx = RemapContext::new(&module, &analysis, config);
    for i in (0..module.sections().len()).rev() {
        let retain = match module.sections_mut()[i] {
            Section::Unparsed { .. } => {
                info!("unparsed section");
                continue
            }
            Section::Custom(ref s) => {
                if !cx.config.keep_debug && s.name().starts_with(".debug_") {
                    false
                } else {
                    info!("skipping custom section: {}", s.name());
                    continue
                }
            }
            Section::Reloc(..) => {
                info!("skipping reloc section");
                continue
            }
            Section::Type(ref mut s) => cx.remap_type_section(s),
            Section::Import(ref mut s) => cx.remap_import_section(s),
            Section::Function(ref mut s) => cx.remap_function_section(s),
            Section::Table(ref mut s) => cx.remap_table_section(s),
            Section::Memory(ref mut s) => cx.remap_memory_section(s),
            Section::Global(ref mut s) => cx.remap_global_section(s),
            Section::Export(ref mut s) => cx.remap_export_section(s),
            Section::Start(ref mut i) => { cx.remap_function_idx(i); true }
            Section::Element(ref mut s) => cx.remap_element_section(s),
            Section::Code(ref mut s) => cx.remap_code_section(s),
            Section::Data(ref mut s) => cx.remap_data_section(s),
            Section::Name(ref mut s) => { cx.remap_name_section(s); true }
        };
        if !retain {
            debug!("remove empty section");
            module.sections_mut().remove(i);
        }
    }
}

#[derive(Default)]
struct Analysis {
    codes: BitSet,
    tables: BitSet,
    memories: BitSet,
    globals: BitSet,
    types: BitSet,
    imports: BitSet,
    exports: BitSet,
    functions: BitSet,
    all_globals: BitSet,
}

enum Memories<'a> {
    Exported(&'a MemorySection),
    Imported(&'a MemoryType),
}

impl<'a> Memories<'a> {
    fn has_entry(&self, idx: usize) -> bool {
        match *self {
            Memories::Exported(memory_section) => idx < memory_section.entries().len(),
            Memories::Imported(_) => idx == 0,
        }
    }
}

struct LiveContext<'a> {
    blacklist: HashSet<&'static str>,
    function_section: Option<&'a FunctionSection>,
    type_section: Option<&'a TypeSection>,
    code_section: Option<&'a CodeSection>,
    table_section: Option<&'a TableSection>,
    memories: Option<Memories<'a>>,
    global_section: Option<&'a GlobalSection>,
    import_section: Option<&'a ImportSection>,
    imported_functions: u32,
    imported_globals: u32,
    analysis: Analysis,
}

impl<'a> LiveContext<'a> {
    fn new(module: &'a Module) -> LiveContext<'a> {
        let memories = module.memory_section().map(Memories::Exported).or_else(|| {
            if let Some(import_section) = module.import_section() {
                for entry in import_section.entries() {
                    if let External::Memory(ref memory_type) = *entry.external() {
                        return Some(Memories::Imported(memory_type));
                    }
                }
            }

            None
        });

        LiveContext {
            blacklist: HashSet::new(),
            function_section: module.function_section(),
            type_section: module.type_section(),
            code_section: module.code_section(),
            table_section: module.table_section(),
            memories: memories,
            global_section: module.global_section(),
            import_section: module.import_section(),
            imported_functions: module.import_section()
                .map(|s| s.functions())
                .unwrap_or(0) as u32,
            imported_globals: module.import_section()
                .map(|s| s.globals())
                .unwrap_or(0) as u32,
            analysis: Analysis::default(),
        }
    }

    fn add_function(&mut self, idx: u32) {
        if !self.analysis.functions.insert(idx) {
            return
        }

        if idx < self.imported_functions {
            let imports = self.import_section.unwrap();
            debug!("adding import: {}", idx);
            let (i, import) = imports.entries()
                .iter()
                .enumerate()
                .filter(|&(_, i)| {
                    match *i.external() {
                        External::Function(_) => true,
                        _ => false,
                    }
                })
                .skip(idx as usize)
                .next()
                .expect("expected an imported function with this index");
            let i = i as u32;
            self.analysis.imports.insert(i);
            return self.add_import_entry(import, i);
        }
        let idx = idx - self.imported_functions;

        if !self.analysis.codes.insert(idx) {
            return
        }

        debug!("adding function: {}", idx);
        let functions = self.function_section.expect("no functions section");
        self.add_type(functions.entries()[idx as usize].type_ref());
        let codes = self.code_section.expect("no codes section");
        self.add_func_body(&codes.bodies()[idx as usize]);
    }

    fn add_table(&mut self, mut idx: u32) {
        if let Some(imports) = self.import_section {
            let imported_tables = imports.entries()
                .iter()
                .filter(|i| {
                    match *i.external() {
                        External::Table(_) => true,
                        _ => false,
                    }
                })
                .count();
            let imported_tables = imported_tables as u32;
            if idx < imported_tables {
                debug!("adding table import: {}", idx);
                let (i, import) = imports.entries()
                    .iter()
                    .enumerate()
                    .filter(|&(_, i)| {
                        match *i.external() {
                            External::Table(_) => true,
                            _ => false,
                        }
                    })
                    .skip(idx as usize)
                    .next()
                    .expect("expected an imported table with this index");
                let i = i as u32;
                self.analysis.imports.insert(i);
                return self.add_import_entry(import, i);
            }
            idx -= imported_tables;
        }
        if !self.analysis.tables.insert(idx) {
            return
        }
        let tables = self.table_section.expect("no table section");
        let table = &tables.entries()[idx as usize];
        drop(table);
    }

    fn add_memory(&mut self, idx: u32) {
        if !self.analysis.memories.insert(idx) {
            return
        }
        let memories = self.memories.as_ref().expect("no memory section or imported memory");
        assert!(memories.has_entry(idx as usize));
    }

    fn add_global(&mut self, idx: u32) {
        if !self.analysis.all_globals.insert(idx) {
            return
        }

        if idx < self.imported_globals {
            let imports = self.import_section.unwrap();
            debug!("adding global import: {}", idx);
            let (i, import) = imports.entries()
                .iter()
                .enumerate()
                .filter(|&(_, i)| {
                    match *i.external() {
                        External::Global(_) => true,
                        _ => false,
                    }
                })
                .skip(idx as usize)
                .next()
                .expect("expected an imported global with this index");
            let i = i as u32;
            self.analysis.imports.insert(i);
            return self.add_import_entry(import, i);
        }
        let idx = idx - self.imported_globals;

        if !self.analysis.globals.insert(idx) {
            return
        }
        let globals = self.global_section.expect("no global section");
        let global = &globals.entries()[idx as usize];
        self.add_global_type(global.global_type());
        self.add_init_expr(global.init_expr());
    }

    fn add_global_type(&mut self, t: &GlobalType) {
        self.add_value_type(&t.content_type());
    }

    fn add_init_expr(&mut self, t: &InitExpr) {
        for opcode in t.code() {
            self.add_opcode(opcode);
        }
    }

    fn add_type(&mut self, idx: u32) {
        if !self.analysis.types.insert(idx) {
            return
        }
        let types = self.type_section.expect("no types section");
        match types.types()[idx as usize] {
            Type::Function(ref f) => {
                for param in f.params() {
                    self.add_value_type(param);
                }
                if let Some(ref ret) = f.return_type() {
                    self.add_value_type(ret);
                }
            }
        }
    }

    fn add_value_type(&mut self, value: &ValueType) {
        match *value {
            ValueType::I32 => {}
            ValueType::I64 => {}
            ValueType::F32 => {}
            ValueType::F64 => {}
        }
    }

    fn add_func_body(&mut self, body: &FuncBody) {
        for local in body.locals() {
            self.add_value_type(&local.value_type());
        }
        self.add_opcodes(body.code());
    }

    fn add_opcodes(&mut self, code: &Instructions) {
        for opcode in code.elements() {
            self.add_opcode(opcode);
        }
    }

    fn add_opcode(&mut self, code: &Instruction) {
        match *code {
            Instruction::Block(ref b) |
            Instruction::Loop(ref b) |
            Instruction::If(ref b) => self.add_block_type(b),
            Instruction::Call(f) => self.add_function(f),
            Instruction::CallIndirect(t, _) => self.add_type(t),
            Instruction::GetGlobal(i) |
            Instruction::SetGlobal(i) => self.add_global(i),
            _ => {}
        }
    }

    fn add_block_type(&mut self, bt: &BlockType) {
        match *bt {
            BlockType::Value(ref v) => self.add_value_type(v),
            BlockType::NoResult => {}
        }
    }

    fn add_export_entry(&mut self, entry: &ExportEntry, idx: u32) {
        if self.blacklist.contains(entry.field()) {
            return
        }
        self.analysis.exports.insert(idx);
        match *entry.internal() {
            Internal::Function(i) => self.add_function(i),
            Internal::Table(i) => self.add_table(i),
            Internal::Memory(i) => self.add_memory(i),
            Internal::Global(i) => self.add_global(i),
        }
    }

    fn add_import_entry(&mut self, entry: &ImportEntry, idx: u32) {
        match *entry.external() {
            External::Function(i) => self.add_type(i),
            External::Table(_) => {},
            External::Memory(_) => {
                self.add_memory(0);
                self.analysis.imports.insert(idx);
            },
            External::Global(_) => {},
        }
    }

    fn add_data_segment(&mut self, data: &DataSegment) {
        self.add_memory(data.index());
        self.add_init_expr(data.offset());
    }

    fn add_element_segment(&mut self, seg: &ElementSegment) {
        for member in seg.members() {
            self.add_function(*member);
        }
        self.add_table(seg.index());
        self.add_init_expr(seg.offset());
    }
}

struct RemapContext<'a> {
    analysis: &'a Analysis,
    config: &'a Config,
    functions: Vec<u32>,
    globals: Vec<u32>,
    types: Vec<u32>,
    tables: Vec<u32>,
    memories: Vec<u32>,
}

impl<'a> RemapContext<'a> {
    fn new(m: &Module, analysis: &'a Analysis, config: &'a Config) -> RemapContext<'a> {
        let mut nfunctions = 0;
        let mut functions = Vec::new();
        let mut nglobals = 0;
        let mut globals = Vec::new();
        let mut types = Vec::new();
        let mut ntables = 0;
        let mut tables = Vec::new();
        let mut nmemories = 0;
        let mut memories = Vec::new();

        if let Some(s) = m.type_section() {
            let mut removed = 0;
            for i in 0..(s.types().len() as u32) {
                if analysis.types.contains(&i) {
                    types.push(i - removed);
                } else {
                    debug!("gc type {}", i);
                    types.push(u32::max_value());
                    removed += 1;
                }
            }
        }
        if let Some(s) = m.import_section() {
            for (i, import) in s.entries().iter().enumerate() {
                let (dst, ndst) = match *import.external() {
                    External::Function(_) => (&mut functions, &mut nfunctions),
                    External::Table(_) => (&mut tables, &mut ntables),
                    External::Memory(_) => (&mut memories, &mut nmemories),
                    External::Global(_) => (&mut globals, &mut nglobals),
                };
                if analysis.imports.contains(&(i as u32)) {
                    dst.push(*ndst);
                    *ndst += 1;
                } else {
                    debug!("gc import {}", i);
                    dst.push(u32::max_value());
                }
            }
        }
        if let Some(s) = m.function_section() {
            for i in 0..(s.entries().len() as u32) {
                if analysis.codes.contains(&i) {
                    functions.push(nfunctions);
                    nfunctions += 1;
                } else {
                    debug!("gc function {}", i);
                    functions.push(u32::max_value());
                }
            }
        }
        if let Some(s) = m.global_section() {
            for i in 0..(s.entries().len() as u32) {
                if analysis.globals.contains(&i) {
                    globals.push(nglobals);
                    nglobals += 1;
                } else {
                    debug!("gc global {}", i);
                    globals.push(u32::max_value());
                }
            }
        }
        if let Some(s) = m.table_section() {
            for i in 0..(s.entries().len() as u32) {
                if analysis.tables.contains(&i) {
                    tables.push(ntables);
                    ntables += 1;
                } else {
                    debug!("gc table {}", i);
                    tables.push(u32::max_value());
                }
            }
        }
        if let Some(s) = m.memory_section() {
            for i in 0..(s.entries().len() as u32) {
                if analysis.memories.contains(&i) {
                    memories.push(nmemories);
                    nmemories += 1;
                } else {
                    debug!("gc memory {}", i);
                    memories.push(u32::max_value());
                }
            }
        }

        RemapContext {
            analysis,
            functions,
            globals,
            memories,
            tables,
            types,
            config,
        }
    }

    fn retain<T>(&self, set: &BitSet, list: &mut Vec<T>, name: &str) {
        for i in (0..list.len()).rev().map(|x| x as u32) {
            if !set.contains(&i) {
                debug!("removing {} {}", name, i);
                list.remove(i as usize);
            }
        }
    }

    fn remap_type_section(&self, s: &mut TypeSection) -> bool {
        self.retain(&self.analysis.types, s.types_mut(), "type");
        for t in s.types_mut() {
            self.remap_type(t);
        }
        s.types().len() > 0
    }

    fn remap_type(&self, t: &mut Type) {
        match *t {
            Type::Function(ref mut t) => self.remap_function_type(t),
        }
    }

    fn remap_function_type(&self, t: &mut FunctionType) {
        for param in t.params_mut() {
            self.remap_value_type(param);
        }
        if let Some(m) = t.return_type_mut().as_mut() {
            self.remap_value_type(m);
        }
    }

    fn remap_value_type(&self, t: &mut ValueType) {
        drop(t);
    }

    fn remap_import_section(&self, s: &mut ImportSection) -> bool {
        self.retain(&self.analysis.imports, s.entries_mut(), "import");
        for i in s.entries_mut() {
            self.remap_import_entry(i);
        }
        s.entries().len() > 0
    }

    fn remap_import_entry(&self, s: &mut ImportEntry) {
        debug!("remap import entry {:?}", s);
        match *s.external_mut() {
            External::Function(ref mut f) => self.remap_type_idx(f),
            External::Table(_) => {}
            External::Memory(_) => {}
            External::Global(_) => {}
        }
    }

    fn remap_function_section(&self, s: &mut FunctionSection) -> bool {
        self.retain(&self.analysis.codes, s.entries_mut(), "function");
        for f in s.entries_mut() {
            self.remap_func(f);
        }
        s.entries().len() > 0
    }

    fn remap_func(&self, f: &mut Func) {
        self.remap_type_idx(f.type_ref_mut());
    }

    fn remap_table_section(&self, s: &mut TableSection) -> bool {
        self.retain(&self.analysis.tables, s.entries_mut(), "table");
        for t in s.entries_mut() {
            drop(t); // TODO
        }
        s.entries().len() > 0
    }

    fn remap_memory_section(&self, s: &mut MemorySection) -> bool {
        self.retain(&self.analysis.memories, s.entries_mut(), "memory");
        for m in s.entries_mut() {
            drop(m); // TODO
        }
        s.entries().len() > 0
    }

    fn remap_global_section(&self, s: &mut GlobalSection) -> bool {
        self.retain(&self.analysis.globals, s.entries_mut(), "global");
        for g in s.entries_mut() {
            self.remap_global_entry(g);
        }
        s.entries().len() > 0
    }

    fn remap_global_entry(&self, s: &mut GlobalEntry) {
        self.remap_global_type(s.global_type_mut());
        self.remap_init_expr(s.init_expr_mut());
    }

    fn remap_global_type(&self, s: &mut GlobalType) {
        drop(s);
    }

    fn remap_init_expr(&self, s: &mut InitExpr) {
        for code in s.code_mut() {
            self.remap_instruction(code);
        }
    }

    fn remap_export_section(&self, s: &mut ExportSection) -> bool {
        self.retain(&self.analysis.exports, s.entries_mut(), "export");
        for s in s.entries_mut() {
            self.remap_export_entry(s);
        }
        s.entries().len() > 0
    }

    fn remap_export_entry(&self, s: &mut ExportEntry) {
        match *s.internal_mut() {
            Internal::Function(ref mut i) => self.remap_function_idx(i),
            Internal::Table(ref mut i) => self.remap_table_idx(i),
            Internal::Memory(ref mut i) => self.remap_memory_idx(i),
            Internal::Global(ref mut i) => self.remap_global_idx(i),
        }
    }

    fn remap_element_section(&self, s: &mut ElementSection) -> bool {
        for s in s.entries_mut() {
            self.remap_element_segment(s);
        }
        true
    }

    fn remap_element_segment(&self, s: &mut ElementSegment) {
        let mut i = s.index();
        self.remap_table_idx(&mut i);
        assert_eq!(s.index(), i);
        for m in s.members_mut() {
            self.remap_function_idx(m);
        }
        self.remap_init_expr(s.offset_mut());
    }

    fn remap_code_section(&self, s: &mut CodeSection) -> bool {
        self.retain(&self.analysis.codes, s.bodies_mut(), "code");
        for s in s.bodies_mut() {
            self.remap_func_body(s);
        }
        s.bodies().len() > 0
    }

    fn remap_func_body(&self, b: &mut FuncBody) {
        self.remap_code(b.code_mut());
    }

    fn remap_code(&self, c: &mut Instructions) {
        for i in c.elements_mut() {
            self.remap_instruction(i);
        }
    }

    fn remap_instruction(&self, i: &mut Instruction) {
        match *i {
            Instruction::Block(ref mut b) |
            Instruction::Loop(ref mut b) |
            Instruction::If(ref mut b) => self.remap_block_type(b),
            Instruction::Call(ref mut f) => self.remap_function_idx(f),
            Instruction::CallIndirect(ref mut t, _) => self.remap_type_idx(t),
            Instruction::GetGlobal(ref mut i) |
            Instruction::SetGlobal(ref mut i) => self.remap_global_idx(i),
            _ => {}
        }
    }

    fn remap_block_type(&self, bt: &mut BlockType) {
        match *bt {
            BlockType::Value(ref mut v) => self.remap_value_type(v),
            BlockType::NoResult => {}
        }
    }

    fn remap_data_section(&self, s: &mut DataSection) -> bool {
        for data in s.entries_mut() {
            self.remap_data_segment(data);
        }
        true
    }

    fn remap_data_segment(&self, segment: &mut DataSegment) {
        let mut i = segment.index();
        self.remap_memory_idx(&mut i);
        assert_eq!(segment.index(), i);
        self.remap_init_expr(segment.offset_mut());
    }

    fn remap_type_idx(&self, i: &mut u32) {
        *i = self.types[*i as usize];
        assert!(*i != u32::max_value());
    }

    fn remap_function_idx(&self, i: &mut u32) {
        *i = self.functions[*i as usize];
        assert!(*i != u32::max_value());
    }

    fn remap_global_idx(&self, i: &mut u32) {
        trace!("global {} => {}", *i, self.globals[*i as usize]);
        *i = self.globals[*i as usize];
        assert!(*i != u32::max_value());
    }

    fn remap_table_idx(&self, i: &mut u32) {
        *i = self.tables[*i as usize];
        assert!(*i != u32::max_value());
    }

    fn remap_memory_idx(&self, i: &mut u32) {
        *i = self.memories[*i as usize];
        assert!(*i != u32::max_value());
    }

    fn remap_name_section(&self, s: &mut NameSection) {
        match *s {
            NameSection::Module(_) => {}
            NameSection::Function(ref mut f) => {
                let map = f.names_mut();
                let new = IndexMap::with_capacity(map.len());
                for (idx, name) in mem::replace(map, new) {
                    let new_idx = self.functions[idx as usize];
                    let name = if self.config.demangle {
                        rustc_demangle::demangle(&name).to_string()
                    } else {
                        name
                    };
                    if new_idx != u32::max_value() {
                        map.insert(new_idx, name);
                    }
                }
            }
            NameSection::Local(ref mut l) => {
                let map = l.local_names_mut();
                let new = IndexMap::with_capacity(map.len());
                for (idx, value) in mem::replace(map, new) {
                    let new_idx = self.functions[idx as usize];
                    if new_idx != u32::max_value() {
                        map.insert(new_idx, value);
                    }
                }
            }
            NameSection::Unparsed { .. } => {}
        }
    }
}
