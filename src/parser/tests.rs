#![cfg(test)]

use crate::parser::{Parser, ParsingError};
use crate::{
    lexer::token::Token,
    parser::ast::{Expression, Statement},
};

use super::ast::{Infix, Prefix, Program};

#[test]
fn test_let_statement() {
    let input = "let x = 5;";
    let expected = Program(vec![Statement::Let(
        Expression::Identifier(String::from("x")),
        Expression::Integer(5),
    )]);
    let program = Parser::parse_program(input).ok().unwrap();
    assert_eq!(program, expected);
}

#[test]
fn test_let_parse_error_if_no_identifier() {
    let input = "let = 5;";
    let expected_errors = vec![ParsingError::UnexpectedToken(Token::Assign)];
    let errors = Parser::parse_program(input).err().unwrap();
    assert_eq!(errors, expected_errors);
}

#[test]
fn test_let_parse_error_if_no_assign() {
    let input = "let x 5;";
    let expected_errors = vec![ParsingError::UnexpectedToken(Token::Int(String::from("5")))];
    let errors = Parser::parse_program(input).err().unwrap();
    assert_eq!(errors, expected_errors);
}

#[test]
fn test_let_statement_parse_error_if_no_semicolon() {
    let input = "let x = 5";
    let expected_errors = vec![ParsingError::UnexpectedEof];
    let errors = Parser::parse_program(input).err().unwrap();
    assert_eq!(errors, expected_errors);
}

#[test]
fn test_let_statement_parse_error_if_no_rhs_expression() {
    let input = "let x =;let y =";
    let expected_errors = vec![
        ParsingError::UnexpectedSemicolon,
        ParsingError::UnexpectedEof,
    ];
    let errors = Parser::parse_program(input).err().unwrap();
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
    let errors = Parser::parse_program(input).err().unwrap();
    assert_eq!(errors, expected_errors);
}

#[test]
fn test_return_statement() {
    let input = "return 10;";
    let expected = Program(vec![Statement::Return(Expression::Integer(10))]);
    let program = Parser::parse_program(input).ok().unwrap();
    assert_eq!(program, expected);
}

#[test]
fn test_return_statement_parse_error_if_no_expression() {
    let input = "return ;";
    let expected_errors = vec![ParsingError::UnexpectedSemicolon];
    let errors = Parser::parse_program(input).err().unwrap();
    assert_eq!(errors, expected_errors);
}

#[test]
fn test_return_statement_parse_error_if_no_semicolon() {
    let input = "return 10
let x = 5;
";
    let expected_errors = vec![ParsingError::UnexpectedToken(Token::Let)];
    let errors = Parser::parse_program(input).err().unwrap();
    assert_eq!(errors, expected_errors);
}

#[test]
fn test_parses_multiple_statements() {
    let input = "let x = 5;
return 10;
";
    let expected = Program(vec![
        Statement::Let(
            Expression::Identifier(String::from("x")),
            Expression::Integer(5),
        ),
        Statement::Return(Expression::Integer(10)),
    ]);
    let program = Parser::parse_program(input).ok().unwrap();
    assert_eq!(program, expected);
}

#[test]
fn test_identifier_expression_statement() {
    let input = "foo;
return 10;
";
    let expected = Program(vec![
        Statement::Expression(Expression::Identifier(String::from("foo"))),
        Statement::Return(Expression::Integer(10)),
    ]);
    let program = Parser::parse_program(input).ok().unwrap();
    assert_eq!(program, expected);
}

#[test]
fn test_expression_statement_parses_without_semicolon() {
    let input = "return 10;
foo
";
    let expected = Program(vec![
        Statement::Return(Expression::Integer(10)),
        Statement::Expression(Expression::Identifier(String::from("foo"))),
    ]);
    let program = Parser::parse_program(input).ok().unwrap();
    assert_eq!(program, expected);
}

#[test]
fn test_integer_expression_statement() {
    let input = "5;";
    let expected = Program(vec![Statement::Expression(Expression::Integer(5))]);
    let program = Parser::parse_program(input).ok().unwrap();
    assert_eq!(program, expected);
}

