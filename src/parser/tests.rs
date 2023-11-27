#![cfg(test)]

use crate::{
    lexer::token::Token,
    parser::{
        ast::{Boolean, Expression, Prefix, Statement},
        Parser,
    },
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
    let expected = vec![Statement::Let(
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
    let expected_errors = vec![ParsingError::UnexpectedToken(Token::Assign)];
    let (ast_nodes, errors) = collect_parsing_results(input);
    assert_eq!(ast_nodes.len(), 0);
    assert_eq!(errors, expected_errors);
}

#[test]
fn test_let_parse_error_if_no_assign() {
    let input = "let x 5;";
    let expected_errors = vec![ParsingError::UnexpectedToken(Token::Int(String::from("5")))];
    let (ast_nodes, errors) = collect_parsing_results(input);
    assert_eq!(ast_nodes.len(), 0);
    assert_eq!(errors, expected_errors);
}

#[test]
fn test_let_statement_parse_error_if_no_semicolon() {
    let input = "let x = 5";
    let expected_errors = vec![ParsingError::UnexpectedEof];
    let (ast_nodes, errors) = collect_parsing_results(input);
    assert_eq!(ast_nodes.len(), 0);
    assert_eq!(errors, expected_errors);
}

#[test]
fn test_let_statement_parse_error_if_no_rhs_expression() {
    let input = "let x =;
let y =";
    let expected_errors = vec![
        ParsingError::UnexpectedSemicolon,
        ParsingError::UnexpectedEof,
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
        ParsingError::UnexpectedSemicolon,
        ParsingError::UnexpectedSemicolon,
        ParsingError::UnexpectedToken(Token::Int(String::from("3"))),
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
    let expected_errors = vec![ParsingError::UnexpectedSemicolon];
    let (ast_nodes, errors) = collect_parsing_results(input);
    assert_eq!(ast_nodes.len(), 0);
    assert_eq!(errors, expected_errors);
}

#[test]
fn test_return_statement_parse_error_if_no_semicolon() {
    let input = "return 10
let x = 5;
";
    let expected_errors = vec![ParsingError::UnexpectedToken(Token::Let)];
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
        ParsingError::UnexpectedSemicolon,
        ParsingError::UnexpectedEof,
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

#[test]
fn test_operator_precedence_one() {
    let input = "-a * b"; // ((-a) * b)
    let expected = [Statement::Expression(Expression::Infix(
        Box::new(Expression::Prefix(
            Prefix::Minus,
            Box::new(Expression::Identifier(String::from("a"))),
        )),
        Infix::Multiply,
        Box::new(Expression::Identifier(String::from("b"))),
    ))];
    let (ast_nodes, _) = collect_parsing_results(input);
    assert_eq!(ast_nodes, expected);
}

#[test]
fn test_operator_precedence_two() {
    let input = "!-a"; // (!(-a))
    let expected = [Statement::Expression(Expression::Prefix(
        Prefix::Bang,
        Box::new(Expression::Prefix(
            Prefix::Minus,
            Box::new(Expression::Identifier(String::from("a"))),
        )),
    ))];
    let (ast_nodes, _) = collect_parsing_results(input);
    assert_eq!(ast_nodes, expected);
}

#[test]
fn test_operator_precedence_three() {
    let input = "a + b + c"; // ((a + b) + c)
    let expected = [Statement::Expression(Expression::Infix(
        Box::new(Expression::Infix(
            Box::new(Expression::Identifier(String::from("a"))),
            Infix::Plus,
            Box::new(Expression::Identifier(String::from("b"))),
        )),
        Infix::Plus,
        Box::new(Expression::Identifier(String::from("c"))),
    ))];
    let (ast_nodes, _) = collect_parsing_results(input);
    assert_eq!(ast_nodes, expected);
}

#[test]
fn test_operator_precedence_four() {
    let input = "a + b - c"; // ((a + b) - c)
    let expected = [Statement::Expression(Expression::Infix(
        Box::new(Expression::Infix(
            Box::new(Expression::Identifier(String::from("a"))),
            Infix::Plus,
            Box::new(Expression::Identifier(String::from("b"))),
        )),
        Infix::Minus,
        Box::new(Expression::Identifier(String::from("c"))),
    ))];
    let (ast_nodes, _) = collect_parsing_results(input);
    assert_eq!(ast_nodes, expected);
}

#[test]
fn test_operator_precedence_five() {
    let input = "a * b * c"; // ((a * b) * c)
    let expected = [Statement::Expression(Expression::Infix(
        Box::new(Expression::Infix(
            Box::new(Expression::Identifier(String::from("a"))),
            Infix::Multiply,
            Box::new(Expression::Identifier(String::from("b"))),
        )),
        Infix::Multiply,
        Box::new(Expression::Identifier(String::from("c"))),
    ))];
    let (ast_nodes, _) = collect_parsing_results(input);
    assert_eq!(ast_nodes, expected);
}

#[test]
fn test_operator_precedence_six() {
    let input = "a * b / c"; // ((a * b) / c)
    let expected = [Statement::Expression(Expression::Infix(
        Box::new(Expression::Infix(
            Box::new(Expression::Identifier(String::from("a"))),
            Infix::Multiply,
            Box::new(Expression::Identifier(String::from("b"))),
        )),
        Infix::Divide,
        Box::new(Expression::Identifier(String::from("c"))),
    ))];
    let (ast_nodes, _) = collect_parsing_results(input);
    assert_eq!(ast_nodes, expected);
}

#[test]
fn test_operator_precedence_seven() {
    let input = "a + b / c"; // (a + (b / c))
    let expected = [Statement::Expression(Expression::Infix(
        Box::new(Expression::Identifier(String::from("a"))),
        Infix::Plus,
        Box::new(Expression::Infix(
            Box::new(Expression::Identifier(String::from("b"))),
            Infix::Divide,
            Box::new(Expression::Identifier(String::from("c"))),
        )),
    ))];
    let (ast_nodes, _) = collect_parsing_results(input);
    assert_eq!(ast_nodes, expected);
}

#[test]
fn test_operator_precedence_eight() {
    let input = "a + b * c + d / e - f"; // (((a + (b * c)) + (d / e)) - f)
    let expected = [Statement::Expression(Expression::Infix(
        Box::new(Expression::Infix(
            Box::new(Expression::Infix(
                Box::new(Expression::Identifier(String::from("a"))),
                Infix::Plus,
                Box::new(Expression::Infix(
                    Box::new(Expression::Identifier(String::from("b"))),
                    Infix::Multiply,
                    Box::new(Expression::Identifier(String::from("c"))),
                )),
            )),
            Infix::Plus,
            Box::new(Expression::Infix(
                Box::new(Expression::Identifier(String::from("d"))),
                Infix::Divide,
                Box::new(Expression::Identifier(String::from("e"))),
            )),
        )),
        Infix::Minus,
        Box::new(Expression::Identifier(String::from("f"))),
    ))];
    let (ast_nodes, _) = collect_parsing_results(input);
    assert_eq!(ast_nodes, expected);
}

#[test]
fn test_operator_precedence_nine() {
    let input = "3 + 4; -5 * 5"; // (3 + 4)((-5) * 5)
    let expected = [
        Statement::Expression(Expression::Infix(
            Box::new(Expression::Integer(3)),
            Infix::Plus,
            Box::new(Expression::Integer(4)),
        )),
        Statement::Expression(Expression::Infix(
            Box::new(Expression::Prefix(
                Prefix::Minus,
                Box::new(Expression::Integer(5)),
            )),
            Infix::Multiply,
            Box::new(Expression::Integer(5)),
        )),
    ];
    let (ast_nodes, _) = collect_parsing_results(input);
    assert_eq!(ast_nodes, expected);
}

#[test]
fn test_operator_precedence_ten() {
    let input = "5 > 4 == 3 < 4"; // ((5 > 4) == (3 < 4))
    let expected = [Statement::Expression(Expression::Infix(
        Box::new(Expression::Infix(
            Box::new(Expression::Integer(5)),
            Infix::GreaterThan,
            Box::new(Expression::Integer(4)),
        )),
        Infix::Equal,
        Box::new(Expression::Infix(
            Box::new(Expression::Integer(3)),
            Infix::LessThan,
            Box::new(Expression::Integer(4)),
        )),
    ))];
    let (ast_nodes, _) = collect_parsing_results(input);
    assert_eq!(ast_nodes, expected);
}

#[test]
fn test_operator_precedence_eleven() {
    let input = "5 < 4 != 3 > 4"; // ((5 < 4) != (3 > 4))
    let expected = [Statement::Expression(Expression::Infix(
        Box::new(Expression::Infix(
            Box::new(Expression::Integer(5)),
            Infix::LessThan,
            Box::new(Expression::Integer(4)),
        )),
        Infix::NotEqual,
        Box::new(Expression::Infix(
            Box::new(Expression::Integer(3)),
            Infix::GreaterThan,
            Box::new(Expression::Integer(4)),
        )),
    ))];
    let (ast_nodes, _) = collect_parsing_results(input);
    assert_eq!(ast_nodes, expected);
}

#[test]
fn test_operator_precedence_twelve() {
    let input = "3 + 4 * 5 == 3 * 1 + 4 * 5"; // ((3 + (4 * 5)) == ((3 * 1) + (4 * 5)))
    let expected = [Statement::Expression(Expression::Infix(
        Box::new(Expression::Infix(
            Box::new(Expression::Integer(3)),
            Infix::Plus,
            Box::new(Expression::Infix(
                Box::new(Expression::Integer(4)),
                Infix::Multiply,
                Box::new(Expression::Integer(5)),
            )),
        )),
        Infix::Equal,
        Box::new(Expression::Infix(
            Box::new(Expression::Infix(
                Box::new(Expression::Integer(3)),
                Infix::Multiply,
                Box::new(Expression::Integer(1)),
            )),
            Infix::Plus,
            Box::new(Expression::Infix(
                Box::new(Expression::Integer(4)),
                Infix::Multiply,
                Box::new(Expression::Integer(5)),
            )),
        )),
    ))];
    let (ast_nodes, _) = collect_parsing_results(input);
    assert_eq!(ast_nodes, expected);
}

#[test]
fn test_prefix_expression_parse_error_if_invalid_prefix() {
    let input = "+4;";
    let expected_errors = vec![ParsingError::InvalidPrefixOperator(Token::Plus)];
    let (_, errors) = collect_parsing_results(input);
    assert_eq!(errors, expected_errors);
}

#[test]
fn test_expression_parse_error_if_invalid_prefix_placement() {
    let input = "6!";
    let expected_errors = vec![ParsingError::UnexpectedEof];
    let (nodes, errors) = collect_parsing_results(input);
    println!("{nodes:?}");
    assert_eq!(errors, expected_errors);
}

#[test]
fn test_adjacent_expressions_parse() {
    let input = "4!4;5(5+3);";
    let expected = vec![
        Statement::Expression(Expression::Integer(4)),
        Statement::Expression(Expression::Prefix(
            Prefix::Bang,
            Box::new(Expression::Integer(4)),
        )),
        Statement::Expression(Expression::Integer(5)),
        Statement::Expression(Expression::Infix(
            Box::new(Expression::Integer(5)),
            Infix::Plus,
            Box::new(Expression::Integer(3)),
        )),
    ];
    let (ast_nodes, _) = collect_parsing_results(input);
    assert_eq!(ast_nodes, expected);
}

#[test]
fn test_infix_expression_parse_error_if_no_rhs_expression() {
    let input = "4 + ;5 +";
    let expected_errors = vec![
        ParsingError::UnexpectedSemicolon,
        ParsingError::UnexpectedEof,
    ];
    let (_, errors) = collect_parsing_results(input);
    assert_eq!(errors, expected_errors);
}

#[test]
fn test_boolean_expression() {
    let input = "let a = true; return false; true == false";
    let expected = vec![
        Statement::Let(
            Expression::Identifier(String::from("a")),
            Expression::Boolean(Boolean::True),
        ),
        Statement::Return(Expression::Boolean(Boolean::False)),
        Statement::Expression(Expression::Infix(
            Box::new(Expression::Boolean(Boolean::True)),
            Infix::Equal,
            Box::new(Expression::Boolean(Boolean::False)),
        )),
    ];
    let (ast_nodes, _) = collect_parsing_results(input);
    assert_eq!(ast_nodes, expected);
}

#[test]
fn test_operator_precedence_thirteen() {
    let input = "3 > 5 == false"; // ((3 > 5) == false)
    let expected = vec![Statement::Expression(Expression::Infix(
        Box::new(Expression::Infix(
            Box::new(Expression::Integer(3)),
            Infix::GreaterThan,
            Box::new(Expression::Integer(5)),
        )),
        Infix::Equal,
        Box::new(Expression::Boolean(Boolean::False)),
    ))];
    let (ast_nodes, _) = collect_parsing_results(input);
    assert_eq!(ast_nodes, expected);
}

#[test]
fn test_precedence_promotion_with_parentheses_one() {
    let input = "(5 + 5) * 2;";
    let expected = vec![Statement::Expression(Expression::Infix(
        Box::new(Expression::Infix(
            Box::new(Expression::Integer(5)),
            Infix::Plus,
            Box::new(Expression::Integer(5)),
        )),
        Infix::Multiply,
        Box::new(Expression::Integer(2)),
    ))];
    let (ast_nodes, _) = collect_parsing_results(input);
    assert_eq!(ast_nodes, expected);
}

#[test]
fn test_precedence_promotion_with_parentheses_two() {
    let input = "-(5 + 5)";
    let expected = vec![Statement::Expression(Expression::Prefix(
        Prefix::Minus,
        Box::new(Expression::Infix(
            Box::new(Expression::Integer(5)),
            Infix::Plus,
            Box::new(Expression::Integer(5)),
        )),
    ))];
    let (ast_nodes, _) = collect_parsing_results(input);
    assert_eq!(ast_nodes, expected);
}

#[test]
fn test_precedence_promotion_with_parentheses_three() {
    let input = "!(true == false)";
    let expected = vec![Statement::Expression(Expression::Prefix(
        Prefix::Bang,
        Box::new(Expression::Infix(
            Box::new(Expression::Boolean(Boolean::True)),
            Infix::Equal,
            Box::new(Expression::Boolean(Boolean::False)),
        )),
    ))];
    let (ast_nodes, _) = collect_parsing_results(input);
    assert_eq!(ast_nodes, expected);
}