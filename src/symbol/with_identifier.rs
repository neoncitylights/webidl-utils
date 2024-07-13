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

#[cfg(test)]
mod test {
	use crate::symbol::SymbolWithIdentifier;
	use weedle::attribute::ExtendedAttributeNoArgs;
	use weedle::common::Identifier;
	use weedle::{CallbackDefinition, Parse};

	#[test]
	fn test_extended_attribute_no_args() {
		let test = ExtendedAttributeNoArgs(Identifier("FooBar"));
		assert_eq!(test.identifier(), Identifier("FooBar"));
	}

	#[test]
	fn test_callback_definition() {
		let (_, callback) = CallbackDefinition::parse(
			"callback AudioDataOutputCallback = undefined(AudioData output);",
		)
		.expect("WebIDL callback parsed with an error");

		assert_eq!(callback.identifier(), Identifier("AudioDataOutputCallback"));
	}
}
