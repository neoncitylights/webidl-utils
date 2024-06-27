use weedle::argument::{SingleArgument, VariadicArgument};
use weedle::dictionary::DictionaryMember;
use weedle::interface::*;
use weedle::mixin::{AttributeMixinMember, OperationMixinMember};
use weedle::namespace::{AttributeNamespaceMember, OperationNamespaceMember};
use weedle::types::{AttributedNonAnyType, AttributedType};
use weedle::*;

/// A WebIDL symbol that may have 0 or more extended attributes
pub trait SymbolWithAttributes<'a> {
	fn attributes(self) -> Option<weedle::attribute::ExtendedAttributeList<'a>>;
}

macro_rules! impl_symbol_with_attributes {
	($($sym:ident),+ $(,)?) => {
		$(
			impl<'a> SymbolWithAttributes<'a> for $sym<'a> {
				fn attributes(self) -> Option<weedle::attribute::ExtendedAttributeList<'a>> {
					self.attributes
				}
			}
		)+
	};
}

impl_symbol_with_attributes!(
	CallbackDefinition,
	CallbackInterfaceDefinition,
	DictionaryDefinition,
	EnumDefinition,
	ImplementsDefinition,
	IncludesStatementDefinition,
	InterfaceDefinition,
	InterfaceMixinDefinition,
	NamespaceDefinition,
	PartialDictionaryDefinition,
	PartialInterfaceDefinition,
	PartialInterfaceMixinDefinition,
	TypedefDefinition,
	SingleArgument,
	VariadicArgument,
	DictionaryMember,
	ConstMember,
	AttributeInterfaceMember,
	ConstructorInterfaceMember,
	OperationInterfaceMember,
	SingleTypedIterable,
	DoubleTypedIterable,
	SingleTypedAsyncIterable,
	DoubleTypedAsyncIterable,
	MaplikeInterfaceMember,
	SetlikeInterfaceMember,
	StringifierMember,
	OperationMixinMember,
	AttributeMixinMember,
	OperationNamespaceMember,
	AttributeNamespaceMember,
	AttributedType,
	AttributedNonAnyType,
);
