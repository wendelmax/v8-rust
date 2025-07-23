//! AST node definitions for V8-Rust JavaScript engine

use serde::{Deserialize, Serialize};

/// Represents a position in the source code
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Position {
    pub line: usize,
    pub column: usize,
}

impl Position {
    pub fn new(line: usize, column: usize) -> Self {
        Self { line, column }
    }
}

/// Represents a span of source code
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Span {
    pub start: Position,
    pub end: Position,
}

impl Span {
    pub fn new(start: Position, end: Position) -> Self {
        Self { start, end }
    }
    
    pub fn from_positions(start_line: usize, start_col: usize, end_line: usize, end_col: usize) -> Self {
        Self {
            start: Position::new(start_line, start_col),
            end: Position::new(end_line, end_col),
        }
    }
}

/// Main AST node enum containing all possible node types
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Node {
    // Program structure
    Program(Program),
    
    // Declarations
    VariableDeclaration(VariableDeclaration),
    FunctionDeclaration(FunctionDeclaration),
    ClassDeclaration(ClassDeclaration),
    ImportDeclaration(ImportDeclaration),
    ExportDeclaration(ExportDeclaration),
    
    // Expressions
    BinaryExpression(BinaryExpression),
    UnaryExpression(UnaryExpression),
    CallExpression(CallExpression),
    NewExpression(NewExpression),
    MemberExpression(MemberExpression),
    AssignmentExpression(AssignmentExpression),
    ConditionalExpression(ConditionalExpression),
    LogicalExpression(LogicalExpression),
    UpdateExpression(UpdateExpression),
    ArrowFunctionExpression(ArrowFunctionExpression),
    FunctionExpression(FunctionExpression),
    ClassExpression(ClassExpression),
    YieldExpression(YieldExpression),
    AwaitExpression(AwaitExpression),
    
    // Statements
    BlockStatement(BlockStatement),
    IfStatement(IfStatement),
    ForStatement(ForStatement),
    WhileStatement(WhileStatement),
    DoWhileStatement(DoWhileStatement),
    SwitchStatement(SwitchStatement),
    TryStatement(TryStatement),
    ThrowStatement(ThrowStatement),
    ReturnStatement(ReturnStatement),
    BreakStatement(BreakStatement),
    ContinueStatement(ContinueStatement),
    LabeledStatement(LabeledStatement),
    WithStatement(WithStatement),
    DebuggerStatement(DebuggerStatement),
    ExpressionStatement(ExpressionStatement),
    
    // Literals
    ArrayLiteral(ArrayLiteral),
    ObjectLiteral(ObjectLiteral),
    TemplateLiteral(TemplateLiteral),
    TaggedTemplateExpression(TaggedTemplateExpression),
    
    // Other
    Property(Property),
    SpreadElement(SpreadElement),
    RestElement(RestElement),
    Super(Super),
    MetaProperty(MetaProperty),
    Identifier(String),
    Number(f64),
    String(String),
    Boolean(bool),
    Null,
    Undefined,
    This,
    RegExp(RegExp),
    BigInt(String),
}

// Program structure
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Program {
    pub body: Vec<Node>,
    pub source_type: String, // "script" or "module"
    pub span: Option<Span>,
}

