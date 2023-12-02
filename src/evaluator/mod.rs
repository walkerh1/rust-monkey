use crate::evaluator::environment::Environment;
use crate::evaluator::object::{Function, Object};
use crate::parser::ast::{Expression, Infix, Prefix, Program, Statement};
use std::cell::RefCell;
use std::rc::Rc;

pub mod environment;
pub mod object;
mod tests;

pub fn eval(program: Program, env: Rc<RefCell<Environment>>) -> Result<Rc<Object>, EvalError> {
    let Program(statements) = program;
    eval_statements(&statements, env)
}

fn eval_statements(
    statements: &[Statement],
    env: Rc<RefCell<Environment>>,
) -> Result<Rc<Object>, EvalError> {
    let mut result = Rc::new(Object::Null);

    for statement in statements.iter() {
        result = eval_statement(statement, Rc::clone(&env))?;
        if let Object::Return(object) = &*result {
            result = Rc::clone(object);
            break;
        }
    }

    Ok(result)
}

fn eval_statement(
    statement: &Statement,
    env: Rc<RefCell<Environment>>,
) -> Result<Rc<Object>, EvalError> {
    Ok(match statement {
        Statement::Let(id, val) => {
            eval_let_statement(id, val, env)?;
            Rc::new(Object::Null)
        }
        Statement::Return(exp) => Rc::new(Object::Return(Rc::clone(&eval_expression(exp, env)?))),
        Statement::Expression(exp) => eval_expression(exp, env)?,
        Statement::BlockStatement(statements) => eval_block_statement(statements, env)?,
    })
}

fn eval_let_statement(
    id: &Expression,
    val: &Expression,
    env: Rc<RefCell<Environment>>,
) -> Result<(), EvalError> {
    if let Expression::Identifier(key) = id {
        let value = eval_expression(val, Rc::clone(&env))?;
        env.borrow_mut().set(key, value);
    }
    Ok(())
}

fn eval_block_statement(
    statements: &[Statement],
    env: Rc<RefCell<Environment>>,
) -> Result<Rc<Object>, EvalError> {
    let mut result = Rc::new(Object::Null);

    for statement in statements.iter() {
        result = eval_statement(statement, Rc::clone(&env))?;
        if let Object::Return(_) = *result {
            break;
        }
    }

    Ok(result)
}

fn eval_expression(
    expression: &Expression,
    env: Rc<RefCell<Environment>>,
) -> Result<Rc<Object>, EvalError> {
    match expression {
        Expression::Identifier(id) => eval_identifier_expression(id, env),
        Expression::Integer(int) => Ok(Rc::new(Object::Integer(*int))),
        Expression::Prefix(operator, operand) => eval_prefix_expressions(operator, operand, env),
        Expression::Infix(left, infix, right) => eval_infix_expression(left, infix, right, env),
        Expression::Boolean(val) => Ok(Rc::new(Object::Boolean(*val))),
        Expression::If(condition, if_block, else_block) => {
            eval_if_expression(condition, if_block, else_block, env)
        }
        Expression::Function(parameters, body) => {
            eval_function_definition_expression(parameters, body, env)
        }
        Expression::Call(func, args) => eval_function_call_expression(func, args, env),
    }
}

fn eval_function_call_expression(
    func: &Expression,
    args: &[Expression],
    env: Rc<RefCell<Environment>>,
) -> Result<Rc<Object>, EvalError> {
    let function = eval_expression(func, Rc::clone(&env))?;
    let arguments: Vec<Rc<Object>> = args
        .iter()
        .map(|exp| eval_expression(exp, Rc::clone(&env)))
        .collect::<Result<Vec<Rc<Object>>, EvalError>>()?;

    apply_function(function, &arguments)
}

fn apply_function(func: Rc<Object>, args: &[Rc<Object>]) -> Result<Rc<Object>, EvalError> {
    if let Object::Function(function) = &*func {
        let extended_env = Environment::new_enclosed(Rc::clone(&function.env));
        function
            .parameters
            .iter()
            .zip(args.iter())
            .for_each(|(p, a)| extended_env.borrow_mut().set(p, Rc::clone(a)));

        let mut result = eval_statement(&function.body, extended_env)?;

        if let Object::Return(object) = &*result {
            result = Rc::clone(object);
        }

        Ok(result)
    } else {
        Err(EvalError::NotAFunction)
    }
}