#[test]
fn test_parsing_bang_prefix_expressions() {
    let input = "let x = !5;";
    let expected = Program(vec![Statement::Let(
        Expression::Identifier(String::from("x")),
        Expression::Prefix(Prefix::Bang, Box::new(Expression::Integer(5))),
    )]);
    let program = Parser::parse_program(input).ok().unwrap();
    assert_eq!(program, expected);
}

#[test]
fn test_prefix_expressions() {
    let input = "let x = -5;
let y = !10;
";
    let expected = Program(vec![
        Statement::Let(
            Expression::Identifier(String::from("x")),
            Expression::Prefix(Prefix::Minus, Box::new(Expression::Integer(5))),
        ),
        Statement::Let(
            Expression::Identifier(String::from("y")),
            Expression::Prefix(Prefix::Bang, Box::new(Expression::Integer(10))),
        ),
    ]);
    let program = Parser::parse_program(input).ok().unwrap();
    assert_eq!(program, expected);
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
    let errors = Parser::parse_program(input).err().unwrap();
    assert_eq!(errors, expected_errors);
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
    let expected = Program(vec![
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
    ]);
    let program = Parser::parse_program(input).ok().unwrap();
    assert_eq!(program, expected);
}

#[test]
fn test_operator_precedence_one() {
    let input = "-a * b"; // ((-a) * b)
    let expected = Program(vec![Statement::Expression(Expression::Infix(
        Box::new(Expression::Prefix(
            Prefix::Minus,
            Box::new(Expression::Identifier(String::from("a"))),
        )),
        Infix::Multiply,
        Box::new(Expression::Identifier(String::from("b"))),
    ))]);
    let program = Parser::parse_program(input).ok().unwrap();
    assert_eq!(program, expected);
}

#[test]
fn test_operator_precedence_two() {
    let input = "!-a"; // (!(-a))
    let expected = Program(vec![Statement::Expression(Expression::Prefix(
        Prefix::Bang,
        Box::new(Expression::Prefix(
            Prefix::Minus,
            Box::new(Expression::Identifier(String::from("a"))),
        )),
    ))]);
    let program = Parser::parse_program(input).ok().unwrap();
    assert_eq!(program, expected);
}

#[test]
fn test_operator_precedence_three() {
    let input = "a + b + c"; // ((a + b) + c)
    let expected = Program(vec![Statement::Expression(Expression::Infix(
        Box::new(Expression::Infix(
            Box::new(Expression::Identifier(String::from("a"))),
            Infix::Plus,
            Box::new(Expression::Identifier(String::from("b"))),
        )),
        Infix::Plus,
        Box::new(Expression::Identifier(String::from("c"))),
    ))]);
    let program = Parser::parse_program(input).ok().unwrap();
    assert_eq!(program, expected);
}

#[test]
fn test_operator_precedence_four() {
    let input = "a + b - c"; // ((a + b) - c)
    let expected = Program(vec![Statement::Expression(Expression::Infix(
        Box::new(Expression::Infix(
            Box::new(Expression::Identifier(String::from("a"))),
            Infix::Plus,
            Box::new(Expression::Identifier(String::from("b"))),
        )),
        Infix::Minus,
        Box::new(Expression::Identifier(String::from("c"))),
    ))]);
    let program = Parser::parse_program(input).ok().unwrap();
    assert_eq!(program, expected);
}

#[test]
fn test_operator_precedence_five() {
    let input = "a * b * c"; // ((a * b) * c)
    let expected = Program(vec![Statement::Expression(Expression::Infix(
        Box::new(Expression::Infix(
            Box::new(Expression::Identifier(String::from("a"))),
            Infix::Multiply,
            Box::new(Expression::Identifier(String::from("b"))),
        )),
        Infix::Multiply,
        Box::new(Expression::Identifier(String::from("c"))),
    ))]);
    let program = Parser::parse_program(input).ok().unwrap();
    assert_eq!(program, expected);
}

#[test]
fn test_operator_precedence_six() {
    let input = "a * b / c"; // ((a * b) / c)
    let expected = Program(vec![Statement::Expression(Expression::Infix(
        Box::new(Expression::Infix(
            Box::new(Expression::Identifier(String::from("a"))),
            Infix::Multiply,
            Box::new(Expression::Identifier(String::from("b"))),
        )),
        Infix::Divide,
        Box::new(Expression::Identifier(String::from("c"))),
    ))]);
    let program = Parser::parse_program(input).ok().unwrap();
    assert_eq!(program, expected);
}

