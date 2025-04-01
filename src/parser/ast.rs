pub enum Lit{
    Int(i64),
    Float(f64),
    Str(String),
    Bool(bool),
    List(Vec<Lit>)
}

pub enum Op{
    Add,
    Sub,
    Mul,
    Div,
    Mod
}

pub enum Expr {
    Lit(Lit),
    Var(String),
    Bin(Box<Expr>, Op, Box<Expr>),
    Un(Op, Box<Expr>),
}

pub struct Assign {
    pub name: String,
    pub val: Expr,
}

pub struct FuncDef {
    pub name: String,
    pub args: Vec<String>,
    pub body: Vec<Expr>,
}

pub struct FuncCall {
    pub name: String,
    pub args: Vec<Expr>,
}

pub struct ForLoop {
    pub var: String,
    pub body: Vec<Expr>,
}

pub struct WhileLoop {
    pub cond: Box<Expr>,
    pub body: Vec<Expr>,
}

// TODO : Finish the AST stuff
