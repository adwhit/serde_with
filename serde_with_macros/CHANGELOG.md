# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](http://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [1.2.0]

### Added

* Add `serde_as` macro. Refer to the `serde_with` crate for details.
* Add two derive macros, `SerializeDisplay` and `DeserializeFromStr`, which implement the `Serialize`/`Deserialize` traits based on `Display` and `FromStr`.
    This is in addition to the already existing methods like `DisplayFromStr`, which act locally, whereas the derive macros provide the traits expected by the rest of the ecosystem.

### Changed

* Convert the code to use 2018 edition.

### Fixed

* The `serde_as` macro now supports serde attributes and no longer panic on unrecognized values in the attribute.

## [1.2.0-alpha.3]

### Added

* Add two derive macros, `SerializeDisplay` and `DeserializeFromStr`, which implement the `Serialize`/`Deserialize` traits based on `Display` and `FromStr`.
    This is in addition to the already existing methods like `DisplayFromStr`, which act locally, whereas the derive macros provide the traits expected by the rest of the ecosystem.

## [1.2.0-alpha.2]

### Fixed

* The `serde_as` macro now supports serde attributes and no longer panic on unrecognized values in the attribute.

## [1.2.0-alpha.1]

### Added

* Add `serde_as` macro. Refer to the `serde_with` crate for details.

### Changed

* Convert the code to use 2018 edition.

## [1.1.0]

### Changed

* Bump minimal Rust version to 1.36.0 to support Rust Edition 2018
* Improved CI pipeline by running `cargo audit` and `tarpaulin` in all configurations now.

## [1.0.1]

### Fixed

* Features for the `syn` dependency were missing.
    This was hidden due to the dev-dependencies whose features leaked into the normal build.

## [1.0.0]

Initial Release

### Added

* Add `skip_serializing_none` attribute, which adds `#[serde(skip_serializing_if = "Option::is_none")]` for each Option in a struct.
    This is helpfull for APIs which have many optional fields.
    The effect of can be negated by adding `serialize_always` on those fields, which should always be serialized.
    Existing `skip_serializing_if` will never be modified and those fields keep their behavior.
