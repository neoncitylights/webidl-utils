use weedle::interface::OperationInterfaceMember;
use weedle::mixin::OperationMixinMember;
use weedle::namespace::OperationNamespaceMember;

/// A WebIDL symbol that may have an identifier
pub trait SymbolWithOptionalIdentifier<'a> {
	fn identifier(self) -> Option<weedle::common::Identifier<'a>>;
}

macro_rules! impl_symbol_with_optional_identifier {
	($($sym:ident),+ $(,)?) => {
		$(
			impl<'a> SymbolWithOptionalIdentifier<'a> for $sym<'a> {
				fn identifier(self) -> Option<weedle::common::Identifier<'a>> {
					self.identifier
				}
			}
		)+
	};
}

impl_symbol_with_optional_identifier!(
	OperationInterfaceMember,
	OperationMixinMember,
	OperationNamespaceMember,
);
