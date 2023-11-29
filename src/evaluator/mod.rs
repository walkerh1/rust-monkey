use crate::evaluator::object::Object;
use crate::parser::ast::{Expression, Program, Statement};

mod object;
mod tests;

pub fn eval(program: Program) -> Result<Object, EvalError> {
    let Program(statements) = program;
    eval_statements(statements)
}

pub fn eval_statements(statements: Vec<Statement>) -> Result<Object, EvalError> {
    let mut result = Object::Null;

    for statement in statements.iter() {
        result = match statement {
            Statement::Let(_, _) => todo!(),
            Statement::Return(_) => todo!(),
            Statement::Expression(exp) => eval_expression(exp)?,
            Statement::BlockStatement(_) => todo!(),
        }
    }

    Ok(result)
}

pub fn eval_expression(expression: &Expression) -> Result<Object, EvalError> {
    let result = match expression {
        Expression::Identifier(_) => todo!(),
        Expression::Integer(int) => Object::Integer(*int),
        Expression::Prefix(_, _) => todo!(),
        Expression::Infix(_, _, _) => todo!(),
        Expression::Boolean(_) => todo!(),
        Expression::If(_, _, _) => todo!(),
        Expression::Function(_, _) => todo!(),
        Expression::Call(_, _) => todo!(),
    };

    Ok(result)
}

#[derive(Debug, PartialEq)]
pub enum EvalError {}
