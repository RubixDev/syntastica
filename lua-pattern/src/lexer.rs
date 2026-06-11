use std::{mem, str::Chars};

use crate::{Class, Error, Result, Token};

pub(crate) struct Lexer<'src> {
    reader: Chars<'src>,
    curr_char: Option<char>,
    next_char: Option<char>,
    tokens: Vec<Token>,
}

impl<'src> Lexer<'src> {
    pub fn lex(pattern: &'src str) -> Result<Vec<Token>> {
        let mut lexer = Self {
            reader: pattern.chars(),
            curr_char: None,
            next_char: None,
            tokens: vec![],
        };
        // advance the lexer twice so that curr_char and next_char are populated
        lexer.next();
        lexer.next();
        lexer.read_pattern()?;
        Ok(lexer.tokens)
    }

    fn next(&mut self) {
        // swap the current and next char so that the old next is the new current
        mem::swap(&mut self.curr_char, &mut self.next_char);
        self.next_char = self.reader.next();
    }

    fn next_and_push(&mut self, tok: Token) {
        self.next();
        self.tokens.push(tok);
    }

    ////////////////////////

    fn read_pattern(&mut self) -> Result<()> {
        if let Some('^') = self.curr_char {
            self.next_and_push(Token::Start);
        }

        while let Some(curr) = self.curr_char {
            match curr {
                '(' => self.next_and_push(Token::LParen),
                ')' => self.next_and_push(Token::RParen),
                '$' if self.next_char.is_none() => self.next_and_push(Token::End),
                '$' => {
                    self.next_and_push(Token::Char('$'));
                    self.read_quantity()?;
                }
                '%' => {
                    self.next();
                    match self.curr_char {
                        Some('b') => {
                            self.next();
                            let open = self.curr_char.ok_or(Error::MissingCharsForBalanced)?;
                            self.next();
                            let close = self.curr_char.ok_or(Error::MissingCharsForBalanced)?;
                            self.next_and_push(Token::Balanced(open, close));
                        }
                        Some('f') => {
                            self.next();
                            if !matches!(self.curr_char, Some('[')) {
                                return Err(Error::MissingSetForFrontier);
                            }
                            self.tokens.push(Token::Frontier);
                            self.read_set()?;
                        }
                        Some(
                            group @ ('0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9'),
                        ) => self.next_and_push(Token::CaptureRef(group as u8 - b'0')),
                        _ => {
                            self.read_escape()?;
                            self.read_quantity()?;
                        }
                    }
                }
                '[' => {
                    self.read_set()?;
                    self.read_quantity()?;
                }
                '.' => {
                    self.next_and_push(Token::Any);
                    self.read_quantity()?;
                }
                char => {
                    self.next_and_push(Token::Char(char));
                    self.read_quantity()?;
                }
            }
        }

        Ok(())
    }

    fn read_quantity(&mut self) -> Result<()> {
        match self.curr_char {
            Some('+') => self.next_and_push(Token::OneOrMore),
            Some('-') => self.next_and_push(Token::ZeroOrMoreLazy),
            Some('*') => self.next_and_push(Token::ZeroOrMore),
            Some('?') => self.next_and_push(Token::ZeroOrOne),
            _ => {}
        }

        Ok(())
    }

    fn read_escape(&mut self) -> Result<()> {
        let curr = self.curr_char.ok_or(Error::UnfinishedEscape)?;

        match Class::try_from(curr) {
            Ok(class) => self.tokens.push(Token::Class(class)),
            Err(()) => self.tokens.push(Token::Escaped(curr)),
        }
        self.next();

        Ok(())
    }

    fn read_set(&mut self) -> Result<()> {
        debug_assert_eq!(self.curr_char, Some('['));
        self.next_and_push(Token::LBrack);

        if let Some('^') = self.curr_char {
            self.next_and_push(Token::Inverse);
        }

        loop {
            let curr = self.curr_char.ok_or(Error::UnclosedSet)?;

            if curr == '%' {
                self.next();
                self.read_escape()?;
            } else {
                self.next_and_push(Token::Char(curr));
            }

            if let Some(']') = self.curr_char {
                break;
            }
        }

        self.next_and_push(Token::RBrack);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn everything() {
        let input = r"^^charsq+w-e*r?.%.(%a%c%d%g%l%p%s%u%w%x%z%A)[asd]%f[^not]%1%b{}[%]a-z]$$";
        assert_eq!(
            Lexer::lex(input),
            Ok(vec![
                Token::Start,
                Token::Char('^'),
                Token::Char('c'),
                Token::Char('h'),
                Token::Char('a'),
                Token::Char('r'),
                Token::Char('s'),
                Token::Char('q'),
                Token::OneOrMore,
                Token::Char('w'),
                Token::ZeroOrMoreLazy,
                Token::Char('e'),
                Token::ZeroOrMore,
                Token::Char('r'),
                Token::ZeroOrOne,
                Token::Any,
                Token::Escaped('.'),
                Token::LParen,
                Token::Class(Class::Letters),
                Token::Class(Class::Controls),
                Token::Class(Class::Digits),
                Token::Class(Class::Printable),
                Token::Class(Class::Lowercase),
                Token::Class(Class::Punctuations),
                Token::Class(Class::Spaces),
                Token::Class(Class::Uppercase),
                Token::Class(Class::Alphanumerics),
                Token::Class(Class::Hexadecimals),
                Token::Class(Class::ZeroByte),
                Token::Class(Class::NotLetters),
                Token::RParen,
                Token::LBrack,
                Token::Char('a'),
                Token::Char('s'),
                Token::Char('d'),
                Token::RBrack,
                Token::Frontier,
                Token::LBrack,
                Token::Inverse,
                Token::Char('n'),
                Token::Char('o'),
                Token::Char('t'),
                Token::RBrack,
                Token::CaptureRef(1),
                Token::Balanced('{', '}'),
                Token::LBrack,
                Token::Escaped(']'),
                Token::Char('a'),
                Token::Char('-'),
                Token::Char('z'),
                Token::RBrack,
                Token::Char('$'),
                Token::End,
            ])
        )
    }
}
