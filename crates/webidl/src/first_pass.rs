//! Because some WebIDL constructs are defined in multiple places
//! (keyword `partial` is used to add to an existing construct),
//! We need to first walk the webidl to collect all non-partial
//! constructs so that we have containers in which to put the
//! partial ones.
//!
//! Only `interface`s, `dictionary`s, `enum`s and `mixin`s can
//! be partial.

use std::{
    collections::{BTreeMap, BTreeSet}, mem,
};

use webidl;

use super::Result;

/// Collection of constructs that may use partial.
#[derive(Default)]
pub(crate) struct FirstPassRecord<'a> {
    pub(crate) interfaces: BTreeSet<String>,
    pub(crate) dictionaries: BTreeSet<String>,
    pub(crate) enums: BTreeSet<String>,
    /// The mixins, mapping their name to the webidl ast node for the mixin.
    pub(crate) mixins: BTreeMap<String, MixinData<'a>>,
}

/// We need to collect mixin data during the first pass, to be used later.
#[derive(Default)]
pub(crate) struct MixinData<'a> {
    /// The non partial mixin, if present. If there is more than one, we are
    /// parsing is a malformed WebIDL file, but the parser will recover by
    /// using the last parsed mixin.
    pub(crate) non_partial: Option<&'a webidl::ast::NonPartialMixin>,
    /// 0 or more partial mixins.
    pub(crate) partials: Vec<&'a webidl::ast::PartialMixin>,
}

/// Implemented on an AST node to populate the `FirstPassRecord` struct.
pub(crate) trait FirstPass {
    /// Populate `record` with any constructs in `self`.
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