fn eval_function_definition_expression(
    parameters: &[Expression],
    body: &Statement,
    env: Rc<RefCell<Environment>>,
) -> Result<Rc<Object>, EvalError> {
    let mut params = vec![];
    parameters.iter().for_each(|exp| {
        if let Expression::Identifier(id) = exp {
            params.push(id.to_string());
        }
    });

    Ok(Rc::new(Object::Function(Function {
        parameters: params,
        body: body.clone(),
        env,
    })))
}

fn eval_identifier_expression(
    id: &str,
    env: Rc<RefCell<Environment>>,
) -> Result<Rc<Object>, EvalError> {
    match env.borrow().get(id) {
        Some(object) => Ok(object),
        None => Err(EvalError::UnrecognisedVariable),
    }
}

fn eval_if_expression(
    condition: &Expression,
    if_block: &Statement,
    maybe_else_block: &Option<Box<Statement>>,
    env: Rc<RefCell<Environment>>,
) -> Result<Rc<Object>, EvalError> {
    let condition = eval_expression(condition, Rc::clone(&env))?;

    if is_truthy(&condition) {
        eval_statement(if_block, Rc::clone(&env))
    } else if let Some(else_block) = maybe_else_block {
        eval_statement(else_block, Rc::clone(&env))
    } else {
        Ok(Rc::new(Object::Null))
    }
}

fn is_truthy(object: &Object) -> bool {
    !matches!(
        object,
        Object::Boolean(false) | Object::Integer(0) | Object::Null
    )
}

fn eval_infix_expression(
    left: &Expression,
    infix: &Infix,
    right: &Expression,
    env: Rc<RefCell<Environment>>,
) -> Result<Rc<Object>, EvalError> {
    let left_object = eval_expression(left, Rc::clone(&env))?;
    let right_object = eval_expression(right, Rc::clone(&env))?;

    Ok(match (&*left_object, infix, &*right_object) {
        (Object::Integer(left_int), _, Object::Integer(right_int)) => {
            eval_integer_infix_expression(*left_int, infix, *right_int)
        }
        (Object::Boolean(left_bool), Infix::Equal, Object::Boolean(right_bool)) => {
            Rc::new(Object::Boolean(left_bool == right_bool))
        }
        (Object::Boolean(left_bool), Infix::NotEqual, Object::Boolean(right_bool)) => {
            Rc::new(Object::Boolean(left_bool != right_bool))
        }
        (Object::Boolean(_), _, Object::Boolean(_)) => return Err(EvalError::UnknownOperator),
        _ => return Err(EvalError::IncompatibleTypes),
    })
}

fn eval_integer_infix_expression(left: i64, infix: &Infix, right: i64) -> Rc<Object> {
    let result = match infix {
        Infix::Plus => Object::Integer(left + right),
        Infix::Minus => Object::Integer(left - right),
        Infix::Multiply => Object::Integer(left * right),
        Infix::Divide => Object::Integer(left / right),
        Infix::GreaterThan => Object::Boolean(left > right),
        Infix::LessThan => Object::Boolean(left < right),
        Infix::Equal => Object::Boolean(left == right),
        Infix::NotEqual => Object::Boolean(left != right),
    };

    Rc::new(result)
}

fn eval_prefix_expressions(
    operator: &Prefix,
    operand: &Expression,
    env: Rc<RefCell<Environment>>,
) -> Result<Rc<Object>, EvalError> {
    let right = eval_expression(operand, env)?;
    match operator {
        Prefix::Minus => eval_minus_operator_expression(&right),
        Prefix::Bang => Ok(eval_bang_operator_expression(&right)),
    }
}

fn eval_minus_operator_expression(object: &Object) -> Result<Rc<Object>, EvalError> {
    match object {
        Object::Integer(int) => Ok(Rc::new(Object::Integer(-int))),
        _ => Err(EvalError::UnknownOperator),
    }
}

fn eval_bang_operator_expression(object: &Object) -> Rc<Object> {
    // false, Null, and 0 are falsy; everything else is truthy
    let result = match object {
        Object::Null => true,
        Object::Integer(int) => *int == 0,
        Object::Boolean(val) => !val,
        _ => false,
    };

    Rc::new(Object::Boolean(result))
}

#[derive(Debug, PartialEq)]
pub enum EvalError {
    IncompatibleTypes,
    UnknownOperator,
    UnrecognisedVariable,
    NotAFunction,
}
