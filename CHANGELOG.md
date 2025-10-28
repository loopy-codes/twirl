# Changelog

## [0.0.2](https://github.com/cadojo/loom/compare/v0.0.1...v0.0.2) (2025-10-28)


### Bug Fixes

* Resolve import error due to invalid module name ([800fcbf](https://github.com/cadojo/loom/commit/800fcbf7a67164589165c58d4d9f25a9a4742c8d))

## 0.0.1 (2025-10-28)


### Features

* Add `supported_extensions` function ([cf65a86](https://github.com/cadojo/loom/commit/cf65a8685dd949ecb1cd522ae6530db4d4f16747))


### Bug Fixes

* Enable integration test `python.rs` ([9cbff8e](https://github.com/cadojo/loom/commit/9cbff8e7bb931658cb339393aca3931f924cc7eb))


### Testing

* Add `treesitter` test for Python code ([1c48b12](https://github.com/cadojo/loom/commit/1c48b12184d382b88b0c7aa29ca88df7414871d2))
* Updated Python integration test with iterating over all children ([9cbff8e](https://github.com/cadojo/loom/commit/9cbff8e7bb931658cb339393aca3931f924cc7eb))


### Project Configuration

* Add `maturin` and `uv` scaffolding ([9e70700](https://github.com/cadojo/loom/commit/9e707009a31ef731ba5d6988defa21bbe2fe2df0))
* Add `pre-commit` for Markdown linting and spell checking ([4091c4f](https://github.com/cadojo/loom/commit/4091c4f47f52a7c8948d9adab0cecb90654257c9))
* Add Git hooks with pre-commit ([13b2c44](https://github.com/cadojo/loom/commit/13b2c4419309c9f52bea3a84051c49c102ca69e3))
* Add languages module with language dependencies ([1e6e8e7](https://github.com/cadojo/loom/commit/1e6e8e73c5bb4bb3f90090f7f2a37d1b71cecea1))
* Add Release Please changelog sections ([2e1fbca](https://github.com/cadojo/loom/commit/2e1fbcabbd28a6b504028aaf66bd046256eb92af))
* Bootstrap releases for path: . ([#1](https://github.com/cadojo/loom/issues/1)) ([ae33ece](https://github.com/cadojo/loom/commit/ae33ece240c8e137baccc2cd09e16a70ae09fcc7))
* Move PyO3 module into Rust modules ([e50c54c](https://github.com/cadojo/loom/commit/e50c54cd51528678b5dcfcec1d05330a7bd1ef5a))
* Move Python unit tests to subfolder ([38851a6](https://github.com/cadojo/loom/commit/38851a67eb8734a2af3784ecf851465de8c0b112))
* Pin first release to v0.0.1 ([4ea2a4a](https://github.com/cadojo/loom/commit/4ea2a4af06ae4fbd6863905ab9eab81168a2cbf6))
* **release-please:** Switch to prerelease ([a6e2fc8](https://github.com/cadojo/loom/commit/a6e2fc8aeda834dbc2d7936af40a4b9242e46f73))
* **release-please:** Update PR configuration ([951cf5b](https://github.com/cadojo/loom/commit/951cf5bfd733956838a2405c5171bbde565d0857))
* Rename Rust module in Python to `_pyo3` ([0c7fe28](https://github.com/cadojo/loom/commit/0c7fe28bdbe26c611ec2fc068c61381e83c1c32f))
* Update dependencies in Cargo lockfile ([b1ac6a8](https://github.com/cadojo/loom/commit/b1ac6a83e52e9e1e95dc2d6ab32b0ff4e2f61fc3))
* Update dependencies in Python lockfile ([67aafd3](https://github.com/cadojo/loom/commit/67aafd3435e3470deb92ab842513557b00535586))
* Update Release Please configuration ([6ab0c15](https://github.com/cadojo/loom/commit/6ab0c159b248b15df965ba08a54cbaf9348f2faa))
* Upgrade `pre-commit` hooks ([df0db57](https://github.com/cadojo/loom/commit/df0db57f9b8e0fbb3290556ae904c300154e284c))


### Code Refactoring

* Change `languages` Python module to `o3` ([4cdf382](https://github.com/cadojo/loom/commit/4cdf382db028d102902e22016d4dc15b8a61efea))
* Move `py` module to `py03` ([45a8697](https://github.com/cadojo/loom/commit/45a86978c6957927d1fb94f1efd275c181646115))
* Rename `_pyo3` to `languages` ([e50c54c](https://github.com/cadojo/loom/commit/e50c54cd51528678b5dcfcec1d05330a7bd1ef5a))
* Rename `extensions` to `languages` ([e50c54c](https://github.com/cadojo/loom/commit/e50c54cd51528678b5dcfcec1d05330a7bd1ef5a))
* Rename project from `pearl` to `loom` ([f00b603](https://github.com/cadojo/loom/commit/f00b603d0e4a2e5d367638ec669558e04ba53df7))
