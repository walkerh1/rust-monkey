#![cfg(test)]

use crate::parser::{
    ast::{Expression, Prefix, Statement},
    Parser,
};

use super::ast::{Infix, ParsingError};

fn collect_parsing_results(input: &str) -> (Vec<Statement>, Vec<ParsingError>) {
    let mut errors = vec![];
    let ast_nodes: Vec<_> = input
        .ast_nodes()
        .filter_map(|node| node.map_err(|e| errors.push(e)).ok())
        .collect();
    (ast_nodes, errors)
}

#[test]
fn test_let_statement() {
    let input = "let x = 5;";
    let expected: Vec<_> = vec![Statement::Let(
        Expression::Identifier(String::from("x")),
        Expression::Integer(5),
    )];
    let (ast_nodes, errors) = collect_parsing_results(input);
    assert_eq!(ast_nodes, expected);
    assert_eq!(errors.len(), 0);
}

#[test]
fn test_let_parse_error_if_no_identifier() {
    let input = "let = 5;";
    let expected_errors = vec![ParsingError(String::from(
        "Expected next token to be 'IDENT', got '=' instead",
    ))];
    let (ast_nodes, errors) = collect_parsing_results(input);
    assert_eq!(ast_nodes.len(), 0);
    assert_eq!(errors, expected_errors);
}

#[test]
fn test_let_parse_error_if_no_assign() {
    let input = "let x 5;";
    let expected_errors = vec![ParsingError(String::from(
        "Expected next token to be '=', got '5' instead",
    ))];
    let (ast_nodes, errors) = collect_parsing_results(input);
    assert_eq!(ast_nodes.len(), 0);
    assert_eq!(errors, expected_errors);
}

#[test]
fn test_let_statement_parse_error_if_no_semicolon() {
    let input = "let x = 5";
    let expected_errors = vec![ParsingError(String::from(
        "Expected next token to be ';', got 'EOF' instead",
    ))];
    let (ast_nodes, errors) = collect_parsing_results(input);
    assert_eq!(ast_nodes.len(), 0);
    assert_eq!(errors, expected_errors);
}

#[test]
fn test_let_statement_parse_error_if_no_rhs_expression() {
    let input = "let x =;
let y =";
    let expected_errors = vec![
        ParsingError(String::from("Expected expression, got ';' instead")),
        ParsingError(String::from("Expected expression, got 'EOF' instead")),
    ];
    let (ast_nodes, errors) = collect_parsing_results(input);
    assert_eq!(ast_nodes.len(), 0);
    assert_eq!(errors, expected_errors);
}

#[test]
fn test_parser_returns_multiple_errors_with_parsed_nodes() {
    let input = "let;
let x =;
let x = 10;
let y 3;";
    let expected_errors = vec![
        ParsingError(String::from(
            "Expected next token to be 'IDENT', got ';' instead",
        )),
        ParsingError(String::from("Expected expression, got ';' instead")),
        ParsingError(String::from(
            "Expected next token to be '=', got '3' instead",
        )),
    ];
    let (ast_nodes, errors) = collect_parsing_results(input);
    assert_eq!(ast_nodes.len(), 1);
    assert_eq!(errors, expected_errors);
}

#[test]
fn test_return_statement() {
    let input = "return 10;";
    let expected = vec![Statement::Return(Expression::Integer(10))];
    let (ast_nodes, errors) = collect_parsing_results(input);
    assert_eq!(errors.len(), 0);
    assert_eq!(ast_nodes, expected);
}

#[test]
fn test_return_statement_parse_error_if_no_expression() {
    let input = "return ;";
    let expected_errors = vec![ParsingError(String::from(
        "Expected expression, got ';' instead",
    ))];
    let (ast_nodes, errors) = collect_parsing_results(input);
    assert_eq!(ast_nodes.len(), 0);
    assert_eq!(errors, expected_errors);
}

#[test]
fn test_return_statement_parse_error_if_no_semicolon() {
    let input = "return 10
let x = 5;
";
    let expected_errors = vec![ParsingError(String::from(
        "Expected next token to be ';', got 'let' instead",
    ))];
    let (ast_nodes, errors) = collect_parsing_results(input);
    assert_eq!(ast_nodes.len(), 0);
    assert_eq!(errors, expected_errors);
}

#[test]
fn test_parses_multiple_statements() {
    let input = "let x = 5;
return 10;
";
    let expected = vec![
        Statement::Let(
            Expression::Identifier(String::from("x")),
            Expression::Integer(5),
        ),
        Statement::Return(Expression::Integer(10)),
    ];
    let (ast_nodes, errors) = collect_parsing_results(input);
    assert_eq!(errors.len(), 0);
    assert_eq!(ast_nodes, expected);
}

