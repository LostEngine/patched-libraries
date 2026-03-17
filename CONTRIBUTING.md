# Contributing to emmy_lsp_types

Thank you for your interest in contributing! We welcome contributions from everyone.

## Getting Started

1. **Fork the repository** on GitHub
2. **Clone your fork** locally:
   ```bash
   git clone https://github.com/YOUR_USERNAME/emmy_lsp_types.git
   cd emmy_lsp_types
   ```
3. **Create a branch** for your changes:
   ```bash
   git checkout -b feature/your-feature-name
   ```

## Development Workflow

### Prerequisites

- Rust 2024 edition or later
- Cargo (comes with Rust)

### Building

```bash
# Debug build
cargo build

# Release build
cargo build --release
```

### Testing

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_name

# Run with output
cargo test -- --nocapture

# Run tests for specific module
cargo test uri::test
```

### Code Quality Checks

Before submitting a PR, ensure:

1. **Formatting**: Code is properly formatted
   ```bash
   cargo fmt
   ```

2. **Linting**: No Clippy warnings
   ```bash
   cargo clippy --all-targets
   ```
   This should produce **0 warnings**.

3. **Tests**: All tests pass
   ```bash
   cargo test
   ```

4. **Documentation**: Code is documented
   ```bash
   cargo doc --no-deps
   ```

## Coding Standards

### Style Guide

- Follow Rust standard style (enforced by `cargo fmt`)
- Use meaningful variable and function names
- Add documentation comments for public APIs
- Keep functions focused and reasonably sized

### Clippy Configuration

We use a comprehensive Clippy configuration. See [CLIPPY_RULES.md](./CLIPPY_RULES.md) for details.

Key points:
- Explicit type names preferred over `Self` for clarity
- Documentation should be clear but not overly verbose
- Test code has more relaxed rules
- Performance optimizations should not sacrifice readability

### Testing

- Add tests for new features
- Include both positive and negative test cases
- Use platform-specific tests where appropriate:
  ```rust
  #[cfg(windows)]
  #[test]
  fn test_windows_specific() {
      // ...
  }
  
  #[cfg(unix)]
  #[test]
  fn test_unix_specific() {
      // ...
  }
  ```

### Documentation

- All public APIs must have documentation comments
- Include examples in doc comments when helpful
- Update README.md for significant changes
- Update CHANGELOG.md (if exists) for all changes

## Pull Request Process

1. **Update tests**: Add or update tests for your changes
2. **Update documentation**: Update relevant documentation
3. **Run checks**: Ensure all checks pass:
   ```bash
   cargo fmt
   cargo clippy --all-targets
   cargo test
   cargo doc --no-deps
   ```
4. **Commit messages**: Write clear, descriptive commit messages
5. **Create PR**: Open a pull request with a clear description

### PR Description Template

```markdown
## Description
Brief description of the changes

## Motivation
Why is this change needed?

## Changes
- List of changes made

## Testing
How have you tested these changes?

## Checklist
- [ ] Code formatted with `cargo fmt`
- [ ] No Clippy warnings (`cargo clippy --all-targets`)
- [ ] All tests pass (`cargo test`)
- [ ] Documentation updated
- [ ] Tests added for new features
```

## Reporting Issues

When reporting issues, please include:

1. **Description**: Clear description of the issue
2. **Steps to reproduce**: Minimal example to reproduce
3. **Expected behavior**: What should happen
4. **Actual behavior**: What actually happens
5. **Environment**: 
   - OS (Windows/Linux/macOS)
   - Rust version (`rustc --version`)
   - Crate version

## Code of Conduct

### Our Standards

- Be respectful and inclusive
- Welcome newcomers
- Focus on what's best for the project
- Show empathy towards others

### Unacceptable Behavior

- Harassment or discriminatory language
- Trolling or insulting comments
- Personal or political attacks
- Publishing others' private information

## Questions?

If you have questions:
- Open a GitHub issue
- Check existing issues and discussions
- Review the documentation

## License

By contributing, you agree that your contributions will be licensed under the same license as the project.

## Recognition

Contributors will be recognized in:
- Git commit history
- GitHub contributors list
- Release notes (for significant contributions)

Thank you for contributing! 🎉
