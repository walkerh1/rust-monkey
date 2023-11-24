#![cfg(test)]

use crate::parser::{
    ast::{Expression, Statement},
    Parser,
};

use super::ast::ParsingError;

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
fn test_statement_parse_error_if_no_semicolon() {
    let input = "let x = 5";
    let expected_errors = vec![ParsingError(String::from(
        "Expected next token to be ';', got 'EOF' instead",
    ))];
    let (ast_nodes, errors) = collect_parsing_results(input);
    assert_eq!(ast_nodes.len(), 0);
    assert_eq!(errors, expected_errors);
}

#[test]
fn test_parser_returns_multiple_errors_with_parsed_nodes() {
    let input = "let = 5;
let x = 10;
let y 3;";
    let expected_errors = vec![
        ParsingError(String::from(
            "Expected next token to be 'IDENT', got '=' instead",
        )),
        ParsingError(String::from(
            "Expected next token to be '=', got '3' instead",
        )),
    ];
    let (ast_nodes, errors) = collect_parsing_results(input);
    assert_eq!(errors, expected_errors);
    assert_eq!(ast_nodes.len(), 1);
}

fn collect_parsing_results(input: &str) -> (Vec<Statement>, Vec<ParsingError>) {
    let mut errors = vec![];
    let ast_nodes: Vec<_> = input
        .ast_nodes()
        .filter_map(|node| node.map_err(|e| errors.push(e)).ok())
        .collect();
    (ast_nodes, errors)
}
