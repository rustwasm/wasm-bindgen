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
    pub(crate) interfaces: BTreeMap<String, InterfaceData>,
    pub(crate) dictionaries: BTreeSet<String>,
    pub(crate) enums: BTreeSet<String>,
    /// The mixins, mapping their name to the webidl ast node for the mixin.
    pub(crate) mixins: BTreeMap<String, MixinData<'a>>,
}

/// We need to collect interface data during the first pass, to be used later.
#[derive(Default)]
pub(crate) struct InterfaceData {
    /// Whether only partial interfaces were encountered
    pub(crate) partial: bool,
    pub(crate) operations: BTreeSet<Option<String>>,
    pub(crate) overloaded_operations: BTreeSet<Option<String>>,
    pub(crate) has_constructor: bool,
    pub(crate) has_overloaded_constructors: bool,
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
pub(crate) trait FirstPass<Ctx> {
    /// Populate `record` with any constructs in `self`.
    fn first_pass<'a>(&'a self, record: &mut FirstPassRecord<'a>, ctx: Ctx) -> Result<()>;
}

impl FirstPass<()> for [webidl::ast::Definition] {
    fn first_pass<'a>(&'a self, record: &mut FirstPassRecord<'a>, (): ()) -> Result<()> {
        for def in self {
            def.first_pass(record, ())?;
        }

        Ok(())
    }
}

impl FirstPass<()> for webidl::ast::Definition {
    fn first_pass<'a>(&'a self, record: &mut FirstPassRecord<'a>, (): ()) -> Result<()> {
        use webidl::ast::Definition::*;

        match self {
            Dictionary(dictionary) => dictionary.first_pass(record, ()),
            Enum(enum_) => enum_.first_pass(record, ()),
            Interface(interface) => interface.first_pass(record, ()),
            Mixin(mixin) => mixin.first_pass(record, ()),
            _ => {
                // Other definitions aren't currently used in the first pass
                Ok(())
            }
        }
    }
}

impl FirstPass<()> for webidl::ast::Dictionary {
    fn first_pass<'a>(&'a self, record: &mut FirstPassRecord<'a>, (): ()) -> Result<()> {
        use webidl::ast::Dictionary::*;

        match self {
            NonPartial(dictionary) => dictionary.first_pass(record, ()),
            _ => {
                // Other dictionaries aren't currently used in the first pass
                Ok(())
            }
        }
    }
}

impl FirstPass<()> for webidl::ast::NonPartialDictionary {
    fn first_pass<'a>(&'a self, record: &mut FirstPassRecord<'a>, (): ()) -> Result<()> {
        if record.dictionaries.insert(self.name.clone()) {
            warn!("Encountered multiple declarations of {}", self.name);
        }

        Ok(())
    }
}

impl FirstPass<()> for webidl::ast::Enum {
    fn first_pass<'a>(&'a self, record: &mut FirstPassRecord<'a>, (): ()) -> Result<()> {
        if record.enums.insert(self.name.clone()) {
            warn!("Encountered multiple declarations of {}", self.name);
        }

        Ok(())
    }
}

impl FirstPass<()> for webidl::ast::Interface {
    fn first_pass<'a>(&'a self, record: &mut FirstPassRecord<'a>, (): ()) -> Result<()> {
        use webidl::ast::Interface::*;

        match self {
            Partial(interface) => interface.first_pass(record, ()),
            NonPartial(interface) => interface.first_pass(record, ()),
            // TODO
            Callback(..) => {
                warn!("Unsupported WebIDL interface: {:?}", self);
                Ok(())
            }
        }
    }
}

impl FirstPass<()> for webidl::ast::NonPartialInterface {
    fn first_pass<'a>(&'a self, record: &mut FirstPassRecord<'a>, (): ()) -> Result<()> {
        record
            .interfaces
            .entry(self.name.clone())
            .and_modify(|interface_data| {
                if interface_data.partial {
                    interface_data.partial = false;
                } else {
                    warn!("Encountered multiple declarations of {}", self.name);
                }
            })
            .or_insert_with(||
                InterfaceData {
                    partial: false,
                    operations: Default::default(),
                    overloaded_operations: Default::default(),
                    has_constructor: false,
                    has_overloaded_constructors: false,
                },
            );

        if ::util::is_chrome_only(&self.extended_attributes) {
            return Ok(())
        }

        for extended_attribute in &self.extended_attributes {
            extended_attribute.first_pass(record, &self.name)?;
        }

        for member in &self.members {
            member.first_pass(record, &self.name)?;
        }

        Ok(())
    }
}

