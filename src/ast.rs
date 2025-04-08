#[derive(Debug, Clone)]
pub enum UsePath {
    Name(String),
    Path(Vec<String>),
    PathWithAlias(Vec<String>, String),
    StringPath(String, Vec<UsePath>),
}

#[derive(Debug, Clone)]
pub struct UseStmt {
    pub paths: Vec<UsePath>,
}

#[derive(Debug, Clone)]
pub enum AstNode {
    Program(Vec<AstNode>),
    Use(UseStmt),
    StructDef {
        name: String,
        fields: Vec<(String, Option<String>)>,
    },
    ImplBlock {
        name: String,
        methods: Vec<AstNode>,
    },
    FnDef {
        name: String,
        params: Vec<String>,
        ret_type: Option<String>,
        body: Box<AstNode>,
    },
    Closure {
        params: Vec<String>,
        body: Box<AstNode>,
    },
    Block(Vec<AstNode>),
    Let {
        name: String,
        value: Box<AstNode>,
    },
    Assign {
        name: String,
        value: Box<AstNode>,
    },
    Return(Box<AstNode>),
    If {
        condition: Box<AstNode>,
        then_branch: Box<AstNode>,
        else_branch: Option<Box<AstNode>>,
    },
    While {
        condition: Box<AstNode>,
        body: Box<AstNode>,
    },
    BinaryOp {
        op: String,
        left: Box<AstNode>,
        right: Box<AstNode>,
    },
    UnaryOp {
        op: String,
        expr: Box<AstNode>,
    },
    Call {
        function: Box<AstNode>,
        args: Vec<AstNode>,
    },
    Index {
        expr: Box<AstNode>,
        index: Box<AstNode>,
    },
    Field {
        expr: Box<AstNode>,
        field: String,
    },
    Ident(String),
    Number(f64),
    String(String),
    Bool(bool),
    Nil,
}
