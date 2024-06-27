use weedle::interface::{AttributeInterfaceMember, MaplikeInterfaceMember, SetlikeInterfaceMember};
use weedle::mixin::AttributeMixinMember;

/// A WebIDL symbol that may have a readonly modifier
pub trait SymbolWithReadOnly {
	fn readonly(self) -> Option<weedle::term::ReadOnly>;
}

macro_rules! impl_symbol_with_readonly {
	($($sym:ident),+ $(,)?) => {
		$(
			impl<'a> SymbolWithReadOnly for $sym<'a> {
				fn readonly(self) -> Option<weedle::term::ReadOnly> {
					self.readonly
				}
			}
		)+
	};
}

impl_symbol_with_readonly!(
	AttributeInterfaceMember,
	MaplikeInterfaceMember,
	SetlikeInterfaceMember,
	AttributeMixinMember,
);
