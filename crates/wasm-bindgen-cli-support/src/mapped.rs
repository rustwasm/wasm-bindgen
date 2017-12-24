use std::collections::hash_map::{HashMap, Entry};

use parity_wasm::elements::*;
use shared;

pub struct Mapped {
    pub module: Module,
    pub imports: HashMap<String, String>,
    pub exports: HashMap<String, String>,
}

impl Mapped {
    pub fn export_name<'a>(&'a self, name: &'a str) -> &'a str {
        self.exports.get(name).map(|s| &**s).unwrap_or(name)
    }

    pub fn orig_export_name<'a>(&'a self, name: &'a str) -> &'a str {
        self.exports.iter()
            .find(|&(_k, v)| name == v)
            .map(|p| &**p.0)
            .unwrap_or(name)
    }

    pub fn import_name<'a>(&'a self, name: &'a str) -> &'a str {
        self.imports.get(name).map(|s| &**s).unwrap_or(name)
    }

    pub fn orig_import_name<'a>(&'a self, name: &'a str) -> &'a str {
        self.imports.iter()
            .find(|&(_k, v)| name == v)
            .map(|p| &**p.0)
            .unwrap_or(name)
    }

    pub fn uglify(&mut self, program: &shared::Program) {
        let mut i = 0;
        let mut generate = || {
            let ret = generate(i);
            i += 1;
            return ret
        };

        for import in program.imports.iter() {
            self.imports.insert(import.name.clone(), generate());
        }

        for f in program.free_functions.iter() {
            self.exports.insert(f.free_function_export_name(), generate());
        }

        for s in program.structs.iter() {
            for f in s.functions.iter() {
                self.exports.insert(f.struct_function_export_name(&s.name),
                    generate());
            }
            for f in s.methods.iter() {
                self.exports.insert(f.function.struct_function_export_name(&s.name),
                    generate());
            }
        }

        for section in self.module.sections_mut() {
            match *section {
                Section::Import(ref mut section) => {
                    for import in section.entries_mut() {
                        let new_name = match self.imports.entry(import.field().to_string()) {
                            Entry::Occupied(n) => n.into_mut(),
                            Entry::Vacant(v) => {
                                if !import.field().starts_with("__wbindgen") {
                                    continue
                                }
                                v.insert(generate())
                            }
                        };
                        *import = ImportEntry::new(
                            import.module().to_string(),
                            new_name.to_string(),
                            import.external().clone(),
                        );
                    }
                }
                Section::Export(ref mut e) => {
                    for e in e.entries_mut() {
                        let new_name = match self.exports.entry(e.field().to_string()) {
                            Entry::Occupied(n) => n.into_mut(),
                            Entry::Vacant(v) => {
                                if !e.field().starts_with("__wbindgen") {
                                    continue
                                }
                                v.insert(generate())
                            }
                        };
                        *e = ExportEntry::new(new_name.to_string(),
                                              e.internal().clone());
                    }
                }
                _ => {}
            }
        }
    }
}

fn generate(mut i: usize) -> String {
    const LETTERS: &str = "\
        abcdefghijklmnopqrstuvwxyz\
        ABCDEFGHIJKLMNOPQRSTUVWXYZ\
    ";
    let mut ret = String::new();
    loop {
        let j = i % LETTERS.len();
        i /= LETTERS.len();
        ret.push(LETTERS.as_bytes()[j] as char);
        if i == 0 {
            break
        }
    }
    return ret;
}
