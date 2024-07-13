# Changelog

## Unreleased (YYYY-MM-DD)

### Features
- Implemented the traits `SymbolWithIdentifier` and `SymbolWithAttributes` for type `PartialNamespaceDefinition`. ([#25](https://github.com/neoncitylights/webidl-utils/pull/25))
- Added 3 traits for symbols with a sided identifier
  - `SymbolWithIdentifier2` trait, for WebIDL symbol defined with 2 identifiers on the left and right side.
  - `SymbolWithIdentifierLhs`, for WebIDL symbols with an identifier on the left-hand side
  - `SymbolWithIdentifierRhs`, for WebIDL symbols with an identifier on the right-hand side


### Breaking changes
- Fixed typo in trait names. ([#24](https://github.com/neoncitylights/webidl-utils/pull/24))
  - Renamed `ExtendPuncutated` to `ExtendPunctuated`.
  - Renamed `ExtendPuncutatedNonEmpty` to `ExtendPunctuatedNonEmpty`.
  - **Note**: While this is technically a "bug fix", this is moreso considered a breaking change because it's changing the trait name as part of the public API.

## 0.4.0 (2024-07-12)

### Breaking changes
- All traits are now under their own module instead of the root, based on their name. ([#22](https://github.com/neoncitylights/webidl-utils/pull/22))
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
