// AST para JavaScript - ECMAScript completo

#[derive(Debug, Clone, PartialEq)]
pub enum Node {
    Program(Program),
    VariableDeclaration(VariableDeclaration),
    FunctionDeclaration(FunctionDeclaration),
    ClassDeclaration(ClassDeclaration),
    ImportDeclaration(ImportDeclaration),
    ExportDeclaration(ExportDeclaration),
    BinaryExpression(BinaryExpression),
    UnaryExpression(UnaryExpression),
    CallExpression(CallExpression),
    NewExpression(NewExpression),
    MemberExpression(MemberExpression),
    AssignmentExpression(AssignmentExpression),
    ConditionalExpression(ConditionalExpression),
    LogicalExpression(LogicalExpression),
    UpdateExpression(UpdateExpression),
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
    CatchClause(CatchClause),
    LabeledStatement(LabeledStatement),
    WithStatement(WithStatement),
    DebuggerStatement(DebuggerStatement),
    ExpressionStatement(ExpressionStatement),
    ArrayLiteral(ArrayLiteral),
    ObjectLiteral(ObjectLiteral),
    Property(Property),
    SpreadElement(SpreadElement),
    RestElement(RestElement),
    TemplateLiteral(TemplateLiteral),
    TaggedTemplateExpression(TaggedTemplateExpression),
    ArrowFunctionExpression(ArrowFunctionExpression),
    FunctionExpression(FunctionExpression),
    ClassExpression(ClassExpression),
    Super(Super),
    MetaProperty(MetaProperty),
    YieldExpression(YieldExpression),
    AwaitExpression(AwaitExpression),
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

#[derive(Debug, Clone, PartialEq)]
pub struct Program {
    pub body: Vec<Node>,
    pub source_type: String, // "script" or "module"
}

#[derive(Debug, Clone, PartialEq)]
pub struct VariableDeclaration {
    pub kind: String, // "var", "let", "const"
    pub declarations: Vec<VariableDeclarator>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct VariableDeclarator {
    pub id: Box<Node>,
    pub init: Option<Box<Node>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionDeclaration {
    pub id: Option<Box<Node>>,
    pub params: Vec<Node>,
    pub body: Box<Node>,
    pub generator: bool,
    pub r#async: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ClassDeclaration {
    pub id: Option<Box<Node>>,
    pub super_class: Option<Box<Node>>,
    pub body: Box<Node>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ClassBody {
    pub body: Vec<Node>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MethodDefinition {
    pub key: Box<Node>,
    pub value: Box<Node>,
    pub kind: String, // "constructor", "method", "get", "set"
    pub computed: bool,
    pub static_: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ImportDeclaration {
    pub specifiers: Vec<Node>,
    pub source: Box<Node>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ImportSpecifier {
    pub local: Box<Node>,
    pub imported: Box<Node>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ImportDefaultSpecifier {
    pub local: Box<Node>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ImportNamespaceSpecifier {
    pub local: Box<Node>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ExportDeclaration {
    pub declaration: Option<Box<Node>>,
    pub specifiers: Vec<Node>,
    pub source: Option<Box<Node>>,
    pub default: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ExportSpecifier {
    pub local: Box<Node>,
    pub exported: Box<Node>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BinaryExpression {
    pub left: Box<Node>,
    pub operator: String,
    pub right: Box<Node>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct UnaryExpression {
    pub operator: String,
    pub argument: Box<Node>,
    pub prefix: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CallExpression {
    pub callee: Box<Node>,
    pub arguments: Vec<Node>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct NewExpression {
    pub callee: Box<Node>,
    pub arguments: Vec<Node>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MemberExpression {
    pub object: Box<Node>,
    pub property: Box<Node>,
    pub computed: bool,
    pub optional: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AssignmentExpression {
    pub left: Box<Node>,
    pub operator: String,
    pub right: Box<Node>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ConditionalExpression {
    pub test: Box<Node>,
    pub consequent: Box<Node>,
    pub alternate: Box<Node>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LogicalExpression {
    pub left: Box<Node>,
    pub operator: String,
    pub right: Box<Node>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct UpdateExpression {
    pub operator: String,
    pub argument: Box<Node>,
    pub prefix: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BlockStatement {
    pub body: Vec<Node>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct IfStatement {
    pub test: Box<Node>,
    pub consequent: Box<Node>,
    pub alternate: Option<Box<Node>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ForStatement {
    pub init: Option<Box<Node>>,
    pub test: Option<Box<Node>>,
    pub update: Option<Box<Node>>,
    pub body: Box<Node>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ForInStatement {
    pub left: Box<Node>,
    pub right: Box<Node>,
    pub body: Box<Node>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ForOfStatement {
    pub left: Box<Node>,
    pub right: Box<Node>,
    pub body: Box<Node>,
    pub r#await: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct WhileStatement {
    pub test: Box<Node>,
    pub body: Box<Node>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DoWhileStatement {
    pub body: Box<Node>,
    pub test: Box<Node>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SwitchStatement {
    pub discriminant: Box<Node>,
    pub cases: Vec<SwitchCase>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SwitchCase {
    pub test: Option<Box<Node>>,
    pub consequent: Vec<Node>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TryStatement {
    pub block: Box<Node>,
    pub handler: Option<Box<Node>>,
    pub finalizer: Option<Box<Node>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CatchClause {
    pub param: Box<Node>,
    pub body: Box<Node>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ThrowStatement {
    pub argument: Box<Node>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ReturnStatement {
    pub argument: Option<Box<Node>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BreakStatement {
    pub label: Option<Box<Node>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ContinueStatement {
    pub label: Option<Box<Node>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LabeledStatement {
    pub label: Box<Node>,
    pub body: Box<Node>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct WithStatement {
    pub object: Box<Node>,
    pub body: Box<Node>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DebuggerStatement {}

#[derive(Debug, Clone, PartialEq)]
pub struct ExpressionStatement {
    pub expression: Box<Node>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ArrayLiteral {
    pub elements: Vec<Option<Node>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ObjectLiteral {
    pub properties: Vec<Node>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Property {
    pub key: Box<Node>,
    pub value: Box<Node>,
    pub kind: String, // "init", "get", "set"
    pub computed: bool,
    pub method: bool,
    pub shorthand: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SpreadElement {
    pub argument: Box<Node>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RestElement {
    pub argument: Box<Node>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TemplateLiteral {
    pub quasis: Vec<TemplateElement>,
    pub expressions: Vec<Node>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TemplateElement {
    pub value: String,
    pub tail: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TaggedTemplateExpression {
    pub tag: Box<Node>,
    pub quasi: Box<Node>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ArrowFunctionExpression {
    pub params: Vec<Node>,
    pub body: Box<Node>,
    pub expression: bool,
    pub r#async: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionExpression {
    pub id: Option<Box<Node>>,
    pub params: Vec<Node>,
    pub body: Box<Node>,
    pub generator: bool,
    pub r#async: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ClassExpression {
    pub id: Option<Box<Node>>,
    pub super_class: Option<Box<Node>>,
    pub body: Box<Node>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Super {}

#[derive(Debug, Clone, PartialEq)]
pub struct MetaProperty {
    pub meta: Box<Node>,
    pub property: Box<Node>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct YieldExpression {
    pub argument: Option<Box<Node>>,
    pub delegate: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AwaitExpression {
    pub argument: Box<Node>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RegExp {
    pub pattern: String,
    pub flags: String,
} 