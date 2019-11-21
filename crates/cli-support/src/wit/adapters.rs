//! Location where `Adapter` functions are actually created.
//!
//! This module is tasked with converting `Descriptor::Function` instances to
//! `Adapter` functions. It uses the incoming/outgoing modules/builders to do
//! most of the heavy lifting, and then this is the glue around the edges to
//! make sure everything is processed, hooked up in the second, and then
//! inserted into the right map.
//!
//! This module is called from `src/wit/mod.rs` exclusively to populate the
//! imports/exports/elements of the bindings section. Most of this module is
//! largely just connecting the dots!

use crate::descriptor::Function;
use crate::wit::incoming::IncomingBuilder;
use crate::wit::outgoing::OutgoingBuilder;
use crate::wit::{Adapter, AdapterId, AdapterJsImportKind, NonstandardWitSection};
use crate::wit::{AdapterKind, AdapterType, Instruction};
use anyhow::{format_err, Error};
use walrus::{FunctionId, Module, ValType};

/// Adds an element to the `bindings.imports` map for the `import` specified
/// that is supposed to have the signature specified in `binding`. This also
/// expects that the imported item is called as `kind`.
pub fn import(
    module: &mut Module,
    adapters: &mut NonstandardWitSection,
    import: walrus::ImportId,
    signature: Function,
    kind: AdapterJsImportKind,
) -> Result<AdapterId, Error> {
    let import = module.imports.get(import);
    let (import_module, import_name) = (import.module.clone(), import.name.clone());
    let id = match import.kind {
        walrus::ImportKind::Function(f) => f,
        _ => unreachable!(),
    };
    let import_id = import.id();

    // Process the returned type first to see if it needs an out-pointer. This
    // happens if the results of the incoming arguments translated to wasm take
    // up more than one type.
    let mut incoming = IncomingBuilder::default();
    incoming.process(&signature.ret)?;
    let uses_retptr = incoming.output.len() > 1;

    // Process the argument next, allocating space of the return value if one
    // was present. Additionally configure the `module` and `adapters` to allow
    // usage of closures going out to the import.
    let mut outgoing = OutgoingBuilder::default();
    outgoing.module = Some(module);
    outgoing.adapters = Some(adapters);
    if uses_retptr {
        outgoing.input.push(AdapterType::I32);
    }
    for arg in signature.arguments.iter() {
        outgoing.process(arg)?;
    }

    // A bit of destructuring to kill the borrow that the outgoing builder has
    // on the module/bindings.
    let OutgoingBuilder {
        input: outgoing_input,
        output: outgoing_output,
        instructions: outgoing_instructions,
        ..
    } = outgoing;

    // Build up the list of instructions for our adapter function. We start out
    // with all the outgoing instructions which convert all wasm params to the
    // desired types to call our import...
    let mut instructions = outgoing_instructions;

    // ... and then we actually call our import. We synthesize an adapter
    // definition for it with the appropriate types here on the fly.
    let f = adapters.append(
        outgoing_output,
        incoming.input,
        AdapterKind::Import {
            module: import_module,
            name: import_name,
            kind,
        },
    );
    instructions.push(Instruction::CallAdapter(f));

    // ... and then we follow up with a conversion of the incoming type back to
    // wasm.
    instructions.extend(incoming.instructions);

    // ... and if a return pointer is in use then we need to store the types on
    // the stack into the wasm return pointer. Note that we iterate in reverse
    // here because the last result is the top value on the stack.
    let results = if uses_retptr {
        for (i, ty) in incoming.output.into_iter().enumerate().rev() {
            instructions.push(Instruction::StoreRetptr { offset: i, ty });
        }
        Vec::new()
    } else {
        incoming.output
    };
    let id = adapters.append(outgoing_input, results, AdapterKind::Local { instructions });
    adapters.implements.push((import_id, id));
    Ok(id)
}

/// Adds an element to `bindings.exports` for the `export` specified to have the
/// `binding` given.
pub fn export(
    module: &mut Module,
    adapters: &mut NonstandardWitSection,
    export: walrus::ExportId,
    signature: Function,
) -> Result<AdapterId, Error> {
    let export = module.exports.get(export);
    let name = export.name.clone();
    let id = match export.item {
        walrus::ExportItem::Function(f) => f,
        _ => unreachable!(),
    };
    let export_id = export.id();
    // Do the actual heavy lifting elsewhere to generate the `binding`.
    let id = register_wasm_export(module, adapters, id, signature)?;
    adapters.exports.push((name, id));
    Ok(id)
}

/// Like `export` except registers an adapter for a table element. In
/// this case ensures that the table element `idx` is specified to have the
/// `signature` specified.
pub fn table_element(
    module: &mut Module,
    adapters: &mut NonstandardWitSection,
    idx: u32,
    signature: Function,
) -> Result<AdapterId, Error> {
    let table = module
        .tables
        .main_function_table()?
        .ok_or_else(|| format_err!("no function table found"))?;
    let table = module.tables.get(table);
    let functions = match &table.kind {
        walrus::TableKind::Function(f) => f,
        _ => unreachable!(),
    };
    let id = functions.elements[idx as usize].unwrap();
    // like above, largely just defer the work elsewhere
    Ok(register_wasm_export(module, adapters, id, signature)?)
}

fn register_wasm_export(
    module: &mut Module,
    adapters: &mut NonstandardWitSection,
    id: walrus::FunctionId,
    signature: Function,
) -> Result<AdapterId, Error> {
    // Figure out how to translate all the incoming arguments ...
    let mut incoming = IncomingBuilder::default();
    for arg in signature.arguments.iter() {
        incoming.process(arg)?;
    }

    // ... then the returned value being translated back
    let mut outgoing = OutgoingBuilder::default();
    outgoing.process(&signature.ret)?;
    let uses_retptr = outgoing.input.len() > 1;

    // Our instruction stream starts out with the return pointer as the first
    // argument to the wasm function, if one is in use. Then we convert
    // everything to wasm types.
    //
    // After calling the core wasm function we need to load all the return
    // pointer arguments if there were any, otherwise we simply convert
    // everything into the outgoing arguments.
    let mut instructions = Vec::new();
    if uses_retptr {
        instructions.push(Instruction::Retptr);
    }
    instructions.extend(incoming.instructions);
    instructions.push(Instruction::Standard(wit_walrus::Instruction::CallCore(id)));
    if uses_retptr {
        for (i, ty) in incoming.output.into_iter().enumerate() {
            instructions.push(Instruction::LoadRetptr { offset: i, ty });
        }
    }
    instructions.extend(outgoing.instructions);

    Ok(adapters.append(
        incoming.input,
        outgoing.output,
        AdapterKind::Local { instructions },
    ))
}
