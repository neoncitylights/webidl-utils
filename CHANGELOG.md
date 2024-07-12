# Changelog

## Unreleased (YYYY-MM-DD)

### Breaking changes
- All traits are now under their own module instead of the root, based on their name.
  - Traits with a name that start with `Extend` are now under the `webidl_utils::extend` module.
  - Traits with a name that start with `Symbol` are now under the `webidl_utils::symbol` module.

## 0.3.0 (2024-06-30)

### Features
- Add `ExtendParenthesized` trait ([#21](https://github.com/neoncitylights/webidl-utils/pull/21))

## 0.2.1 (2024-06-30)

### Documentation
- Fixed a minor typo in documentation for `ExtendPunctuatedNonEmpty` trait ([#20](https://github.com/neoncitylights/webidl-utils/pull/20))

## 0.2.0 (2024-06-29)

### Bugfixes
- `ExtendNonAnyType` trait:  Fixed `array_buffer_view()` and `array_buffer_view_opt()` to properly return `NonAnyType::ArrayBufferView`. Previously, they returned `NonAnyType::ArrayBuffer`. ([#15](https://github.com/neoncitylights/webidl-utils/pull/15))

### Breaking changes
- `ExtendNonAnyType` trait: the function signatures of `identifier()` and `identifier_opt()` are simplified. They now take a string literal. Previously, they both took an `Identifier` type (a newtype that stores a string literal) ([#12](https://github.com/neoncitylights/webidl-utils/pull/12))

### Features
- Add traits for extending types from `weedle::common` module: `ExtendBraced`, `ExtendBracketed`, `ExtendDocstring`, `ExtendGenerics`, `ExtendPunctuated`, `ExtendPunctuatedNonEmpty` ([#16](https://github.com/neoncitylights/webidl-utils/pull/16))
- Add `ExtendNonAnyType::is_required()` ([#14](https://github.com/neoncitylights/webidl-utils/pull/14))
- Add `ExtendMayBeNull::is_required()` ([#13](https://github.com/neoncitylights/webidl-utils/pull/13))
- Add and implement `ExtendRecordKeyType` trait for `weedle::types::RecordKeyType` ([#11](https://github.com/neoncitylights/webidl-utils/pull/11))

### Internal changes
- Expand code coverage for `ExtendNonAnyType` ([#15](https://github.com/neoncitylights/webidl-utils/pull/15))
- Fix badges in README.md ([#17](https://github.com/neoncitylights/webidl-utils/pull/17))

## 0.1.0 (2024-06-27)

- Initial release of the `webidl-utils` library
