use weedle::argument::{SingleArgument, VariadicArgument};
use weedle::dictionary::DictionaryMember;
use weedle::interface::{AttributeInterfaceMember, ConstMember};
use weedle::mixin::AttributeMixinMember;
use weedle::namespace::AttributeNamespaceMember;
use weedle::*;

/// A WebIDL symbol with an identifier
pub trait SymbolWithIdentifier<'a> {
	fn identifier(self) -> weedle::common::Identifier<'a>;
}

macro_rules! impl_symbol_with_identifier {
	($($sym:ident),+ $(,)?) => {
		$(
			impl<'a> SymbolWithIdentifier<'a> for $sym<'a> {
				fn identifier(self) -> weedle::common::Identifier<'a> {
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
	TypedefDefinition,
	SingleArgument,
	VariadicArgument,
	DictionaryMember,
	ConstMember,
	AttributeInterfaceMember,
	AttributeMixinMember,
	AttributeNamespaceMember,
);
