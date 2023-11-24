use std::iter::Peekable;

use self::ast::Statement;
use crate::lexer::{Lexer, TokensIter};

mod ast;

pub struct NodesIter<'a> {
    iter: Peekable<TokensIter<'a>>,
}

impl<'a> Iterator for NodesIter<'a> {
    type Item = Statement;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

pub trait Parser {
    fn ast_nodes(&self) -> NodesIter;
}

impl<L: Lexer> Parser for L {
    fn ast_nodes(&self) -> NodesIter {
        NodesIter {
            iter: self.tokens().peekable(),
        }
    }
}
