use weedle::attribute::{ExtendedAttributeIdent, ExtendedAttributeNamedArgList};
use weedle::{ImplementsDefinition, IncludesStatementDefinition};

/// A WebIDL symbol with 2 identifiers on the left and right side
pub trait SymbolWithIdentifier2<'a>:
	SymbolWithIdentifierLhs<'a> + SymbolWithIdentifierRhs<'a>
{
}

/// A WebIDL symbol with a left-hand side identifier
pub trait SymbolWithIdentifierRhs<'a> {
	fn rhs_identifier(&self) -> weedle::common::Identifier<'a>;
}

/// A WebIDL symbol with a right-hand side identifier
pub trait SymbolWithIdentifierLhs<'a> {
	fn lhs_identifier(&self) -> weedle::common::Identifier<'a>;
}

macro_rules! impl_symbol_with_identifier_lhs {
	($($sym:ident),+ $(,)?) => {
		$(
			impl<'a> SymbolWithIdentifierLhs<'a> for $sym<'a> {
				fn lhs_identifier(&self) -> weedle::common::Identifier<'a> {
					self.lhs_identifier
				}
			}
		)+
	}
}

macro_rules! impl_symbol_with_identifier_rhs {
	($($sym:ident),+ $(,)?) => {
		$(
			impl<'a> SymbolWithIdentifierRhs<'a> for $sym<'a> {
				fn rhs_identifier(&self) -> weedle::common::Identifier<'a> {
					self.rhs_identifier
				}
			}
		)+
	}
}

impl_symbol_with_identifier_lhs!(
	ImplementsDefinition,
	IncludesStatementDefinition,
	ExtendedAttributeNamedArgList,
	ExtendedAttributeIdent,
);

impl_symbol_with_identifier_rhs!(
	ImplementsDefinition,
	IncludesStatementDefinition,
	ExtendedAttributeNamedArgList
);

impl<'a> SymbolWithIdentifier2<'a> for ImplementsDefinition<'a> {}
impl<'a> SymbolWithIdentifier2<'a> for IncludesStatementDefinition<'a> {}
impl<'a> SymbolWithIdentifier2<'a> for ExtendedAttributeNamedArgList<'a> {}

#[cfg(test)]
mod test_identifier2 {
	use crate::symbol::{SymbolWithIdentifierLhs, SymbolWithIdentifierRhs};
	use weedle::common::Identifier;
	use weedle::{parse, Definition};

	#[rustfmt::skip]
	#[test]
	fn test_implements_definition() {
		let defs =
			parse("FooBar implements Foo;")
			.expect("WebIDL content parsed with an error");

		let def = &defs[0];
		if let Definition::Implements(i) = def {
			assert_eq!(i.lhs_identifier(), Identifier("FooBar"));
			assert_eq!(i.rhs_identifier(), Identifier("Foo"));
		} else {
			panic!("Expected an Implements definition");
		}
	}
}
