#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AstNode {
    Program(Vec<AstNode>),
    StructDef {
        name: String,
        fields: Vec<(String, Option<String>)>,
    },
    ImplBlock {
        name: String,
        methods: Vec<AstNode>,
    },
    FunctionDef {
        name: String,
        params: Vec<String>,
        body: Vec<AstNode>,
    },
    Closure {
        params: Vec<String>,
        body: Vec<AstNode>,
    },
    LetAssign {
        name: String,
        value: Box<AstNode>,
    },
    BinaryOp {
        op: BinaryOperator,
        left: Box<AstNode>,
        right: Box<AstNode>,
    },
    FnCall {
        function: Box<AstNode>,
        args: Vec<AstNode>,
    },
    MemberAccess {
        object: Box<AstNode>,
        property: String,
    },
    Identifier(String),
    Number(i64),
    String(String),
    Return(Box<AstNode>),
}
pub enum BinaryOperator {
    Add,      // +
    Subtract, // -
    Multiply, // *
    Divide,   // /
    Modulo,   // %

    Equal,          // ==
    NotEqual,       // !=
    LessThan,       // <
    LessThanOrEqual,// <=
    GreaterThan,    // >
    GreaterThanOrEqual, // >=

    LogicalAnd, // &&
    LogicalOr,  // ||
}