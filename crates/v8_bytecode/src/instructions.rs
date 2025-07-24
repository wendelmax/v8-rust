//! Bytecode instructions definition

/// Enum representing all possible bytecode instructions
#[derive(Debug, Clone, PartialEq)]
pub enum Instruction {
    // Stack operations
    PushConst(usize), // index in constant pool
    Pop,
    Dup,
    // Arithmetic
    Add, Sub, Mul, Div, Mod, Inc, Dec,
    // Logical
    And, Or, Not, Xor,
    // Comparison
    Eq, Ne, Lt, Gt, Le, Ge, StrictEq, StrictNe,
    // Variables
    LoadGlobal(String), StoreGlobal(String),
    LoadLocal(usize), StoreLocal(usize),
    // Control flow
    Jump(usize), JumpIfTrue(usize), JumpIfFalse(usize),
    // Functions
    Call(usize), Return,
    // Objects/Arrays
    NewObject, NewArray(usize), SetProperty, GetProperty,
    // Special
    TypeOf, InstanceOf, In, Delete, New,
    // Classes/Prototypes
    NewClass, GetPrototype, SetPrototype,
    // Async/Generators
    Await, Yield,
    // Exception handling
    Throw, Try(usize, usize), Catch, Finally,
    // Modern JS
    Spread, Destructure, OptionalChain, NullishCoalesce,
    // Literals
    PushNull, PushUndefined, PushTrue, PushFalse, PushSymbol(usize), PushBigInt(usize),
}

/// Pool of constants used by the bytecode
#[derive(Debug, Default)]
pub struct ConstantPool {
    pub values: Vec<Constant>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Constant {
    Number(f64),
    String(String),
    Boolean(bool),
    Symbol(String),
    BigInt(String),
    // ... outros tipos conforme necessário
} 