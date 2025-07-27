# Contributing to V8-Rust

Thank you for your interest in contributing to V8-Rust! This document provides guidelines and information for contributors to help maintain code quality and project consistency.

## Table of Contents
- [Getting Started](#getting-started)
- [Development Setup](#development-setup)
- [Project Structure](#project-structure)
- [Coding Standards](#coding-standards)
- [Testing Guidelines](#testing-guidelines)
- [Commit Guidelines](#commit-guidelines)
- [Pull Request Process](#pull-request-process)
- [Code Review Process](#code-review-process)
- [Reporting Issues](#reporting-issues)
- [Feature Requests](#feature-requests)

## Getting Started

### Prerequisites
- **Rust**: Version 1.75 or higher
- **Cargo**: Latest stable version
- **Git**: For version control
- **Development Tools**: Standard Rust development environment

### Quick Start
```bash
# Clone the repository
git clone https://github.com/wendelmax/v8-rust.git
cd v8-rust

# Build the project
cargo build --all

# Run all tests
cargo test --all

# Run benchmarks
cargo bench --all
```

## Development Setup

### Workspace Structure
V8-Rust is organized as a Cargo workspace with 9 specialized crates:

```
v8-rust/
├── crates/
│   ├── v8_lexer/      # Lexical analysis
│   ├── v8_ast/        # Abstract Syntax Tree
│   ├── v8_parser/     # Syntax analysis
│   ├── v8_semantic/   # Semantic analysis
│   ├── v8_bytecode/   # Bytecode generation
│   ├── v8_vm/         # Virtual Machine
│   ├── v8_runtime/    # Runtime environment
│   ├── v8_gc/         # Garbage collection
│   └── v8_api/        # Public API
├── tests/             # Integration tests
└── docs/              # Documentation
```

### Development Workflow
1. **Create a feature branch**: `git checkout -b feature/your-feature-name`
2. **Make your changes**: Follow coding standards below
3. **Add tests**: Ensure comprehensive test coverage
4. **Run tests**: `cargo test --all`
5. **Check formatting**: `cargo fmt --all`
6. **Run clippy**: `cargo clippy --all`
7. **Commit changes**: Follow commit guidelines
8. **Submit PR**: Create pull request with detailed description

## Project Structure

### Crate Responsibilities

#### **v8_lexer**
- Tokenization of JavaScript source code
- Unicode support and error handling
- Position tracking and source mapping

#### **v8_ast**
- Abstract Syntax Tree representation
- Serialization and deserialization
- Visitor pattern implementation

#### **v8_parser**
- Syntax analysis and AST generation
- Error recovery and reporting
- ECMAScript compliance

#### **v8_semantic**
- Type checking and scope analysis
- Semantic validation
- Error detection and reporting

#### **v8_bytecode**
- Bytecode generation from AST
- Instruction set definition
- Constant pool optimization

#### **v8_vm**
- Virtual machine execution
- Stack and register management
- Instruction execution

#### **v8_runtime**
- Runtime value system
- Object and function management
- Context and scope handling

#### **v8_gc**
- Garbage collection algorithms
- Memory management
- Object lifecycle tracking

#### **v8_api**
- Public API for embedding
- Engine configuration
- Integration interfaces

## Coding Standards

### Rust Code Style
- Follow [Rust Style Guide](https://doc.rust-lang.org/1.0.0/style/style/naming/README.html)
- Use `cargo fmt` for consistent formatting
- Run `cargo clippy` to catch common issues
- Maximum line length: 100 characters

### Naming Conventions
- **Crates**: Use snake_case (`v8_lexer`, `v8_ast`)
- **Modules**: Use snake_case (`token.rs`, `lexer.rs`)
- **Functions**: Use snake_case (`tokenize`, `parse`)
- **Variables**: Use snake_case (`source_code`, `token_list`)
- **Constants**: Use SCREAMING_SNAKE_CASE (`MAX_TOKENS`, `DEFAULT_BUFFER_SIZE`)
- **Types**: Use PascalCase (`Token`, `Lexer`, `ParseError`)

### Documentation Standards
- **Public APIs**: Must have doc comments (`///`)
- **Modules**: Include module-level documentation (`//!`)
- **Examples**: Provide usage examples in doc comments
- **Error Types**: Document all possible error conditions

### Code Organization
```rust
// 1. Module documentation
//! Module description

// 2. Imports (external first, then internal)
use std::collections::HashMap;
use crate::token::Token;

// 3. Public types and constants
pub struct Lexer {
    // ...
}

// 4. Private types and constants
const DEFAULT_BUFFER_SIZE: usize = 1024;

// 5. Public functions
impl Lexer {
    pub fn new(source: &str) -> Self {
        // ...
    }
}

// 6. Private functions
impl Lexer {
    fn tokenize_number(&mut self) -> Result<Token, LexerError> {
        // ...
    }
}

// 7. Tests (at the bottom of the file)
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_lexer_creation() {
        // ...
    }
}
```

## Testing Guidelines

### Test Structure
- **Unit Tests**: In the same file as the code being tested
- **Integration Tests**: In `tests/` directory for each crate
- **Benchmarks**: In `benches/` directory for performance-critical code

### Test Requirements
- **Coverage**: Aim for 100% test coverage for new code
- **Edge Cases**: Test error conditions and boundary cases
- **Performance**: Include benchmarks for performance-critical code
- **Documentation**: Tests should serve as usage examples

### Test Naming
- **Unit Tests**: `test_function_name_scenario`
- **Integration Tests**: `test_module_integration_scenario`
- **Benchmarks**: `bench_operation_name`

### Example Test Structure
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lexer_tokenizes_simple_identifier() {
        let source = "hello";
        let tokens = tokenize(source).unwrap();
        assert_eq!(tokens.len(), 2); // identifier + EOF
        assert!(matches!(tokens[0].kind, TokenKind::Identifier(_)));
    }

    #[test]
    fn test_lexer_handles_unicode_identifiers() {
        let source = "let π = 3.14;";
        let tokens = tokenize(source).unwrap();
        // Verify Unicode support
    }

    #[test]
    fn test_lexer_reports_errors_appropriately() {
        let source = "\"unterminated string";
        let result = tokenize(source);
        assert!(result.is_err());
    }
}
```

## Commit Guidelines

### Commit Message Format
```
<type>(<scope>): <description>

[optional body]

[optional footer]
```

### Types
- **feat**: New feature
- **fix**: Bug fix
- **docs**: Documentation changes
- **style**: Code style changes (formatting, etc.)
- **refactor**: Code refactoring
- **test**: Adding or updating tests
- **perf**: Performance improvements
- **ci**: CI/CD changes
- **chore**: Maintenance tasks

### Examples
```
feat(lexer): add Unicode identifier support

- Support Unicode identifiers like π, 你好, 🚀
- Add comprehensive test coverage
- Update documentation with examples

Closes #123
```

```
fix(vm): resolve memory leak in function calls

The function call stack wasn't being properly cleaned up,
causing memory leaks in recursive functions.
```

## Pull Request Process

### Before Submitting
1. **Ensure tests pass**: `cargo test --all`
2. **Check formatting**: `cargo fmt --all`
3. **Run clippy**: `cargo clippy --all`
4. **Update documentation**: Add/update relevant docs
5. **Add tests**: Include tests for new functionality

### PR Description Template
```markdown
## Description
Brief description of the changes

## Type of Change
- [ ] Bug fix
- [ ] New feature
- [ ] Breaking change
- [ ] Documentation update

## Testing
- [ ] Unit tests added/updated
- [ ] Integration tests added/updated
- [ ] Benchmarks added/updated
- [ ] All tests pass

## Checklist
- [ ] Code follows project style guidelines
- [ ] Self-review completed
- [ ] Documentation updated
- [ ] No breaking changes (or breaking changes documented)

## Related Issues
Closes #123
```

## Code Review Process

### Review Guidelines
- **Be constructive**: Provide helpful feedback
- **Focus on code**: Avoid personal comments
- **Ask questions**: If something is unclear
- **Suggest improvements**: Offer specific suggestions
- **Check for**: Correctness, performance, security, maintainability

### Review Checklist
- [ ] Code follows project standards
- [ ] Tests are comprehensive
- [ ] Documentation is updated
- [ ] No performance regressions
- [ ] Error handling is appropriate
- [ ] Security considerations addressed

## Reporting Issues

### Bug Reports
When reporting bugs, please include:
- **Description**: Clear description of the issue
- **Reproduction**: Steps to reproduce the problem
- **Expected vs Actual**: What you expected vs what happened
- **Environment**: Rust version, OS, etc.
- **Code Example**: Minimal code to reproduce the issue

### Issue Template
```markdown
## Bug Description
[Clear description of the bug]

## Steps to Reproduce
1. [Step 1]
2. [Step 2]
3. [Step 3]

## Expected Behavior
[What you expected to happen]

## Actual Behavior
[What actually happened]

## Environment
- Rust Version: [e.g., 1.75.0]
- OS: [e.g., Ubuntu 22.04]
- Architecture: [e.g., x86_64]

## Additional Information
[Any other relevant information]
```

## Feature Requests

### Feature Request Guidelines
- **Clear description**: What the feature should do
- **Use case**: Why this feature is needed
- **Implementation ideas**: How it might be implemented
- **Priority**: High/Medium/Low priority

### Feature Request Template
```markdown
## Feature Description
[Clear description of the requested feature]

## Use Case
[Why this feature is needed and how it would be used]

## Proposed Implementation
[Optional: Ideas for how to implement this feature]

## Priority
[High/Medium/Low]

## Additional Information
[Any other relevant information]
```

## Getting Help

### Communication Channels
- **GitHub Issues**: For bug reports and feature requests
- **GitHub Discussions**: For questions and general discussion
- **Pull Requests**: For code contributions

### Resources
- [Rust Book](https://doc.rust-lang.org/book/)
- [Rust Reference](https://doc.rust-lang.org/reference/)
- [ECMAScript Specification](https://tc39.es/ecma262/)
- [V8 Engine Documentation](https://v8.dev/)

## License

By contributing to V8-Rust, you agree that your contributions will be licensed under the MIT License.

---

Thank you for contributing to V8-Rust! Your contributions help make this project better for everyone. 