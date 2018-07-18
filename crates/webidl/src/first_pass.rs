use std::{
    collections::{BTreeMap, BTreeSet},
    mem,
};

use weedle;

use super::Result;

#[derive(Default)]
pub(crate) struct FirstPassRecord<'a> {
    pub(crate) interfaces: BTreeSet<&'a str>,
    pub(crate) dictionaries: BTreeSet<&'a str>,
    pub(crate) enums: BTreeSet<&'a str>,
    pub(crate) interface_mixins: BTreeMap<&'a str, MixinData<'a>>,
}

#[derive(Default)]
pub(crate) struct MixinData<'a> {
    pub(crate) non_partial: Option<&'a weedle::InterfaceMixinDefinition>,
    pub(crate) partials: Vec<&'a weedle::PartialInterfaceMixinDefinition>,
}

pub(crate) trait FirstPass {
    fn first_pass<'a>(&'a self, record: &mut FirstPassRecord<'a>) -> Result<()>;
}

impl FirstPass for weedle::Definitions {
    fn first_pass<'a>(&'a self, record: &mut FirstPassRecord<'a>) -> Result<()> {
        for def in &self.definitions {
            def.first_pass(record)?;
        }

        Ok(())
    }
}

impl FirstPass for weedle::Definition {
    fn first_pass<'a>(&'a self, record: &mut FirstPassRecord<'a>) -> Result<()> {
        use weedle::Definition::*;

        match self {
            Dictionary(dictionary) => dictionary.first_pass(record),
            Enum(enum_) => enum_.first_pass(record),
            Interface(interface) => interface.first_pass(record),
            InterfaceMixin(interface_mixin) => interface_mixin.first_pass(record),
            PartialInterfaceMixin(partial_interface_mixin) => {
                partial_interface_mixin.first_pass(record)
            }
            _ => {
                // Other definitions aren't currently used in the first pass
                Ok(())
            }
        }
    }
}

impl FirstPass for weedle::DictionaryDefinition {
    fn first_pass<'a>(&'a self, record: &mut FirstPassRecord<'a>) -> Result<()> {
        if record.dictionaries.insert(&self.identifier.name) {
            warn!(
                "Encountered multiple declarations of {}",
                self.identifier.name
            );
        }

        Ok(())
    }
}

impl FirstPass for weedle::EnumDefinition {
    fn first_pass<'a>(&'a self, record: &mut FirstPassRecord<'a>) -> Result<()> {
        if record.enums.insert(&self.identifier.name) {
            warn!(
                "Encountered multiple declarations of {}",
                self.identifier.name
            );
        }

        Ok(())
    }
}

impl FirstPass for weedle::InterfaceDefinition {
    fn first_pass<'a>(&'a self, record: &mut FirstPassRecord<'a>) -> Result<()> {
        if record.interfaces.insert(&self.identifier.name) {
            warn!(
                "Encountered multiple declarations of {}",
                self.identifier.name
            );
        }

        Ok(())
    }
}

impl FirstPass for weedle::InterfaceMixinDefinition {
    fn first_pass<'a>(&'a self, record: &mut FirstPassRecord<'a>) -> Result<()> {
        let entry = record
            .interface_mixins
            .entry(&self.identifier.name)
            .or_insert_with(Default::default);
        if mem::replace(&mut entry.non_partial, Some(self)).is_some() {
            warn!(
                "Encounterd multiple declarations of {}, using last encountered",
                self.identifier.name
            );
        }

        Ok(())
    }
}

impl FirstPass for weedle::PartialInterfaceMixinDefinition {
    fn first_pass<'a>(&'a self, record: &mut FirstPassRecord<'a>) -> Result<()> {
        let entry = record
            .interface_mixins
            .entry(&self.identifier.name)
            .or_insert_with(Default::default);
        entry.partials.push(self);

        Ok(())
    }
}
