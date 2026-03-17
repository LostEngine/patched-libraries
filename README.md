# emmy_lsp_types

[![CI](https://github.com/EmmyLuaLs/emmy_lsp_types/workflows/CI/badge.svg)](https://github.com/EmmyLuaLs/emmy_lsp_types/actions)
[![Crates.io](https://img.shields.io/crates/v/emmy_lsp_types.svg)](https://crates.io/crates/emmy_lsp_types)
[![Documentation](https://docs.rs/emmy_lsp_types/badge.svg)](https://docs.rs/emmy_lsp_types)

Language Server Protocol types for Rust - A fork of [lsp-types](https://github.com/gluon-lang/lsp-types) with enhancements.

## 🎯 Overview

This library provides Rust types for the [Language Server Protocol](https://microsoft.github.io/language-server-protocol/) (LSP). It's a fork of the excellent [lsp-types](https://github.com/gluon-lang/lsp-types) crate with additional features and improvements.

## ✨ Key Differences from lsp-types

### 1. **Modern URI Handling**
- ✅ Migrated from `fluent-uri` to the battle-tested `url` crate (100M+ downloads)
- ✅ Built-in serde support without custom implementations
- ✅ Full trait support: `Hash`, `Eq`, `Ord`, `FromStr`
- ✅ 46 comprehensive URI tests covering Windows/Unix platforms

### 2. **LSP 3.18 Protocol Support**
- ✅ `SnippetTextEdit` - Code snippet editing with tab stops
- ✅ `DocumentRangesFormattingParams` - Format multiple ranges at once
- ✅ `OneOf3` enum - Three-way type unions

### 3. **Enhanced Code Quality**
- ✅ Comprehensive Clippy configuration (all/pedantic/nursery)
- ✅ Zero Clippy warnings with pragmatic allow rules
- ✅ 78 tests with 100% pass rate
- ✅ Platform-specific tests for Windows and Unix

## 📦 Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
emmy_lsp_types = "0.1.0"
```

## 🧪 Testing

```bash
# Run all tests
cargo test

# Run with output
cargo test -- --nocapture

# Run specific test module
cargo test uri::test
```

## 🔍 Code Quality

This project maintains high code quality standards:

```bash
# Check formatting
cargo fmt --check

# Run Clippy (should produce 0 warnings)
cargo clippy --all-targets

# Build in release mode
cargo build --release
```

## 📊 Test Coverage

- **78 tests** covering all major LSP types
- **46 URI tests** for cross-platform path handling
- **9 formatting tests** for LSP 3.18 features
- **Platform-specific tests** using `#[cfg(windows)]` and `#[cfg(unix)]`

## 🛠️ Development

### Prerequisites

- Rust 2024 edition or later
- Cargo

### Building

```bash
# Debug build
cargo build

# Release build
cargo build --release

# Run tests
cargo test

# Format code
cargo fmt

# Check lints
cargo clippy --all-targets
```

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

### Guidelines

1. Run `cargo fmt` before committing
2. Ensure `cargo clippy --all-targets` produces no warnings
3. Add tests for new features
4. Update documentation as needed

## 📄 License

This project is licensed under the same terms as the original [lsp-types](https://github.com/gluon-lang/lsp-types) crate.

## 🙏 Acknowledgments

This project is a fork of [lsp-types](https://github.com/gluon-lang/lsp-types) by the Gluon team. We're grateful for their excellent work on the original implementation.

### Why Fork?

We created this fork to:
- Modernize URI handling with the `url` crate
- Add LSP 3.18 protocol support
- Implement stricter code quality standards
- Provide more comprehensive testing
- Maintain faster iteration on new features

## 🔗 Links

- [Original lsp-types](https://github.com/gluon-lang/lsp-types)
- [LSP Specification](https://microsoft.github.io/language-server-protocol/)
- [LSP 3.18 Specification](https://microsoft.github.io/language-server-protocol/specifications/lsp/3.18/specification/)

## 📈 Status

- ✅ All tests passing (78/78)
- ✅ Zero Clippy warnings
- ✅ LSP 3.18 support
- ✅ Cross-platform tested
- 🚧 Actively maintained
