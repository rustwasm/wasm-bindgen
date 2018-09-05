//! Because some WebIDL constructs are defined in multiple places
//! (keyword `partial` is used to add to an existing construct),
//! We need to first walk the webidl to collect all non-partial
//! constructs so that we have containers in which to put the
//! partial ones.
//!
//! Only `interface`s, `dictionary`s, `enum`s and `mixin`s can
//! be partial.

use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet};

use proc_macro2::Ident;
use weedle::{DictionaryDefinition, PartialDictionaryDefinition};
use weedle::argument::Argument;
use weedle::attribute::*;
use weedle::interface::*;
use weedle::mixin::*;
use weedle;

use super::Result;
use util::camel_case_ident;
use util;

/// Collection of constructs that may use partial.
#[derive(Default)]
pub(crate) struct FirstPassRecord<'src> {
    pub(crate) builtin_idents: BTreeSet<Ident>,
    pub(crate) interfaces: BTreeMap<&'src str, InterfaceData<'src>>,
    pub(crate) enums: BTreeMap<&'src str, &'src weedle::EnumDefinition<'src>>,
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
    pub(crate) attributes: Vec<&'src AttributeInterfaceMember<'src>>,
    pub(crate) consts: Vec<&'src ConstMember<'src>>,
    pub(crate) operations: BTreeMap<OperationId<'src>, OperationData<'src>>,
    pub(crate) superclass: Option<&'src str>,
    pub(crate) definition_attributes: Option<&'src ExtendedAttributeList<'src>>,
}

/// We need to collect mixin data during the first pass, to be used later.
#[derive(Default)]
pub(crate) struct MixinData<'src> {
    /// Whether only partial mixins were encountered
    pub(crate) partial: bool,
    pub(crate) attributes: Vec<&'src AttributeMixinMember<'src>>,
    pub(crate) consts: Vec<&'src ConstMember<'src>>,
    pub(crate) operations: BTreeMap<OperationId<'src>, OperationData<'src>>,
    pub(crate) definition_attributes: Option<&'src ExtendedAttributeList<'src>>,
}

/// We need to collect namespace data during the first pass, to be used later.
#[derive(Default)]
pub(crate) struct NamespaceData<'src> {
    /// Whether only partial namespaces were encountered
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
    Constructor(IgnoreTraits<&'src str>),
    Operation(Option<&'src str>),
    IndexingGetter,
    IndexingSetter,
    IndexingDeleter,
}

#[derive(Default)]
pub(crate) struct OperationData<'src> {
    pub(crate) signatures: Vec<Signature<'src>>,
    pub(crate) is_static: bool,
}

#[derive(Clone)]
pub(crate) struct Signature<'src> {
    pub(crate) args: Vec<Arg<'src>>,
    pub(crate) ret: weedle::types::ReturnType<'src>,
    pub(crate) attrs: &'src Option<ExtendedAttributeList<'src>>,
}

#[derive(Clone)]
pub(crate) struct Arg<'src> {
    pub(crate) name: &'src str,
    pub(crate) ty: &'src weedle::types::Type<'src>,
    pub(crate) optional: bool,
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

            Implements(_) => Ok(()),
            Callback(..) => {
                warn!("Unsupported WebIDL Callback definition: {:?}", self);
                Ok(())
            }
            CallbackInterface(..) => {
                warn!("Unsupported WebIDL CallbackInterface definition: {:?}", self);
                Ok(())
            }
        }
    }
}

impl<'src> FirstPass<'src, ()> for weedle::DictionaryDefinition<'src> {
    fn first_pass(&'src self, record: &mut FirstPassRecord<'src>, (): ()) -> Result<()> {
        if util::is_chrome_only(&self.attributes) {
            return Ok(());
        }

        record.dictionaries.entry(self.identifier.0)
            .or_default()
            .definition = Some(self);
        Ok(())
    }
}

