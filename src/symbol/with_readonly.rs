use weedle::interface::{AttributeInterfaceMember, MaplikeInterfaceMember, SetlikeInterfaceMember};
use weedle::mixin::AttributeMixinMember;

/// A WebIDL symbol that may have a readonly modifier
pub trait SymbolWithReadOnly {
	fn readonly(&self) -> Option<weedle::term::ReadOnly>;
}

macro_rules! impl_symbol_with_readonly {
	($($sym:ident),+ $(,)?) => {
		$(
			impl<'a> SymbolWithReadOnly for $sym<'a> {
				fn readonly(&self) -> Option<weedle::term::ReadOnly> {
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

#[cfg(test)]
mod tests {
	use crate::symbol::SymbolWithReadOnly;
	use weedle::interface::AttributeInterfaceMember;
	use weedle::Parse;

	#[test]
	fn test_attribute_interface_member() {
		let (_, member) = AttributeInterfaceMember::parse("readonly attribute float width;")
			.expect("AttributeInterfaceMember parsed with an error");

		assert!(member.readonly().is_some());
	}
}
