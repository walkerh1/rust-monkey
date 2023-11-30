use crate::evaluator::object::Object;
use crate::parser::ast::{Expression, Prefix, Program, Statement};

mod object;
mod tests;

pub fn eval(program: Program) -> Result<Object, EvalError> {
    let Program(statements) = program;
    eval_statements(statements)
}

fn eval_statements(statements: Vec<Statement>) -> Result<Object, EvalError> {
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

fn eval_expression(expression: &Expression) -> Result<Object, EvalError> {
    let result = match expression {
        Expression::Identifier(_) => todo!(),
        Expression::Integer(int) => Object::Integer(*int),
        Expression::Prefix(operator, operand) => eval_prefix_expressions(operator, operand)?,
        Expression::Infix(_, _, _) => todo!(),
        Expression::Boolean(val) => Object::Boolean(*val),
        Expression::If(_, _, _) => todo!(),
        Expression::Function(_, _) => todo!(),
        Expression::Call(_, _) => todo!(),
    };

    Ok(result)
}

fn eval_prefix_expressions(operator: &Prefix, operand: &Expression) -> Result<Object, EvalError> {
    let right = eval_expression(operand)?;
    match operator {
        Prefix::Minus => eval_minus_operator_expression(&right),
        Prefix::Bang => Ok(eval_bang_operator_expression(&right)),
    }
}

fn eval_minus_operator_expression(object: &Object) -> Result<Object, EvalError> {
    match object {
        Object::Integer(int) => Ok(Object::Integer(-int)),
        _ => return Err(EvalError::ExpectedInteger),
    }
}

fn eval_bang_operator_expression(object: &Object) -> Object {
    match object {
        Object::Null => Object::Boolean(true),
        Object::Integer(int) => Object::Boolean(if *int == 0 { true } else { false }),
        Object::Boolean(val) => Object::Boolean(!val),
    }
}

#[derive(Debug, PartialEq)]
pub enum EvalError {
    ExpectedInteger,
}
