use weedle::argument::{SingleArgument, VariadicArgument};
use weedle::dictionary::DictionaryMember;
use weedle::interface::AttributeInterfaceMember;
use weedle::mixin::AttributeMixinMember;
use weedle::namespace::AttributeNamespaceMember;
use weedle::types::{AttributedNonAnyType, AttributedType, NonAnyType, Type};
use weedle::TypedefDefinition;

/// A WebIDL symbol with a WebIDL type
pub trait SymbolWithType {
	type Ty;
	fn type_(self) -> Self::Ty;
}

macro_rules! impl_symbol_with_type {
	($($sym:ident | $ty:ty),+ $(,)?) => {
		$(
			impl<'a> SymbolWithType for $sym<'a> {
				type Ty = $ty;
				fn type_(self) -> Self::Ty {
					self.type_
				}
			}
		)+
	}
}

impl_symbol_with_type!(
	TypedefDefinition | AttributedType<'a>,
	SingleArgument | AttributedType<'a>,
	VariadicArgument | Type<'a>,
	DictionaryMember | Type<'a>,
	AttributeInterfaceMember | AttributedType<'a>,
	AttributeMixinMember | AttributedType<'a>,
	AttributeNamespaceMember | AttributedType<'a>,
	AttributedType | Type<'a>,
	AttributedNonAnyType | NonAnyType<'a>,
);