// Declarations
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VariableDeclaration {
    pub kind: String, // "var", "let", "const"
    pub declarations: Vec<VariableDeclarator>,
    pub span: Option<Span>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VariableDeclarator {
    pub id: Box<Node>,
    pub init: Option<Box<Node>>,
    pub span: Option<Span>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FunctionDeclaration {
    pub id: Option<Box<Node>>,
    pub params: Vec<Node>,
    pub body: Box<Node>,
    pub generator: bool,
    pub r#async: bool,
    pub span: Option<Span>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClassDeclaration {
    pub id: Option<Box<Node>>,
    pub super_class: Option<Box<Node>>,
    pub body: Box<Node>,
    pub span: Option<Span>,
}

// Expressions
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BinaryExpression {
    pub left: Box<Node>,
    pub operator: String,
    pub right: Box<Node>,
    pub span: Option<Span>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UnaryExpression {
    pub operator: String,
    pub argument: Box<Node>,
    pub prefix: bool,
    pub span: Option<Span>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CallExpression {
    pub callee: Box<Node>,
    pub arguments: Vec<Node>,
    pub span: Option<Span>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NewExpression {
    pub callee: Box<Node>,
    pub arguments: Vec<Node>,
    pub span: Option<Span>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MemberExpression {
    pub object: Box<Node>,
    pub property: Box<Node>,
    pub computed: bool,
    pub optional: bool,
    pub span: Option<Span>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AssignmentExpression {
    pub left: Box<Node>,
    pub operator: String,
    pub right: Box<Node>,
    pub span: Option<Span>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConditionalExpression {
    pub test: Box<Node>,
    pub consequent: Box<Node>,
    pub alternate: Box<Node>,
    pub span: Option<Span>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LogicalExpression {
    pub left: Box<Node>,
    pub operator: String,
    pub right: Box<Node>,
    pub span: Option<Span>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UpdateExpression {
    pub operator: String,
    pub argument: Box<Node>,
    pub prefix: bool,
    pub span: Option<Span>,
}

// Statements
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BlockStatement {
    pub body: Vec<Node>,
    pub span: Option<Span>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IfStatement {
    pub test: Box<Node>,
    pub consequent: Box<Node>,
    pub alternate: Option<Box<Node>>,
    pub span: Option<Span>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ForStatement {
    pub init: Option<Box<Node>>,
    pub test: Option<Box<Node>>,
    pub update: Option<Box<Node>>,
    pub body: Box<Node>,
    pub span: Option<Span>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WhileStatement {
    pub test: Box<Node>,
    pub body: Box<Node>,
    pub span: Option<Span>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DoWhileStatement {
    pub body: Box<Node>,
    pub test: Box<Node>,
    pub span: Option<Span>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SwitchStatement {
    pub discriminant: Box<Node>,
    pub cases: Vec<SwitchCase>,
    pub span: Option<Span>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SwitchCase {
    pub test: Option<Box<Node>>,
    pub consequent: Vec<Node>,
    pub span: Option<Span>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TryStatement {
    pub block: Box<Node>,
    pub handler: Option<Box<Node>>,
    pub finalizer: Option<Box<Node>>,
    pub span: Option<Span>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CatchClause {
    pub param: Box<Node>,
    pub body: Box<Node>,
    pub span: Option<Span>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ThrowStatement {
    pub argument: Box<Node>,
    pub span: Option<Span>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReturnStatement {
    pub argument: Option<Box<Node>>,
    pub span: Option<Span>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BreakStatement {
    pub label: Option<Box<Node>>,
    pub span: Option<Span>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContinueStatement {
    pub label: Option<Box<Node>>,
    pub span: Option<Span>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LabeledStatement {
    pub label: Box<Node>,
    pub body: Box<Node>,
    pub span: Option<Span>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WithStatement {
    pub object: Box<Node>,
    pub body: Box<Node>,
    pub span: Option<Span>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DebuggerStatement {
    pub span: Option<Span>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExpressionStatement {
    pub expression: Box<Node>,
    pub span: Option<Span>,
}

// Literals
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ArrayLiteral {
    pub elements: Vec<Option<Node>>,
    pub span: Option<Span>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ObjectLiteral {
    pub properties: Vec<Node>,
    pub span: Option<Span>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Property {
    pub key: Box<Node>,
    pub value: Box<Node>,
    pub kind: String, // "init", "get", "set"
    pub computed: bool,
    pub method: bool,
    pub shorthand: bool,
    pub span: Option<Span>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SpreadElement {
    pub argument: Box<Node>,
    pub span: Option<Span>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RestElement {
    pub argument: Box<Node>,
    pub span: Option<Span>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TemplateLiteral {
    pub quasis: Vec<TemplateElement>,
    pub expressions: Vec<Node>,
    pub span: Option<Span>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TemplateElement {
    pub value: String,
    pub tail: bool,
    pub span: Option<Span>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TaggedTemplateExpression {
    pub tag: Box<Node>,
    pub quasi: Box<Node>,
    pub span: Option<Span>,
}

// Other expressions
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ArrowFunctionExpression {
    pub params: Vec<Node>,
    pub body: Box<Node>,
    pub expression: bool,
    pub r#async: bool,
    pub span: Option<Span>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FunctionExpression {
    pub id: Option<Box<Node>>,
    pub params: Vec<Node>,
    pub body: Box<Node>,
    pub generator: bool,
    pub r#async: bool,
    pub span: Option<Span>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClassExpression {
    pub id: Option<Box<Node>>,
    pub super_class: Option<Box<Node>>,
    pub body: Box<Node>,
    pub span: Option<Span>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Super {
    pub span: Option<Span>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MetaProperty {
    pub meta: Box<Node>,
    pub property: Box<Node>,
    pub span: Option<Span>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct YieldExpression {
    pub argument: Option<Box<Node>>,
    pub delegate: bool,
    pub span: Option<Span>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AwaitExpression {
    pub argument: Box<Node>,
    pub span: Option<Span>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RegExp {
    pub pattern: String,
    pub flags: String,
    pub span: Option<Span>,
}

// Import/Export declarations
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ImportDeclaration {
    pub specifiers: Vec<Node>,
    pub source: Box<Node>,
    pub span: Option<Span>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ImportSpecifier {
    pub local: Box<Node>,
    pub imported: Box<Node>,
    pub span: Option<Span>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ImportDefaultSpecifier {
    pub local: Box<Node>,
    pub span: Option<Span>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ImportNamespaceSpecifier {
    pub local: Box<Node>,
    pub span: Option<Span>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExportDeclaration {
    pub declaration: Option<Box<Node>>,
    pub specifiers: Vec<Node>,
    pub source: Option<Box<Node>>,
    pub default: bool,
    pub span: Option<Span>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExportSpecifier {
    pub local: Box<Node>,
    pub exported: Box<Node>,
    pub span: Option<Span>,
} 