impl<'src> FirstPass<'src, ()> for weedle::PartialDictionaryDefinition<'src> {
    fn first_pass(&'src self, record: &mut FirstPassRecord<'src>, (): ()) -> Result<()> {
        if util::is_chrome_only(&self.attributes) {
            return Ok(());
        }

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

        if record.enums.insert(self.identifier.0, self).is_some() {
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
    arguments: &'src [Argument<'src>],
    ret: &weedle::types::ReturnType<'src>,
    attrs: &'src Option<ExtendedAttributeList<'src>>,
    is_static: bool,
) {
    if util::is_chrome_only(attrs) {
        return
    }

    let mut names = Vec::with_capacity(arguments.len());
    for argument in arguments {
        match argument {
            Argument::Single(single) => names.push(single.identifier.0),
            Argument::Variadic(variadic) => names.push(variadic.identifier.0),
        }
    }
    let operations = match first_pass_operation_type {
        FirstPassOperationType::Interface => {
            let x = record
                .interfaces
                .get_mut(self_name)
                .expect(&format!("not found {} interface", self_name));
            &mut x.operations
        },
        FirstPassOperationType::Mixin => {
            let x = record
                .mixins
                .get_mut(self_name)
                .expect(&format!("not found {} mixin", self_name));
            &mut x.operations
        },
        FirstPassOperationType::Namespace => {
            let x = record
                .namespaces
                .get_mut(self_name)
                .expect(&format!("not found {} namespace", self_name));
            &mut x.operations
        },
    };
    let mut args = Vec::with_capacity(arguments.len());
    for argument in arguments {
        let arg = match argument {
            Argument::Single(single) => single,
            Argument::Variadic(v) => {
                warn!("Unsupported variadic argument {} in {}",
                      v.identifier.0,
                      self_name);
                return
            }
        };
        args.push(Arg {
            name: arg.identifier.0,
            ty: &arg.type_.type_,
            optional: arg.optional.is_some(),
        });
    }
    for id in ids {
        let op = operations.entry(*id).or_default();
        op.is_static = is_static;
        op.signatures.push(Signature {
            args: args.clone(),
            ret: ret.clone(),
            attrs,
        });
    }
}

impl<'src> FirstPass<'src, ()> for weedle::InterfaceDefinition<'src> {
    fn first_pass(&'src self, record: &mut FirstPassRecord<'src>, (): ()) -> Result<()> {
        if util::is_chrome_only(&self.attributes) {
            return Ok(());
        }

        if util::is_no_interface_object(&self.attributes) {
            info!("Skipping because of `NoInterfaceObject` attribute: {:?}", self.identifier.0);
            return Ok(());
        }

        {
            let interface_data = record
                .interfaces
                .entry(self.identifier.0)
                .or_default();
            interface_data.partial = false;
            interface_data.superclass = self.inheritance.map(|s| s.identifier.0);
            interface_data.definition_attributes = self.attributes.as_ref();
        }
        if let Some(attrs) = &self.attributes {
            for attr in attrs.body.list.iter() {
                process_interface_attribute(
                    record,
                    self.identifier.0,
                    attr,
                );
            }
        }

        for member in &self.members.body {
            member.first_pass(record, self.identifier.0)?;
        }

        Ok(())
    }
}

