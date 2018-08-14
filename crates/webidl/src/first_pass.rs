//! Because some WebIDL constructs are defined in multiple places
//! (keyword `partial` is used to add to an existing construct),
//! We need to first walk the webidl to collect all non-partial
//! constructs so that we have containers in which to put the
//! partial ones.
//!
//! Only `interface`s, `dictionary`s, `enum`s and `mixin`s can
//! be partial.

use std::collections::{BTreeMap, BTreeSet};

use weedle::{DictionaryDefinition, PartialDictionaryDefinition};
use weedle::argument::Argument;
use weedle::attribute::ExtendedAttribute;
use weedle::interface::{StringifierOrStatic, Special};
use weedle::mixin::MixinMember;
use weedle::namespace::NamespaceMember;
use weedle;

use super::Result;
use util;
use util::camel_case_ident;

/// Collection of constructs that may use partial.
#[derive(Default)]
pub(crate) struct FirstPassRecord<'src> {
    pub(crate) interfaces: BTreeMap<&'src str, InterfaceData<'src>>,
    pub(crate) enums: BTreeSet<&'src str>,
    /// The mixins, mapping their name to the webidl ast node for the mixin.
    pub(crate) mixins: BTreeMap<&'src str, MixinData<'src>>,
    pub(crate) typedefs: BTreeMap<&'src str, &'src weedle::types::Type<'src>>,
    pub(crate) namespaces: BTreeMap<&'src str, NamespaceData<'src>>,
    pub(crate) includes: BTreeMap<&'src str, BTreeSet<&'src str>>,
    pub(crate) dictionaries: BTreeMap<&'src str, DictionaryData<'src>>,
}

/// We need to collect interface data during the first pass, to be used later.
#[derive(Default)]
pub(crate) struct InterfaceData<'src> {
    /// Whether only partial interfaces were encountered
    pub(crate) partial: bool,
    pub(crate) global: bool,
    pub(crate) operations: BTreeMap<OperationId<'src>, OperationData<'src>>,
    pub(crate) superclass: Option<&'src str>,
}

/// We need to collect mixin data during the first pass, to be used later.
#[derive(Default)]
pub(crate) struct MixinData<'src> {
    /// Whether only partial mixins were encountered
    pub(crate) partial: bool,
    pub(crate) members: Vec<&'src MixinMember<'src>>,
    pub(crate) operations: BTreeMap<OperationId<'src>, OperationData<'src>>,
}

/// We need to collect namespace data during the first pass, to be used later.
#[derive(Default)]
pub(crate) struct NamespaceData<'src> {
    /// Whether only partial namespaces were encountered
    pub(crate) partial: bool,
    pub(crate) members: Vec<&'src NamespaceMember<'src>>,
    pub(crate) operations: BTreeMap<OperationId<'src>, OperationData<'src>>,
}

#[derive(Default)]
pub(crate) struct DictionaryData<'src> {
    /// Whether only partial namespaces were encountered
    pub(crate) partials: Vec<&'src PartialDictionaryDefinition<'src>>,
    pub(crate) definition: Option<&'src DictionaryDefinition<'src>>,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
pub(crate) enum OperationId<'src> {
    Constructor,
    Operation(Option<&'src str>),
    IndexingGetter,
    IndexingSetter,
    IndexingDeleter,
}

#[derive(Default)]
pub(crate) struct OperationData<'src> {
    pub(crate) overloaded: bool,
    /// Map from argument names to whether they are the same for multiple overloads
    pub(crate) argument_names_same: BTreeMap<Vec<&'src str>, bool>,
}

/// Implemented on an AST node to populate the `FirstPassRecord` struct.
pub(crate) trait FirstPass<'src, Ctx> {
    /// Populate `record` with any constructs in `self`.
    fn first_pass(&'src self, record: &mut FirstPassRecord<'src>, ctx: Ctx) -> Result<()>;
}

impl<'src> FirstPass<'src, ()> for [weedle::Definition<'src>] {
    fn first_pass(&'src self, record: &mut FirstPassRecord<'src>, (): ()) -> Result<()> {
        for def in self {
            def.first_pass(record, ())?;
        }

        Ok(())
    }
}

