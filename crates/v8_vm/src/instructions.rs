//! Instruction set for the V8-Rust VM

#[derive(Debug, Clone, PartialEq)]
pub enum Instruction {
    // Stack operations
    PushConst(usize),
    Pop,
    Dup,
    // Arithmetic
    Add, Sub, Mul, Div, Mod, Inc, Dec,
    // Logical
    And, Or, Not, Xor,
    // Comparison
    Eq, Ne, Lt, Gt, Le, Ge, StrictEq, StrictNe,
    // Variables
    LoadGlobal(usize), StoreGlobal(usize),
    LoadLocal(usize), StoreLocal(usize),
    LoadArg(usize), // Nova instrução para acessar argumentos da função
    LoadThisFunction, // Nova instrução para acessar a função atual (útil para recursão)
    LoadThis, // Nova instrução para acessar o valor de this
    LoadClosureVar(String), // Nova instrução para acessar variáveis de closure
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
    CallFunction(usize, usize), // (handle, argc) - chama função por handle direto
} 