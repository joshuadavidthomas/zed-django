# zed-django

A [Django](https://djangoproject.com) extension for [Zed](https://zed.dev).

## Installation

> [!NOTE]
> The extension is currently awaiting approval for the official Zed extension store ([relevant PR](https://github.com/zed-industries/extensions/pull/3525)).
>
> In the meantime, you can install it as a dev extension. To do so, you will need a Rust toolchain available on your machine, the easiest and recommended way is via [rustup](https://rust-lang.org/tools/install). See the Zed docs on [developing an extension locally](https://zed.dev/docs/extensions/developing-extensions#developing-an-extension-locally) for more information.
>
> Once you have Rust available, you can follow these steps:
>
> 1. Clone the [zed-django](https://github.com/joshuadavidthomas/zed-django) repository locally
> 2. Open the Extensions panel (`zed: extensions` in the command palette or `ctrl-shift-x`/`cmd-shift-x`)
> 3. Click "Install Dev Extension" in the upper right corner and select the cloned repo folder

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

### Using an Alternative Language Server

By default, the extension uses [Django Language Server](https://github.com/joshuadavidthomas/django-language-server) as its default language server. If you prefer to use a different language server, such as the [Django Template LSP server](https://github.com/fourdigits/django-template-lsp), you can disable the default server and configure your own in your Zed settings:

```json
{
  "lsp": {
    "django-language-server": {
      "enabled": false
    },
    "django-template-lsp": {
      "binary": {
        "path": "/path/to/django-template-lsp",
        "arguments": []
      },
      "languages": ["Django"]
    }
  }
}
```

## Development

To develop this extension, see the [Developing Extensions](https://zed.dev/docs/extensions/developing-extensions) section of the Zed docs.

## License

zed-django is licensed under the Apache License, Version 2.0. See the [`LICENSE`](LICENSE) file for more information.

---

zed-django is not associated with the Django Software Foundation.

Django is a registered trademark of the Django Software Foundation.