impl<'src> FirstPass<'src, ()> for weedle::Definition<'src> {
    fn first_pass(&'src self, record: &mut FirstPassRecord<'src>, (): ()) -> Result<()> {
        use weedle::Definition::*;

        match self {
            Dictionary(dictionary) => dictionary.first_pass(record, ()),
            PartialDictionary(dictionary) => dictionary.first_pass(record, ()),
            Enum(enum_) => enum_.first_pass(record, ()),
            IncludesStatement(includes) => includes.first_pass(record, ()),
            Interface(interface) => interface.first_pass(record, ()),
            PartialInterface(interface) => interface.first_pass(record, ()),
            InterfaceMixin(mixin) => mixin.first_pass(record, ()),
            PartialInterfaceMixin(mixin) => mixin.first_pass(record, ()),
            Namespace(namespace) => namespace.first_pass(record, ()),
            PartialNamespace(namespace) => namespace.first_pass(record, ()),
            Typedef(typedef) => typedef.first_pass(record, ()),
            _ => {
                // Other definitions aren't currently used in the first pass
                Ok(())
            }
        }
    }
}

impl<'src> FirstPass<'src, ()> for weedle::DictionaryDefinition<'src> {
    fn first_pass(&'src self, record: &mut FirstPassRecord<'src>, (): ()) -> Result<()> {
        record.dictionaries.entry(self.identifier.0)
            .or_default()
            .definition = Some(self);
        Ok(())
    }
}

impl<'src> FirstPass<'src, ()> for weedle::PartialDictionaryDefinition<'src> {
    fn first_pass(&'src self, record: &mut FirstPassRecord<'src>, (): ()) -> Result<()> {
        record.dictionaries.entry(self.identifier.0)
            .or_default()
            .partials
            .push(self);
        Ok(())
    }
}

impl<'src> FirstPass<'src, ()> for weedle::EnumDefinition<'src> {
    fn first_pass(&'src self, record: &mut FirstPassRecord<'src>, (): ()) -> Result<()> {
        if util::is_chrome_only(&self.attributes) {
            return Ok(());
        }

        if !record.enums.insert(self.identifier.0) {
            info!("Encountered multiple enum declarations: {}", self.identifier.0);
        }

        Ok(())
    }
}

impl<'src> FirstPass<'src, ()> for weedle::IncludesStatementDefinition<'src> {
    fn first_pass(&'src self, record: &mut FirstPassRecord<'src>, (): ()) -> Result<()> {
        if util::is_chrome_only(&self.attributes) {
            return Ok(());
        }

        record
            .includes
            .entry(self.lhs_identifier.0)
            .or_default()
            .insert(self.rhs_identifier.0);

        Ok(())
    }
}

#[derive(Clone, Copy)]
enum FirstPassOperationType {
    Interface,
    Mixin,
    Namespace,
}

fn first_pass_operation<'src>(
    record: &mut FirstPassRecord<'src>,
    first_pass_operation_type: FirstPassOperationType,
    self_name: &'src str,
    ids: &[OperationId<'src>],
    arguments: &[Argument<'src>],
) -> Result<()> {
    let mut names = Vec::with_capacity(arguments.len());
    for argument in arguments {
        match argument {
            Argument::Single(single) => names.push(single.identifier.0),
            Argument::Variadic(variadic) => names.push(variadic.identifier.0),
        }
    }
    let operations = match first_pass_operation_type{
        FirstPassOperationType::Interface => {
            &mut record
                .interfaces
                .get_mut(self_name)
                .expect(&format!("not found {} interface", self_name))
                .operations
        },
        FirstPassOperationType::Mixin => {
            &mut record
                .mixins
                .get_mut(self_name)
                .expect(&format!("not found {} mixin", self_name))
                .operations
        },
        FirstPassOperationType::Namespace => {
            &mut record
                .namespaces
                .get_mut(self_name)
                .expect(&format!("not found {} namesace", self_name))
                .operations
        },
    };
    for id in ids {
        operations
            .entry(*id)
            .and_modify(|operation_data| operation_data.overloaded = true)
            .or_default()
            .argument_names_same
            .entry(names.clone())
            .and_modify(|same_argument_names| *same_argument_names = true)
            .or_insert(false);
    }

    Ok(())
}

impl<'src> FirstPass<'src, ()> for weedle::InterfaceDefinition<'src> {
    fn first_pass(&'src self, record: &mut FirstPassRecord<'src>, (): ()) -> Result<()> {
        if util::is_chrome_only(&self.attributes) {
            return Ok(());
        }

        {
            let interface_data = record
                .interfaces
                .entry(self.identifier.0)
                .or_default();
            interface_data.partial = false;
            interface_data.superclass = self.inheritance.map(|s| s.identifier.0);
        }

        if let Some(attrs) = &self.attributes {
            for attr in &attrs.body.list {
                attr.first_pass(record, self.identifier.0)?;
            }
        }

        for member in &self.members.body {
            member.first_pass(record, self.identifier.0)?;
        }

        Ok(())
    }
}

