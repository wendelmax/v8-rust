# v8_bytecode

This crate is responsible for bytecode generation for the V8-Rust JavaScript engine. It transforms the Abstract Syntax Tree (AST) into a sequence of bytecode instructions, which can later be executed by the virtual machine (VM).

## Purpose
- Convert ECMAScript AST nodes into a compact, efficient bytecode format.
- Serve as the intermediate layer between parsing/semantic analysis and execution.
- Enable future optimizations, JIT compilation, and advanced VM features.

## Main Components
- **generator.rs**: Implements the `BytecodeGenerator` struct, which traverses the AST and emits bytecode instructions.
- **instructions.rs**: Defines the `Instruction` enum (the bytecode instruction set) and the `ConstantPool` for literals and constants.

## Current Status
- Basic bytecode generation is implemented for:
  - Program structure (statements, blocks)
  - Variable and function declarations (partial)
  - Arithmetic and binary expressions (+, -, *, /)
  - Function calls and new expressions
  - Array and object literals
  - Control flow (if, for, while, do-while)
  - Literals (number, string, boolean, null, undefined)
- Many advanced features (classes, async, generators, exception handling, etc.) are marked as TODO or unimplemented.
- The instruction set is extensible and covers most modern JavaScript features, but not all are wired up yet.

## Bytecode Instruction Set (Partial)
```
// Arithmetic
Add, Sub, Mul, Div, Mod, Inc, Dec
// Stack
PushConst(idx), Pop, Dup
// Variables
LoadGlobal(name), StoreGlobal(name), LoadLocal(idx), StoreLocal(idx)
// Control flow
Jump(addr), JumpIfTrue(addr), JumpIfFalse(addr)
// Functions
Call(argc), Return
// Objects/Arrays
NewObject, NewArray(len), SetProperty, GetProperty
// Special
TypeOf, InstanceOf, In, Delete, New
// Literals
PushNull, PushUndefined, PushTrue, PushFalse
```
See `instructions.rs` for the full list.

## Example Usage
```
use v8_bytecode::generator::BytecodeGenerator;
use v8_ast::Node;

let ast: Node = /* parsed AST */;
let mut gen = BytecodeGenerator::new();
gen.generate(&ast);
let bytecode = gen.instructions;
let constants = gen.constants;
```

## Next Steps / TODO
- Implement support for classes, import/export, async/await, generators, and exception handling.
- Improve local/global variable management and scoping.
- Add more tests and validation for complex JavaScript features.
- Integrate with the VM for execution and debugging.

---

> This crate is under active development as part of the bytecode generation milestone for V8-Rust. 