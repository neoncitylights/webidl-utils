use weedle::dictionary::DictionaryMember;
use weedle::interface::*;
use weedle::namespace::{AttributeNamespaceMember, OperationNamespaceMember};
use weedle::*;

/// A WebIDL symbol that may have a documentation string
pub trait SymbolWithDocstring {
	/// Returns a string literal with the starting whitespace removed.
	/// If the symbol has no docstring, it will return an empty string literal.
	///
	/// ## Examples
	/// The following examples compare the behavior and APIs of `weedle2` and `webidl-utils`.
	/// To summarize:
	///  - weedle2:
	///    - Symbols that can contain a docstring will have type `Option<Docstring>`.
	///    - If there is no docstring, it will be `None`.
	///    - A parsed `Docstring` will have a non-normalized string. For example, if
	///      a docstring is written as `/// This is an enum`, the resulting string will be
	///      `" This is an enum"`.
	///  - webidl-utils:
	///    - `SymbolWithDocString` will return a `&str`.
	///    - If there is no docstring, it will be `""` (empty string literal).
	///    - It will normalize strings by removing the beginning whitespace.
	///
	/// ```
	/// use webidl_utils::symbol::SymbolWithDocstring;
	/// use weedle::{EnumDefinition, Parse};
	///
	/// let (_, enum_def) = EnumDefinition::parse(
	///     r#"
	///         /// This is an enum
	///         enum Color { "red", "green", "blue" };
	///     "#)
	///     .expect("EnumDefinition parsed with an error");
	///
	/// // weedle2 behavior
	/// assert_eq!(enum_def.docstring.clone().unwrap().0.as_str(), " This is an enum");
	///
	/// // webidl-utils behavior
	/// assert_eq!(enum_def.docstring(), "This is an enum");
	/// ```
	fn docstring(&self) -> &str;
	fn has_docstring(&self) -> bool;
}

macro_rules! impl_symbol_with_docstring {
	($($sym:ident),+ $(,)?) => {
		$(
			impl<'a> SymbolWithDocstring for $sym<'a> {
				fn docstring(&self) -> &str {
					self.docstring.as_ref()
						.map(|docstring| &docstring.0[..])
						.unwrap_or("")
						.trim_start()
				}

				fn has_docstring(&self) -> bool {
					self.docstring.is_some()
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

#[cfg(test)]
mod tests {
	use crate::symbol::SymbolWithDocstring;
	use weedle::{EnumDefinition, Parse};

	#[test]
	fn test() {
		let (_, enum_def) = EnumDefinition::parse(
			r#"
				/// This is an enum
				enum Color { "red", "green", "blue" };
			"#,
		)
		.expect("EnumDefinition parsed with an error");

		assert_eq!(enum_def.docstring(), "This is an enum");
		assert_eq!(enum_def.has_docstring(), true);
	}
}
