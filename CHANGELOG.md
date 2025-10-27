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

### Added

- Support for multiple language servers (Django Language Server and Django Template LSP)
- Modular language server architecture with trait-based design
- Comprehensive documentation for switching between language servers
- Zero-install support for `django-template-lsp` via `uvx --from django-template-lsp djlsp` (ephemeral execution)
- Simplified installation strategy: prefers `uvx` → checks for existing `djlsp` installation → helpful error message
- No automatic installation - users either have `uv` (recommended) or manually install `djlsp`
- With `uv` installed, `django-template-lsp` works immediately with no setup required

### Changed

- **BREAKING**: Language server ID changed from `djls` to `django-language-server`
- Refactored extension to support multiple language servers simultaneously
- Updated README with detailed language server comparison and configuration instructions
- Users can now choose between `django-language-server` (default, auto-installs) and `django-template-lsp` (manual install, more features)

### Migration Guide

The language server ID has changed from `djls` to `django-language-server`. If you have custom LSP settings:

**Old configuration:**
```json
{
  "lsp": {
    "djls": {
      "settings": { ... }
    }
  }
}
```

**New configuration:**
```json
{
  "lsp": {
    "django-language-server": {
      "settings": { ... }
    }
  }
}
```

If you were using the default configuration with no custom settings, no changes are required.

## [0.1.2]

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

[unreleased]: https://github.com/joshuadavidthomas/zed-django/compare/v0.1.2...HEAD
[0.1.0]: https://github.com/joshuadavidthomas/zed-django/releases/tag/v0.1.0
[0.1.1]: https://github.com/joshuadavidthomas/zed-django/releases/tag/v0.1.1
[0.1.2]: https://github.com/joshuadavidthomas/zed-django/releases/tag/v0.1.2
