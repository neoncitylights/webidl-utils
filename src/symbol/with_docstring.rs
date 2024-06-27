use weedle::dictionary::DictionaryMember;
use weedle::interface::*;
use weedle::namespace::{AttributeNamespaceMember, OperationNamespaceMember};
use weedle::*;

/// A WebIDL symbol that may have a documentation string
pub trait SymbolWithDocString {
	fn docstring(&self) -> Option<weedle::common::Docstring>;
}

macro_rules! impl_symbol_with_docstring {
	($($sym:ident),+ $(,)?) => {
		$(
			impl<'a> SymbolWithDocString for $sym<'a> {
				fn docstring(&self) -> Option<weedle::common::Docstring> {
					self.docstring
				}
			}
		)+
	};
}

impl_symbol_with_docstring!(
	CallbackInterfaceDefinition,
	InterfaceDefinition,
	NamespaceDefinition,
	DictionaryDefinition,
	EnumDefinition,
	EnumVariant,
	DictionaryMember,
	ConstructorInterfaceMember,
	OperationInterfaceMember,
	OperationNamespaceMember,
	AttributeNamespaceMember,
);
