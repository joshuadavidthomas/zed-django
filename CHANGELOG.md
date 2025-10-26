# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project attempts to adhere to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

<!--
## [${version}]
### Added - for new features
### Changed - for changes in existing functionality
### Deprecated - for soon-to-be removed features
### Removed - for now removed features
### Fixed - for any bug fixes
### Security - in case of vulnerabilities
[${version}]: https://github.com/joshuadavidthomas/zed-django/releases/tag/v${version}
-->

## [Unreleased]

### Fixed

- Fixed path duplication issue when downloading language server binary that caused "Permission denied" errors

## [0.1.1]

### Fixed

- Fixed duplicate version prefix in language server asset names that prevented download on first installation

## [0.1.0]

### Added

- Initial Django template support for Zed editor
- Syntax highlighting via tree-sitter-htmldjango grammar
- Language server integration with django-language-server
- Automatic detection of files with `.dj.html`, `.dj.md`, or `.dj.txt` extensions
- Automatic detection of Django templates starting with `{% extends` or `{% load`
- Automatic download and installation of django-language-server binaries
- Support for macOS (darwin), Linux, and Windows on arm64 and x64 architectures
- Configuration options for using alternative language servers
- Documentation for file type associations and glob patterns

### New Contributors

- Josh Thomas <josh@joshthomas.dev> (maintainer)

[unreleased]: https://github.com/joshuadavidthomas/zed-django/compare/v0.1.1...HEAD
[0.1.0]: https://github.com/joshuadavidthomas/zed-django/releases/tag/v0.1.0
[0.1.1]: https://github.com/joshuadavidthomas/zed-django/releases/tag/v0.1.1
