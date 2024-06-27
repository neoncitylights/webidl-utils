use crate::ExtendMayBeNull;
use weedle::common::Identifier;
use weedle::term::*;
use weedle::types::*;

/// Extension methods for `NonAnyType`
pub trait ExtendNonAnyType<'a> {
	fn is_optional(&self) -> bool;
	fn promise(p: PromiseType<'a>) -> Self;
	fn integer(i: IntegerType) -> Self;
	fn integer_opt(i: IntegerType) -> Self;
	fn floating_point(f: FloatingPointType) -> Self;
	fn floating_point_opt(f: FloatingPointType) -> Self;
	fn boolean() -> Self;
	fn boolean_opt() -> Self;
	fn byte() -> Self;
	fn byte_opt() -> Self;
	fn octet() -> Self;
	fn octet_opt() -> Self;
	fn byte_string() -> Self;
	fn byte_string_opt() -> Self;
	fn dom_string() -> Self;
	fn dom_string_opt() -> Self;
	fn usv_string() -> Self;
	fn usv_string_opt() -> Self;
	fn sequence(s: SequenceType<'a>) -> Self;
	fn sequence_opt(s: SequenceType<'a>) -> Self;
	fn object() -> Self;
	fn object_opt() -> Self;
	fn symbol() -> Self;
	fn symbol_opt() -> Self;
	fn error() -> Self;
	fn error_opt() -> Self;
	fn array_buffer() -> Self;
	fn array_buffer_opt() -> Self;
	fn data_view() -> Self;
	fn data_view_opt() -> Self;
	fn int8_array() -> Self;
	fn int8_array_opt() -> Self;
	fn int16_array() -> Self;
	fn int16_array_opt() -> Self;
	fn int32_array() -> Self;
	fn int32_array_opt() -> Self;
	fn uint8_array() -> Self;
	fn uint8_array_opt() -> Self;
	fn uint16_array() -> Self;
	fn uint16_array_opt() -> Self;
	fn uint32_array() -> Self;
	fn uint32_array_opt() -> Self;
	fn uint8_clamped_array() -> Self;
	fn uint8_clamped_array_opt() -> Self;
	fn float32_array() -> Self;
	fn float32_array_opt() -> Self;
	fn float64_array() -> Self;
	fn float64_array_opt() -> Self;
	fn array_buffer_view() -> Self;
	fn array_buffer_view_opt() -> Self;
	fn buffer_source() -> Self;
	fn buffer_source_opt() -> Self;
	fn frozen_array(f: FrozenArrayType<'a>) -> Self;
	fn frozen_array_opt(f: FrozenArrayType<'a>) -> Self;
	fn record(r: RecordType<'a>) -> Self;
	fn record_opt(r: RecordType<'a>) -> Self;
	fn identifier(i: Identifier<'a>) -> Self;
	fn identifier_opt(i: Identifier<'a>) -> Self;
}

impl<'a> ExtendNonAnyType<'a> for NonAnyType<'a> {
	fn is_optional(&self) -> bool {
		match self {
			Self::Promise(_) => false,
			Self::Integer(t) => t.is_optional(),
			Self::FloatingPoint(t) => t.is_optional(),
			Self::Boolean(t) => t.is_optional(),
			Self::Byte(t) => t.is_optional(),
			Self::Octet(t) => t.is_optional(),
			Self::ByteString(t) => t.is_optional(),
			Self::DOMString(t) => t.is_optional(),
			Self::USVString(t) => t.is_optional(),
			Self::Sequence(t) => t.is_optional(),
			Self::Object(t) => t.is_optional(),
			Self::Symbol(t) => t.is_optional(),
			Self::Error(t) => t.is_optional(),
			Self::ArrayBuffer(t) => t.is_optional(),
			Self::DataView(t) => t.is_optional(),
			Self::Int8Array(t) => t.is_optional(),
			Self::Int16Array(t) => t.is_optional(),
			Self::Int32Array(t) => t.is_optional(),
			Self::Uint8Array(t) => t.is_optional(),
			Self::Uint16Array(t) => t.is_optional(),
			Self::Uint32Array(t) => t.is_optional(),
			Self::Uint8ClampedArray(t) => t.is_optional(),
			Self::Float32Array(t) => t.is_optional(),
			Self::Float64Array(t) => t.is_optional(),
			Self::ArrayBufferView(t) => t.is_optional(),
			Self::BufferSource(t) => t.is_optional(),
			Self::FrozenArrayType(t) => t.is_optional(),
			Self::RecordType(t) => t.is_optional(),
			Self::Identifier(t) => t.is_optional(),
		}
	}

	fn promise(p: PromiseType<'a>) -> Self {
		Self::Promise(p)
	}

	fn integer(i: IntegerType) -> Self {
		Self::Integer(MayBeNull::new_required(i))
	}

	fn integer_opt(i: IntegerType) -> Self {
		Self::Integer(MayBeNull::new_optional(i))
	}

	fn floating_point(f: FloatingPointType) -> Self {
		Self::FloatingPoint(MayBeNull::new_required(f))
	}

	fn floating_point_opt(f: FloatingPointType) -> Self {
		Self::FloatingPoint(MayBeNull::new_optional(f))
	}

	fn boolean() -> Self {
		Self::Boolean(MayBeNull::new_required(Boolean))
	}

	fn boolean_opt() -> Self {
		Self::Boolean(MayBeNull::new_optional(Boolean))
	}

	fn byte() -> Self {
		Self::Byte(MayBeNull::new_required(Byte))
	}

	fn byte_opt() -> Self {
		Self::Byte(MayBeNull::new_optional(Byte))
	}

	fn octet() -> Self {
		Self::Octet(MayBeNull::new_required(Octet))
	}

	fn octet_opt() -> Self {
		Self::Octet(MayBeNull::new_optional(Octet))
	}

	fn byte_string() -> Self {
		Self::ByteString(MayBeNull::new_required(ByteString))
	}

	fn byte_string_opt() -> Self {
		Self::ByteString(MayBeNull::new_optional(ByteString))
	}

	fn dom_string() -> Self {
		Self::DOMString(MayBeNull::new_required(DOMString))
	}

	fn dom_string_opt() -> Self {
		Self::DOMString(MayBeNull::new_optional(DOMString))
	}

	fn usv_string() -> Self {
		Self::USVString(MayBeNull::new_required(USVString))
	}

	fn usv_string_opt() -> Self {
		Self::USVString(MayBeNull::new_optional(USVString))
	}

	fn sequence(s: SequenceType<'a>) -> Self {
		Self::Sequence(MayBeNull::new_required(s))
	}

	fn sequence_opt(s: SequenceType<'a>) -> Self {
		Self::Sequence(MayBeNull::new_optional(s))
	}

	fn object() -> Self {
		Self::Object(MayBeNull::new_required(Object))
	}

	fn object_opt() -> Self {
		Self::Object(MayBeNull::new_optional(Object))
	}

	fn symbol() -> Self {
		Self::Symbol(MayBeNull::new_required(Symbol))
	}

	fn symbol_opt() -> Self {
		Self::Symbol(MayBeNull::new_optional(Symbol))
	}

	fn error() -> Self {
		Self::Error(MayBeNull::new_required(Error))
	}

	fn error_opt() -> Self {
		Self::Error(MayBeNull::new_optional(Error))
	}

	fn array_buffer() -> Self {
		Self::ArrayBuffer(MayBeNull::new_required(ArrayBuffer))
	}

	fn array_buffer_opt() -> Self {
		Self::ArrayBuffer(MayBeNull::new_optional(ArrayBuffer))
	}

	fn data_view() -> Self {
		Self::DataView(MayBeNull::new_required(DataView))
	}

	fn data_view_opt() -> Self {
		Self::DataView(MayBeNull::new_optional(DataView))
	}

	fn int8_array() -> Self {
		Self::Int8Array(MayBeNull::new_required(Int8Array))
	}

	fn int8_array_opt() -> Self {
		Self::Int8Array(MayBeNull::new_optional(Int8Array))
	}

	fn int16_array() -> Self {
		Self::Int16Array(MayBeNull::new_required(Int16Array))
	}

	fn int16_array_opt() -> Self {
		Self::Int16Array(MayBeNull::new_optional(Int16Array))
	}

	fn int32_array() -> Self {
		Self::Int32Array(MayBeNull::new_required(Int32Array))
	}

	fn int32_array_opt() -> Self {
		Self::Int32Array(MayBeNull::new_optional(Int32Array))
	}

	fn uint8_array() -> Self {
		Self::Uint8Array(MayBeNull::new_required(Uint8Array))
	}

	fn uint8_array_opt() -> Self {
		Self::Uint8Array(MayBeNull::new_optional(Uint8Array))
	}

	fn uint16_array() -> Self {
		Self::Uint16Array(MayBeNull::new_required(Uint16Array))
	}

	fn uint16_array_opt() -> Self {
		Self::Uint16Array(MayBeNull::new_optional(Uint16Array))
	}

	fn uint32_array() -> Self {
		Self::Uint32Array(MayBeNull::new_required(Uint32Array))
	}

	fn uint32_array_opt() -> Self {
		Self::Uint32Array(MayBeNull::new_optional(Uint32Array))
	}

	fn uint8_clamped_array() -> Self {
		Self::Uint8ClampedArray(MayBeNull::new_required(Uint8ClampedArray))
	}

	fn uint8_clamped_array_opt() -> Self {
		Self::Uint8ClampedArray(MayBeNull::new_optional(Uint8ClampedArray))
	}

	fn float32_array() -> Self {
		Self::Float32Array(MayBeNull::new_required(Float32Array))
	}

	fn float32_array_opt() -> Self {
		Self::Float32Array(MayBeNull::new_optional(Float32Array))
	}

	fn float64_array() -> Self {
		Self::Float64Array(MayBeNull::new_required(Float64Array))
	}

	fn float64_array_opt() -> Self {
		Self::Float64Array(MayBeNull::new_optional(Float64Array))
	}

	fn array_buffer_view() -> Self {
		Self::ArrayBuffer(MayBeNull::new_required(ArrayBuffer))
	}

	fn array_buffer_view_opt() -> Self {
		Self::ArrayBuffer(MayBeNull::new_optional(ArrayBuffer))
	}

	fn buffer_source() -> Self {
		Self::BufferSource(MayBeNull::new_required(BufferSource))
	}

	fn buffer_source_opt() -> Self {
		Self::BufferSource(MayBeNull::new_optional(BufferSource))
	}

	fn frozen_array(f: FrozenArrayType<'a>) -> Self {
		Self::FrozenArrayType(MayBeNull::new_required(f))
	}

	fn frozen_array_opt(f: FrozenArrayType<'a>) -> Self {
		Self::FrozenArrayType(MayBeNull::new_optional(f))
	}

	fn record(r: RecordType<'a>) -> Self {
		Self::RecordType(MayBeNull::new_required(r))
	}

	fn record_opt(r: RecordType<'a>) -> Self {
		Self::RecordType(MayBeNull::new_optional(r))
	}

	fn identifier(i: Identifier<'a>) -> Self {
		Self::Identifier(MayBeNull::new_required(i))
	}

	fn identifier_opt(i: Identifier<'a>) -> Self {
		Self::Identifier(MayBeNull::new_optional(i))
	}
}

#[cfg(test)]
mod extend_non_any {
	use crate::{
		ExtendFrozenArrayType, ExtendNonAnyType, ExtendRecordType, ExtendSequenceType, ExtendType,
	};
	use weedle::common::Identifier;
	use weedle::term::ByteString;
	use weedle::types::{
		FrozenArrayType, NonAnyType, RecordKeyType, RecordType, SequenceType, Type,
	};

	#[test]
	fn test_non_opt() {
		assert!(!NonAnyType::boolean().is_optional());
		assert!(!NonAnyType::byte().is_optional());
		assert!(!NonAnyType::octet().is_optional());
		assert!(!NonAnyType::byte_string().is_optional());
		assert!(!NonAnyType::dom_string().is_optional());
		assert!(!NonAnyType::usv_string().is_optional());
		assert!(!NonAnyType::object().is_optional());
		assert!(!NonAnyType::symbol().is_optional());
		assert!(!NonAnyType::error().is_optional());
		assert!(!NonAnyType::array_buffer().is_optional());
		assert!(!NonAnyType::data_view().is_optional());
		assert!(!NonAnyType::int8_array().is_optional());
		assert!(!NonAnyType::int16_array().is_optional());
		assert!(!NonAnyType::int32_array().is_optional());
		assert!(!NonAnyType::uint8_array().is_optional());
		assert!(!NonAnyType::uint16_array().is_optional());
		assert!(!NonAnyType::uint32_array().is_optional());
		assert!(!NonAnyType::uint8_clamped_array().is_optional());
		assert!(!NonAnyType::float32_array().is_optional());
		assert!(!NonAnyType::float64_array().is_optional());
		assert!(!NonAnyType::array_buffer_view().is_optional());
		assert!(!NonAnyType::buffer_source().is_optional());

		assert!(!NonAnyType::sequence(SequenceType::new(Type::single_any())).is_optional());

		assert!(!NonAnyType::frozen_array(FrozenArrayType::new(Type::single_any())).is_optional());

		assert!(!NonAnyType::record(RecordType::new(
			RecordKeyType::Byte(ByteString),
			Type::single_any()
		))
		.is_optional());

		assert!(!NonAnyType::identifier(Identifier("FooBar")).is_optional());
	}
}
