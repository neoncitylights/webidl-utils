use weedle::common::Generics;
use weedle::interface::*;
use weedle::term::Comma;
use weedle::types::*;

/// A WebIDL symbol with generic types
pub trait SymbolWithGenerics {
	type GenericType;
	fn generics(self) -> Generics<Self::GenericType>;
}

macro_rules! impl_symbol_with_generics {
	($($sym:ident | $t: ty),+ $(,)?) => {
		$(
			impl<'a> SymbolWithGenerics for $sym<'a> {
				type GenericType = $t;
				fn generics(self) -> Generics<Self::GenericType> {
					self.generics
				}
			}
		)+
	};
}

impl_symbol_with_generics!(
	SingleTypedIterable | AttributedType<'a>,
	DoubleTypedIterable | (AttributedType<'a>, Comma, AttributedType<'a>),
	SingleTypedAsyncIterable | AttributedType<'a>,
	DoubleTypedAsyncIterable | (AttributedType<'a>, Comma, AttributedType<'a>),
	MaplikeInterfaceMember | (AttributedType<'a>, Comma, AttributedType<'a>),
	SetlikeInterfaceMember | AttributedType<'a>,
	SequenceType | Box<Type<'a>>,
	FrozenArrayType | Box<Type<'a>>,
	PromiseType | Box<ReturnType<'a>>,
	RecordType | (Box<RecordKeyType<'a>>, Comma, Box<Type<'a>>),
);