#[test]
fn test_operator_precedence_seven() {
    let input = "a + b / c"; // (a + (b / c))
    let expected = Program(vec![Statement::Expression(Expression::Infix(
        Box::new(Expression::Identifier(String::from("a"))),
        Infix::Plus,
        Box::new(Expression::Infix(
            Box::new(Expression::Identifier(String::from("b"))),
            Infix::Divide,
            Box::new(Expression::Identifier(String::from("c"))),
        )),
    ))]);
    let program = Parser::parse_program(input).ok().unwrap();
    assert_eq!(program, expected);
}

#[test]
fn test_operator_precedence_eight() {
    let input = "a + b * c + d / e - f"; // (((a + (b * c)) + (d / e)) - f)
    let expected = Program(vec![Statement::Expression(Expression::Infix(
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
    ))]);
    let program = Parser::parse_program(input).ok().unwrap();
    assert_eq!(program, expected);
}

#[test]
fn test_operator_precedence_nine() {
    let input = "3 + 4; -5 * 5"; // (3 + 4)((-5) * 5)
    let expected = Program(vec![
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
    ]);
    let program = Parser::parse_program(input).ok().unwrap();
    assert_eq!(program, expected);
}

#[test]
fn test_operator_precedence_ten() {
    let input = "5 > 4 == 3 < 4"; // ((5 > 4) == (3 < 4))
    let expected = Program(vec![Statement::Expression(Expression::Infix(
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
    ))]);
    let program = Parser::parse_program(input).ok().unwrap();
    assert_eq!(program, expected);
}

#[test]
fn test_operator_precedence_eleven() {
    let input = "5 < 4 != 3 > 4"; // ((5 < 4) != (3 > 4))
    let expected = Program(vec![Statement::Expression(Expression::Infix(
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
    ))]);
    let program = Parser::parse_program(input).ok().unwrap();
    assert_eq!(program, expected);
}

#[test]
fn test_operator_precedence_twelve() {
    let input = "3 + 4 * 5 == 3 * 1 + 4 * 5"; // ((3 + (4 * 5)) == ((3 * 1) + (4 * 5)))
    let expected = Program(vec![Statement::Expression(Expression::Infix(
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
    ))]);
    let program = Parser::parse_program(input).ok().unwrap();
    assert_eq!(program, expected);
}

#[test]
fn test_prefix_expression_parse_error_if_invalid_prefix() {
    let input = "+4;";
    let expected_errors = vec![ParsingError::InvalidPrefixOperator(Token::Plus)];
    let errors = Parser::parse_program(input).err().unwrap();
    assert_eq!(errors, expected_errors);
}

#[test]
fn test_expression_parse_error_if_invalid_prefix_placement() {
    let input = "6!";
    let expected_errors = vec![ParsingError::UnexpectedEof];
    let errors = Parser::parse_program(input).err().unwrap();
    assert_eq!(errors, expected_errors);
}

#[test]
fn test_adjacent_expressions_parse() {
    let input = "4!4;";
    let expected = Program(vec![
        Statement::Expression(Expression::Integer(4)),
        Statement::Expression(Expression::Prefix(
            Prefix::Bang,
            Box::new(Expression::Integer(4)),
        )),
    ]);
    let program = Parser::parse_program(input).ok().unwrap();
    assert_eq!(program, expected);
}

#[test]
fn test_infix_expression_parse_error_if_no_rhs_expression() {
    let input = "4 + ;5 +";
    let expected_errors = vec![
        ParsingError::UnexpectedSemicolon,
        ParsingError::UnexpectedEof,
    ];
    let errors = Parser::parse_program(input).err().unwrap();
    assert_eq!(errors, expected_errors);
}

#[test]
fn test_boolean_expression() {
    let input = "let a = true; return false; true == false";
    let expected = Program(vec![
        Statement::Let(
            Expression::Identifier(String::from("a")),
            Expression::Boolean(true),
        ),
        Statement::Return(Expression::Boolean(false)),
        Statement::Expression(Expression::Infix(
            Box::new(Expression::Boolean(true)),
            Infix::Equal,
            Box::new(Expression::Boolean(false)),
        )),
    ]);
    let program = Parser::parse_program(input).ok().unwrap();
    assert_eq!(program, expected);
}

