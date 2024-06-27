use weedle::interface::OperationInterfaceMember;
use weedle::mixin::OperationMixinMember;
use weedle::namespace::OperationNamespaceMember;
use weedle::CallbackDefinition;

/// A WebIDL symbol with a WebIDL return type
pub trait SymbolWithReturnType<'a> {
	fn return_type(self) -> weedle::types::ReturnType<'a>;
}

macro_rules! impl_symbol_with_return_type {
	($($sym:ident),+ $(,)?) => {
		$(
			impl<'a> SymbolWithReturnType<'a> for $sym<'a> {
				fn return_type(self) -> weedle::types::ReturnType<'a> {
					self.return_type
				}
			}
		)+
	};
}

impl_symbol_with_return_type!(
	CallbackDefinition,
	OperationInterfaceMember,
	OperationMixinMember,
	OperationNamespaceMember,
);