fn process_interface_attribute<'src>(
    record: &mut FirstPassRecord<'src>,
    self_name: &'src str,
    attr: &'src ExtendedAttribute<'src>
) {
    let ident = weedle::common::Identifier(self_name);
    let non_null = weedle::types::MayBeNull { type_: ident, q_mark: None };
    let non_any = weedle::types::NonAnyType::Identifier(non_null);
    let single = weedle::types::SingleType::NonAny(non_any);
    let ty = weedle::types::Type::Single(single);
    let return_ty = weedle::types::ReturnType::Type(ty);
    match attr {
        ExtendedAttribute::ArgList(list)
            if list.identifier.0 == "Constructor" =>
        {
            first_pass_operation(
                record,
                FirstPassOperationType::Interface,
                self_name,
                &[OperationId::Constructor(IgnoreTraits(self_name))],
                &list.args.body.list,
                &return_ty,
                &None,
                false,
            );
        }
        ExtendedAttribute::NoArgs(other) if (other.0).0 == "Constructor" => {
            first_pass_operation(
                record,
                FirstPassOperationType::Interface,
                self_name,
                &[OperationId::Constructor(IgnoreTraits(self_name))],
                &[],
                &return_ty,
                &None,
                false,
            );
        }
        ExtendedAttribute::NamedArgList(list)
            if list.lhs_identifier.0 == "NamedConstructor" =>
        {
            first_pass_operation(
                record,
                FirstPassOperationType::Interface,
                self_name,
                &[OperationId::Constructor(IgnoreTraits(list.rhs_identifier.0))],
                &list.args.body.list,
                &return_ty,
                &None,
                false,
            );
        }
        ExtendedAttribute::Ident(id) if id.lhs_identifier.0 == "Global" => {
            record.interfaces.get_mut(self_name).unwrap().global = true;
        }
        ExtendedAttribute::IdentList(id) if id.identifier.0 == "Global" => {
            record.interfaces.get_mut(self_name).unwrap().global = true;
        }
        _ => {}
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
                    ..Default::default()
                },
            );
        for member in &self.members.body {
            member.first_pass(record, self.identifier.0)?;
        }
        Ok(())
    }
}

impl<'src> FirstPass<'src, &'src str> for weedle::interface::InterfaceMember<'src> {
    fn first_pass(&'src self, record: &mut FirstPassRecord<'src>, self_name: &'src str) -> Result<()> {
        match self {
            InterfaceMember::Attribute(attr) => {
                attr.first_pass(record, self_name)
            }
            InterfaceMember::Operation(op) => {
                op.first_pass(record, self_name)
            }
            InterfaceMember::Const(const_) => {
                if util::is_chrome_only(&const_.attributes) {
                    return Ok(());
                }
                record.interfaces
                    .get_mut(self_name)
                    .unwrap()
                    .consts
                    .push(const_);
                Ok(())
            }
            InterfaceMember::Iterable(_iterable) => {
                warn!("Unsupported WebIDL iterable interface member: {:?}", self);
                Ok(())
            }
            // TODO
            InterfaceMember::Maplike(_) => {
                warn!("Unsupported WebIDL Maplike interface member: {:?}", self);
                Ok(())
            }
            InterfaceMember::Stringifier(_) => {
                warn!("Unsupported WebIDL Stringifier interface member: {:?}", self);
                Ok(())
            }
            InterfaceMember::Setlike(_) => {
                warn!("Unsupported WebIDL Setlike interface member: {:?}", self);
                Ok(())
            }
        }
    }
}

impl<'src> FirstPass<'src, &'src str> for weedle::interface::OperationInterfaceMember<'src> {
    fn first_pass(&'src self, record: &mut FirstPassRecord<'src>, self_name: &'src str) -> Result<()> {
        if self.specials.len() > 1 {
            warn!("Unsupported webidl operation: {:?}", self);
            return Ok(())
        }
        let is_static = match self.modifier {
            Some(StringifierOrStatic::Stringifier(_)) => {
                warn!("Unsupported webidl stringifier: {:?}", self);
                return Ok(())
            }
            Some(StringifierOrStatic::Static(_)) => true,
            None => false,
        };

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
            &self.return_type,
            &self.attributes,
            is_static,
        );
        Ok(())
    }
}

