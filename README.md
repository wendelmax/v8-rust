![JetCrab Logo](assets/jetcrab-logo.png)

# JetCrab: JavaScript Engine in Rust

## Overview
JetCrab is a JavaScript engine inspired by Google's V8 architecture, implemented in Rust. The project covers all stages of modern JavaScript execution pipeline: Lexer, Parser, AST, Semantic Analysis, Bytecode Generation and (in the future) Virtual Machine and JIT.

## Project Statistics
- **Total Lines of Code**: 13,704+ lines of Rust
- **Source Files**: 71 Rust files across 9 crates
- **Test Files**: 30+ test files with comprehensive coverage
- **Benchmarks**: Performance benchmarks for lexer operations
- **Integration Tests**: End-to-end pipeline testing

## Architecture

### Core Components
The project is organized into 9 specialized crates, each handling a specific aspect of JavaScript execution:

#### **v8_lexer** - Lexical Analysis
- Complete ECMAScript tokenization with precise position tracking
- Unicode support for international identifiers
- Error handling and recovery mechanisms
- Performance benchmarks for optimization

#### **v8_ast** - Abstract Syntax Tree
- Complete AST implementation compatible with ES2015+
- Serialization support (JSON)
- Visitor pattern for tree traversal
- Source location tracking for debugging

#### **v8_parser** - Syntax Analysis
- Robust parsing with error recovery
- ECMAScript 5/6+ compatibility
- Handles complex JavaScript constructs
- Generates validated AST nodes

#### **v8_semantic** - Semantic Analysis
- Type system and scope management
- ECMAScript validation rules
- Error detection and reporting
- Static analysis capabilities

#### **v8_bytecode** - Code Generation
- **100% complete** bytecode generation from AST
- Instruction set inspired by V8's Ignition
- Constant pool optimization
- Support for all ECMAScript features

#### **v8_vm** - Virtual Machine
- **Phases 1-4 complete**: Functions, closures, and contexts
- Stack-based execution engine
- Register management system
- Heap and memory management

#### **v8_runtime** - Runtime Environment
- Value system (primitives and objects)
- Context management
- Function execution framework
- Object and array operations

#### **v8_gc** - Garbage Collection
- Memory management infrastructure
- Mark-sweep collector framework
- Object tracking system
- Heap management utilities

#### **v8_api** - Public API
- Integration layer for external usage
- Engine initialization and configuration
- Public interfaces for embedding

## Current Status
- **Lexer, AST, Parser, Semantic Analysis**: 100% complete
- **Bytecode**: 100% complete, with total AST coverage and tests for all node types
- **VM**: Phases 1-4 complete (functions, closures and contexts)
- **Integration Tests**: Comprehensive end-to-end testing
- **Performance**: Benchmarked lexer operations

## Technical Features

### ECMAScript Compatibility
- **ES2015+ Support**: Modern JavaScript features including:
  - Template literals and tagged templates
  - Arrow functions and classes
  - Destructuring and spread operators
  - Async/await and generators
  - Modules and imports/exports

### Advanced Capabilities
- **Unicode Support**: Full Unicode identifier support (π, 你好, 🚀)
- **Error Recovery**: Robust error handling and recovery mechanisms
- **Position Tracking**: Precise line/column tracking for debugging
- **Serialization**: AST serialization for tooling integration
- **Visitor Pattern**: Extensible tree traversal for analysis

### Performance Optimizations
- **Constant Pool**: Optimized storage for literals and constants
- **Instruction Set**: Efficient bytecode instruction design
- **Memory Management**: Structured heap and garbage collection
- **Benchmarking**: Performance measurement and optimization

## Usage Examples

### Basic Lexical Analysis
```rust
use v8_lexer::tokenize;

let source = "let x = 42;";
let tokens = tokenize(source).unwrap();
println!("Found {} tokens", tokens.len());
```

### AST Generation and Serialization
```rust
use v8_ast::Node;
use v8_bytecode::generator::BytecodeGenerator;

let ast = Node::Number(42.0);
let mut gen = BytecodeGenerator::new();
gen.generate(&ast);
assert_eq!(gen.instructions.len(), 1);
```

### Virtual Machine Execution
```rust
use v8_vm::{Executor, Bytecode, Instruction};

let bytecode = Bytecode::new(vec![Instruction::PushConst(0)]);
let mut executor = Executor::new();
let result = executor.execute(&bytecode);
```

## Next Steps
- VM implementation for bytecode execution
- Optimizations and profiling
- Integration with JIT and garbage collection

## Immediate Improvements Needed
- **Build Issues Fix**: Resolve missing modules in v8_api crate
- **Code Cleanup**: Fix compiler warnings and unused code
- **End-to-End Integration**: Connect VM with parser and bytecode generator for complete execution
- **Performance Optimization**: Implement JIT compilation and advanced optimizations

## Development

### Prerequisites
- Rust 1.75+ and Cargo
- Standard development tools

### Building
```bash
cargo build --all
```

### Testing
```bash
cargo test --all
```

### Benchmarks
```bash
cargo bench --all
```

## Documentation

Comprehensive documentation is available in the [`docs/`](./docs/) directory:

- **[📋 Implementation Checklists](./docs/checklists/)** - Project status and progress tracking
- **[🔍 ECMAScript Compliance Checklist](./docs/checklists/ecmascript-compliance-checklist.md)** - Complete ECMAScript compliance requirements
- **[🏗️ Architecture Documentation](./docs/architecture/)** - Technical design and architecture
- **[🛠️ Development Guides](./docs/development/)** - Setup, testing, and contribution guides
- **[📚 API Reference](./docs/api/)** - Usage examples and API documentation

### Quick Start
For new contributors, start with the **[Getting Started Guide](./docs/development/getting-started.md)**.

## Contributing
This project follows Rust development best practices:
- Comprehensive test coverage
- Performance benchmarking
- Modular architecture
- Clear documentation

For detailed contribution guidelines, see **[CONTRIBUTING.md](./CONTRIBUTING.md)**.

## License
MIT License - see LICENSE file for details 