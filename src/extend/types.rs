use weedle::common::Generics;
use weedle::term::*;
use weedle::types::*;

/// Extension methods for `MayBeNull<T>`
pub trait ExtendMayBeNull<T> {
	fn new(type_: T, q_mark: Option<QMark>) -> Self;
	fn new_optional(type_: T) -> Self;
	fn new_required(type_: T) -> Self;
	fn is_optional(&self) -> bool;
}

impl<T> ExtendMayBeNull<T> for MayBeNull<T> {
	fn new(type_: T, q_mark: Option<QMark>) -> Self {
		Self { type_, q_mark }
	}

	fn new_optional(type_: T) -> Self {
		Self::new(type_, Some(QMark))
	}

	fn new_required(type_: T) -> Self {
		Self::new(type_, None)
	}

	fn is_optional(&self) -> bool {
		self.q_mark.is_some()
	}
}

/// Extension methods for floating point types
pub trait ExtendFloatingPointType {
	fn unrestricted(&self) -> Option<Unrestricted>;
}

/// Extension methods for creating floating point types
pub trait ExtendFloatingPointTypeNew {
	type FloatType;
	fn new(unrestricted: Option<Unrestricted>, ty: Self::FloatType) -> Self;
	fn new_unrestricted() -> Self;
	fn new_restricted() -> Self;
}

impl ExtendFloatingPointType for FloatingPointType {
	fn unrestricted(&self) -> Option<Unrestricted> {
		match self {
			Self::Float(f) => f.unrestricted,
			Self::Double(f) => f.unrestricted,
		}
	}
}

impl ExtendFloatingPointType for DoubleType {
	fn unrestricted(&self) -> Option<Unrestricted> {
		self.unrestricted
	}
}

impl ExtendFloatingPointTypeNew for DoubleType {
	type FloatType = Double;

	fn new(unrestricted: Option<Unrestricted>, ty: Self::FloatType) -> Self {
		Self {
			unrestricted,
			double: ty,
		}
	}

	fn new_unrestricted() -> Self {
		Self::new(Some(Unrestricted), Double)
	}

	fn new_restricted() -> Self {
		Self::new(None, Double)
	}
}

impl ExtendFloatingPointType for FloatType {
	fn unrestricted(&self) -> Option<Unrestricted> {
		self.unrestricted
	}
}

impl ExtendFloatingPointTypeNew for FloatType {
	type FloatType = Float;

	fn new(unrestricted: Option<Unrestricted>, ty: Self::FloatType) -> Self {
		Self {
			unrestricted,
			float: ty,
		}
	}

	fn new_unrestricted() -> Self {
		Self::new(Some(Unrestricted), Float)
	}

	fn new_restricted() -> Self {
		Self::new(None, Float)
	}
}

/// Extension methods for integer types
pub trait ExtendIntegerType {
	fn unsigned(&self) -> Option<Unsigned>;
}

/// Extension methods for creating integer types
pub trait ExtendIntegerTypeNew {
	type IntegerType;
	fn new(unsigned: Option<Unsigned>, ty: Self::IntegerType) -> Self;
	fn new_signed() -> Self;
	fn new_unsigned() -> Self;
}

impl ExtendIntegerType for IntegerType {
	fn unsigned(&self) -> Option<Unsigned> {
		match self {
			Self::Short(i) => i.unsigned,
			Self::Long(i) => i.unsigned,
			Self::LongLong(i) => i.unsigned,
		}
	}
}

impl ExtendIntegerType for ShortType {
	fn unsigned(&self) -> Option<Unsigned> {
		self.unsigned
	}
}

impl ExtendIntegerTypeNew for ShortType {
	type IntegerType = Short;

	fn new(unsigned: Option<Unsigned>, short: Self::IntegerType) -> Self {
		Self { unsigned, short }
	}

	fn new_signed() -> Self {
		Self::new(None, Short)
	}

	fn new_unsigned() -> Self {
		Self::new(Some(Unsigned), Short)
	}
}

impl ExtendIntegerType for LongType {
	fn unsigned(&self) -> Option<Unsigned> {
		self.unsigned
	}
}

impl ExtendIntegerTypeNew for LongType {
	type IntegerType = Long;

	fn new(unsigned: Option<Unsigned>, long: Self::IntegerType) -> Self {
		Self { unsigned, long }
	}

	fn new_signed() -> Self {
		Self::new(None, Long)
	}

	fn new_unsigned() -> Self {
		Self::new(Some(Unsigned), Long)
	}
}

impl ExtendIntegerType for LongLongType {
	fn unsigned(&self) -> Option<Unsigned> {
		self.unsigned
	}
}

