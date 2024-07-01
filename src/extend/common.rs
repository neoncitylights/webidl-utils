/// Extension methods for `Braced`
pub trait ExtendBraced<T> {
	fn new(body: T) -> Self;
}

impl<T> ExtendBraced<T> for weedle::common::Braced<T> {
	fn new(body: T) -> Self {
		Self {
			open_brace: weedle::term::OpenBrace,
			body,
			close_brace: weedle::term::CloseBrace,
		}
	}
}

/// Extension methods for `Bracketed`
pub trait ExtendBracketed<T> {
	fn new(body: T) -> Self;
}

impl<T> ExtendBracketed<T> for weedle::common::Bracketed<T> {
	fn new(body: T) -> Self {
		Self {
			open_bracket: weedle::term::OpenBracket,
			body,
			close_bracket: weedle::term::CloseBracket,
		}
	}
}

/// Extension methods for `Docstring`
pub trait ExtendDocstring {
	fn new(s: &str) -> Self;
}

impl ExtendDocstring for weedle::common::Docstring {
	fn new(s: &str) -> Self {
		Self(String::from(s))
	}
}

/// Extension methods for `Generics`
pub trait ExtendGenerics<T> {
	fn new(body: T) -> Self;
}

impl<T> ExtendGenerics<T> for weedle::common::Generics<T> {
	fn new(body: T) -> Self {
		Self {
			open_angle: weedle::term::LessThan,
			body,
			close_angle: weedle::term::GreaterThan,
		}
	}
}

/// Extension methods for `Parenthesized`
pub trait ExtendParenthesized<T> {
	fn new(body: T) -> Self;
}

impl<T> ExtendParenthesized<T> for weedle::common::Parenthesized<T> {
	fn new(body: T) -> Self {
		Self {
			open_paren: weedle::term::OpenParen,
			body,
			close_paren: weedle::term::CloseParen,
		}
	}
}

/// Extension methods for `Punctuated`
pub trait ExtendPuncutated<T, S> {
	fn new(list: Vec<T>, separator: S) -> Self;
}

impl<T, S> ExtendPuncutated<T, S> for weedle::common::Punctuated<T, S> {
	fn new(list: Vec<T>, separator: S) -> Self {
		Self { list, separator }
	}
}

/// Extension methods for `PunctuatedNonEmpty`
pub trait ExtendPuncutatedNonEmpty<T, S> {
	fn new(list: Vec<T>, separator: S) -> Self;
}

impl<T, S> ExtendPuncutatedNonEmpty<T, S> for weedle::common::PunctuatedNonEmpty<T, S> {
	fn new(list: Vec<T>, separator: S) -> Self {
		Self { list, separator }
	}
}

#[cfg(test)]
mod extend_braced {
	use crate::ExtendBraced;
	use weedle::common::{Braced, Identifier};

	#[test]
	fn test() {
		assert_eq!(Braced::new(Identifier("Foo")).body, Identifier("Foo"));
	}
}

#[cfg(test)]
mod extend_bracketed {
	use crate::ExtendBracketed;
	use weedle::common::{Bracketed, Identifier};

	#[test]
	fn test() {
		assert_eq!(Bracketed::new(Identifier("Foo")).body, Identifier("Foo"));
	}
}

#[cfg(test)]
mod extend_docstring {
	use crate::ExtendDocstring;
	use weedle::common::Docstring;

	#[test]
	fn test() {
		assert_eq!(Docstring::new("Foo").0, String::from("Foo"));
	}
}

#[cfg(test)]
mod extend_generics {
	use crate::{ExtendGenerics, ExtendType};
	use weedle::common::Generics;
	use weedle::types::Type;

	#[test]
	fn test() {
		assert_eq!(Generics::new(Type::single_any()).body, Type::single_any());
	}
}

#[cfg(test)]
mod extend_parenthesized {
	use crate::ExtendParenthesized;
	use weedle::common::{Identifier, Parenthesized};

	#[test]
	fn test() {
		assert_eq!(
			Parenthesized::new(Identifier("Foo")).body,
			Identifier("Foo")
		);
	}
}

#[cfg(test)]
mod extend_punctuated {
	use crate::ExtendPuncutated;
	use weedle::common::Punctuated;
	use weedle::term::Comma;

	#[test]
	fn test() {
		let punct = Punctuated::new(vec!["Alice", "Bob"], Comma);
		assert_eq!(punct.list, vec!["Alice", "Bob"]);
	}
}

#[cfg(test)]
mod extend_punctuated_non_empty {
	use crate::ExtendPuncutatedNonEmpty;
	use weedle::common::PunctuatedNonEmpty;
	use weedle::term::Comma;

	#[test]
	fn test() {
		let punct = PunctuatedNonEmpty::new(vec!["Alice", "Bob"], Comma);
		assert_eq!(punct.list, vec!["Alice", "Bob"]);
	}
}
