pub enum Statement {
    Let(Expression, Expression),
    Return(Expression),
    If(Expression),
}

pub enum Expression {
    Identifier(String),
}

struct Program {
    statements: Vec<Statement>,
}
