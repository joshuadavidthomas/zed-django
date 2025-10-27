# zed-django

A [Django](https://djangoproject.com) extension for [Zed](https://zed.dev).

## Installation

Install the extension from the Zed extensions directory:

1. Open the Extensions panel (`zed: extensions` in the command palette or `ctrl-shift-x`/`cmd-shift-x`)
2. Search for "Django"
3. Click "Install"

## Usage

### File Associations

By default, this extension automatically detects:

- Files with `.dj.html`, `.dj.md`, or `.dj.txt` extensions
- Files starting with `{% extends` or `{% load`

#### Using Django syntax in `.html` files

Since `.html` files conflict with the built-in HTML extension, you'll need to manually configure file associations for your Django templates.

##### Per-file (Quick)

Right-click on an `.html` file in Zed and select `Select Language` → `Django`, or click the language indicator in the status bar at the bottom right of the editor.

> [!NOTE]
> This only applies to the current session. The file will revert to HTML next time you open it.

##### Django templates directory (Recommended)

Add to your `.zed/settings.json` in your Django project:

```json
{
  "file_types": {
    "Django": ["**/templates/**/*.html"]
  }
}
```

This matches all `.html` files in any `templates` directory, following Django's standard project structure.

##### All HTML files (use with caution)

To treat all `.html` files as Django templates:

```json
{
  "file_types": {
    "Django": ["html"]
  }
}
```

> [!NOTE]
> This will override the built-in HTML language for all `.html` files and may affect non-Django HTML files.

##### Global settings

Add to your global Zed settings (`zed: open settings`):

```json
{
  "file_types": {
    "Django": ["**/templates/**/*.html"]
  }
}
```

Global settings affect all projects. Project-specific settings are recommended for now.

#### Using Django syntax in other file types

Of course, Django templates aren't limited to HTML. You can use glob patterns to match templates of any file type.

##### Directory-based matching

Match multiple file types within your templates directory:

```json
{
  "file_types": {
    "Django": [
      "**/templates/**/*.html",
      "**/templates/**/*.md",
      "**/templates/**/*.txt"
    ]
  }
}
```

##### Extension-based matching

Use a `.dj.*` naming convention to mark Django templates:

```json
{
  "file_types": {
    "Django": ["*.dj.*"]
  }
}
```

This matches any file with `.dj.` in the name (e.g., `.dj.html`, `.dj.xml`, `.dj.md`), allowing you to use Django templates with any file extension anywhere in your project.

## Language Servers

There are two language servers available for Django templates. The extension supports both:

- [Django Language Server](https://github.com/joshuadavidthomas/django-language-server)
- [Django Template LSP](https://github.com/fourdigits/django-template-lsp)

Both provide autocompletion and other LSP features for Django templates, but with different feature sets. You should choose one based on your needs. Note that you typically should not use both simultaneously.

**Django Language Server** is enabled by default. To switch to **Django Template LSP**, see the [Configuration](#configuration) section.

### Feature Comparison

| Feature | Django Language Server | Django Template LSP |
|---------|----------------------|---------------------|
| Diagnostics | ✅ | ❌ |
| Completions | ✅ (template tags) | ✅ (tags, filters, templates, load, static, URLs) |
| Go to definition | ✅ (extend/include tags) | ✅ (templates, URLs, tags/filters, context) |
| Find references | ✅ (extend/include tags) | ❌ |
| Hover documentation | ❌ | ✅ (tags, filters, URLs) |

### Installation

Both language servers check if their binary (`djls` or `djlsp`) is on your PATH first.

If not found on PATH:

- Django Language Server automatically downloads and installs from GitHub releases
- Django Template LSP runs via `uvx --from django-template-lsp djlsp` if `uv` is available. See the [fourdigits/django-template-lsp repository](https://github.com/fourdigits/django-template-lsp) for installation instructions.

### Configuration

Django Language Server is enabled by default and requires no configuration.

To switch to Django Template LSP, add the following to your `settings.json`:

```json
{
  "languages": {
    "Django": {
      "language_servers": ["django-template-lsp", "!django-language-server", "..."]
    }
  }
}
```

For project-specific configuration, create `.zed/settings.json` in your Django project root.

For detailed documentation and advanced configuration, see their respective repositories:

- [joshuadavidthomas/django-language-server](https://github.com/joshuadavidthomas/django-language-server)
- [fourdigits/django-template-lsp](https://github.com/fourdigits/django-template-lsp)

## Development

To develop this extension, see the [Developing Extensions](https://zed.dev/docs/extensions/developing-extensions) section of the Zed docs.

## License

zed-django is licensed under the Apache License, Version 2.0. See the [`LICENSE`](LICENSE) file for more information.

---

zed-django is not associated with the Django Software Foundation.

Django is a registered trademark of the Django Software Foundation.
