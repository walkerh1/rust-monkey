#[derive(Debug, PartialEq)]
pub struct Program(pub Vec<Statement>);

#[derive(Clone, Debug, PartialEq)]
pub enum Statement {
    Let(Expression, Expression),
    Return(Expression),
    Expression(Expression),
    BlockStatement(Vec<Statement>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Expression {
    Identifier(String),
    Integer(i64),
    Prefix(Prefix, Box<Expression>),
    Infix(Box<Expression>, Infix, Box<Expression>),
    Boolean(bool),
    If(Box<Expression>, Box<Statement>, Option<Box<Statement>>),
    Function(Vec<Expression>, Box<Statement>),
    Call(Box<Expression>, Vec<Expression>),
    String(String),
    Array(Vec<Expression>),
    Index(Box<Expression>, Box<Expression>),
    Hash(Vec<(Expression, Expression)>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Prefix {
    Minus,
    Bang,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Infix {
    Plus,
    Minus,
    Multiply,
    Divide,
    GreaterThan,
    LessThan,
    Equal,
    NotEqual,
    And,
    Or,
}