impl<'src> FirstPass<'src, ()> for weedle::PartialInterfaceDefinition<'src> {
    fn first_pass(&'src self, record: &mut FirstPassRecord<'src>, (): ()) -> Result<()> {
        if util::is_chrome_only(&self.attributes) {
            return Ok(());
        }

        record
            .interfaces
            .entry(self.identifier.0)
            .or_insert_with(||
                InterfaceData {
                    partial: true,
                    global: false,
                    operations: Default::default(),
                    superclass: None,
                },
            );

        for member in &self.members.body {
            member.first_pass(record, self.identifier.0)?;
        }

        Ok(())
    }
}

impl<'src> FirstPass<'src, &'src str> for ExtendedAttribute<'src> {
    fn first_pass(&'src self, record: &mut FirstPassRecord<'src>, self_name: &'src str) -> Result<()> {
        match self {
            ExtendedAttribute::ArgList(list) if list.identifier.0 == "Constructor" => {
                first_pass_operation(
                    record,
                    FirstPassOperationType::Interface,
                    self_name,
                    &[OperationId::Constructor],
                    &list.args.body.list,
                )
            }
            ExtendedAttribute::NoArgs(name) if (name.0).0 == "Constructor" => {
                first_pass_operation(
                    record,
                    FirstPassOperationType::Interface,
                    self_name,
                    &[OperationId::Constructor],
                    &[],
                )
            }
            ExtendedAttribute::NamedArgList(list)
                if list.lhs_identifier.0 == "NamedConstructor" =>
            {
                first_pass_operation(
                    record,
                    FirstPassOperationType::Interface,
                    self_name,
                    &[OperationId::Constructor],
                    &list.args.body.list,
                )
            }
            ExtendedAttribute::Ident(id) if id.lhs_identifier.0 == "Global" => {
                record.interfaces.get_mut(self_name).unwrap().global = true;
                Ok(())
            }
            ExtendedAttribute::IdentList(id) if id.identifier.0 == "Global" => {
                record.interfaces.get_mut(self_name).unwrap().global = true;
                Ok(())
            }
            _ => Ok(())
        }
    }
}

impl<'src> FirstPass<'src, &'src str> for weedle::interface::InterfaceMember<'src> {
    fn first_pass(&'src self, record: &mut FirstPassRecord<'src>, self_name: &'src str) -> Result<()> {
        match self {
            weedle::interface::InterfaceMember::Operation(op) => {
                op.first_pass(record, self_name)
            }
            _ => Ok(()),
        }
    }
}

impl<'src> FirstPass<'src, &'src str> for weedle::interface::OperationInterfaceMember<'src> {
    fn first_pass(&'src self, record: &mut FirstPassRecord<'src>, self_name: &'src str) -> Result<()> {
        if util::is_chrome_only(&self.attributes) {
            return Ok(());
        }

        if self.specials.len() > 1 {
            warn!("Unsupported webidl operation: {:?}", self);
            return Ok(())
        }
        if let Some(StringifierOrStatic::Stringifier(_)) = self.modifier {
            warn!("Unsupported webidl stringifier: {:?}", self);
            return Ok(())
        }
        let mut ids = vec![OperationId::Operation(self.identifier.map(|s| s.0))];
        for special in self.specials.iter() {
            ids.push(match special {
                Special::Getter(_) => OperationId::IndexingGetter,
                Special::Setter(_) => OperationId::IndexingSetter,
                Special::Deleter(_) => OperationId::IndexingDeleter,
                Special::LegacyCaller(_) => continue,
            });
        }
        first_pass_operation(
            record,
            FirstPassOperationType::Interface,
            self_name,
            &ids,
            &self.args.body.list,
        )
    }
}

impl<'src> FirstPass<'src, ()> for weedle::InterfaceMixinDefinition<'src>{
    fn first_pass(&'src self, record: &mut FirstPassRecord<'src>, (): ()) -> Result<()> {
        if util::is_chrome_only(&self.attributes) {
            return Ok(());
        }

        {
            let mixin_data = record
                .mixins
                .entry(self.identifier.0)
                .or_default();
            mixin_data.partial = false;
            mixin_data.members.extend(&self.members.body);
        }

        for member in &self.members.body {
            member.first_pass(record, self.identifier.0)?;
        }

        Ok(())
    }
}

impl<'src> FirstPass<'src, ()> for weedle::PartialInterfaceMixinDefinition<'src> {
    fn first_pass(&'src self, record: &mut FirstPassRecord<'src>, (): ()) -> Result<()> {
        if util::is_chrome_only(&self.attributes) {
            return Ok(());
        }

        record
            .mixins
            .entry(self.identifier.0)
            .or_insert_with(||
                MixinData {
                    partial: true,
                    members: Default::default(),
                    operations: Default::default(),
                },
            )
            .members
            .extend(&self.members.body);

        for member in &self.members.body {
            member.first_pass(record, self.identifier.0)?;
        }

        Ok(())
    }
}

