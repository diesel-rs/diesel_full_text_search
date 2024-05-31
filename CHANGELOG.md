# Change Log

All user visible changes to this project will be documented in this file.
This project adheres to [Semantic Versioning](http://semver.org/), as described
for Rust libraries in [RFC #1105](https://github.com/rust-lang/rfcs/blob/master/text/1105-api-evolution.md)

## Unreleased

### [2.2.0] - 2024-05-31

* Update diesel version to 2.2.0

### [2.1.1] - 2023-10-19

* Add a missing `#[derive(FromSqlRow)]` for the `RegConfig` type to allow loading it from the database

### [2.1.0] 

* Diesel 2.1 compatibility

### [2.0.0]

* Diesel 2.0 compatiblity

### [1.0.1] - 2018-04-11

* `TsVector::concat` now properly wraps its values in parenthesis

## 1.0.0 - 2018-01-02

* Initial release

[1.0.1]: https://github.com/diesel-rs/diesel_full_text_search/compare/v1.0.0...v1.0.1
[2.0.0]: https://github.com/diesel-rs/diesel_full_text_search/compare/v1.0.1...v2.0.0
[2.1.0]: https://github.com/diesel-rs/diesel_full_text_search/compare/v2.0.0...v2.1.0
[2.1.1]: https://github.com/diesel-rs/diesel_full_text_search/compare/v2.1.0...v2.1.1
[2.2.0]: https://github.com/diesel-rs/diesel_full_text_search/compare/v2.1.1...v2.2.0
