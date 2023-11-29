use crate::evaluator::object::Object;
use crate::parser::ast::{ParsingError, Statement};
use crate::parser::Parser;

mod object;
mod tests;

pub fn eval(program: &str) -> Result<Object, Vec<ProgramError>> {
    let mut errors = vec![];

    let statements: Vec<_> = program
        .ast_nodes()
        .filter_map(|node| node.map_err(|e| errors.push(ProgramError::Parsing(e))).ok())
        .collect();

    if errors.len() > 0 {
        return Err(errors);
    }

    eval_statements(statements).map_err(|e| {
        errors.push(ProgramError::Eval(e));
        errors
    })
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