impl<'src> FirstPass<'src, &'src str> for weedle::mixin::MixinMember<'src> {
    fn first_pass(&'src self, record: &mut FirstPassRecord<'src>, self_name: &'src str) -> Result<()> {
        match self {
            weedle::mixin::MixinMember::Operation(op) => {
                op.first_pass(record, self_name)
            }
            _ => Ok(()),
        }
    }
}

impl<'src> FirstPass<'src, &'src str> for weedle::mixin::OperationMixinMember<'src> {
    fn first_pass(&'src self, record: &mut FirstPassRecord<'src>, self_name: &'src str) -> Result<()> {
        if util::is_chrome_only(&self.attributes) {
            return Ok(());
        }

        if self.stringifier.is_some() {
            warn!("Unsupported webidl stringifier: {:?}", self);
            return Ok(())
        }

        first_pass_operation(
            record,
            FirstPassOperationType::Mixin,
            self_name,
            &[OperationId::Operation(self.identifier.map(|s| s.0.clone()))],
            &self.args.body.list,
        )
    }
}

impl<'src> FirstPass<'src, ()> for weedle::TypedefDefinition<'src> {
    fn first_pass(&'src self, record: &mut FirstPassRecord<'src>, (): ()) -> Result<()> {
        if util::is_chrome_only(&self.attributes) {
            return Ok(());
        }

        if record.typedefs.insert(self.identifier.0, &self.type_.type_).is_some() {
            info!("Encountered multiple typedef declarations: {}", self.identifier.0);
        }

        Ok(())
    }
}

impl<'src> FirstPass<'src, ()> for weedle::NamespaceDefinition<'src> {
    fn first_pass(&'src self, record: &mut FirstPassRecord<'src>, (): ()) -> Result<()> {
        if util::is_chrome_only(&self.attributes) {
            return Ok(())
        }

        record
            .namespaces
            .entry(self.identifier.0)
            .and_modify(|namespace_data| namespace_data.partial = false)
            .or_insert_with(||
                NamespaceData {
                    partial: true,
                    members: Default::default(),
                    operations: Default::default(),
                },
            )
            .members
            .extend(&self.members.body);

        for member in &self.members.body {
            member.first_pass(record, self.identifier.0)?;
        }

        Ok(())
    }
}

impl<'src> FirstPass<'src, ()> for weedle::PartialNamespaceDefinition<'src> {
    fn first_pass(&'src self, record: &mut FirstPassRecord<'src>, (): ()) -> Result<()> {
        if util::is_chrome_only(&self.attributes) {
            return Ok(())
        }

        record
            .namespaces
            .entry(self.identifier.0)
            .or_default()
            .members
            .extend(&self.members.body);

        for member in &self.members.body {
            member.first_pass(record, self.identifier.0)?;
        }

        Ok(())
    }
}

impl<'src> FirstPass<'src, &'src str> for weedle::namespace::NamespaceMember<'src> {
    fn first_pass(&'src self, record: &mut FirstPassRecord<'src>, self_name: &'src str) -> Result<()> {
        match self {
            weedle::namespace::NamespaceMember::Operation(op) => {
                op.first_pass(record, self_name)
            }
            _ => Ok(()),
        }
    }
}

impl<'src> FirstPass<'src, &'src str> for weedle::namespace::OperationNamespaceMember<'src> {
    fn first_pass(&'src self, record: &mut FirstPassRecord<'src>, self_name: &'src str) -> Result<()> {
        if util::is_chrome_only(&self.attributes) {
            return Ok(())
        }

        first_pass_operation(
            record,
            FirstPassOperationType::Namespace,
            self_name,
            &[OperationId::Operation(self.identifier.map(|s| s.0.clone()))],
            &self.args.body.list,
        )
    }
}

impl<'a> FirstPassRecord<'a> {
    pub fn all_superclasses<'me>(&'me self, interface: &str)
        -> impl Iterator<Item = String> + 'me
    {
        let mut set = BTreeSet::new();
        self.fill_superclasses(interface, &mut set);
        set.into_iter()
    }

    fn fill_superclasses(&self, interface: &str, set: &mut BTreeSet<String>) {
        let data = match self.interfaces.get(interface) {
            Some(data) => data,
            None => return,
        };
        let superclass = match &data.superclass {
            Some(class) => class,
            None => return,
        };
        if set.insert(camel_case_ident(superclass)) {
            self.fill_superclasses(superclass, set);
        }
    }
}