impl FirstPass<()> for webidl::ast::PartialInterface {
    fn first_pass<'a>(&'a self, record: &mut FirstPassRecord<'a>, (): ()) -> Result<()> {
        record
            .interfaces
            .entry(self.name.clone())
            .or_insert_with(||
                InterfaceData {
                    partial: true,
                    operations: Default::default(),
                    overloaded_operations: Default::default(),
                    has_constructor: false,
                    has_overloaded_constructors: false,
                },
            );

        if ::util::is_chrome_only(&self.extended_attributes) {
            return Ok(())
        }

        for member in &self.members {
            member.first_pass(record, &self.name)?;
        }

        Ok(())
    }
}

impl<'b> FirstPass<&'b str> for webidl::ast::ExtendedAttribute {
    fn first_pass<'a>(&'a self, record: &mut FirstPassRecord<'a>, self_name: &'b str) -> Result<()> {
        let interface_data = record.interfaces.get_mut(self_name).unwrap();
        if match self {
            webidl::ast::ExtendedAttribute::ArgumentList(
                webidl::ast::ArgumentListExtendedAttribute { name, .. }
            )
            if name == "Constructor" => true,
            webidl::ast::ExtendedAttribute::NoArguments(webidl::ast::Other::Identifier(name))
            if name == "Constructor" => true,
            webidl::ast::ExtendedAttribute::NamedArgumentList(
                webidl::ast::NamedArgumentListExtendedAttribute { lhs_name, .. },
            )
            if lhs_name == "NamedConstructor" => true,
            _ => false,
        } {
            if interface_data.has_constructor {
                interface_data.has_overloaded_constructors = true;
            } else {
                interface_data.has_constructor = true;
            }
        }

        Ok(())
    }
}

impl<'b> FirstPass<&'b str> for webidl::ast::InterfaceMember {
    fn first_pass<'a>(&'a self, record: &mut FirstPassRecord<'a>, self_name: &'b str) -> Result<()> {
        match self {
            webidl::ast::InterfaceMember::Operation(op) => op.first_pass(record, self_name),
            _ => Ok(()),
        }
    }
}

impl<'b> FirstPass<&'b str> for webidl::ast::Operation {
    fn first_pass<'a>(&'a self, record: &mut FirstPassRecord<'a>, self_name: &'b str) -> Result<()> {
        match self {
            webidl::ast::Operation::Regular(op) => op.first_pass(record, self_name),
            webidl::ast::Operation::Static(op) => op.first_pass(record, self_name),
            // TODO
            webidl::ast::Operation::Special(_) | webidl::ast::Operation::Stringifier(_) => {
                warn!("Unsupported WebIDL operation: {:?}", self);
                Ok(())
            }
        }
    }
}

impl<'b> FirstPass<&'b str> for webidl::ast::RegularOperation {
    fn first_pass<'a>(&'a self, record: &mut FirstPassRecord<'a>, self_name: &'b str) -> Result<()> {
        let interface_data = record.interfaces.get_mut(self_name).unwrap();
        if interface_data.operations.contains(&self.name) {
            interface_data.overloaded_operations.insert(self.name.clone());
        } else {
            interface_data.operations.insert(self.name.clone());
        }

        Ok(())
    }
}

impl<'b> FirstPass<&'b str> for webidl::ast::StaticOperation {
    fn first_pass<'a>(&'a self, record: &mut FirstPassRecord<'a>, self_name: &'b str) -> Result<()> {
        let interface_data = record.interfaces.get_mut(self_name).unwrap();
        if interface_data.operations.contains(&self.name) {
            interface_data.overloaded_operations.insert(self.name.clone());
        } else {
            interface_data.operations.insert(self.name.clone());
        }

        Ok(())
    }
}

impl FirstPass<()> for webidl::ast::Mixin {
    fn first_pass<'a>(&'a self, record: &mut FirstPassRecord<'a>, (): ()) -> Result<()> {
        use webidl::ast::Mixin::*;

        match self {
            NonPartial(mixin) => mixin.first_pass(record, ()),
            Partial(mixin) => mixin.first_pass(record, ()),
        }
    }
}

impl FirstPass<()> for webidl::ast::NonPartialMixin {
    fn first_pass<'a>(&'a self, record: &mut FirstPassRecord<'a>, (): ()) -> Result<()> {
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

impl FirstPass<()> for webidl::ast::PartialMixin {
    fn first_pass<'a>(&'a self, record: &mut FirstPassRecord<'a>, (): ()) -> Result<()> {
        let entry = record
            .mixins
            .entry(self.name.clone())
            .or_insert_with(Default::default);
        entry.partials.push(self);

        Ok(())
    }
}
