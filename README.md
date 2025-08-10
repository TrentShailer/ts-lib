# `ts-lib`

A collection of small-ish library crates specifically designed for my personal projects.

> [!CAUTION]
>
> ### This is a personal project
>
> Maintenance, bug fixes, new features, and support will only be provided when/if I feel like it.
> Updates may violate semantic versioning.

## Crates

| Crate name    | Description                                                            | Target dependent | Size    |
| ------------- | ---------------------------------------------------------------------- | ---------------- | ------- |
| `ts-ansi`     | Constant ANSI codes for easy styling and formatting helpers            | `mixed`          | `small` |
| `ts-config`   | Helpers for application config                                         | `binary`         | `large` |
| `ts-error`    | Traits for convenient error reporting, and error report/stack creation | `mixed`          | `small` |
| `ts-io`       | Helpers for input/output related work                                  | `mixed`          | `small` |
| `ts-json`     | JSON schema validation and reporting                                   | `mixed`          | `large` |
| `ts-path`     | Helpers for working with paths                                         | `mixed`          | `small` |
| `ts-terminal` | Helpers for creating my CLIs                                           | `binary`         | `small` |

## Publishing

```bash
cargo publish -p ts-ansi
cargo publish -p ts-path
cargo publish -p ts-error
cargo publish -p ts-io
cargo publish -p ts-json
cargo publish -p ts-config
cargo publish -p ts-terminal
```