impl<'src> FirstPass<'src, &'src str> for weedle::interface::AttributeInterfaceMember<'src> {
    fn first_pass(&'src self, record: &mut FirstPassRecord<'src>, self_name: &'src str) -> Result<()> {
        if util::is_chrome_only(&self.attributes) {
            return Ok(());
        }

        record.interfaces
            .get_mut(self_name)
            .unwrap()
            .attributes
            .push(self);
        Ok(())
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
            mixin_data.definition_attributes = self.attributes.as_ref();
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
                    ..Default::default()
                },
            );

        for member in &self.members.body {
            member.first_pass(record, self.identifier.0)?;
        }

        Ok(())
    }
}

impl<'src> FirstPass<'src, &'src str> for weedle::mixin::MixinMember<'src> {
    fn first_pass(&'src self, record: &mut FirstPassRecord<'src>, self_name: &'src str) -> Result<()> {
        match self {
            MixinMember::Operation(op) => op.first_pass(record, self_name),
            MixinMember::Attribute(a) => a.first_pass(record, self_name),
            MixinMember::Const(a) => {
                if util::is_chrome_only(&a.attributes) {
                    return Ok(());
                }
                record.mixins
                    .get_mut(self_name)
                    .unwrap()
                    .consts
                    .push(a);
                Ok(())
            }
            MixinMember::Stringifier(_) => {
                warn!("Unsupported WebIDL stringifier mixin member: {:?}", self);
                Ok(())
            }
        }
    }
}

impl<'src> FirstPass<'src, &'src str> for weedle::mixin::OperationMixinMember<'src> {
    fn first_pass(&'src self, record: &mut FirstPassRecord<'src>, self_name: &'src str) -> Result<()> {
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
            &self.return_type,
            &self.attributes,
            false,
        );
        Ok(())
    }
}

impl<'src> FirstPass<'src, &'src str> for weedle::mixin::AttributeMixinMember<'src> {
    fn first_pass(&'src self, record: &mut FirstPassRecord<'src>, self_name: &'src str) -> Result<()> {
        if util::is_chrome_only(&self.attributes) {
            return Ok(());
        }
        record.mixins
            .get_mut(self_name)
            .unwrap()
            .attributes
            .push(self);
        Ok(())
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
            .or_default();

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
            .or_default();

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
        first_pass_operation(
            record,
            FirstPassOperationType::Namespace,
            self_name,
            &[OperationId::Operation(self.identifier.map(|s| s.0.clone()))],
            &self.args.body.list,
            &self.return_type,
            &self.attributes,
            true,
        );
        Ok(())
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
        if self.interfaces.contains_key(superclass) {
            if set.insert(camel_case_ident(superclass)) {
                self.fill_superclasses(superclass, set);
            }
        }
    }

    pub fn all_mixins<'me>(&'me self, interface: &str)
        -> impl Iterator<Item = &'me MixinData<'a>> + 'me
    {
        let mut set = Vec::new();
        self.fill_mixins(interface, interface, &mut set);
        set.into_iter()
    }

    fn fill_mixins<'me>(
        &'me self,
        self_name: &str,
        mixin_name: &str,
        list: &mut Vec<&'me MixinData<'a>>,
    ) {
        if let Some(mixin_data) = self.mixins.get(mixin_name) {
            list.push(mixin_data);
        }
        if let Some(mixin_names) = self.includes.get(mixin_name) {
            for mixin_name in mixin_names {
                self.fill_mixins(self_name, mixin_name, list);
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct IgnoreTraits<T>(pub T);

impl<T> PartialEq for IgnoreTraits<T> {
    fn eq(&self, _other: &IgnoreTraits<T>) -> bool { true }
}

impl<T> Eq for IgnoreTraits<T> {}

impl<T> PartialOrd for IgnoreTraits<T> {
    fn partial_cmp(&self, _other: &IgnoreTraits<T>) -> Option<Ordering> {
        Some(Ordering::Equal)
    }
}

impl<T> Ord for IgnoreTraits<T> {
    fn cmp(&self, _other: &IgnoreTraits<T>) -> Ordering {
        Ordering::Equal
    }
}
