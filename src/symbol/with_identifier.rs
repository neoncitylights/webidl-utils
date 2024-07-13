use weedle::argument::{SingleArgument, VariadicArgument};
use weedle::attribute::ExtendedAttributeNoArgs;
use weedle::dictionary::DictionaryMember;
use weedle::interface::{AttributeInterfaceMember, ConstMember, Inheritance};
use weedle::mixin::AttributeMixinMember;
use weedle::namespace::AttributeNamespaceMember;
use weedle::*;

/// A WebIDL symbol with an identifier
pub trait SymbolWithIdentifier<'a> {
	fn identifier(&self) -> weedle::common::Identifier<'a>;
}

impl<'a> SymbolWithIdentifier<'a> for ExtendedAttributeNoArgs<'a> {
	fn identifier(&self) -> weedle::common::Identifier<'a> {
		self.0
	}
}

macro_rules! impl_symbol_with_identifier {
	($($sym:ident),+ $(,)?) => {
		$(
			impl<'a> SymbolWithIdentifier<'a> for $sym<'a> {
				fn identifier(&self) -> weedle::common::Identifier<'a> {
					self.identifier
				}
			}
		)+
	}
}

impl_symbol_with_identifier!(
	CallbackDefinition,
	CallbackInterfaceDefinition,
	DictionaryDefinition,
	EnumDefinition,
	InterfaceDefinition,
	InterfaceMixinDefinition,
	NamespaceDefinition,
	PartialDictionaryDefinition,
	PartialInterfaceDefinition,
	PartialInterfaceMixinDefinition,
	PartialNamespaceDefinition,
	TypedefDefinition,
	SingleArgument,
	VariadicArgument,
	DictionaryMember,
	ConstMember,
	AttributeInterfaceMember,
	AttributeMixinMember,
	AttributeNamespaceMember,
	Inheritance,
);
