# Zed Razor

A [Razor](https://learn.microsoft.com/en-us/aspnet/core/razor-pages/) (.cshtml / .razor) extension for [Zed](https://zed.dev).

## Features

- Syntax highlighting for Razor constructs (`@model`, `@using`, `@inject`, `@section`, etc.)
- HTML injection inside control flow blocks (`@if`, `@foreach`, `@section`)
- C# injection inside code blocks (`@{ ... }`) and explicit expressions (`@( ... )`)
- Razor comment highlighting (`@* ... *@`)
- Bracket matching for Razor blocks

## Language Server

This extension uses [rzls](https://github.com/Crashdummyy/rzls), a standalone Razor Language Server extracted from the Roslyn toolchain. The binary is automatically downloaded on first use.

> **Note:** rzls is a co-host language server that requires a proxy layer to forward requests between Roslyn (C#) and HTML language servers. Full IntelliSense support (go-to-definition, hover, completions) is not yet available — syntax highlighting works fully.

## Development

To develop this extension, see the [Developing Extensions](https://zed.dev/docs/extensions/developing-extensions) section of the Zed docs.

## License

MIT — see [LICENSE](LICENSE).
