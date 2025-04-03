#[derive(Debug, PartialEq)]
pub enum Lit{
    Int(i64),
    Float(f64),
    Str(String),
    Bool(bool),
    List(Vec<Lit>),
    Range(Box<Lit>, Box<Lit>)
}

#[derive(Debug, PartialEq)]
pub enum UnOp{
    Not
}

#[derive(Debug, PartialEq)]
pub enum BinOp{
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Eq,
    Gt,
    Lt,
    Ne,
    Gte,
    Lte,
    And,
    Or
}

#[derive(Debug, PartialEq)]
pub enum Expr {
    Lit(Lit),
    Var(String),
    Bin(Box<Expr>, BinOp, Box<Expr>),
    Un(UnOp, Box<Expr>),
}

#[derive(Debug, PartialEq)]
pub struct Assign {
    pub name: String,
    pub val: Expr,
}

#[derive(Debug, PartialEq)]
pub struct CreateVar {
    pub name: String,
    pub wtype: String,
}

#[derive(Debug, PartialEq)]
pub struct FuncDef {
    pub name: String,
    pub args: Vec<Expr>,
    pub body: Vec<Stmt>,
}

#[derive(Debug, PartialEq)]
pub struct FuncCall {
    pub name: String,
    pub args: Vec<Expr>,
    pub return_to: Option<Expr>,
}

#[derive(Debug, PartialEq)]
pub struct ForLoop {
    pub var: String,
    pub body: Vec<Stmt>,
}

#[derive(Debug, PartialEq)]
pub struct WhileLoop {
    pub cond: Box<Expr>,
    pub body: Vec<Stmt>,
}

#[derive(Debug, PartialEq)]
pub struct IfStmt {
    pub cond: Box<Expr>,
    pub body: Vec<Stmt>,
    pub else_body: Option<Vec<Stmt>>,
}

#[derive(Debug, PartialEq)]
pub enum IOOp {
    Echo(Vec<Expr>),
    Read(String)
}

#[derive(Debug, PartialEq)]
pub enum FileOp {
    Cat(Vec<Expr>),
    Cp(Vec<Expr>, Vec<Expr>),
    Mv(Vec<Expr>, Vec<Expr>),
    Append(Vec<Expr>, Vec<Expr>),
    Overwrite(Vec<Expr>, Vec<Expr>)
}

#[derive(Debug, PartialEq)]
pub enum Stmt {
    Assign(Assign),
    CreateVar(CreateVar),
    FuncDef(FuncDef),
    FuncCall(FuncCall),
    ForLoop(ForLoop),
    WhileLoop(WhileLoop),
    IfStmt(IfStmt),
    IO(IOOp),
    FileOp(FileOp),
    Convert(String, String, Box<Expr>),
    Return(Box<Expr>),
    ShellCommand(String)
}