#[test]
fn test_identifier_expression_statement() {
    let input = "foo;
return 10;
";
    let expected = vec![
        Statement::Expression(Expression::Identifier(String::from("foo"))),
        Statement::Return(Expression::Integer(10)),
    ];
    let (ast_nodes, errors) = collect_parsing_results(input);
    assert_eq!(ast_nodes, expected);
    assert_eq!(errors.len(), 0);
}

#[test]
fn test_expression_statement_parses_without_semicolon() {
    let input = "return 10;
foo
";
    let expected = vec![
        Statement::Return(Expression::Integer(10)),
        Statement::Expression(Expression::Identifier(String::from("foo"))),
    ];
    let (ast_nodes, errors) = collect_parsing_results(input);
    println!("{errors:?}");
    assert_eq!(ast_nodes, expected);
    assert_eq!(errors.len(), 0);
}

#[test]
fn test_integer_expression_statement() {
    let input = "5;";
    let expected = vec![Statement::Expression(Expression::Integer(5))];
    let (ast_nodes, errors) = collect_parsing_results(input);
    assert_eq!(ast_nodes, expected);
    assert_eq!(errors.len(), 0);
}

#[test]
fn test_parsing_bang_prefix_expressions() {
    let input = "let x = !5;";
    let expected = vec![Statement::Let(
        Expression::Identifier(String::from("x")),
        Expression::Prefix(Prefix::Bang, Box::new(Expression::Integer(5))),
    )];
    let (ast_nodes, errors) = collect_parsing_results(input);
    assert_eq!(ast_nodes, expected);
    assert_eq!(errors.len(), 0);
}

#[test]
fn test_prefix_expressions() {
    let input = "let x = -5;
let y = !10;
";
    let expected = vec![
        Statement::Let(
            Expression::Identifier(String::from("x")),
            Expression::Prefix(Prefix::Minus, Box::new(Expression::Integer(5))),
        ),
        Statement::Let(
            Expression::Identifier(String::from("y")),
            Expression::Prefix(Prefix::Bang, Box::new(Expression::Integer(10))),
        ),
    ];
    let (ast_nodes, errors) = collect_parsing_results(input);
    assert_eq!(ast_nodes, expected);
    assert_eq!(errors.len(), 0);
}

#[test]
fn test_prefix_expressions_error_if_no_right_expression() {
    let input = "
!;
-";
    let expected_errors = vec![
        ParsingError(String::from(
            "Expected right expression for '!', got ';' instead",
        )),
        ParsingError(String::from(
            "Expected right expression for '-', got 'EOF' instead",
        )),
    ];
    let (ast_nodes, errors) = collect_parsing_results(input);
    assert_eq!(errors, expected_errors);
    assert_eq!(ast_nodes.len(), 0);
}

#[test]
fn test_infix_expressions() {
    let input = "5 + 5;
5 - 5;
5 * 5;
5 / 5;
5 > 5;
5 < 5;
5 == 5;
5 != 5;
";
    let expected = vec![
        Statement::Expression(Expression::Infix(
            Box::new(Expression::Integer(5)),
            Infix::Plus,
            Box::new(Expression::Integer(5)),
        )),
        Statement::Expression(Expression::Infix(
            Box::new(Expression::Integer(5)),
            Infix::Minus,
            Box::new(Expression::Integer(5)),
        )),
        Statement::Expression(Expression::Infix(
            Box::new(Expression::Integer(5)),
            Infix::Multiply,
            Box::new(Expression::Integer(5)),
        )),
        Statement::Expression(Expression::Infix(
            Box::new(Expression::Integer(5)),
            Infix::Divide,
            Box::new(Expression::Integer(5)),
        )),
        Statement::Expression(Expression::Infix(
            Box::new(Expression::Integer(5)),
            Infix::GreaterThan,
            Box::new(Expression::Integer(5)),
        )),
        Statement::Expression(Expression::Infix(
            Box::new(Expression::Integer(5)),
            Infix::LessThan,
            Box::new(Expression::Integer(5)),
        )),
        Statement::Expression(Expression::Infix(
            Box::new(Expression::Integer(5)),
            Infix::Equal,
            Box::new(Expression::Integer(5)),
        )),
        Statement::Expression(Expression::Infix(
            Box::new(Expression::Integer(5)),
            Infix::NotEqual,
            Box::new(Expression::Integer(5)),
        )),
    ];
    let (ast_nodes, errors) = collect_parsing_results(input);
    println!("{errors:?}");
    assert_eq!(ast_nodes, expected);
}