#[test]
fn test_operator_precedence_thirteen() {
    let input = "3 > 5 == false"; // ((3 > 5) == false)
    let expected = Program(vec![Statement::Expression(Expression::Infix(
        Box::new(Expression::Infix(
            Box::new(Expression::Integer(3)),
            Infix::GreaterThan,
            Box::new(Expression::Integer(5)),
        )),
        Infix::Equal,
        Box::new(Expression::Boolean(false)),
    ))]);
    let program = Parser::parse_program(input).ok().unwrap();
    assert_eq!(program, expected);
}

#[test]
fn test_precedence_promotion_with_parentheses_one() {
    let input = "(5 + 5) * 2;";
    let expected = Program(vec![Statement::Expression(Expression::Infix(
        Box::new(Expression::Infix(
            Box::new(Expression::Integer(5)),
            Infix::Plus,
            Box::new(Expression::Integer(5)),
        )),
        Infix::Multiply,
        Box::new(Expression::Integer(2)),
    ))]);
    let program = Parser::parse_program(input).ok().unwrap();
    assert_eq!(program, expected);
}

#[test]
fn test_precedence_promotion_with_parentheses_two() {
    let input = "-(5 + 5)";
    let expected = Program(vec![Statement::Expression(Expression::Prefix(
        Prefix::Minus,
        Box::new(Expression::Infix(
            Box::new(Expression::Integer(5)),
            Infix::Plus,
            Box::new(Expression::Integer(5)),
        )),
    ))]);
    let program = Parser::parse_program(input).ok().unwrap();
    assert_eq!(program, expected);
}

#[test]
fn test_precedence_promotion_with_parentheses_three() {
    let input = "!(true == false)";
    let expected = Program(vec![Statement::Expression(Expression::Prefix(
        Prefix::Bang,
        Box::new(Expression::Infix(
            Box::new(Expression::Boolean(true)),
            Infix::Equal,
            Box::new(Expression::Boolean(false)),
        )),
    ))]);
    let program = Parser::parse_program(input).ok().unwrap();
    assert_eq!(program, expected);
}

#[test]
fn test_if_expression() {
    let input = "if (x < y) { x }";
    let expected = Program(vec![Statement::Expression(Expression::If(
        Box::new(Expression::Infix(
            Box::new(Expression::Identifier(String::from("x"))),
            Infix::LessThan,
            Box::new(Expression::Identifier(String::from("y"))),
        )),
        Box::new(Statement::BlockStatement(vec![Statement::Expression(
            Expression::Identifier(String::from("x")),
        )])),
        None,
    ))]);
    let program = Parser::parse_program(input).ok().unwrap();
    assert_eq!(program, expected);
}

#[test]
fn test_if_expression_with_else() {
    let input = "if (x < y) { x } else { y }";
    let expected = Program(vec![Statement::Expression(Expression::If(
        Box::new(Expression::Infix(
            Box::new(Expression::Identifier(String::from("x"))),
            Infix::LessThan,
            Box::new(Expression::Identifier(String::from("y"))),
        )),
        Box::new(Statement::BlockStatement(vec![Statement::Expression(
            Expression::Identifier(String::from("x")),
        )])),
        Some(Box::new(Statement::BlockStatement(vec![
            Statement::Expression(Expression::Identifier(String::from("y"))),
        ]))),
    ))]);
    let program = Parser::parse_program(input).ok().unwrap();
    assert_eq!(program, expected);
}

#[test]
fn test_if_expression_error_if_missing_brace() {
    let input = "if (x < y) { x  else { y }";
    let expected_errors = vec![ParsingError::InvalidPrefixOperator(Token::Else)];
    let errors = Parser::parse_program(input).err().unwrap();
    assert_eq!(errors, expected_errors);
}

