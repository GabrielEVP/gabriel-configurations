# Development

Toolchain: Rust 1.98.1, pinned in rust-toolchain.toml. The package contains a library and executable; Cargo.lock is part of the project.

Comprobaciones:

```sh
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
```

Each behavior is developed with a failing test for the missing functionality, a minimal implementation, and a green refactor. Environment or compiler errors do not replace that evidence. Do not add comments to project code or tests; explain decisions here and preserve third-party legal notices.

Planned review units: environment detection; catalog and plan; providers; recoverable application; updates; per-module migration; distribution. Each unit keeps its tests and documentation with its implementation. They can be grouped into chained PRs when publication is requested.

## TDD evidence

| Increment | Red | Green | Refactor/verification |
|---|---|---|---|
| Platform detection | cargo test --test platform: 2 failures for missing detection | 5 tests pass | cargo fmt and clippy without warnings |

| User paths | 5 failures for missing resolution | 5 tests pass | No directory creation; paths with spaces |
| Catalog and graph | 5 failures for missing loading/order | 7 tests pass | fmt and clippy; localized manifest errors |
| Processes and Homebrew | 3 failures for missing execution/query | 4 tests pass | Literal arguments; errors without credentials |

| Planning | 3 failures for missing plan | 4 tests pass | Classification and transitive exclusion |
| Composition | 3 failures for missing composition | 5 tests pass | Templates, escaping, and links |
| State and updates | 6 failures for missing application | 8 tests pass after fixing identical adoption | Backups, locking, and journal recovery |

| Initial CLI | 3 failures for missing help/selection/plan | 3 tests pass | dry-run without creating HOME or state |

## Learning notes

Platform groups related values in a struct. OperatingSystem and Architecture are enums, preventing unsupported systems from becoming valid states. Result returns a valid platform or an error; tests exercise both paths without querying or modifying the host.

## Integración continua

GitHub Actions verifies Ubuntu and macOS. .github/workflows/ci.yml runs fmt, clippy, and tests on ubuntu-24.04 and macos-15; real installations and graphical tests require separate acceptance jobs. The workflow file alone does not prove a successful remote run.