impl ExtendIntegerTypeNew for LongLongType {
	type IntegerType = (Long, Long);

	#[rustfmt::skip]
	fn new(unsigned: Option<Unsigned>, long_long: Self::IntegerType) -> Self {
		Self { unsigned, long_long }
	}

	fn new_signed() -> Self {
		Self::new(None, (Long, Long))
	}

	fn new_unsigned() -> Self {
		Self::new(Some(Unsigned), (Long, Long))
	}
}

/// Extension methods for `Generics`
pub trait ExtendGenerics<T> {
	fn new(body: T) -> Self;
}

impl<T> ExtendGenerics<T> for Generics<T> {
	fn new(body: T) -> Self {
		Self {
			open_angle: LessThan,
			body,
			close_angle: GreaterThan,
		}
	}
}

/// Extension methods for `PromiseType`
pub trait ExtendPromiseType<'a> {
	fn new(ty: ReturnType<'a>) -> Self;
}

impl<'a> ExtendPromiseType<'a> for PromiseType<'a> {
	fn new(ty: ReturnType<'a>) -> Self {
		Self {
			promise: Promise,
			generics: Generics::new(Box::new(ty)),
		}
	}
}

/// Extension methods for `SequenceType`
pub trait ExtendSequenceType<'a> {
	fn new(ty: Type<'a>) -> Self;
}

impl<'a> ExtendSequenceType<'a> for SequenceType<'a> {
	fn new(ty: Type<'a>) -> Self {
		Self {
			sequence: Sequence,
			generics: Generics::new(Box::new(ty)),
		}
	}
}

/// Extension methods for `FrozenArrayType`
pub trait ExtendFrozenArrayType<'a> {
	fn new(ty: Type<'a>) -> Self;
}

impl<'a> ExtendFrozenArrayType<'a> for FrozenArrayType<'a> {
	fn new(ty: Type<'a>) -> Self {
		Self {
			frozen_array: FrozenArray,
			generics: Generics::new(Box::new(ty)),
		}
	}
}

/// Extension methods for `RecordType`
pub trait ExtendRecordType<'a> {
	fn new(key: RecordKeyType<'a>, value: Type<'a>) -> Self;
}

impl<'a> ExtendRecordType<'a> for RecordType<'a> {
	fn new(key: RecordKeyType<'a>, value: Type<'a>) -> Self {
		Self {
			record: Record,
			generics: Generics::new((Box::new(key), Comma, Box::new(value))),
		}
	}
}

/// Extension methods for `RecordKeyType`
pub trait ExtendRecordKeyType<'a> {
	fn byte() -> Self;
	fn dom() -> Self;
	fn usv() -> Self;
	fn non_any(nat: NonAnyType<'a>) -> Self;
}

impl<'a> ExtendRecordKeyType<'a> for RecordKeyType<'a> {
	fn byte() -> Self {
		Self::Byte(ByteString)
	}

	fn dom() -> Self {
		Self::DOM(DOMString)
	}

	fn usv() -> Self {
		Self::USV(USVString)
	}

	fn non_any(nat: NonAnyType<'a>) -> Self {
		Self::NonAny(nat)
	}
}

/// Extension methods for `Type`
pub trait ExtendType {
	fn single_any() -> Self;
}

impl<'a> ExtendType for Type<'a> {
	fn single_any() -> Self {
		Self::Single(SingleType::any())
	}
}

/// Extension methods for `SingleType`
pub trait ExtendSingleType {
	fn any() -> Self;
}

impl<'a> ExtendSingleType for SingleType<'a> {
	fn any() -> Self {
		Self::Any(Any)
	}
}

#[cfg(test)]
mod extend_maybenull {
	use crate::ExtendMayBeNull;
	use weedle::term::{Byte, QMark};
	use weedle::types::MayBeNull;

	#[test]
	fn test_new() {
		let maybe = MayBeNull::new(Byte, Some(QMark));
		assert_eq!(maybe.type_, Byte);
		assert_eq!(maybe.q_mark, Some(QMark));
	}
}

#[cfg(test)]
mod extend_float {
	use crate::{ExtendFloatingPointType, ExtendFloatingPointTypeNew};
	use weedle::term::Unrestricted;
	use weedle::types::{DoubleType, FloatType, FloatingPointType};

	#[test]
	fn test_unrestricted_webidl_f32() {
		let wi_f32 = FloatingPointType::Float(FloatType::new_unrestricted());
		assert_eq!(wi_f32.unrestricted(), Some(Unrestricted));
	}

