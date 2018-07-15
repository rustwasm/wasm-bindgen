use std::{
    collections::{BTreeMap, BTreeSet}, mem,
};

use webidl;

use super::Result;

#[derive(Default)]
pub(crate) struct FirstPassRecord<'a> {
    pub(crate) interfaces: BTreeSet<String>,
    pub(crate) dictionaries: BTreeSet<String>,
    pub(crate) enums: BTreeSet<String>,
    pub(crate) mixins: BTreeMap<String, MixinData<'a>>,
}

#[derive(Default)]
pub(crate) struct MixinData<'a> {
    pub(crate) non_partial: Option<&'a webidl::ast::NonPartialMixin>,
    pub(crate) partials: Vec<&'a webidl::ast::PartialMixin>,
}

pub(crate) trait FirstPass {
    fn first_pass<'a>(&'a self, record: &mut FirstPassRecord<'a>) -> Result<()>;
}

impl FirstPass for [webidl::ast::Definition] {
    fn first_pass<'a>(&'a self, record: &mut FirstPassRecord<'a>) -> Result<()> {
        for def in self {
            def.first_pass(record)?;
        }

        Ok(())
    }
}

impl FirstPass for webidl::ast::Definition {
    fn first_pass<'a>(&'a self, record: &mut FirstPassRecord<'a>) -> Result<()> {
        use webidl::ast::Definition::*;

        match self {
            Dictionary(dictionary) => dictionary.first_pass(record),
            Enum(enum_) => enum_.first_pass(record),
            Interface(interface) => interface.first_pass(record),
            Mixin(mixin) => mixin.first_pass(record),
            _ => {
                // Other definitions aren't currently used in the first pass
                Ok(())
            }
        }
    }
}

impl FirstPass for webidl::ast::Dictionary {
    fn first_pass<'a>(&'a self, record: &mut FirstPassRecord<'a>) -> Result<()> {
        use webidl::ast::Dictionary::*;

        match self {
            NonPartial(dictionary) => dictionary.first_pass(record),
            _ => {
                // Other dictionaries aren't currently used in the first pass
                Ok(())
            }
        }
    }
}

impl FirstPass for webidl::ast::NonPartialDictionary {
    fn first_pass<'a>(&'a self, record: &mut FirstPassRecord<'a>) -> Result<()> {
        if record.dictionaries.insert(self.name.clone()) {
            warn!("Encountered multiple declarations of {}", self.name);
        }

        Ok(())
    }
}

impl FirstPass for webidl::ast::Enum {
    fn first_pass<'a>(&'a self, record: &mut FirstPassRecord<'a>) -> Result<()> {
        if record.enums.insert(self.name.clone()) {
            warn!("Encountered multiple declarations of {}", self.name);
        }

        Ok(())
    }
}

impl FirstPass for webidl::ast::Interface {
    fn first_pass<'a>(&'a self, record: &mut FirstPassRecord<'a>) -> Result<()> {
        use webidl::ast::Interface::*;

        match self {
            NonPartial(interface) => interface.first_pass(record),
            _ => {
                // Other interfaces aren't currently used in the first pass
                Ok(())
            }
        }
    }
}

impl FirstPass for webidl::ast::NonPartialInterface {
    fn first_pass<'a>(&'a self, record: &mut FirstPassRecord<'a>) -> Result<()> {
        if record.interfaces.insert(self.name.clone()) {
            warn!("Encountered multiple declarations of {}", self.name);
        }

        Ok(())
    }
}

impl FirstPass for webidl::ast::Mixin {
    fn first_pass<'a>(&'a self, record: &mut FirstPassRecord<'a>) -> Result<()> {
        use webidl::ast::Mixin::*;

        match self {
            NonPartial(mixin) => mixin.first_pass(record),
            Partial(mixin) => mixin.first_pass(record),
        }
    }
}

impl FirstPass for webidl::ast::NonPartialMixin {
    fn first_pass<'a>(&'a self, record: &mut FirstPassRecord<'a>) -> Result<()> {
        let entry = record
            .mixins
            .entry(self.name.clone())
            .or_insert_with(Default::default);
        if mem::replace(&mut entry.non_partial, Some(self)).is_some() {
            warn!(
                "Encounterd multiple declarations of {}, using last encountered",
                self.name
            );
        }

        Ok(())
    }
}

impl FirstPass for webidl::ast::PartialMixin {
    fn first_pass<'a>(&'a self, record: &mut FirstPassRecord<'a>) -> Result<()> {
        let entry = record
            .mixins
            .entry(self.name.clone())
            .or_insert_with(Default::default);
        entry.partials.push(self);

        Ok(())
    }
}