#[test]
fn test_function_literal() {
    let input = "fn(x, y) { x + y; };";
    let expected = Program(vec![Statement::Expression(Expression::Function(
        vec![
            Expression::Identifier(String::from("x")),
            Expression::Identifier(String::from("y")),
        ],
        Box::new(Statement::BlockStatement(vec![Statement::Expression(
            Expression::Infix(
                Box::new(Expression::Identifier(String::from("x"))),
                Infix::Plus,
                Box::new(Expression::Identifier(String::from("y"))),
            ),
        )])),
    ))]);
    let program = Parser::parse_program(input).ok().unwrap();
    assert_eq!(program, expected);
}

#[test]
fn test_function_literal_no_parameters() {
    let input = "fn() { 1 };";
    let expected = Program(vec![Statement::Expression(Expression::Function(
        vec![],
        Box::new(Statement::BlockStatement(vec![Statement::Expression(
            Expression::Integer(1),
        )])),
    ))]);
    let program = Parser::parse_program(input).ok().unwrap();
    assert_eq!(program, expected);
}

#[test]
fn test_function_literal_error_if_missing_brace() {
    let input = "fn(x, y) { x + y; ";
    let expected_errors = vec![ParsingError::UnexpectedEof];
    let errors = Parser::parse_program(input).err().unwrap();
    assert_eq!(errors, expected_errors);
}

#[test]
fn test_function_literal_error_if_misplaced_comma() {
    let input = "fn(x, y,) { x + y }";
    let expected_errors = vec![ParsingError::UnexpectedToken(Token::Rparen)];
    let errors = Parser::parse_program(input).err().unwrap();
    assert_eq!(errors, expected_errors);
}

#[test]
fn test_block_statement_with_multiple_statements() {
    let input = "if (x < y) { x; x + y; 5 }";
    let expected = Program(vec![Statement::Expression(Expression::If(
        Box::new(Expression::Infix(
            Box::new(Expression::Identifier(String::from("x"))),
            Infix::LessThan,
            Box::new(Expression::Identifier(String::from("y"))),
        )),
        Box::new(Statement::BlockStatement(vec![
            Statement::Expression(Expression::Identifier(String::from("x"))),
            Statement::Expression(Expression::Infix(
                Box::new(Expression::Identifier(String::from("x"))),
                Infix::Plus,
                Box::new(Expression::Identifier(String::from("y"))),
            )),
            Statement::Expression(Expression::Integer(5)),
        ])),
        None,
    ))]);
    let program = Parser::parse_program(input).ok().unwrap();
    assert_eq!(program, expected);
}

#[test]
fn test_call_expression_basic() {
    let input = "add(2, 3);";
    let expected = Program(vec![Statement::Expression(Expression::Call(
        Box::new(Expression::Identifier(String::from("add"))),
        vec![Expression::Integer(2), Expression::Integer(3)],
    ))]);
    let program = Parser::parse_program(input).ok().unwrap();
    assert_eq!(program, expected);
}

#[test]
fn test_call_expression_no_arguments() {
    let input = "add();";
    let expected = Program(vec![Statement::Expression(Expression::Call(
        Box::new(Expression::Identifier(String::from("add"))),
        vec![],
    ))]);
    let program = Parser::parse_program(input).ok().unwrap();
    assert_eq!(program, expected);
}

#[test]
fn test_call_expression_one_arguments() {
    let input = "add(1);";
    let expected = Program(vec![Statement::Expression(Expression::Call(
        Box::new(Expression::Identifier(String::from("add"))),
        vec![Expression::Integer(1)],
    ))]);
    let program = Parser::parse_program(input).ok().unwrap();
    assert_eq!(program, expected);
}

#[test]
fn test_call_expression_inlined_function() {
    let input = "fn(x, y) { x + y }(2, 3);";
    let expected = Program(vec![Statement::Expression(Expression::Call(
        Box::new(Expression::Function(
            vec![
                Expression::Identifier(String::from("x")),
                Expression::Identifier(String::from("y")),
            ],
            Box::new(Statement::BlockStatement(vec![Statement::Expression(
                Expression::Infix(
                    Box::new(Expression::Identifier(String::from("x"))),
                    Infix::Plus,
                    Box::new(Expression::Identifier(String::from("y"))),
                ),
            )])),
        )),
        vec![Expression::Integer(2), Expression::Integer(3)],
    ))]);
    let program = Parser::parse_program(input).ok().unwrap();
    assert_eq!(program, expected);
}

