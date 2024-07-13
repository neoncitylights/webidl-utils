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
	fn has_attributes(&self) -> bool;
}

macro_rules! impl_symbol_with_attributes {
	($($sym:ident),+ $(,)?) => {
		$(
			impl<'a> SymbolWithAttributes<'a> for $sym<'a> {
				fn attributes(self) -> Option<weedle::attribute::ExtendedAttributeList<'a>> {
					self.attributes
				}

				fn has_attributes(&self) -> bool {
					self.attributes.is_some()
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
	PartialNamespaceDefinition,
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
