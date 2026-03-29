# CLAUDE.md

## Project Overview

`bloom` is a minimal Rust CLI binary that runs a configurable bloom filter over lines or tokens from STDIN. It deduplicates input lines, printing only the first occurrence of each unique line (or token).

## Repository Structure

```
bloom/
├── src/main.rs           # Entire application (~46 lines)
├── Cargo.toml            # Package manifest and dependencies
├── Cargo.lock            # Pinned dependency versions
├── README.md             # User-facing documentation and examples
├── .github/
│   ├── workflows/rust.yml  # GitHub Actions CI (build + test)
│   └── dependabot.yml      # Dependabot config (not yet fully configured)
└── .gitignore
```

## Development Commands

```bash
# Check for compile errors (fast, no binary)
cargo check

# Debug build
cargo build --verbose

# Run (pass args after --)
cargo run -- --help
cargo run -- -d ';' -i 1

# Release/optimized build
cargo build --release
# Binary output: target/release/bloom

# Run tests
cargo test --verbose

# Format code
cargo fmt

# Lint
cargo clippy
```

## Key Dependencies

| Crate | Version | Purpose |
|-------|---------|---------|
| `bloomfilter` | 1.0.12 | Bloom filter implementation (bit-vec + siphasher) |
| `clap` | 4.4.8 | CLI argument parsing via derive macros |

## CLI Interface

```
Usage: bloom [OPTIONS]

Options:
  -b, --bitmap-bytes <BITMAP_BYTES>  Bloom filter bitmap size in bytes [default: 16384]
  -c, --count <COUNT>                Expected number of items [default: 16384]
  -d, --delimiter <DELIMITER>        Field delimiter for token extraction
  -i, --index <INDEX>                Zero-based field index to use as the dedup key
  -h, --help                         Print help
  -V, --version                      Print version
```

When `-d` and `-i` are both provided, deduplication is based on the extracted token, but the **full original line** is printed (not just the token).

## Code Architecture

All logic lives in `src/main.rs`:

1. `Args` struct — derives `clap::Parser` for CLI argument parsing
2. `main()` — initializes the bloom filter, reads STDIN line-by-line, skips blank lines, optionally extracts a token via delimiter+index, checks the bloom filter, and prints unseen lines

Key implementation details:
- Uses `Cow<str>` to avoid allocation when no delimiter/index is specified
- `bloom.check_and_set()` returns `true` if the item was already present; unseen items return `false` and are printed
- Panics with a descriptive message if a line has fewer fields than the requested index

## CI/CD

GitHub Actions (`.github/workflows/rust.yml`) runs on pushes and PRs to `master`:
1. `cargo build --verbose`
2. `cargo test --verbose`

There are currently no tests in the project. The CI passes because `cargo test` succeeds with zero test cases.

## Conventions

- **Single-file project**: keep all logic in `src/main.rs` unless the file grows substantially
- **Rust edition 2021**: use modern idioms (let-else, `?` operator, etc.)
- **No external config**: the tool is configured entirely via CLI flags
- **Panic on invalid input**: the current approach panics on malformed input (missing delimiter field); this is acceptable for a Unix filter tool
- **No tests yet**: if adding tests, use inline `#[cfg(test)]` modules or a `tests/` directory for integration tests

## Bloom Filter Behavior

A bloom filter can produce false positives (reporting an item as seen when it hasn't been), but never false negatives. The false positive rate is controlled by:
- `--bitmap-bytes`: larger values reduce false positives (more memory)
- `--count`: set to approximate number of unique items expected

Default settings (16 KiB bitmap, 16384 items) are suitable for moderate deduplication workloads. For large datasets, increase both values.