#[test]
fn test_call_expression_error_if_missing_paren() {
    let input = "add(2, 3";
    let expected_errors = vec![ParsingError::UnexpectedEof];
    let errors = Parser::parse_program(input).err().unwrap();
    assert_eq!(errors, expected_errors);
}

#[test]
fn test_call_expression_error_if_extra_comma() {
    let input = "add(2, 3,)";
    let expected_errors = vec![ParsingError::InvalidPrefixOperator(Token::Rparen)];
    let errors = Parser::parse_program(input).err().unwrap();
    assert_eq!(errors, expected_errors);
}

#[test]
fn test_string_expression() {
    let input = "\"hello world\"";
    let expected = Program(vec![Statement::Expression(Expression::String(
        String::from("hello world"),
    ))]);
    let program = Parser::parse_program(input).ok().unwrap();
    assert_eq!(program, expected);
}

#[test]
fn test_array_expression() {
    let input = "[1, 2, 3 * 4, 1 + 1]";
    let expected = Program(vec![Statement::Expression(Expression::Array(vec![
        Expression::Integer(1),
        Expression::Integer(2),
        Expression::Infix(
            Box::new(Expression::Integer(3)),
            Infix::Multiply,
            Box::new(Expression::Integer(4)),
        ),
        Expression::Infix(
            Box::new(Expression::Integer(1)),
            Infix::Plus,
            Box::new(Expression::Integer(1)),
        ),
    ]))]);
    let program = Parser::parse_program(input).ok().unwrap();
    assert_eq!(program, expected);
}

#[test]
fn test_array_expression_empty() {
    let input = "[]";
    let expected = Program(vec![Statement::Expression(Expression::Array(vec![]))]);
    let program = Parser::parse_program(input).ok().unwrap();
    assert_eq!(program, expected);
}

#[test]
fn test_array_expression_errors() {
    let input = "[1, 2, 3,]; [1, 2, 3";
    let expected_errors = vec![
        ParsingError::InvalidPrefixOperator(Token::Rbracket),
        ParsingError::UnexpectedEof,
    ];
    let errors = Parser::parse_program(input).err().unwrap();
    assert_eq!(errors, expected_errors);
}

#[test]
fn test_array_index_operator_expression() {
    let input = "myArray[1 + 1]; [1, 2][0];";
    let expected = Program(vec![
        Statement::Expression(Expression::Index(
            Box::new(Expression::Identifier(String::from("myArray"))),
            Box::new(Expression::Infix(
                Box::new(Expression::Integer(1)),
                Infix::Plus,
                Box::new(Expression::Integer(1)),
            )),
        )),
        Statement::Expression(Expression::Index(
            Box::new(Expression::Array(vec![
                Expression::Integer(1),
                Expression::Integer(2),
            ])),
            Box::new(Expression::Integer(0)),
        )),
    ]);
    let program = Parser::parse_program(input).ok().unwrap();
    assert_eq!(program, expected);
}

#[test]
fn test_array_index_operator_precedence() {
    let input = "a * myArray[1 + 1] + b"; // ((a * (myArray[(1 + 1)])) + b)
    let expected = Program(vec![Statement::Expression(Expression::Infix(
        Box::new(Expression::Infix(
            Box::new(Expression::Identifier(String::from("a"))),
            Infix::Multiply,
            Box::new(Expression::Index(
                Box::new(Expression::Identifier(String::from("myArray"))),
                Box::new(Expression::Infix(
                    Box::new(Expression::Integer(1)),
                    Infix::Plus,
                    Box::new(Expression::Integer(1)),
                )),
            )),
        )),
        Infix::Plus,
        Box::new(Expression::Identifier(String::from("b"))),
    ))]);
    let program = Parser::parse_program(input).ok().unwrap();
    assert_eq!(program, expected);
}

#[test]
fn test_index_expression_errors() {
    let input = "myArray[1; myArray[1, 2]";
    let expected_errors = vec![
        ParsingError::UnexpectedSemicolon,
        ParsingError::UnexpectedToken(Token::Comma),
    ];
    let errors = Parser::parse_program(input).err().unwrap();
    assert_eq!(errors, expected_errors);
}
