# Contributing to FileFire

We welcome contributions to FileFire! This document provides guidelines for contributing to the project.

## Getting Started

### Prerequisites

- Rust 1.75 or later
- Flutter 3.0 or later (for Flutter bindings)
- Docker (for cloud development)
- Git

### Development Setup

1. Fork and clone the repository:
```bash
git clone https://github.com/mubarakhammed/fileFire.git
cd fileFire
```

2. Install dependencies:
```bash
make install
```

3. Run tests to ensure everything works:
```bash
make test
```

4. Start the development environment:
```bash
make dev
```

## Project Structure

```
/filefire
├── core/              # Rust core engine
├── plugins/           # Plugin implementations
├── bindings/          # Platform-specific adapters
├── examples/          # Example applications
├── cloud/            # Cloud API and Docker setup
├── docs/             # Documentation
├── tests/            # Integration tests
└── .github/          # CI/CD workflows
```

## Contributing Guidelines

### Code Style

- **Rust**: Follow standard Rust formatting (`cargo fmt`)
- **Dart/Flutter**: Follow Dart style guide (`dart format`)
- **Documentation**: Use clear, concise language with examples

### Commit Messages

Use conventional commits format:
```
type(scope): description

[optional body]

[optional footer]
```

Types:
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Code style changes
- `refactor`: Code refactoring
- `test`: Adding or updating tests
- `chore`: Maintenance tasks

Examples:
```
feat(core): add PDF annotation support
fix(flutter): resolve memory leak in document loading
docs(plugins): add OCR plugin development guide
```

### Pull Request Process

1. **Create a feature branch**: `git checkout -b feature/your-feature-name`
2. **Make your changes**: Follow the coding standards
3. **Add tests**: Ensure new functionality is tested
4. **Update documentation**: Include relevant documentation updates
5. **Run pre-commit checks**: `make pre-commit`
6. **Submit pull request**: Use the provided template

### Testing Requirements

- **Unit tests**: Required for all new functionality
- **Integration tests**: Required for cross-component features
- **Documentation tests**: Examples must be working code
- **Performance tests**: For performance-critical changes

## Development Workflow

### Core Engine Development

1. Navigate to `core/` directory
2. Make changes to Rust code
3. Run tests: `cargo test`
4. Build: `cargo build --release`

### Plugin Development

1. Create new plugin in `plugins/` directory
2. Follow plugin development guide in `docs/plugins.md`
3. Test plugin integration with core
4. Update plugin documentation

### Platform Binding Development

1. Navigate to appropriate `bindings/` directory
2. Update binding code
3. Test with example applications
4. Ensure cross-platform compatibility

### Example Application Development

1. Navigate to `examples/` directory
2. Update example to showcase new features
3. Test on target platforms
4. Update example documentation

## Issue Guidelines

### Bug Reports

Include:
- FileFire version
- Platform and OS version
- Steps to reproduce
- Expected vs actual behavior
- Relevant logs or error messages
- Minimal code example if applicable

### Feature Requests

Include:
- Clear description of the feature
- Use cases and motivation  
- Proposed API or interface
- Implementation considerations
- Breaking change assessment

### Security Issues

**Do not** open public issues for security vulnerabilities. Instead:
1. Email security@filefire.dev
2. Include detailed description
3. Provide proof of concept if safe
4. Allow time for assessment and fix

## Code Review Process

### Review Criteria

- **Functionality**: Does it work as intended?
- **Testing**: Are there adequate tests?
- **Documentation**: Is it properly documented?
- **Performance**: Any performance implications?
- **Security**: Any security considerations?
- **Breaking changes**: Are they necessary and documented?

### Review Timeline

- **Small changes**: 1-2 business days
- **Medium changes**: 3-5 business days  
- **Large changes**: 1-2 weeks

## Release Process

### Version Numbering

We follow Semantic Versioning (SemVer):
- **MAJOR**: Breaking changes
- **MINOR**: New features (backward compatible)
- **PATCH**: Bug fixes (backward compatible)

### Release Checklist

- [ ] All tests pass
- [ ] Documentation updated
- [ ] CHANGELOG.md updated
- [ ] Version numbers updated
- [ ] Release notes prepared
- [ ] Binaries built for all platforms
- [ ] Docker images published

## Communication

### Channels

- **GitHub Issues**: Bug reports and feature requests
- **GitHub Discussions**: General questions and ideas
- **Discord**: Real-time chat (link in README)
- **Email**: security@filefire.dev for security issues

### Code of Conduct

We follow the [Contributor Covenant Code of Conduct](CODE_OF_CONDUCT.md). Please read and follow it in all interactions.

## Recognition

Contributors are recognized in:
- `CONTRIBUTORS.md` file
- Release notes
- Annual contributor report
- Special mentions for significant contributions

## Getting Help

- **Documentation**: Check `docs/` directory
- **Examples**: Look at `examples/` directory
- **Issues**: Search existing GitHub issues
- **Discussions**: Ask in GitHub Discussions
- **Discord**: Join our Discord server

## Development Tips

### Debugging

- Use `RUST_LOG=debug` for verbose logging
- Flutter: Use `flutter logs` for device logs
- Cloud API: Check container logs with `docker logs`

### Performance Profiling

```bash
# Rust profiling
cargo build --release
perf record --call-graph=dwarf target/release/your-binary
perf report

# Flutter profiling
flutter run --profile
# Use Flutter DevTools
```

### Memory Debugging

```bash
# Rust memory debugging
valgrind --tool=memcheck target/debug/your-binary

# Address Sanitizer
RUSTFLAGS="-Z sanitizer=address" cargo run
```

## Platform-Specific Notes

### iOS Development

- Requires Xcode and iOS SDK
- Test on physical devices when possible
- Follow Apple's guidelines for App Store distribution

### Android Development

- Requires Android SDK and NDK
- Test on various Android versions
- Consider Android-specific security requirements

### WebAssembly

- Test in multiple browsers
- Consider WASM limitations and file size
- Ensure CORS compatibility for web deployment

### Desktop Platforms

- Test on Windows, macOS, and Linux
- Consider platform-specific file systems and permissions
- Handle platform-specific UI conventions

Thank you for contributing to FileFire! 🔥