![JetCrab Logo](../assets/jetcrab-logo.png)

# JetCrab Documentation

Welcome to the JetCrab documentation! This directory contains comprehensive documentation for the JavaScript engine implementation.

## Documentation Structure

### 📋 [Checklists](./checklists/)
Project implementation status and progress tracking:
- **[Main Checklist](./checklists/main-checklist.md)** - Overall project status and milestones
- **[VM Checklist](./checklists/vm-checklist.md)** - Virtual Machine implementation progress
- **[GC Checklist](./checklists/gc-checklist.md)** - Garbage Collection implementation status

### 🏗️ [Architecture](./architecture/)
Technical architecture and design documents:
- **[Engine Overview](./architecture/engine-overview.md)** - High-level architecture
- **[Crate Architecture](./architecture/crate-architecture.md)** - Individual crate responsibilities
- **[Data Flow](./architecture/data-flow.md)** - How data flows through the engine
- **[Memory Management](./architecture/memory-management.md)** - Heap and GC design

### 🛠️ [Development](./development/)
Development guides and technical information:
- **[Getting Started](./development/getting-started.md)** - Setup and first steps
- **[Testing Guide](./development/testing-guide.md)** - Testing strategies and examples
- **[Performance Guide](./development/performance-guide.md)** - Optimization and benchmarking
- **[Debugging Guide](./development/debugging-guide.md)** - Debugging techniques and tools

### 📚 [API Reference](./api/)
API documentation and usage examples:
- **[Lexer API](./api/lexer-api.md)** - Tokenization and lexical analysis
- **[Parser API](./api/parser-api.md)** - Syntax analysis and AST generation
- **[VM API](./api/vm-api.md)** - Virtual machine and execution
- **[Runtime API](./api/runtime-api.md)** - Runtime environment and values

## Quick Navigation

### For New Contributors
1. Start with [Getting Started](./development/getting-started.md)
2. Review [Engine Overview](./architecture/engine-overview.md)
3. Check [Main Checklist](./checklists/main-checklist.md) for current status

### For Developers
1. [Crate Architecture](./architecture/crate-architecture.md) for understanding the codebase
2. [Testing Guide](./development/testing-guide.md) for contribution guidelines
3. [API Reference](./api/) for specific functionality

### For Maintainers
1. [Main Checklist](./checklists/main-checklist.md) for project status
2. [VM Checklist](./checklists/vm-checklist.md) and [GC Checklist](./checklists/gc-checklist.md) for implementation progress
3. [Performance Guide](./development/performance-guide.md) for optimization

## Documentation Standards

- **Accuracy**: All documentation should reflect the current state of the codebase
- **Examples**: Include practical code examples where appropriate
- **Cross-references**: Link between related documentation sections
- **Updates**: Documentation should be updated alongside code changes

## Contributing to Documentation

When contributing to documentation:

1. **Follow the structure**: Place new documentation in the appropriate directory
2. **Use clear language**: Write in clear, professional English
3. **Include examples**: Provide practical code examples
4. **Update indexes**: Update this README when adding new documentation
5. **Cross-reference**: Link to related documentation sections

## External Resources

- **[Project README](../README.md)** - Project overview and quick start
- **[CONTRIBUTING.md](../CONTRIBUTING.md)** - Contribution guidelines
- **[GitHub Repository](https://github.com/wendelmax/jetcrab)** - Source code and issues
- **[ECMAScript Specification](https://tc39.es/ecma262/)** - JavaScript language reference
- **[V8 Engine Documentation](https://v8.dev/)** - V8 engine reference

---

*This documentation is maintained alongside the JetCrab codebase. For questions or suggestions, please open an issue on GitHub.* 