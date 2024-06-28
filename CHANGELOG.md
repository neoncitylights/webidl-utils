# Changelog

## 0.1.1 (Unreleased / YYYY-MM-DD)

### Breaking changes
- `ExtendNonAnyType` trait: the function signatures of `identifier()` and `identifier_opt()` are simplified. They now take a string literal. Previously, they both took an `Identifier` type (a newtype that stores a string literal) ([#12](https://github.com/neoncitylights/webidl-utils/pull/12))

### Features
- Add and implement `ExtendRecordKeyType` trait for `weedle::types::RecordKeyType`

## 0.1.0 (2024-06-27)

- Initial release of the `webidl-utils` library
