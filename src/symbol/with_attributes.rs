use weedle::argument::{SingleArgument, VariadicArgument};
use weedle::attribute::ExtendedAttribute;
use weedle::common::{Bracketed, Punctuated};
use weedle::dictionary::DictionaryMember;
use weedle::interface::*;
use weedle::mixin::{AttributeMixinMember, OperationMixinMember};
use weedle::namespace::{AttributeNamespaceMember, OperationNamespaceMember};
use weedle::types::{AttributedNonAnyType, AttributedType};
use weedle::*;

/// An iterator over WebIDL attributes
pub struct AttributesIterator<'a> {
	pub attributes: weedle::attribute::ExtendedAttributeList<'a>,
}

impl<'a> AttributesIterator<'a> {
	pub const fn new(attributes: weedle::attribute::ExtendedAttributeList<'a>) -> Self {
		Self { attributes }
	}
}

impl<'a> IntoIterator for AttributesIterator<'a> {
	type Item = ExtendedAttribute<'a>;
	type IntoIter = std::vec::IntoIter<Self::Item>;

	fn into_iter(self) -> Self::IntoIter {
		self.attributes.body.list.into_iter()
	}
}

/// An `IntoIterator` over WebIDL attributes
pub trait IntoAttributesIterator<'a> {
	fn into_iter_attributes(self) -> AttributesIterator<'a>;
}

macro_rules! impl_into_attributes_iterator {
	($($target:ident),+ $(,)?) => {
		$(
			impl<'a> IntoAttributesIterator<'a> for $target<'a> {
				fn into_iter_attributes(self) -> AttributesIterator<'a> {
					match self.attributes.as_ref() {
						Some(a) =>	AttributesIterator::new(a.clone()),
						None => AttributesIterator::new(
							Bracketed {
								open_bracket: weedle::term::OpenBracket,
								body: Punctuated {
									list: vec![],
									separator: weedle::term::Comma,
								},
								close_bracket: weedle::term::CloseBracket,
							}
						)
					}
				}
			}
		)+
	}
}

impl_into_attributes_iterator!(
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

#[cfg(test)]
mod tests {
	use crate::IntoAttributesIterator;
	use weedle::attribute::{ExtendedAttribute, ExtendedAttributeIdent, IdentifierOrString};
	use weedle::common::Identifier;
	use weedle::term::Assign;
	use weedle::{parse, Definition};

	#[test]
	fn test_attributes_iter() {
		let text = "[Exposed=Window] typedef short Number;";
		let parse_result = parse(text);
		if let Ok(definitions) = parse_result {
			let def1 = &definitions[0];
			match def1 {
				Definition::Typedef(t) => {
					let mut iter = t.clone().into_iter_attributes().into_iter();
					assert_eq!(
						iter.next(),
						Some(ExtendedAttribute::Ident(ExtendedAttributeIdent {
							lhs_identifier: Identifier("Exposed"),
							assign: Assign,
							rhs: IdentifierOrString::Identifier(Identifier("Window")),
						}))
					);
				}
				_ => panic!("Not a typedef"),
			}
		} else {
			panic!("weedle2: Could not parse {}", text);
		}
	}
}
