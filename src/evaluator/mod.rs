use crate::evaluator::object::Object;
use crate::parser::ast::{Expression, Infix, Prefix, Program, Statement};

mod object;
mod tests;

pub fn eval(program: Program) -> Result<Object, EvalError> {
    let Program(statements) = program;
    eval_statements(&statements)
}

fn eval_statements(statements: &Vec<Statement>) -> Result<Object, EvalError> {
    let mut result = Object::Null;

    for statement in statements.iter() {
        result = eval_statement(statement)?
    }

    Ok(result)
}

fn eval_statement(statement: &Statement) -> Result<Object, EvalError> {
    Ok(match statement {
        Statement::Let(_, _) => todo!(),
        Statement::Return(_) => todo!(),
        Statement::Expression(exp) => eval_expression(exp)?,
        Statement::BlockStatement(stats) => eval_statements(stats)?,
    })
}

fn eval_expression(expression: &Expression) -> Result<Object, EvalError> {
    let result = match expression {
        Expression::Identifier(_) => todo!(),
        Expression::Integer(int) => Ok(Object::Integer(*int)),
        Expression::Prefix(operator, operand) => eval_prefix_expressions(operator, operand),
        Expression::Infix(left, infix, right) => eval_infix_expression(left, infix, right),
        Expression::Boolean(val) => Ok(Object::Boolean(*val)),
        Expression::If(condition, if_block, else_block) => {
            eval_if_expression(condition, if_block, else_block)
        }
        Expression::Function(_, _) => todo!(),
        Expression::Call(_, _) => todo!(),
    };

    result
}

fn eval_if_expression(
    condition: &Expression,
    if_block: &Statement,
    maybe_else_block: &Option<Box<Statement>>,
) -> Result<Object, EvalError> {
    let condition = eval_expression(condition)?;

    if is_truthy(&condition) {
        Ok(eval_statement(if_block)?)
    } else if let Some(else_block) = maybe_else_block {
        Ok(eval_statement(else_block)?)
    } else {
        Ok(Object::Null)
    }
}

fn is_truthy(object: &Object) -> bool {
    match object {
        Object::Boolean(false) | Object::Integer(0) | Object::Null => false,
        _ => true,
    }
}

fn eval_infix_expression(
    left: &Expression,
    infix: &Infix,
    right: &Expression,
) -> Result<Object, EvalError> {
    let left_object = eval_expression(left)?;
    let right_object = eval_expression(right)?;

    Ok(match (left_object, infix, right_object) {
        (Object::Integer(left_int), _, Object::Integer(right_int)) => {
            eval_integer_infix_expression(left_int, infix, right_int)
        }
        (Object::Boolean(left_bool), Infix::Equal, Object::Boolean(right_bool)) => {
            Object::Boolean(left_bool == right_bool)
        }
        (Object::Boolean(left_bool), Infix::NotEqual, Object::Boolean(right_bool)) => {
            Object::Boolean(left_bool != right_bool)
        }
        (Object::Boolean(_), _, Object::Boolean(_)) => return Err(EvalError::UnknownOperator),
        _ => return Err(EvalError::IncompatibleTypes),
    })
}

fn eval_integer_infix_expression(left: i64, infix: &Infix, right: i64) -> Object {
    match infix {
        Infix::Plus => Object::Integer(left + right),
        Infix::Minus => Object::Integer(left - right),
        Infix::Multiply => Object::Integer(left * right),
        Infix::Divide => Object::Integer(left / right),
        Infix::GreaterThan => Object::Boolean(left > right),
        Infix::LessThan => Object::Boolean(left < right),
        Infix::Equal => Object::Boolean(left == right),
        Infix::NotEqual => Object::Boolean(left != right),
    }
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
        _ => return Err(EvalError::UnknownOperator),
    }
}

fn eval_bang_operator_expression(object: &Object) -> Object {
    // false, Null, and 0 are falsy; everything else is truthy
    match object {
        Object::Null => Object::Boolean(true),
        Object::Integer(int) => Object::Boolean(if *int == 0 { true } else { false }),
        Object::Boolean(val) => Object::Boolean(!val),
    }
}

#[derive(Debug, PartialEq)]
pub enum EvalError {
    IncompatibleTypes,
    UnknownOperator,
}
