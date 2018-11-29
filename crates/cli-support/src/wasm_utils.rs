use std::mem;

use parity_wasm::elements::*;

pub struct Remap<F>(pub F);

impl<F> Remap<F> where F: FnMut(u32) -> u32 {
    pub fn remap_module(&mut self, module: &mut Module) {
        for section in module.sections_mut() {
            match section {
                Section::Export(e) => self.remap_export_section(e),
                Section::Element(e) => self.remap_element_section(e),
                Section::Code(e) => self.remap_code_section(e),
                Section::Start(i) => {
                    self.remap_idx(i);
                }
                Section::Name(n) => self.remap_name_section(n),
                _ => {}
            }
        }
    }

    fn remap_export_section(&mut self, section: &mut ExportSection) {
        for entry in section.entries_mut() {
            self.remap_export_entry(entry);
        }
    }

    fn remap_export_entry(&mut self, entry: &mut ExportEntry) {
        match entry.internal_mut() {
            Internal::Function(i) => {
                self.remap_idx(i);
            }
            _ => {}
        }
    }

    fn remap_element_section(&mut self, section: &mut ElementSection) {
        for entry in section.entries_mut() {
            self.remap_element_entry(entry);
        }
    }

    fn remap_element_entry(&mut self, entry: &mut ElementSegment) {
        for member in entry.members_mut() {
            self.remap_idx(member);
        }
    }

    fn remap_code_section(&mut self, section: &mut CodeSection) {
        for body in section.bodies_mut() {
            self.remap_func_body(body);
        }
    }

    fn remap_func_body(&mut self, body: &mut FuncBody) {
        self.remap_instructions(body.code_mut());
    }

    fn remap_instructions(&mut self, code: &mut Instructions) {
        for instr in code.elements_mut() {
            self.remap_instruction(instr);
        }
    }

    fn remap_instruction(&mut self, instr: &mut Instruction) {
        match instr {
            Instruction::Call(i) => {
                self.remap_idx(i);
            }
            _ => {}
        }
    }

    fn remap_name_section(&mut self, names: &mut NameSection) {
        match names {
            NameSection::Function(f) => self.remap_function_name_section(f),
            NameSection::Local(f) => self.remap_local_name_section(f),
            _ => {}
        }
    }

    fn remap_function_name_section(&mut self, names: &mut FunctionNameSection) {
        let map = names.names_mut();
        let new = IndexMap::with_capacity(map.len());
        for (mut idx, name) in mem::replace(map, new) {
            self.remap_idx(&mut idx);
            map.insert(idx, name);
        }
    }

    fn remap_local_name_section(&mut self, names: &mut LocalNameSection) {
        let map = names.local_names_mut();
        let new = IndexMap::with_capacity(map.len());
        for (mut idx, name) in mem::replace(map, new) {
            self.remap_idx(&mut idx);
            map.insert(idx, name);
        }
    }

    fn remap_idx(&mut self, idx: &mut u32) {
        *idx = (self.0)(*idx);
    }
}
