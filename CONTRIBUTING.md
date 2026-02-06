# Contributing to Railgun

Thank you for your interest in contributing to Railgun! This document provides guidelines and instructions for contributing.

## Development Setup

### Prerequisites

- Rust 1.75 or later
- Cargo (comes with Rust)

### Building from Source

```bash
# Clone the repository
git clone https://github.com/douglance/railgun.git
cd railgun

# Build
cargo build

# Build release binary
cargo build --release

# Run tests
cargo test

# Run specific crate tests
cargo test -p rg-policy
cargo test -p rg-types
```

## Code Standards

### Linting

All code must pass clippy without warnings:

```bash
cargo clippy --all-targets --all-features
```

### Formatting

All code must be formatted with rustfmt:

```bash
cargo fmt

# Check formatting without modifying
cargo fmt -- --check
```

### Testing

- All new features must include tests
- All bug fixes should include a regression test
- Target: 70% code coverage minimum
- Tests must pass on all supported platforms

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run a specific test
cargo test test_name
```

### Documentation

- Public APIs must have doc comments
- Use `///` for item documentation
- Include examples in doc comments where appropriate

## Pull Request Process

### Before Submitting

1. **Create an issue first** - Discuss significant changes before implementing
2. **Fork the repository** - Work on your own fork
3. **Create a feature branch** - Branch from `main`
4. **Write tests** - Ensure your changes are tested
5. **Update documentation** - Keep docs in sync with code changes

### Submission Checklist

- [ ] Code compiles without errors (`cargo build`)
- [ ] All tests pass (`cargo test`)
- [ ] Clippy passes (`cargo clippy`)
- [ ] Code is formatted (`cargo fmt`)
- [ ] Documentation is updated
- [ ] Commit messages are clear and descriptive

### Review Process

1. Submit your pull request against `main`
2. Ensure CI checks pass
3. Request review from maintainers
4. Address any feedback
5. Once approved, a maintainer will merge

## Project Structure

```
railgun/
├── bin/rg/           # CLI binary
│   └── src/
│       ├── main.rs       # Entry point
│       ├── cli.rs        # Argument parsing
│       ├── hook.rs       # Hook implementation
│       ├── install.rs    # Install/uninstall logic
│       └── lint.rs       # Config validation
├── crates/
│   ├── rg-types/     # Shared types (Config, Verdict, HookInput)
│   └── rg-policy/    # Policy engine (secrets, commands, paths, network)
└── docs/             # Documentation
```

## Commit Messages

Follow conventional commits format:

```
type(scope): description

[optional body]

[optional footer]
```

Types:
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation only
- `test`: Adding tests
- `refactor`: Code change that neither fixes a bug nor adds a feature
- `chore`: Maintenance tasks

Examples:
```
feat(policy): add support for custom secret patterns
fix(hook): handle empty stdin gracefully
docs: update installation instructions
```

## Getting Help

- Open an issue for bugs or feature requests
- Discussions for questions and ideas
- Check existing issues before creating new ones

## License

By contributing, you agree that your contributions will be licensed under the MIT License.