	#[test]
	fn test_unrestricted_webidl_f64() {
		let wi_f64 = FloatingPointType::Double(DoubleType::new_unrestricted());
		assert_eq!(wi_f64.unrestricted(), Some(Unrestricted));
	}
}

#[cfg(test)]
mod extend_integer {
	use crate::{ExtendIntegerType, ExtendIntegerTypeNew};
	use weedle::term::Unsigned;
	use weedle::types::{IntegerType, LongLongType, LongType, ShortType};

	#[test]
	fn test_short() {
		let int1 = IntegerType::Short(ShortType::new_unsigned());
		assert_eq!(int1.unsigned(), Some(Unsigned));

		let int2 = IntegerType::Short(ShortType::new_signed());
		assert_eq!(int2.unsigned(), None);
	}

	#[test]
	fn test_long() {
		let int1 = IntegerType::Long(LongType::new_unsigned());
		assert_eq!(int1.unsigned(), Some(Unsigned));

		let int2 = IntegerType::Long(LongType::new_signed());
		assert_eq!(int2.unsigned(), None);
	}

	#[test]
	fn test_long_long() {
		let int1 = IntegerType::LongLong(LongLongType::new_unsigned());
		assert_eq!(int1.unsigned(), Some(Unsigned));

		let int2 = IntegerType::LongLong(LongLongType::new_signed());
		assert_eq!(int2.unsigned(), None);
	}
}

#[cfg(test)]
mod extend_generics {
	use crate::{ExtendGenerics, ExtendType};
	use weedle::common::Generics;
	use weedle::types::Type;

	#[test]
	fn test_new() {
		assert_eq!(Generics::new(Type::single_any()).body, Type::single_any(),);
	}
}

#[cfg(test)]
mod extend_promise {
	use crate::{ExtendGenerics, ExtendNonAnyType, ExtendPromiseType};
	use weedle::common::Generics;
	use weedle::term::Promise;
	use weedle::types::{NonAnyType, PromiseType, ReturnType, SingleType, Type};

	#[test]
	fn test_new() {
		let p = PromiseType::new(ReturnType::Type(Type::Single(SingleType::NonAny(
			NonAnyType::boolean(),
		))));

		assert_eq!(p.promise, Promise);
		assert_eq!(
			p.generics,
			Generics::new(Box::new(ReturnType::Type(Type::Single(
				SingleType::NonAny(NonAnyType::boolean())
			))))
		);
	}
}

#[cfg(test)]
mod extend_frozen_array {
	use crate::{ExtendFrozenArrayType, ExtendGenerics, ExtendNonAnyType};
	use weedle::common::Generics;
	use weedle::term::FrozenArray;
	use weedle::types::{FrozenArrayType, NonAnyType, SingleType, Type};

	#[test]
	fn test_new() {
		let f = FrozenArrayType::new(Type::Single(SingleType::NonAny(NonAnyType::boolean())));

		assert_eq!(f.frozen_array, FrozenArray);
		assert_eq!(
			f.generics,
			Generics::new(Box::new(Type::Single(SingleType::NonAny(
				NonAnyType::boolean()
			))))
		);
	}
}

#[cfg(test)]
mod extend_record {
	use crate::{ExtendRecordKeyType, ExtendRecordType, ExtendType};
	use weedle::types::{RecordKeyType, RecordType, Type};

	#[test]
	fn test_new() {
		let r = RecordType::new(RecordKeyType::byte(), Type::single_any());

		assert_eq!(*(r.generics.body.0), RecordKeyType::byte());
		assert_eq!(*(r.generics.body.2), Type::single_any());
	}
}

#[cfg(test)]
mod extend_record_key {
	use crate::{ExtendNonAnyType, ExtendRecordKeyType};
	use weedle::term::{Boolean, ByteString, DOMString, USVString};
	use weedle::types::{MayBeNull, NonAnyType, RecordKeyType};

	#[test]
	fn test_variant_byte() {
		assert_eq!(RecordKeyType::byte(), RecordKeyType::Byte(ByteString));
	}

	#[test]
	fn test_variant_dom() {
		assert_eq!(RecordKeyType::dom(), RecordKeyType::DOM(DOMString));
	}

	#[test]
	fn test_variant_usv() {
		assert_eq!(RecordKeyType::usv(), RecordKeyType::USV(USVString));
	}

	#[test]
	fn test_variant_nat() {
		assert_eq!(
			RecordKeyType::non_any(NonAnyType::boolean()),
			RecordKeyType::NonAny(NonAnyType::Boolean(MayBeNull {
				type_: Boolean,
				q_mark: None,
			}))
		);
	}
}
