use crate::evaluator::object::Object;
use crate::parser::ast::Statement;
use crate::parser::ParsingError;

mod object;
mod tests;

pub fn eval(program: &str) -> Result<Object, Vec<ProgramError>> {
    todo!()
}

pub fn eval_statements(statements: Vec<Statement>) -> Result<Object, EvalError> {
    let mut result = Object::Null;

    statements.iter().for_each(|statement| match statement {
        Statement::Let(_, _) => todo!(),
        Statement::Return(_) => todo!(),
        Statement::Expression(exp) => todo!(),
        Statement::BlockStatement(_) => todo!(),
    });

    Ok(result)
}

#[derive(Debug, PartialEq)]
pub enum EvalError {}

#[derive(Debug, PartialEq)]
pub enum ProgramError {
    Parsing(ParsingError),
    Eval(EvalError),
}
