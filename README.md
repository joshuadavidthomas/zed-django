# zed-django

A [Django](https://djangoproject.com) extension for [Zed](https://zed.dev).

## Using an Alternative Language Server

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
