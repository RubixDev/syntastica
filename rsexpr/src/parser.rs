use std::{unreachable, vec};

use crate::{lex::Token, Error, ParenKind, Sexpr};

pub(crate) struct Parser<'src> {
    iter: vec::IntoIter<Token<'src>>,
    curr_tok: Option<Token<'src>>,
    errors: Vec<Error>,
}

impl<'src> Parser<'src> {
    pub fn parse(tokens: Vec<Token<'src>>) -> (Vec<Sexpr<'src>>, Vec<Error>) {
        let mut iter = tokens.into_iter();
        let mut parser = Self {
            curr_tok: iter.next(),
            iter,
            errors: vec![],
        };

        let mut sexprs = vec![];
        while let Some(sexpr) = parser.sexpr() {
            sexprs.push(sexpr);
        }

        if let Some(tok) = parser.curr_tok {
            let paren_kind = match tok {
                Token::RParen => ParenKind::Round,
                Token::RBrack => ParenKind::Square,
                _ => unreachable!("`sexpr` function only returns `None` for above token types"),
            };
            parser.errors.push(Error::ExtraClosingParen(paren_kind));
        }

        (sexprs, parser.errors)
    }

    fn next(&mut self) {
        self.curr_tok = self.iter.next();
    }

    fn expect_closing(&mut self, expected: ParenKind) {
        match &self.curr_tok {
            Some(tok) if tok == &Token::from(&expected) => self.next(),
            Some(_) | None => self.errors.push(Error::MissingClosingParen(expected)),
        }
    }

    fn sexpr(&mut self) -> Option<Sexpr<'src>> {
        match self.curr_tok.take()? {
            Token::LParen => Some(self.list()),
            Token::LBrack => Some(self.group()),
            Token::String(string) => {
                self.next();
                Some(Sexpr::String(string))
            }
            Token::Atom(atom) => {
                self.next();
                Some(Sexpr::Atom(atom))
            }
            tok @ (Token::RParen | Token::RBrack) => {
                self.curr_tok = Some(tok);
                None
            }
        }
    }

    fn list(&mut self) -> Sexpr<'src> {
        self.next(); // skip opening paren

        let mut children = vec![];
        while let Some(child) = self.sexpr() {
            children.push(child);
        }

        self.expect_closing(ParenKind::Round);

        Sexpr::List(children)
    }

    fn group(&mut self) -> Sexpr<'src> {
        self.next(); // skip opening bracket

        let mut children = vec![];
        while let Some(child) = self.sexpr() {
            children.push(child);
        }

        self.expect_closing(ParenKind::Square);

        Sexpr::Group(children)
    }
}
